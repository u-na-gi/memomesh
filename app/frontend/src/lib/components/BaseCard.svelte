<script lang="ts">
	import type { RouteString } from '$lib/routes';

	export let routePath: RouteString;
</script>

<a href={routePath}>
	<div
		class="cursor-pointer rounded-lg bg-white p-6 shadow-md transition-shadow duration-200 hover:shadow-lg"
	>
		<slot />
	</div>
</a>
