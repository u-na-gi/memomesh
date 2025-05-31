import { ScenarioFlow } from "scenario-flow";

import { createMemo } from "./memo-create.sf.ts";
import { DataList } from "./types/src/api/memo/get_data.ts";

export const getMemoData = new ScenarioFlow(
  "Get Memo Data Flow",
  createMemo
).step("Get Memo Data", async (ctx) => {
  // 今日の日付を YYYY-MM-DD 形式で取得
  const today = new Date().toISOString().split("T")[0];

  const sessionCookie = ctx.getContext("session") as string;
  const res = await ctx.fetcher({
    method: "GET",
    path: `/memo/data?date=${today}`,
    headers: {
      Cookie: sessionCookie || "",
    },
  });

  if (!res.ok) {
    throw new Error(`Get memo data failed: ${res.status} ${res.statusText}`);
  }

  const data = await res.arrayBuffer();
  const uint8 = new Uint8Array(data);
  const response = DataList.decode(uint8);
  console.log("Get memo data response:", response);

  // 作成したメモが取得できているかを確認
  const createdMemo = ctx.getContext("createdMemo") as {
    parentId: string;
    contents: string;
  };
  const foundMemo = response.data.find(
    (memo) => memo.contents === createdMemo.contents
  );

  if (!foundMemo) {
    console.warn("作成したメモが取得結果に含まれていません");
  } else {
    console.log("作成したメモが正常に取得されました:", foundMemo);
  }

  ctx.addContext("retrievedMemoData", response);
});

if (import.meta.main) {
  await getMemoData.execute();
}
