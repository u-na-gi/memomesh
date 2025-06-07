<script lang="ts">
	import BaseForm from '$lib/components/BaseForm.svelte';
	import { createMemo } from '$lib/store/memo/api';
	import { setItemToLocalStorage, removeItemFromLocalStorage } from '$lib/utils/local-storage';
	import MarkdownPreview from './MarkdownPreview.svelte';

	export let content = '';
	export let contentKey = '';
	let loading = false;

	// +ボタンをどこから押したかで決まる
	let parentId = ''; // 親IDを保持する変数（必要に応じて使用）

	const handleSubmit = async (): Promise<void> => {
		loading = true;
		try {
			await createMemo({
				contents: content,
				parentId: parentId
			});
			// API成功後にローカルストレージからコンテンツを削除
			removeItemFromLocalStorage(contentKey);
			// history.back()を実行
			history.back();
		} catch (error) {
			console.error(error);
		} finally {
			loading = false;
		}
	};

	$: if (content.trim() !== '') {
		setItemToLocalStorage(contentKey, content);
	}
</script>

<BaseForm {loading} onSubmit={handleSubmit}>
	<div class="mb-4">
		<label for="content" class="mb-2 block text-gray-700">やったこと</label>
		<textarea
			id="content"
			bind:value={content}
			class="w-full rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
			rows={10}
			placeholder="朝ジョギングした&#10;API設計した"
			required
		></textarea>
	</div>

	<label for="content" class="block text-gray-700">リアルタイムプレビュー</label>
	<div class="mb-8 mt-4 rounded border p-4">
		<MarkdownPreview children={content} />
	</div>
</BaseForm>
