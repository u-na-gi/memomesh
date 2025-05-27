<script lang="ts">
	export let items: unknown[] = [];
	export let fallback = 'データがありません';
</script>

{#if items.length > 0}
	<!-- grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3 -->
	<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
		<slot />
	</div>
{:else}
	<div class="rounded-lg bg-white p-6 text-center shadow-md">
		<p class="mb-4 text-gray-600">{fallback}</p>
	</div>
{/if}
