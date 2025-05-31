import { login } from "./login.sf.ts";

export const logout = login.step(async (ctx) => {
  const sessionId = ctx.getContext("session_id") as string;

  console.log("Logging out with session ID:", sessionId);
  const res = await ctx.fetcher({
    method: "DELETE",
    urlPaths: ["auth", "logout"],
    headers: {
      "Content-Type": "application/json",
      Cookie: sessionId,
    },
  });

  console.log("Logout response status:", res.status);
});

if (import.meta.main) {
  await logout.execute();
}
