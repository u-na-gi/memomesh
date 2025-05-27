<script lang="ts">
	import '../app.css';
	import AuthGuard from '$lib/auth/components/AuthGuard.svelte';

	let { children } = $props();
</script>

<AuthGuard>
	{@render children()}
</AuthGuard>
