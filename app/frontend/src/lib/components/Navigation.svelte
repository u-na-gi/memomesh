<script lang="ts">
	import { page } from '$app/state';
	import { logout } from '$lib/auth/store';

	const pathname = $derived(page.url.pathname); // This will correctly update id for usage on this page

	// ログイン状態を確認
	const isLoggedIn = $derived(localStorage.getItem('isLoggedIn') === 'true');

	// ログアウト処理
	async function handleLogout() {
		const result = await logout();
		if (result.isErr()) {
			console.error('ログアウトに失敗しました:', result.error);
		}
	}
</script>

<nav class="bg-blue-600 p-4 text-white">
	<div class="container mx-auto flex items-center justify-between">
		<a href="/" class="text-xl font-bold"> MemoMesh </a>

		{#if isLoggedIn}
			<button onclick={handleLogout} class="rounded px-4 py-2 transition-colors hover:bg-blue-700">
				ログアウト
			</button>
		{/if}
	</div>
</nav>
