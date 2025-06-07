<script lang="ts">
	import type { GetMemoByIdResponse } from '$lib/types/src/api/memo/get_by_id';

	export let memo: GetMemoByIdResponse;

	const displayDate = new Date(memo.date).toLocaleDateString('ja-JP', {
		year: 'numeric',
		month: '2-digit',
		day: '2-digit'
	});

	const formatDateTime = (dateTimeString: string) => {
		if (!dateTimeString) return '';
		return new Date(dateTimeString).toLocaleString('ja-JP');
	};
</script>

<div class="rounded-lg bg-white p-6 shadow-md">
	<div class="mb-4">
		<h2 class="mb-2 text-xl font-semibold text-gray-800">メモ詳細</h2>
		<div class="mb-4 text-sm text-gray-600">
			<p>ID: {memo.id}</p>
			<p>日付: {displayDate}</p>
			{#if memo.parentId}
				<p>親ID: {memo.parentId}</p>
			{/if}
			<p>作成日時: {formatDateTime(memo.createdAt)}</p>
			<p>更新日時: {formatDateTime(memo.updatedAt)}</p>
		</div>
	</div>
	<div class="border-t pt-4">
		<h3 class="mb-2 text-lg font-medium text-gray-800">内容</h3>
		<div class="max-w-none">
			<pre class="whitespace-pre-wrap text-gray-900">{memo.contents}</pre>
		</div>
	</div>
</div>
