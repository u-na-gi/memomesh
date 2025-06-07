import { ScenarioFlow } from "scenario-flow";

import { login } from "./login.sf.ts";
import {
  CreateMemoRequest,
  CreateMemoResponse,
} from "./types/src/api/memo/create.ts";

export const createMemo = new ScenarioFlow("Create Memo Flow", login).step(
  "Create Memo",
  async (ctx) => {
    let req: CreateMemoRequest = {
      parentId: "test-parent-id",
      contents: "これはテスト用のメモです。",
    };
    req = CreateMemoRequest.create(req);
    const bytes = CreateMemoRequest.encode(req).finish();

    const jwt = ctx.getContext("jwt") as string;
    const res = await ctx.fetcher({
      method: "POST",
      path: "/memo/create",
      headers: {
        "Content-Type": "application/octet-stream",
        Authorization: "Bearer " + jwt,
      },
      body: bytes.buffer as ArrayBuffer,
    });

    if (!res.ok) {
      throw new Error(`Create memo failed: ${res.status} ${res.statusText}`);
    }
    const data = await res.arrayBuffer();
    const uint8 = new Uint8Array(data);
    const response = CreateMemoResponse.decode(uint8);
    console.log("Create memo response:", response);

    ctx.addContext("createdMemo", {
      parentId: req.parentId,
      contents: req.contents,
    });
  }
);

if (import.meta.main) {
  await createMemo.execute();
}
