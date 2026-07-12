<script lang="ts">
	import { onMount } from 'svelte';
	import { applicationControllerPatch } from '$lib/client/sdk.gen';
	import type { ApplicationResponseDto } from '$lib/client/types.gen';
	import loader from '@monaco-editor/loader';

	type Props = { app: ApplicationResponseDto; onUpdated: (a: ApplicationResponseDto) => void; active: boolean };
	let { app, onUpdated, active }: Props = $props();

	let envVars = $state(app.env_var ?? '');
	let saving = $state(false);
	let saved = $state(false);
	let editorContainer = $state<HTMLDivElement | null>(null);
	let editorInstance: any = null;

	onMount(() => {
		let disposed = false;
		loader.init().then((monaco) => {
			if (disposed || !editorContainer) return;

			if (!monaco.languages.getLanguages().some((l: any) => l.id === 'ini')) {
				monaco.languages.register({ id: 'ini' });
				monaco.languages.setMonarchTokensProvider('ini', {
					tokenizer: {
						root: [
							[/#.*$/, 'comment'],
							[/;.*$/, 'comment'],
							[/^\s*\[.*\]/, 'tag'],
							[/[a-zA-Z_][\w]*(?==)/, 'key'],
							[/=/, 'delimiter'],
							[/"([^"\\]|\\.)*"/, 'string'],
							[/'([^'\\]|\\.)*'/, 'string']
						]
					}
				} as any);
			}

			editorInstance = monaco.editor.create(editorContainer!, {
				value: app.env_var ?? '',
				language: 'ini',
				theme: 'vs-dark',
				fontSize: 13,
				fontFamily: 'monospace',
				minimap: { enabled: false },
				scrollBeyondLastLine: false,
				lineNumbers: 'on',
				wordWrap: 'on',
				automaticLayout: true,
				padding: { top: 8, bottom: 8 }
			});
			editorInstance.onDidChangeModelContent(() => { envVars = editorInstance.getValue(); });
		});
		return () => {
			disposed = true;
			editorInstance?.dispose();
		};
	});

	$effect(() => {
		if (active && editorInstance) setTimeout(() => editorInstance.layout(), 10);
	});

	async function save() {
		saving = true;
		try {
			const res = await applicationControllerPatch({ path: { id: app.id }, body: { env_var: envVars } });
			if (res.data) { onUpdated(res.data); saved = true; setTimeout(() => (saved = false), 2000); }
		} finally { saving = false; }
	}
</script>

<div class="bg-card border border-border rounded-lg p-6 flex flex-col gap-4">
	<div>
		<h2 class="text-base font-semibold">Environment Variables</h2>
		<p class="text-sm text-muted-foreground mt-1">Set environment variables. One per line in <code class="font-mono">KEY=VALUE</code> format.</p>
	</div>
	<div class="rounded-md overflow-hidden border border-border" style="height: 420px">
		<div bind:this={editorContainer} style="height: 100%; width: 100%"></div>
	</div>
	<div class="flex justify-end items-center gap-3">
		{#if saved}<span class="text-sm text-green-600">Saved!</span>{/if}
		<button class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50" onclick={save} disabled={saving}>
			{#if saving}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>Saving…{:else}Save{/if}
		</button>
	</div>
</div>
