<script lang="ts">
	import { marked } from 'marked';
	import DOMPurify from 'dompurify';
	import { onMount } from 'svelte';

	export let children: string = '';
	let html: string = '';
	let purify: ReturnType<typeof DOMPurify>;

	onMount(() => {
		purify = DOMPurify(window);
		convertMarkdown(children);
	});

	const convertMarkdown = async (input: string) => {
		const raw = await marked.parse(input, {
			gfm: true,
			breaks: true
		});

		html = purify.sanitize(raw);
	};

	// children が変化したら変換
	$: if (purify != undefined && children !== '') {
		convertMarkdown(children);
	} else {
		html = '';
	}
</script>

<div class="prose">
	{@html html}
</div>
