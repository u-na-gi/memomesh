<script lang="ts">
	import DateCard from './components/DateCard.svelte';
	import Navigation from '$lib/components/Navigation.svelte';

	import { onMount } from 'svelte';

	import EmptyAwareList from '$lib/components/EmptyAwareList.svelte';
	import SectionHeader from './components/SectionHeader.svelte';
	import Section from './components/Section.svelte';
	import { getDataByDate } from '$lib/store/memo/api';
	import { getUtcDateRange } from '$lib/utils/date';
	import type { CountNotesByDate } from '$lib/types/src/api/memo/get_count_by_date';

	// トップページでやること
	let countByDateList: CountNotesByDate[] = [];
	// ページの読み込み状態
	let isLoading = true;
	onMount(async () => {
		const { start, end } = getUtcDateRange(30);
		const result = await getDataByDate(start, end);

		if (result) {
			console.log('データの取得に成功しました: result.countNotesByDate', result.countNotesByDate);
			countByDateList = result.countNotesByDate;
		} else {
			console.error('データの取得に失敗しました');
		}
		isLoading = false;
	});
</script>

<!-- // ユーザーの持ってるカードを表示する -->

{#if isLoading}
	<div class="flex h-screen items-center justify-center">
		<p class="text-lg">読み込み中...</p>
	</div>
{:else}
	<Navigation />
	<Section>
		<SectionHeader />

		<EmptyAwareList items={countByDateList}>
			{#each countByDateList as data}
				<DateCard date={data.date}>
					{data.count}
					{data.count === 1 ? 'エントリー' : 'エントリー'}
				</DateCard>
			{/each}
		</EmptyAwareList>
	</Section>
{/if}
