<script lang="ts">
	export let value: string;
	export let inputId: string;
	let type = 'text';

	if (inputId === 'password') {
		type = 'password';
	}
</script>

<input
	{type}
	id={inputId}
	bind:value
	class="w-full rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
	required
/>
