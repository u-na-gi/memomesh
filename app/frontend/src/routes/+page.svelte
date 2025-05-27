<script lang="ts">
	import { onMount } from 'svelte';

	onMount(() => {
		console.log('Root page: onMount');
		// ログイン状態をチェック
		const isLoggedIn = localStorage.getItem('isLoggedIn') === 'true';
		console.log('Root page: isLoggedIn =', isLoggedIn);

		// 直接リダイレクト
		setTimeout(() => {
			if (isLoggedIn) {
				console.log('Root page: Redirecting to memomesh');
				window.location.href = '/memomesh';
			} else {
				console.log('Root page: Redirecting to login');
				window.location.href = '/login';
			}
		}, 100);
	});
</script>

<div class="flex h-screen items-center justify-center">
	<p class="text-lg">リダイレクト中...</p>
</div>
