import { login } from "./login.sf.ts";

export const checkSession = login.step(async (ctx) => {
  const sessionId = ctx.getContext("session_id") as string;

  const res = await ctx.fetcher({
    method: "GET",
    urlPaths: ["auth", "session"],
    headers: {
      "Content-Type": "application/json",
      Cookie: sessionId,
    },
  });

  if (!res.ok) {
    throw new Error(`Session check failed: ${res.status} ${res.statusText}`);
  }
});

if (import.meta.main) {
  await checkSession.execute();
}
