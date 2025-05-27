<script lang="ts">
	export let displayLabel: string;
	export let inputId: string;
</script>

<label for={inputId} class="mb-2 block text-gray-700"> {displayLabel} </label>
