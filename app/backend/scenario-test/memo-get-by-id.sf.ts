import { ScenarioFlow } from "scenario-flow";

import { createMemo } from "./memo-create.sf.ts";
import { GetMemoByIdResponse } from "./types/src/api/memo/get_by_id.ts";

export const getMemoById = new ScenarioFlow(
  "Get Memo By ID Flow",
  createMemo
).step("Get Memo By ID", async (ctx) => {
  // 作成したメモのIDを取得
  const createdMemo = ctx.getContext("createdMemo") as {
    parentId: string;
    contents: string;
  };

  // 実際のメモIDが必要ですが、ここではサンプルIDを使用
  const memoId = "sample_memo_id";

  const jwt = ctx.getContext("jwt") as string;
  const res = await ctx.fetcher({
    method: "GET",
    path: `/memo/id?id=${memoId}`,
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${jwt}`,
    },
  });

  if (!res.ok) {
    throw new Error(`Get memo by ID failed: ${res.status} ${res.statusText}`);
  }

  const data = await res.arrayBuffer();
  const uint8 = new Uint8Array(data);
  const response = GetMemoByIdResponse.decode(uint8);
  console.log("Get memo by ID response:", response);

  // レスポンスの検証
  if (!response.id) {
    throw new Error("メモIDが取得できませんでした");
  }

  if (!response.contents) {
    throw new Error("メモの内容が取得できませんでした");
  }

  console.log("メモが正常に取得されました:", {
    id: response.id,
    date: response.date,
    contents: response.contents,
    parentId: response.parentId,
    createdAt: response.createdAt,
    updatedAt: response.updatedAt,
  });

  ctx.addContext("retrievedMemo", response);
});

if (import.meta.main) {
  await getMemoById.execute();
}
