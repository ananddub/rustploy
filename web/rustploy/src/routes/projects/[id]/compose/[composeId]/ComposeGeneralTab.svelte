<script lang="ts">
	import { onMount } from 'svelte';
	import { RocketIcon, RefreshCw, Hammer, Play, Terminal } from '@lucide/svelte';
	import { composeControllerPatch, composeControllerDeploy, composeControllerReload, composeControllerRedeploy, composeControllerStart } from '$lib/client/sdk.gen';
	import type { ComposeResponseDto } from '$lib/client/types.gen';
	import loader from '@monaco-editor/loader';

	type Props = { compose: ComposeResponseDto; onUpdated: (c: ComposeResponseDto) => void };
	let { compose, onUpdated }: Props = $props();

	const PROVIDERS = [{ id: 'GIT', label: 'Git' }, { id: 'RAW', label: 'Raw' }];
	let provider = $state(compose.source_type ?? 'RAW');
	let composeFile = $state(compose.compose_file ?? '');
	let gitUrl = $state(compose.custom_git_url ?? '');
	let gitBranch = $state(compose.custom_git_branch ?? '');
	let autoDeploy = $state(true);
	let cleanCache = $state(false);
	let saving = $state(false);

	let editorContainer = $state<HTMLDivElement | null>(null);
	let editorInstance: any = null;

	onMount(() => {
		let disposed = false;
		loader.init().then((monaco) => {
			if (disposed || !editorContainer) return;

			if (!monaco.languages.getLanguages().some((l: any) => l.id === 'yaml')) {
				monaco.languages.register({ id: 'yaml' });
				monaco.languages.setMonarchTokensProvider('yaml', {
					tokenizer: {
						root: [
							[/#.*$/, 'comment'],
							[/[a-zA-Z_][\w.-]*\s*:/, 'key'],
							[/"([^"\\]|\\.)*"/, 'string'],
							[/'([^'\\]|\\.)*'/, 'string'],
							[/\b\d+(\.\d+)?\b/, 'number'],
							[/[[\]{}]/, 'delimiter.bracket'],
							[/[|>][-+]?\d*/, 'keyword']
						]
					}
				} as any);
			}

			editorInstance = monaco.editor.create(editorContainer!, {
				value: composeFile,
				language: 'yaml',
				theme: 'vs-dark',
				fontSize: 13,
				fontFamily: 'monospace',
				minimap: { enabled: false },
				scrollBeyondLastLine: false,
				lineNumbers: 'on',
				wordWrap: 'off',
				automaticLayout: true,
				padding: { top: 8, bottom: 8 }
			});
			editorInstance.onDidChangeModelContent(() => { composeFile = editorInstance.getValue(); });
			setTimeout(() => editorInstance?.layout(), 50);
		});
		return () => {
			disposed = true;
			editorInstance?.dispose();
		};
	});

	async function save() {
		saving = true;
		try {
			const res = await composeControllerPatch({ path: { id: compose.id }, body: { compose_file: composeFile } });
			if (res.data) onUpdated(res.data);
		} finally { saving = false; }
	}
</script>

<div class="flex flex-col gap-6">
	<section class="bg-card border border-border rounded-lg p-6">
		<div class="flex items-center justify-between mb-4">
			<div>
				<h2 class="text-base font-semibold">Deploy Settings</h2>
				<p class="text-sm text-muted-foreground mt-0.5">Deploy and manage this compose service.</p>
			</div>
		</div>
		<div class="flex flex-wrap items-center gap-2">
			<button onclick={async () => await composeControllerDeploy({ path: { id: compose.id } })} class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90">
				<RocketIcon class="w-3.5 h-3.5" /> Deploy
			</button>
			<button onclick={async () => await composeControllerReload({ path: { id: compose.id } })} class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent">
				<RefreshCw class="w-3.5 h-3.5" /> Reload
			</button>
			<button onclick={async () => await composeControllerRedeploy({ path: { id: compose.id } })} class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent">
				<Hammer class="w-3.5 h-3.5" /> Redeploy
			</button>
			<button onclick={async () => await composeControllerStart({ path: { id: compose.id } })} class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent">
				<Play class="w-3.5 h-3.5" /> Start
			</button>
			<button class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent">
				<Terminal class="w-3.5 h-3.5" /> Terminal
			</button>
			<label class="flex items-center gap-2 text-sm text-muted-foreground" for="composegeneraltab-1">Autodeploy <input type="checkbox" bind:checked={autoDeploy} /></label>
			<label class="flex items-center gap-2 text-sm text-muted-foreground">Clean Cache <input type="checkbox" bind:checked={cleanCache} /></label>
		</div>
	</section>

	<section class="bg-card border border-border rounded-lg p-6">
		<h2 class="text-base font-semibold mb-4">Provider</h2>
		<div class="flex gap-0 border-b border-border mb-5">
			{#each PROVIDERS as p}
				<button
					class="px-4 py-2 text-sm transition-colors border-b-2 -mb-px {provider === p.id ? 'border-foreground text-foreground font-medium' : 'border-transparent text-muted-foreground hover:text-foreground'}"
					onclick={() => { provider = p.id; if (p.id === 'RAW') setTimeout(() => editorInstance?.layout(), 50); }}
				>{p.label}</button>
			{/each}
		</div>

		{#if provider === 'RAW'}
			<p class="text-sm font-medium mb-1">Compose File</p>
			<p class="text-xs text-muted-foreground mb-3">Configure your Docker Compose file for this service.</p>
			<div class="rounded-md overflow-hidden border border-border" style="height: 420px">
				<div bind:this={editorContainer} style="height: 100%; width: 100%"></div>
			</div>
			<div class="flex justify-end mt-4">
				<button class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50" onclick={save} disabled={saving}>
					{#if saving}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>Saving…{:else}Save{/if}
				</button>
			</div>
		{:else}
			<div class="flex flex-col gap-4">
				<div class="flex flex-col gap-1.5">
					<label class="text-sm font-medium text-muted-foreground">Repository URL</label>

					<input id="composegeneraltab-1"  class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm font-mono placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring" placeholder="https://github.com/user/repo.git" bind:value={gitUrl} />
				</div>
				<div class="flex flex-col gap-1.5">
					<label class="text-sm font-medium text-muted-foreground" for="composegeneraltab-2">Branch</label>

					<input id="composegeneraltab-2"  class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring" placeholder="main" bind:value={gitBranch} />
				</div>
				<div class="flex justify-end">
					<button class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50" onclick={save} disabled={saving}>
						{#if saving}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>Saving…{:else}Save{/if}
					</button>
				</div>
			</div>
		{/if}
	</section>
</div>
