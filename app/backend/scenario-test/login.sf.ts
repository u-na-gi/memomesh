import { LoginRequest } from "./types/src/api/auth/login/login.ts"; // Import types if needed

import { registerUser } from "./register-user.sf.ts";
import { RegisterRequest } from "./types/src/api/user/register.ts";
import { ScenarioFlow } from "scenario-flow";

export const login = new ScenarioFlow("Login Flow", registerUser).step(
  "Login User",
  async (ctx) => {
    const payload = ctx.getContext("user") as RegisterRequest;
    const req: LoginRequest = {
      username: payload.username,
      password: payload.password,
    };
    const res = await ctx.fetcher({
      method: "POST",
      path: "/auth/login",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(req),
    });

    if (!res.ok) {
      throw new Error(`Login failed: ${res.status} ${res.statusText}`);
    }
    const data = await res.json();

    const cookies = res.headers.getSetCookie();

    ctx.addContext("cookies", cookies);
    cookies.forEach((cookie) => {
      const [name, value] = cookie.split(";")[0].split("=");
      ctx.addContext(name, `${name}=${value}`);
    });

    ctx.addContext("jwt", data.jwt);

    console.log("Login successful:", ctx);
  }
);

if (import.meta.main) {
  await login.execute();
}
