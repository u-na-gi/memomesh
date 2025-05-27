<script lang="ts">
	import { memomeshByDatePage } from '$lib/routes';

	import BaseCard from '$lib/components/BaseCard.svelte';

	export let date: string;

	const displayDate = new Date(date).toLocaleDateString('ja-JP', {
		month: '2-digit',
		day: '2-digit'
	});
	const routeDate = new Date(date).toISOString().slice(0, 10); // "2025-04-27"を作る
	const routePath = memomeshByDatePage(routeDate);
</script>

<BaseCard {routePath}>
	<div class="text-xl font-bold">{displayDate}</div>
	<div class="mt-2 text-gray-600">
		<slot />
	</div>
</BaseCard>
