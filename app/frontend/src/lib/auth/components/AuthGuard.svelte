<script lang="ts">
	import { page } from '$app/state'; // `$app/state`ではなく`$app/stores`
	import { onMount } from 'svelte';
	import { get, writable, type Writable } from 'svelte/store';
	import { session } from '../store/check-session';
	import { afterNavigate } from '$app/navigation';

	type SkipAuthGuardType = { [key: string]: boolean };

	interface AuthState {
		isAuthenticated: boolean;
		isLoading: boolean;
		redirectTo: string | null;
	}

	const skipAuthGuard: SkipAuthGuardType = {
		'/': true,
		'/login': true,
		'/register': true,
		'/forgot-password': true
	};

	const state: Writable<AuthState> = writable({
		isAuthenticated: false,
		isLoading: true,
		redirectTo: null
	});

	let currentState: AuthState;
	state.subscribe((s) => {
		currentState = s;
		console.log('AuthGuard state updated:', s);
	});

	// 現在のパスをリアクティブに取得
	const pathname = writable(page.url.pathname);
	afterNavigate(() => {
		// ページ遷移後に現在のパスを更新
		pathname.set(page.url.pathname);
		console.log('AuthGuard: Current pathname updated:', $pathname);
	});

	// redirectTo が設定されたらリダイレクト実行
	$: if (currentState.redirectTo) {
		setTimeout(() => {
			window.location.href = currentState.redirectTo!;
		}, 100);
	}

	function updateState(newState: Partial<AuthState>) {
		state.update((s) => ({ ...s, ...newState }));
	}

	async function checkSessionAndRedirect() {
		try {
			await session();
			console.log('AuthGuard: Session is valid');
			updateState({
				isAuthenticated: true,
				isLoading: false
			});

			if (get(pathname) === '/') {
				console.log('AuthGuard: Redirecting to /memomesh');
				updateState({ redirectTo: '/memomesh' });
			}
		} catch (error) {
			console.error('AuthGuard: Session check failed', error);
			updateState({
				isAuthenticated: false,
				isLoading: false
			});

			if (!(get(pathname) in skipAuthGuard)) {
				console.log('AuthGuard: Redirecting to /login');
				updateState({ redirectTo: '/login' });
			}
		}
	}

	onMount(() => {
		console.log('AuthGuard: Component mounted');

		if (get(pathname) in skipAuthGuard && get(pathname) !== '/') {
			console.log('AuthGuard: No auth required for this path');
			updateState({
				isAuthenticated: true,
				isLoading: false
			});
			return;
		}

		// '/' で未ログインならログインへ、ログイン済みなら memomesh へ
		if (get(pathname) === '/') {
			updateState({ redirectTo: '/login' });
		}

		checkSessionAndRedirect();
	});
</script>

{#if currentState.redirectTo}
	<div class="flex h-screen items-center justify-center">
		<p class="text-lg">リダイレクト中...</p>
	</div>
{:else if currentState.isLoading && !(get(pathname) in skipAuthGuard)}
	<div class="flex h-screen items-center justify-center">
		<p class="text-lg">読み込み中...</p>
	</div>
{:else}
	<slot />
{/if}
