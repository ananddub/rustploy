<script lang="ts">
	import { X, Info } from '@lucide/svelte';
	import { environmentControllerPatch } from '$lib/client/sdk.gen';
	import type { EnvironmentResponseDto } from '$lib/client/types.gen';
	import { onMount } from 'svelte';
	import loader from '@monaco-editor/loader';

	type Props = { env: EnvironmentResponseDto; onClose: () => void; onUpdated: (e: EnvironmentResponseDto) => void };
	let { env, onClose, onUpdated }: Props = $props();

	let envVars = $state(env.env_var ?? '');
	// envVars is intentionally initialized once from env.env_var (user edits from there)
	let loading = $state(false);
	let error = $state('');
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
				value: envVars,
				language: 'ini',
				theme: 'vs-dark',
				fontSize: 13,
				fontFamily: 'monospace',
				minimap: { enabled: false },
				scrollBeyondLastLine: false,
				lineNumbers: 'on',
				wordWrap: 'on',
				automaticLayout: true,
				tabSize: 2,
				padding: { top: 8, bottom: 8 }
			});
			editorInstance.onDidChangeModelContent(() => { envVars = editorInstance.getValue(); });
			setTimeout(() => editorInstance?.layout(), 50);
		});
		return () => {
			disposed = true;
			editorInstance?.dispose();
		};
	});

	async function submit(e: SubmitEvent) {
		e.preventDefault(); error = ''; loading = true;
		try {
			const res = await environmentControllerPatch({ path: { id: env.id }, body: { env_var: envVars } });
			if (res.error || !res.data) throw new Error('Failed to update environment');
			onUpdated(res.data); onClose();
		} catch (err) { error = err instanceof Error ? err.message : 'Something went wrong'; }
		finally { loading = false; }
	}
</script>

<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1" aria-label="Close" onclick={onClose} onkeydown={() => {}}></div>
<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
	<div class="bg-card border border-border rounded-lg w-full max-w-2xl shadow-2xl pointer-events-auto">
		<div class="flex items-start justify-between px-6 pt-5 pb-3">
			<div>
				<h2 class="font-semibold">Project Environment</h2>
				<p class="text-sm text-muted-foreground mt-0.5">Update the env variables accessible to all services of this project.</p>
			</div>
			<button class="text-muted-foreground hover:text-foreground transition-colors ml-4 mt-0.5" onclick={onClose}><X class="w-4 h-4" /></button>
		</div>

		<div class="mx-6 mb-4">
			<div class="flex items-start gap-2 bg-primary/10 border border-primary/20 rounded-md px-3 py-2.5">
				<Info class="w-4 h-4 text-primary shrink-0 mt-0.5" />
				<p class="text-xs text-primary/90 leading-relaxed">
					Use this syntax to reference project-level variables:
					<code class="font-mono font-semibold">DATABASE_URL=${'{'}{'{'} project.DATABASE_URL {'}'}{'}'}</code>
				</p>
			</div>
		</div>

		<form onsubmit={submit} class="px-6 pb-5 flex flex-col gap-3">
			<p class="text-sm font-medium text-muted-foreground">Environment variables</p>
			<div bind:this={editorContainer} class="rounded-md overflow-hidden border border-border" style="height: 380px"></div>
			{#if error}<div class="rounded-md bg-destructive/10 border border-destructive/30 px-3 py-2 text-sm text-destructive">{error}</div>{/if}
			<div class="flex justify-end">
				<button type="submit" class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50" disabled={loading}>
					{#if loading}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>Updating…{:else}Update{/if}
				</button>
			</div>
		</form>
	</div>
</div>
