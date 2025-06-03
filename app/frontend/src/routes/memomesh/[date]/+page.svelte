<script lang="ts">
	import Navigation from '$lib/components/Navigation.svelte';
	import GoBackButton from '$lib/components/GoBackButton.svelte';
	import SectionHeader from '../components/SectionHeader.svelte';
	import EmptyAwareList from '$lib/components/EmptyAwareList.svelte';
	import { onMount } from 'svelte';
	import { getMemoByDate } from '$lib/store/memo/api';
	import { page } from '$app/state';
	import type { MemoList } from '$lib/types/src/api/memo/get_data';
	import ShortMemoCard from '../components/ShortMemoCard.svelte';

	const routePath = page.url.pathname;
	const routeDate = routePath.split('/').pop() || '';

	let data: MemoList = { data: [] };

	onMount(async () => {
		// This is a placeholder for any initialization logic you might need
		data = await getMemoByDate({
			date: routeDate
		});
	});
</script>

<Navigation />
<main class="container mx-auto p-4 md:p-6">
	<div class="mb-6 flex items-center">
		<GoBackButton />
	</div>
	<SectionHeader />
	<EmptyAwareList items={data.data}>
		{#each data.data as memos}
			<ShortMemoCard date={memos.date} id={memos.id}>
				{memos.shortContents}
			</ShortMemoCard>
		{/each}
	</EmptyAwareList>
</main>
