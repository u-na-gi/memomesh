<script lang="ts">
	const goBack = () => {
		history.back();
	};
</script>

<button on:click={goBack} class="mr-4 text-blue-600 hover:text-blue-800"> ← 戻る </button>
