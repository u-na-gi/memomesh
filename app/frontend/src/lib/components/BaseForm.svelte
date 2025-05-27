<script lang="ts">
	export let onSubmit: () => Promise<void>; // 親から渡す
	export let loading: boolean = false;
</script>

<form on:submit|preventDefault={onSubmit}>
	<slot />
	<!-- 中身は親が差し込む -->

	<button
		type="submit"
		disabled={loading}
		class="mt-4 w-full rounded-md bg-blue-600 px-4 py-2 text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50"
	>
		{loading ? '保存中...' : '保存'}
	</button>
</form>
