<script lang="ts">
	import GoBackButton from '$lib/components/GoBackButton.svelte';
	import LifeLogForm from '$lib/components/LifeLogForm.svelte';
	import { getDateTime } from '$lib/utils/date';
	import { getItemFromLocalStorage } from '$lib/utils/local-storage';
	import { onDestroy, onMount } from 'svelte';
	import { contentKey } from './const';
	import { createMemo } from '$lib/store/memo/api';

	let currentDateTime: string = '';
	let errorMessage = '';
	let content = ''; // フォームの内容を保持する変数
	// 今日の日付を更新するロジック
	let intervalId: ReturnType<typeof setInterval>;

	onMount(() => {
		const dateTime = getDateTime();
		currentDateTime = dateTime;

		intervalId = setInterval(() => {
			currentDateTime = getDateTime();
		}, 1000);

		// localStorageから初期読み込み
		content = getItemFromLocalStorage(contentKey);
	});

	onDestroy(() => {
		clearInterval(intervalId);
	});
</script>

<div class="mx-auto max-w-2xl p-6">
	<div class="mb-6 flex items-center">
		<GoBackButton />
		<h1 class="text-2xl font-bold">新規作成</h1>
	</div>
	{#if errorMessage}
		<div class="mb-4 rounded border border-red-400 bg-red-100 px-4 py-3 text-red-700">
			{errorMessage}
		</div>
	{/if}
	<div class="mb-6 rounded-lg bg-white p-6 shadow-md">
		<div class="mb-4 flex items-center justify-between">
			<div class="text-gray-700">{currentDateTime}</div>
		</div>

		<LifeLogForm {content} {contentKey} />
	</div>
</div>
