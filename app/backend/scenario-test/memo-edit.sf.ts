import { ScenarioFlow } from "scenario-flow";

import { createMemo } from "./memo-create.sf.ts";
import {
  EditMemoRequest,
  EditMemoResponse,
} from "./types/src/api/memo/edit.ts";

export const editMemo = new ScenarioFlow("Edit Memo Flow", createMemo).step(
  "Edit Memo",
  async (ctx) => {
    const createdMemo = ctx.getContext("createdMemo") as {
      parentId: string;
      contents: string;
    };

    let req: EditMemoRequest = {
      memoId: "test-memo-id", // 実際の実装では、作成されたメモのIDを使用
      contents: "これは編集されたテスト用のメモです。",
    };
    req = EditMemoRequest.create(req);
    const bytes = EditMemoRequest.encode(req).finish();

    const jwt = ctx.getContext("jwt") as string;
    const res = await ctx.fetcher({
      method: "POST",
      path: "/memo/edit",
      headers: {
        "Content-Type": "application/octet-stream",
        Authorization: `Bearer ${jwt}`,
      },
      body: bytes.buffer as ArrayBuffer,
    });

    if (!res.ok) {
      throw new Error(`Edit memo failed: ${res.status} ${res.statusText}`);
    }
    const data = await res.arrayBuffer();
    const uint8 = new Uint8Array(data);
    const response = EditMemoResponse.decode(uint8);
    console.log("Edit memo response:", response);

    if (!response.success) {
      throw new Error(`Edit memo failed: ${response.message}`);
    }

    ctx.addContext("editedMemo", {
      memoId: req.memoId,
      contents: req.contents,
    });
  }
);

if (import.meta.main) {
  await editMemo.execute();
}
