<script lang="ts">
	import { getDateTime } from '$lib/utils/date';
	import { onDestroy, onMount } from 'svelte';

	let currentDateTime: string = '';
	let content = '';

	// 今日の日付を更新するロジック
	let intervalId: ReturnType<typeof setInterval>;

	onMount(() => {
		const dateTime = getDateTime();
		currentDateTime = dateTime;

		intervalId = setInterval(() => {
			currentDateTime = getDateTime();
		}, 1000);

		// localStorageから初期読み込み
		const savedContent = localStorage.getItem('content');
		if (savedContent) {
			content = savedContent;
		}
	});

	onDestroy(() => {
		clearInterval(intervalId);
	});

	const handleSubmit = async () => {
		// api側に送信するロジックが必要になる
	};
</script>

<div class="mb-6 rounded-lg bg-white p-6 shadow-md">
	<div class="mb-4">
		<textarea
			id="content"
			bind:value={content}
			class="w-full rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
			rows={10}
			placeholder="朝ジョギングした&#10;API設計した"
			required
		></textarea>
	</div>
</div>
