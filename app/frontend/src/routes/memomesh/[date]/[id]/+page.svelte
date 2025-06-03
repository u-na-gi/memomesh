<script lang="ts">
	import Navigation from '$lib/components/Navigation.svelte';
	import GoBackButton from '$lib/components/GoBackButton.svelte';
	import SectionHeader from '../../components/SectionHeader.svelte';
	import EmptyAwareList from '$lib/components/EmptyAwareList.svelte';

	let data = { data: [] };
</script>

<Navigation />
<main class="container mx-auto p-4 md:p-6">
	<div class="mb-6 flex items-center">
		<GoBackButton />
	</div>
	<SectionHeader />
	<EmptyAwareList items={data.data}>
		{#each data.data as memos}
			<p>hello</p>
		{/each}
	</EmptyAwareList>
</main>
