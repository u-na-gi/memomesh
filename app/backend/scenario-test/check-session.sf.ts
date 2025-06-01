import { ScenarioFlow } from "scenario-flow";
import { login } from "./login.sf.ts";

export const checkSession = new ScenarioFlow("Check Session Flow", login).step(
  "Check User Session",
  async (ctx) => {
    const sessionId = ctx.getContext("session_id") as string;

    const res = await ctx.fetcher({
      method: "GET",
      path: "/auth/session",
      headers: {
        "Content-Type": "application/json",
        Cookie: sessionId,
      },
    });

    if (!res.ok) {
      throw new Error(`Session check failed: ${res.status} ${res.statusText}`);
    }
  }
);

if (import.meta.main) {
  await checkSession.execute();
}
