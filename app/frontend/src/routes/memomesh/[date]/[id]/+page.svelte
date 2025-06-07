<script lang="ts">
	import Navigation from '$lib/components/Navigation.svelte';
	import GoBackButton from '$lib/components/GoBackButton.svelte';
	import SectionHeader from '../../components/SectionHeader.svelte';
	import EmptyAwareList from '$lib/components/EmptyAwareList.svelte';
	import LongMemoCard from '../../components/LongMemoCard.svelte';
	import LoadingComponent from '../../components/LoadingComponent.svelte';
	import ErrorComponent from '../../components/ErrorComponent.svelte';
	import EmptyMemoComponent from '../../components/EmptyMemoComponent.svelte';
	import { onMount } from 'svelte';
	import { getMemoById } from '$lib/store/memo/api';
	import { page } from '$app/state';
	import type { GetMemoByIdResponse } from '$lib/types/src/api/memo/get_by_id';

	const routePath = page.url.pathname;
	const routeId = routePath.split('/').pop() || '';

	let memo: GetMemoByIdResponse | null = null;
	let loading = true;
	let error: string | null = null;

	// EmptyAwareListで使用するためのデータ配列
	$: displayItems = loading ? ['loading'] : error ? ['error'] : memo ? [memo] : [];

	onMount(async () => {
		try {
			memo = await getMemoById(routeId);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to fetch memo';
			console.error('Error fetching memo:', e);
		} finally {
			loading = false;
		}
	});
</script>

<Navigation />
<main class="container mx-auto p-4 md:p-6">
	<div class="mb-6 flex items-center">
		<GoBackButton />
	</div>
	<SectionHeader />

	<EmptyAwareList items={displayItems}>
		{#each displayItems as item}
			{#if loading}
				<LoadingComponent />
			{:else if error}
				<ErrorComponent {error} />
			{:else if memo}
				<LongMemoCard {memo} />
			{:else}
				<EmptyMemoComponent />
			{/if}
		{/each}
	</EmptyAwareList>
</main>
