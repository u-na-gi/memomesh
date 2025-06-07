import { ScenarioFlow } from "scenario-flow";

import { baseUrl } from "./const.ts";
import {
  RegisterRequest,
  RegisterResponse,
} from "./types/src/api/user/register.ts";

export const registerUser = new ScenarioFlow("Register User Flow", {
  apiBaseUrl: baseUrl,
}).step("Register User", async (ctx) => {
  let req: RegisterRequest = {
    email: "test@example.com",
    password: "test",
  };
  req = RegisterRequest.create(req);
  const bytes = RegisterRequest.encode(req).finish();
  const res = await ctx.fetcher({
    method: "POST",
    path: "/user/register",
    headers: {
      "Content-Type": "application/octet-stream",
    },
    body: bytes.buffer as ArrayBuffer,
  });

  if (!res.ok) {
    throw new Error(`Login failed: ${res.status} ${res.statusText}`);
  }
  const data = await res.arrayBuffer();
  const uint8 = new Uint8Array(data);
  const response = RegisterResponse.decode(uint8);
  console.log("Register response:", response);

  ctx.addContext("user", {
    email: req.email,
    password: req.password,
  });
});

if (import.meta.main) {
  await registerUser.execute();
}
