import { ScenarioFlow } from "scenario-flow";
import { login } from "./login.sf.ts";

export const logout = new ScenarioFlow("Logout Flow", login).step(
  "Logout User",
  async (ctx) => {
    const sessionId = ctx.getContext("session_id") as string;

    console.log("Logging out with session ID:", sessionId);
    const res = await ctx.fetcher({
      method: "DELETE",
      path: "/auth/logout",
      headers: {
        "Content-Type": "application/json",
        Cookie: sessionId,
      },
    });

    console.log("Logout response status:", res.status);
  }
);

if (import.meta.main) {
  await logout.execute();
}
