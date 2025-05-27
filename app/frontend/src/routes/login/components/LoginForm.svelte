<script lang="ts">
	import { goto } from '$app/navigation';
	import { login } from '$lib/auth/store';
	import BaseForm from '../../../lib/components/BaseForm.svelte';
	import BaseLoginInput from './LoginInput.svelte';
	import BaseLoginLavel from './LoginLavel.svelte';

	let username = '';
	let password = '';
	let errorMessage = '';
	let loading = false;

	const passwordInputId = 'password';
	const usernameInputId = 'username';

	const handleSubmit = async (): Promise<void> => {
		loading = true;
		errorMessage = '';

		const result = await login({
			username,
			password
		});

		loading = false;
		if (result.isErr()) {
			errorMessage = result.error.message;
		} else {
			await goto('/memomesh');
		}
	};
</script>

<div class="mx-auto max-w-md rounded-lg bg-white p-6 shadow-md">
	<h2 class="mb-6 text-center text-2xl font-bold">ログイン</h2>
	{#if errorMessage}
		<div class="mb-4 rounded border border-red-400 bg-red-100 px-4 py-3 text-red-700">
			{errorMessage}
		</div>
	{/if}
	<BaseForm {loading} onSubmit={handleSubmit}>
		<div class="mb-4">
			<BaseLoginLavel displayLabel="ユーザー名" inputId={usernameInputId} />
			<BaseLoginInput inputId={usernameInputId} bind:value={username} />
		</div>

		<div class="mb-6">
			<BaseLoginLavel displayLabel="パスワード" inputId={passwordInputId} />
			<BaseLoginInput inputId={passwordInputId} bind:value={password} />
		</div>
	</BaseForm>
</div>
