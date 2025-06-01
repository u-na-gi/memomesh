import { ScenarioFlow } from "scenario-flow";

import { createMemo } from "./memo-create.sf.ts";
import {
  DeleteMemoRequest,
  DeleteMemoResponse,
} from "./types/src/api/memo/delete.ts";

export const deleteMemo = new ScenarioFlow("Delete Memo Flow", createMemo).step(
  "Delete Memo",
  async (ctx) => {
    const createdMemo = ctx.getContext("createdMemo") as {
      parentId: string;
      contents: string;
    };

    let req: DeleteMemoRequest = {
      memoId: "test-memo-id", // 実際の実装では、作成されたメモのIDを使用
    };
    req = DeleteMemoRequest.create(req);
    const bytes = DeleteMemoRequest.encode(req).finish();

    const sessionCookie = ctx.getContext("session") as string;
    const res = await ctx.fetcher({
      method: "DELETE",
      path: "/memo/delete",
      headers: {
        "Content-Type": "application/octet-stream",
        Cookie: sessionCookie || "",
      },
      body: bytes.buffer as ArrayBuffer,
    });

    if (!res.ok) {
      throw new Error(`Delete memo failed: ${res.status} ${res.statusText}`);
    }
    const data = await res.arrayBuffer();
    const uint8 = new Uint8Array(data);
    const response = DeleteMemoResponse.decode(uint8);
    console.log("Delete memo response:", response);

    if (!response.success) {
      throw new Error(`Delete memo failed: ${response.message}`);
    }

    ctx.addContext("deletedMemo", {
      memoId: req.memoId,
    });
  }
);

if (import.meta.main) {
  await deleteMemo.execute();
}
