import { ForgotPasswordRequest } from "./types/src/api/auth/forgot_password/forgot_password.ts";
import { ScenarioFlow } from "scenario-flow";
import { baseUrl } from "./const.ts";

export const forgotPassword = new ScenarioFlow("Forgot Password Flow", {
  apiBaseUrl: baseUrl,
}).step("Send Forgot Password Request", async (ctx) => {
  const req: ForgotPasswordRequest = {
    email: "test@example.com",
  };

  const res = await ctx.fetcher({
    method: "POST",
    path: "/auth/forgot-password",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(req),
  });

  if (!res.ok) {
    throw new Error(`Forgot password failed: ${res.status} ${res.statusText}`);
  }

  const data = await res.json();
  console.log("Forgot password successful:", data);

  // レスポンスの検証
  if (!data.message) {
    throw new Error("Response should contain a message");
  }

  console.log("Message:", data.message);
  ctx.addContext("response", data);
});

// 無効なメールアドレスのテストケース
export const forgotPasswordInvalidEmail = new ScenarioFlow(
  "Forgot Password Invalid Email Flow",
  {
    apiBaseUrl: baseUrl,
  }
).step("Send Forgot Password Request with Invalid Email", async (ctx) => {
  const req: ForgotPasswordRequest = {
    email: "invalid-email",
  };

  const res = await ctx.fetcher({
    method: "POST",
    path: "/auth/forgot-password",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(req),
  });

  if (!res.ok) {
    throw new Error(`Forgot password failed: ${res.status} ${res.statusText}`);
  }

  const data = await res.json();
  console.log("Forgot password with invalid email:", data);

  // セキュリティ上の理由で、無効なメールでも同じレスポンスが返される
  if (!data.message) {
    throw new Error("Response should contain a message");
  }

  console.log("Message:", data.message);
  ctx.addContext("response", data);
});

if (import.meta.main) {
  console.log("=== Testing valid email ===");
  await forgotPassword.execute();

  console.log("\n=== Testing invalid email ===");
  await forgotPasswordInvalidEmail.execute();
}
