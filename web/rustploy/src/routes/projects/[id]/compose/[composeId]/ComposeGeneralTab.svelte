<script lang="ts">
	import { onMount } from 'svelte';
	import { RocketIcon, RefreshCw, Hammer, Play, Square, Terminal, Save } from '@lucide/svelte';
	import {
		composeControllerPatch,
		composeControllerPatchRawSource,
		composeControllerPatchCustomGitSource,
		composeControllerPatchGithubSource,
		composeControllerPatchGitlabSource,
		composeControllerPatchGiteaSource,
		composeControllerPatchBitbucketSource,
		composeControllerDeploy,
		composeControllerReload,
		composeControllerRedeploy,
		composeControllerStart,
		composeControllerStop
	} from '$lib/client/sdk.gen';
	import type { ComposeResponseDto } from '$lib/client/types.gen';
	import Switch from '$lib/components/Switch.svelte';
	import { withToast } from '$lib/toast';
	import BranchSelect from '$lib/components/BranchSelect.svelte';
	import loader from '@monaco-editor/loader';

	type Props = { compose: ComposeResponseDto; onUpdated: (c: ComposeResponseDto) => void };
	let { compose, onUpdated }: Props = $props();

	// ── Action states ──────────────────────────────────────────────
	let deploying  = $state(false);
	let reloading  = $state(false);
	let redeploying = $state(false);
	let starting   = $state(false);
	let stopping   = $state(false);
	let autoDeploy = $state(true);
	let cleanCache = $state(false);

	async function run(setter: (v: boolean) => void, fn: () => Promise<any>) {
		setter(true); try { await fn(); } finally { setter(false); }
	}

	// ── Provider ───────────────────────────────────────────────────
	const PROVIDERS = [
		{ id: 'GITHUB',    label: 'GitHub' },
		{ id: 'GITLAB',    label: 'GitLab' },
		{ id: 'GITEA',     label: 'Gitea' },
		{ id: 'BITBUCKET', label: 'Bitbucket' },
		{ id: 'GIT',       label: 'Git' },
		{ id: 'RAW',       label: 'Raw' },
	];

	let provider = $state(
		compose.source_type === 'GITHUB' ? 'GITHUB'
		: compose.source_type === 'GITLAB' ? 'GITLAB'
		: compose.source_type === 'GITEA' ? 'GITEA'
		: compose.source_type === 'BITBUCKET' ? 'BITBUCKET'
		: compose.source_type === 'GIT' ? 'GIT'
		: 'RAW'
	);

	// Hosted git fields
	let hostedBranch    = $state(compose.branch ?? '');
	let hostedBuildPath = $state(compose.compose_path ?? './docker-compose.yml');

	// Custom git fields
	let gitUrl  = $state(compose.custom_git_url ?? '');
	let gitBranch = $state(compose.custom_git_branch ?? '');
	let gitComposePath = $state(compose.compose_path ?? './docker-compose.yml');

	// RAW compose file
	let composeFile = $state(compose.compose_file ?? '');

	let srcSaving = $state(false);
	let srcError  = $state('');
	let srcSaved  = $state(false);

	function hostedOwner(): string {
		if (provider === 'GITLAB')    return compose.gitlab_owner ?? '';
		if (provider === 'GITEA')     return compose.gitea_owner ?? '';
		if (provider === 'BITBUCKET') return compose.bitbucket_owner ?? '';
		return compose.owner ?? '';
	}
	function hostedRepo(): string {
		if (provider === 'GITLAB')    return compose.gitlab_repository ?? '';
		if (provider === 'GITEA')     return compose.gitea_repository ?? '';
		if (provider === 'BITBUCKET') return compose.bitbucket_repository ?? '';
		return compose.repository ?? '';
	}

	async function saveSource() {
		srcSaving = true; srcError = ''; srcSaved = false;
		try {
			let res: any;
			if (provider === 'RAW') {
				res = await composeControllerPatchRawSource({
					path: { id: compose.id },
					body: { compose_file: composeFile } as any
				});
			} else if (provider === 'GIT') {
				res = await composeControllerPatchCustomGitSource({
					path: { id: compose.id },
					body: { custom_git_url: gitUrl, custom_git_branch: gitBranch || 'main' } as any
				});
			} else if (provider === 'GITHUB') {
				res = await composeControllerPatchGithubSource({
					path: { id: compose.id },
					body: { owner: compose.owner ?? '', repository: compose.repository ?? '', branch: hostedBranch } as any
				});
			} else if (provider === 'GITLAB') {
				res = await composeControllerPatchGitlabSource({
					path: { id: compose.id },
					body: { gitlab_owner: compose.gitlab_owner ?? '', gitlab_repository: compose.gitlab_repository ?? '', gitlab_branch: hostedBranch } as any
				});
			} else if (provider === 'GITEA') {
				res = await composeControllerPatchGiteaSource({
					path: { id: compose.id },
					body: { gitea_owner: compose.gitea_owner ?? '', gitea_repository: compose.gitea_repository ?? '', gitea_branch: hostedBranch } as any
				});
			} else if (provider === 'BITBUCKET') {
				res = await composeControllerPatchBitbucketSource({
					path: { id: compose.id },
					body: { bitbucket_owner: compose.bitbucket_owner ?? '', bitbucket_repository: compose.bitbucket_repository ?? '', bitbucket_branch: hostedBranch } as any
				});
			}
			if (res?.data) { onUpdated(res.data); srcSaved = true; setTimeout(() => (srcSaved = false), 2000); }
		} catch (e: any) { srcError = e?.message ?? 'Failed to save'; }
		finally { srcSaving = false; }
	}

	// ── Monaco editor (RAW mode) ───────────────────────────────────
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
		return () => { disposed = true; editorInstance?.dispose(); };
	});

	// Re-layout editor when switching to RAW
	$effect(() => {
		if (provider === 'RAW') setTimeout(() => editorInstance?.layout(), 60);
	});

	const inputCls = 'flex h-9 w-full rounded-md border border-input bg-secondary px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring';
</script>

<div class="flex flex-col gap-6 animate-fade-up">

	<!-- ── Deploy actions ── -->
	<section class="bg-card border border-border rounded-lg p-6">
		<h2 class="text-base font-semibold mb-4">Deploy Settings</h2>
		<div class="flex flex-wrap items-center gap-2">
			<button
				onclick={() => run(d => (deploying = d), async () => { await withToast(() => composeControllerDeploy({ path: { id: compose.id } }), { loading: 'Deploying…', success: 'Deploy triggered!', successDescription: 'Your compose service is being deployed.' }); })}
				disabled={deploying}
				class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50"
			>
				{#if deploying}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>{:else}<RocketIcon size={14} />{/if}
				Deploy
			</button>
			<button
				onclick={() => run(d => (reloading = d), () => withToast(() => composeControllerReload({ path: { id: compose.id } }), { loading: 'Reloading…', success: 'Reload triggered!' }))}
				disabled={reloading}
				class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent disabled:opacity-50"
			>
				{#if reloading}<div class="w-3.5 h-3.5 border-2 border-current/30 border-t-current rounded-full animate-spin"></div>{:else}<RefreshCw size={14} />{/if}
				Reload
			</button>
			<button
				onclick={() => run(d => (redeploying = d), () => withToast(() => composeControllerRedeploy({ path: { id: compose.id } }), { loading: 'Redeploying…', success: 'Redeploy triggered!' }))}
				disabled={redeploying}
				class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent disabled:opacity-50"
			>
				{#if redeploying}<div class="w-3.5 h-3.5 border-2 border-current/30 border-t-current rounded-full animate-spin"></div>{:else}<Hammer size={14} />{/if}
				Redeploy
			</button>
			<button
				onclick={() => run(d => (starting = d), () => withToast(() => composeControllerStart({ path: { id: compose.id } }), { loading: 'Starting…', success: 'Service started!' }))}
				disabled={starting}
				class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent disabled:opacity-50"
			>
				{#if starting}<div class="w-3.5 h-3.5 border-2 border-current/30 border-t-current rounded-full animate-spin"></div>{:else}<Play size={14} />{/if}
				Start
			</button>
			<button
				onclick={() => run(d => (stopping = d), () => withToast(() => composeControllerStop({ path: { id: compose.id } }), { loading: 'Stopping…', success: 'Service stopped!' }))}
				disabled={stopping}
				class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-destructive hover:bg-destructive/10 disabled:opacity-50"
			>
				{#if stopping}<div class="w-3.5 h-3.5 border-2 border-current/30 border-t-current rounded-full animate-spin"></div>{:else}<Square size={14} />{/if}
				Stop
			</button>
			<button class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent">
				<Terminal size={14} /> Terminal
			</button>
			<div class="ml-2 flex items-center gap-3">
				<div class="flex items-center gap-2">
					<Switch checked={autoDeploy} onchange={(v) => (autoDeploy = v)} />
					<span class="text-sm text-muted-foreground">Autodeploy</span>
				</div>
				<div class="flex items-center gap-2">
					<Switch checked={cleanCache} onchange={(v) => (cleanCache = v)} />
					<span class="text-sm text-muted-foreground">Clean Cache</span>
				</div>
			</div>
		</div>
	</section>

	<!-- ── Provider ── -->
	<section class="bg-card border border-border rounded-lg p-6">
		<h2 class="text-base font-semibold mb-1">Provider</h2>
		<p class="text-sm text-muted-foreground mb-4">Select the source of your compose file</p>

		<div class="flex flex-wrap gap-0 border-b border-border mb-5 -mx-1">
			{#each PROVIDERS as p}
				<button
					type="button"
					class="px-4 py-2 text-sm transition-colors border-b-2 -mb-px mx-1 {provider === p.id ? 'border-foreground text-foreground font-medium' : 'border-transparent text-muted-foreground hover:text-foreground'}"
					onclick={() => (provider = p.id)}
				>{p.label}</button>
			{/each}
		</div>

		<!-- RAW — Monaco editor -->
		{#if provider === 'RAW'}
			<p class="text-sm font-medium mb-1">Compose File</p>
			<p class="text-xs text-muted-foreground mb-3">Edit your Docker Compose YAML directly.</p>
			<div class="rounded-md overflow-hidden border border-border" style="height: 420px">
				<div bind:this={editorContainer} style="height: 100%; width: 100%"></div>
			</div>
			{#if srcError}<div class="mt-3 rounded-md bg-destructive/10 border border-destructive/30 px-3 py-2 text-sm text-destructive">{srcError}</div>{/if}
			<div class="flex justify-end items-center gap-3 mt-4">
				{#if srcSaved}<span class="text-sm text-green-500">Saved!</span>{/if}
				<button onclick={saveSource} disabled={srcSaving} class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50">
					{#if srcSaving}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>Saving…{:else}<Save size={13} />Save{/if}
				</button>
			</div>

		<!-- Hosted providers (GitHub/GitLab/Gitea/Bitbucket) -->
		{:else if ['GITHUB','GITLAB','GITEA','BITBUCKET'].includes(provider)}
			<div class="flex flex-col gap-4">
				<div class="flex flex-col gap-1.5">
					<label for="cgn-account" class="text-sm font-medium text-muted-foreground">{provider} Account</label>
					<select id="cgn-account" class="h-9 w-full rounded-md border border-input bg-secondary px-3 text-sm focus:outline-none focus:ring-1 focus:ring-ring">
						<option>Select a {provider} Account</option>
					</select>
				</div>
				<div class="flex flex-col gap-1.5">
					<label for="cgn-repo" class="text-sm font-medium text-muted-foreground">Repository</label>
					<select id="cgn-repo" class="h-9 w-full rounded-md border border-input bg-secondary px-3 text-sm focus:outline-none focus:ring-1 focus:ring-ring">
						<option>Select repository</option>
						{#if hostedRepo()}<option value={hostedRepo()} selected>{hostedOwner()}/{hostedRepo()}</option>{/if}
					</select>
				</div>
				<div class="grid grid-cols-2 gap-4">
					<div class="flex flex-col gap-1.5">
						<p class="text-sm font-medium text-muted-foreground">Branch</p>
						<BranchSelect
							value={hostedBranch}
							onchange={(v) => (hostedBranch = v)}
							owner={hostedOwner()}
							repo={hostedRepo()}
							{provider}
						/>
					</div>
					<div class="flex flex-col gap-1.5">
						<label for="hosted-compose-path" class="text-sm font-medium text-muted-foreground">Compose Path</label>
						<input id="hosted-compose-path" class={inputCls} placeholder="./docker-compose.yml" bind:value={hostedBuildPath} />
					</div>
				</div>
				{#if srcError}<div class="rounded-md bg-destructive/10 border border-destructive/30 px-3 py-2 text-sm text-destructive">{srcError}</div>{/if}
				<div class="flex justify-end items-center gap-3">
					{#if srcSaved}<span class="text-sm text-green-500">Saved!</span>{/if}
					<button onclick={saveSource} disabled={srcSaving} class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50">
						{#if srcSaving}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>Saving…{:else}<Save size={13} />Save{/if}
					</button>
				</div>
			</div>

		<!-- Custom GIT -->
		{:else if provider === 'GIT'}
			<div class="flex flex-col gap-4">
				<div class="flex flex-col gap-1.5">
					<label for="cgit-url" class="text-sm font-medium text-muted-foreground">Repository URL</label>
					<input id="cgit-url" class="{inputCls} font-mono" placeholder="https://github.com/user/repo.git" bind:value={gitUrl} />
					<p class="text-xs text-muted-foreground">Branches are fetched automatically as you type.</p>
				</div>
				<div class="flex flex-col gap-1.5">
					<label for="cgn-sshkey" class="text-sm font-medium text-muted-foreground">SSH Key</label>
					<select id="cgn-sshkey" class="h-9 w-full rounded-md border border-input bg-secondary px-3 text-sm focus:outline-none focus:ring-1 focus:ring-ring">
						<option value="">None (use HTTPS)</option>
					</select>
				</div>
				<div class="grid grid-cols-2 gap-4">
					<div class="flex flex-col gap-1.5">
						<p class="text-sm font-medium text-muted-foreground">Branch</p>
						<BranchSelect value={gitBranch} onchange={(v) => (gitBranch = v)} repoUrl={gitUrl} />
					</div>
					<div class="flex flex-col gap-1.5">
						<label for="cgit-compose-path" class="text-sm font-medium text-muted-foreground">Compose Path</label>
						<input id="cgit-compose-path" class={inputCls} placeholder="./docker-compose.yml" bind:value={gitComposePath} />
					</div>
				</div>
				{#if srcError}<div class="rounded-md bg-destructive/10 border border-destructive/30 px-3 py-2 text-sm text-destructive">{srcError}</div>{/if}
				<div class="flex justify-end items-center gap-3">
					{#if srcSaved}<span class="text-sm text-green-500">Saved!</span>{/if}
					<button onclick={saveSource} disabled={srcSaving || !gitUrl.trim()} class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50">
						{#if srcSaving}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>Saving…{:else}<Save size={13} />Save{/if}
					</button>
				</div>
			</div>
		{/if}
	</section>
</div>
