<script lang="ts">
	import { memomeshByIdPage } from '$lib/routes';

	import BaseCard from '$lib/components/BaseCard.svelte';

	export let id: string;
	export let date: string;

	const displayDate = new Date(date).toLocaleDateString('ja-JP', {
		month: '2-digit',
		day: '2-digit'
	});

	const routePath = memomeshByIdPage(date, id);
</script>

<BaseCard {routePath}>
	<div class="text-xl font-bold">{displayDate}</div>
	<div class="mt-2 text-gray-600">
		<slot />
	</div>
</BaseCard>
