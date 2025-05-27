<script lang="ts">
	import Navigation from '$lib/components/Navigation.svelte';
	import GoBackButton from '$lib/components/GoBackButton.svelte';
	import SectionHeader from '../components/SectionHeader.svelte';
	import EmptyAwareList from '$lib/components/EmptyAwareList.svelte';
	import type { ContentsDateCard } from '../type';
	import DateCard from '../components/DateCard.svelte';

	const getData: ContentsDateCard[] = [
		{ dateTime: '2025-05-18', contents: 'バナナ' },
		{ dateTime: '2025-04-28', contents: 'orange' },
		{ dateTime: '2025-04-29', contents: 'takoyaki' },
		{ dateTime: '2025-04-30', contents: 'sushi' },
		{ dateTime: '2025-05-01', contents: 'sashimi' },
		{ dateTime: '2025-05-02', contents: 'sashimi' },
		{ dateTime: '2025-05-03', contents: 'sashimi' },
		{ dateTime: '2025-05-04', contents: 'sashimi' },
		{ dateTime: '2025-05-05', contents: 'sashimi' }
	];
</script>

<Navigation />
<main class="container mx-auto p-4 md:p-6">
	<div class="mb-6 flex items-center">
		<GoBackButton />
	</div>
	<SectionHeader />
	<EmptyAwareList items={getData}>
		{#each getData as data}
			<DateCard date={data.dateTime}>
				{data.contents}
			</DateCard>
		{/each}
	</EmptyAwareList>
</main>
