<script lang="ts">
	import { RocketIcon, RefreshCw, Hammer, Play, Square, Terminal, Save, Info, Settings } from '@lucide/svelte';
	import {
		applicationControllerPatch,
		applicationControllerPatchBuild,
		applicationControllerPatchDockerSource,
		applicationControllerPatchCustomGitSource,
		applicationControllerPatchGithubSource,
		applicationControllerPatchGitlabSource,
		applicationControllerPatchGiteaSource,
		applicationControllerPatchBitbucketSource,
		applicationControllerDeploy,
		applicationControllerReload,
		applicationControllerRebuild,
		applicationControllerStart,
		applicationControllerCancel
	} from '$lib/client/sdk.gen';
	import type { ApplicationResponseDto } from '$lib/client/types.gen';
	import Switch from '$lib/components/Switch.svelte';
	import { withToast } from '$lib/toast';
	import BranchSelect from '$lib/components/BranchSelect.svelte';

	type Props = { app: ApplicationResponseDto; onUpdated: (a: ApplicationResponseDto) => void };
	let { app, onUpdated }: Props = $props();

	// ── Action states ──────────────────────────────────────────────
	let deploying  = $state(false);
	let reloading  = $state(false);
	let rebuilding = $state(false);
	let starting   = $state(false);
	let cancelling = $state(false);
	let autoDeploy = $state(true);
	let cleanCache = $state(false);

	async function run(
		setter: (v: boolean) => void,
		fn: () => Promise<any>,
		label: string
	) {
		setter(true);
		try {
			await withToast(fn, {
				loading: `${label}ing…`,
				success: `${label} triggered successfully`,
				error: `${label} failed`
			});
		} finally { setter(false); }
	}

	// ── Provider / Source ─────────────────────────────────────────
	const PROVIDERS = [
		{ id: 'GITHUB',    label: 'GitHub' },
		{ id: 'GITLAB',    label: 'GitLab' },
		{ id: 'GITEA',     label: 'Gitea' },
		{ id: 'BITBUCKET', label: 'Bitbucket' },
		{ id: 'GIT',       label: 'Git' },
		{ id: 'DOCKER',    label: 'Docker' },
		{ id: 'DROP',      label: 'Drop' },
	];

	let provider = $state(app.source_type ?? 'GIT');

	// Hosted git (GitHub/GitLab/Gitea/Bitbucket)
	let hostedBranch    = $state(app.branch ?? '');
	let hostedBuildPath = $state('/');
	let hostedWatchPaths = $state('');
	let hostedSubmodules = $state(false);

	// Custom Git
	let gitUrl       = $state(app.custom_git_url ?? '');
	let gitBranch    = $state(app.custom_git_branch ?? '');
	let gitBuildPath = $state('/');
	let gitWatchPaths = $state('');
	let gitSubmodules = $state(false);

	// Docker
	let dockerImage = $state(app.docker_image ?? '');
	let dockerUser  = $state('');
	let dockerPass  = $state('');
	let registryUrl = $state(app.registry_url ?? '');

	let srcSaving = $state(false);
	let srcError  = $state('');
	let srcSaved  = $state(false);

	// helpers for hosted
	function hostedOwner(): string {
		if (provider === 'GITLAB')    return app.gitlab_owner ?? '';
		if (provider === 'GITEA')     return app.gitea_owner ?? '';
		if (provider === 'BITBUCKET') return app.bitbucket_owner ?? '';
		return app.owner ?? '';
	}
	function hostedRepo(): string {
		if (provider === 'GITLAB')    return app.gitlab_repository ?? '';
		if (provider === 'GITEA')     return app.gitea_repository ?? '';
		if (provider === 'BITBUCKET') return app.bitbucket_repository ?? '';
		return app.repository ?? '';
	}

	async function saveSource() {
		srcSaving = true; srcError = ''; srcSaved = false;
		try {
			let res: any;
			if (provider === 'DOCKER') {
				res = await applicationControllerPatchDockerSource({
					path: { id: app.id },
					body: {
						docker_image: dockerImage,
						...(dockerUser ? { docker_username: dockerUser } : {}),
						...(dockerPass ? { docker_password: dockerPass } : {}),
						...(registryUrl ? { registry_url: registryUrl } : {})
					}
				});
			} else if (provider === 'GIT') {
				res = await applicationControllerPatchCustomGitSource({
					path: { id: app.id },
					body: {
						custom_git_url: gitUrl,
						custom_git_branch: gitBranch || 'main',
						custom_git_build_path: gitBuildPath || '/'
					} as any
				});
			} else if (provider === 'GITHUB') {
				res = await applicationControllerPatchGithubSource({
					path: { id: app.id },
					body: { owner: app.owner ?? '', repository: app.repository ?? '', branch: hostedBranch, build_path: hostedBuildPath } as any
				});
			} else if (provider === 'GITLAB') {
				res = await applicationControllerPatchGitlabSource({
					path: { id: app.id },
					body: { gitlab_owner: app.gitlab_owner ?? '', gitlab_repository: app.gitlab_repository ?? '', gitlab_branch: hostedBranch, gitlab_build_path: hostedBuildPath } as any
				});
			} else if (provider === 'GITEA') {
				res = await applicationControllerPatchGiteaSource({
					path: { id: app.id },
					body: { gitea_owner: app.gitea_owner ?? '', gitea_repository: app.gitea_repository ?? '', gitea_branch: hostedBranch, gitea_build_path: hostedBuildPath } as any
				});
			} else if (provider === 'BITBUCKET') {
				res = await applicationControllerPatchBitbucketSource({
					path: { id: app.id },
					body: { bitbucket_owner: app.bitbucket_owner ?? '', bitbucket_repository: app.bitbucket_repository ?? '', bitbucket_branch: hostedBranch } as any
				});
			}
			if (res?.data) { onUpdated(res.data); srcSaved = true; setTimeout(() => (srcSaved = false), 2000); }
		} catch (e: any) { srcError = e?.message ?? 'Failed to save'; }
		finally { srcSaving = false; }
	}

	// ── Build Type ────────────────────────────────────────────────
	const BUILD_TYPES = [
		{ id: 'DOCKERFILE',        label: 'Dockerfile' },
		{ id: 'RAILPACK',          label: 'Railpack', isNew: true },
		{ id: 'NIXPACKS',          label: 'Nixpacks' },
		{ id: 'HEROKU_BUILDPACKS', label: 'Heroku Buildpacks' },
		{ id: 'PAKETO_BUILDPACKS', label: 'Paketo Buildpacks' },
		{ id: 'STATIC',            label: 'Static' },
	];

	let buildType   = $state(app.build_type ?? 'NIXPACKS');
	let publishDir  = $state('');
	let buildSaving = $state(false);
	let buildError  = $state('');
	let buildSaved  = $state(false);

	async function saveBuildType() {
		buildSaving = true; buildError = ''; buildSaved = false;
		try {
			const res = await applicationControllerPatch({
				path: { id: app.id },
				body: { build_type: buildType }
			});
			if (res.data) { onUpdated(res.data); buildSaved = true; setTimeout(() => (buildSaved = false), 2000); }
		} catch (e: any) { buildError = e?.message ?? 'Failed'; }
		finally { buildSaving = false; }
	}

	const inputCls = 'flex h-9 w-full rounded-md border border-input bg-secondary px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring';
</script>

<div class="flex flex-col gap-6 animate-fade-up">

	<!-- ── Deploy Settings ── -->
	<section class="bg-card border border-border rounded-lg p-6">
		<h2 class="text-base font-semibold mb-4">Deploy Settings</h2>
		<div class="flex flex-wrap items-center gap-2">
			<button
				onclick={() => run(d => (deploying = d), () => applicationControllerDeploy({ path: { id: app.id } }), 'Deploy')}
				disabled={deploying}
				class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50"
			>
				{#if deploying}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>{:else}<RocketIcon size={14} />{/if}
				Deploy
			</button>
			<button
				onclick={() => run(d => (reloading = d), () => applicationControllerReload({ path: { id: app.id } }), 'Reload')}
				disabled={reloading}
				class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent disabled:opacity-50"
			>
				{#if reloading}<div class="w-3.5 h-3.5 border-2 border-current/30 border-t-current rounded-full animate-spin"></div>{:else}<RefreshCw size={14} />{/if}
				Reload
			</button>
			<button
				onclick={() => run(d => (rebuilding = d), () => applicationControllerRebuild({ path: { id: app.id } }), 'Rebuild')}
				disabled={rebuilding}
				class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent disabled:opacity-50"
			>
				{#if rebuilding}<div class="w-3.5 h-3.5 border-2 border-current/30 border-t-current rounded-full animate-spin"></div>{:else}<Hammer size={14} />{/if}
				Rebuild
			</button>
			<button
				onclick={() => run(d => (starting = d), () => applicationControllerStart({ path: { id: app.id } }), 'Start')}
				disabled={starting}
				class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent disabled:opacity-50"
			>
				{#if starting}<div class="w-3.5 h-3.5 border-2 border-current/30 border-t-current rounded-full animate-spin"></div>{:else}<Play size={14} />{/if}
				Start
			</button>
			<button
				onclick={() => run(d => (cancelling = d), () => applicationControllerCancel({ path: { id: app.id } }), 'Cancel')}
				disabled={cancelling}
				class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-destructive hover:bg-destructive/10 disabled:opacity-50"
			>
				{#if cancelling}<div class="w-3.5 h-3.5 border-2 border-current/30 border-t-current rounded-full animate-spin"></div>{:else}<Square size={14} />{/if}
				Cancel
			</button>
			<button class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent">
				<Terminal size={14} /> Terminal
			</button>

			<div class="ml-2 flex items-center gap-3">
				<div class="flex items-center gap-2">
					<Switch
						checked={autoDeploy}
						onchange={(v) => (autoDeploy = v)}
					/>
					<span class="text-sm text-muted-foreground">Autodeploy</span>
				</div>
				<div class="flex items-center gap-2">
					<Switch
						checked={cleanCache}
						onchange={(v) => (cleanCache = v)}
					/>
					<span class="text-sm text-muted-foreground">Clean Cache</span>
				</div>
			</div>
		</div>
	</section>

	<!-- ── Provider ── -->
	<section class="bg-card border border-border rounded-lg p-6">
		<h2 class="text-base font-semibold mb-1">Provider</h2>
		<p class="text-sm text-muted-foreground mb-4">Select the source of your code</p>

		<!-- Provider tab bar -->
		<div class="flex flex-wrap gap-0 border-b border-border mb-5 -mx-1">
			{#each PROVIDERS as p}
				<button
					type="button"
					class="px-4 py-2 text-sm transition-colors border-b-2 -mb-px mx-1 {provider === p.id ? 'border-foreground text-foreground font-medium' : 'border-transparent text-muted-foreground hover:text-foreground'}"
					onclick={() => (provider = p.id)}
				>
					{p.label}
				</button>
			{/each}
		</div>

		<!-- GitHub / GitLab / Gitea / Bitbucket -->
		{#if ['GITHUB','GITLAB','GITEA','BITBUCKET'].includes(provider)}
			<div class="flex flex-col gap-4">
				<div class="flex flex-col gap-1.5">
					<label for="gen-account" class="text-sm font-medium text-muted-foreground">{provider} Account</label>
					<select id="gen-account" class="h-9 w-full rounded-md border border-input bg-secondary px-3 text-sm focus:outline-none focus:ring-1 focus:ring-ring">
						<option>Select a {provider} Account</option>
					</select>
				</div>
				<div class="flex flex-col gap-1.5">
					<label for="gen-repo" class="text-sm font-medium text-muted-foreground">Repository</label>
					<select id="gen-repo" class="h-9 w-full rounded-md border border-input bg-secondary px-3 text-sm focus:outline-none focus:ring-1 focus:ring-ring">
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
						<label for="hosted-build-path" class="text-sm font-medium text-muted-foreground">Build Path</label>
						<input id="hosted-build-path" class={inputCls} bind:value={hostedBuildPath} placeholder="/" />
					</div>
				</div>
				<div class="flex flex-col gap-1.5">
					<label for="hosted-watch" class="text-sm font-medium text-muted-foreground">Watch Paths</label>
					<input id="hosted-watch" class={inputCls} placeholder="src/**, dist/*.js" bind:value={hostedWatchPaths} />
					<p class="text-sm text-muted-foreground">Comma-separated glob patterns. Leave empty to watch everything.</p>
				</div>
				<Switch
					checked={hostedSubmodules}
					onchange={(v) => (hostedSubmodules = v)}
					label="Enable Submodules"
					description="Initialize and update git submodules during build"
				/>
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
					<label for="git-url" class="text-sm font-medium text-muted-foreground">Repository URL</label>
					<input id="git-url" class="{inputCls} font-mono" placeholder="https://github.com/user/repo.git" bind:value={gitUrl} />
					<p class="text-sm text-muted-foreground">Branches are fetched automatically as you type.</p>
				</div>
				<div class="flex flex-col gap-1.5">
					<label for="gen-sshkey" class="text-sm font-medium text-muted-foreground">SSH Key</label>
					<select id="gen-sshkey" class="h-9 w-full rounded-md border border-input bg-secondary px-3 text-sm focus:outline-none focus:ring-1 focus:ring-ring">
						<option value="">None (use HTTPS)</option>
					</select>
				</div>
				<div class="grid grid-cols-2 gap-4">
					<div class="flex flex-col gap-1.5">
						<p class="text-sm font-medium text-muted-foreground">Branch</p>
						<BranchSelect
							value={gitBranch}
							onchange={(v) => (gitBranch = v)}
							repoUrl={gitUrl}
						/>
					</div>
					<div class="flex flex-col gap-1.5">
						<label for="git-build-path" class="text-sm font-medium text-muted-foreground">Build Path</label>
						<input id="git-build-path" class={inputCls} bind:value={gitBuildPath} placeholder="/" />
					</div>
				</div>
				<div class="flex flex-col gap-1.5">
					<label for="git-watch" class="text-sm font-medium text-muted-foreground">Watch Paths</label>
					<input id="git-watch" class={inputCls} placeholder="src/**, dist/*.js" bind:value={gitWatchPaths} />
					<p class="text-sm text-muted-foreground">Comma-separated glob patterns. Leave empty to watch everything.</p>
				</div>
				<Switch
					checked={gitSubmodules}
					onchange={(v) => (gitSubmodules = v)}
					label="Enable Submodules"
					description="Initialize and update git submodules during build"
				/>
				{#if srcError}<div class="rounded-md bg-destructive/10 border border-destructive/30 px-3 py-2 text-sm text-destructive">{srcError}</div>{/if}
				<div class="flex justify-end items-center gap-3">
					{#if srcSaved}<span class="text-sm text-green-500">Saved!</span>{/if}
					<button onclick={saveSource} disabled={srcSaving} class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50">
						{#if srcSaving}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>Saving…{:else}<Save size={13} />Save{/if}
					</button>
				</div>
			</div>

		<!-- Docker -->
		{:else if provider === 'DOCKER'}
			<div class="flex flex-col gap-4">
				<div class="flex flex-col gap-1.5">
					<label for="dock-img" class="text-sm font-medium text-muted-foreground">Docker Image <span class="text-destructive">*</span></label>
					<input id="dock-img" class="{inputCls} font-mono" placeholder="nginx:latest" bind:value={dockerImage} />
					<p class="text-sm text-muted-foreground">e.g. nginx:latest · ghcr.io/user/repo:tag · custom-registry.com/image:v1</p>
				</div>
				<div class="grid grid-cols-2 gap-4">
					<div class="flex flex-col gap-1.5">
						<label for="dock-user" class="text-sm font-medium text-muted-foreground">Registry Username</label>
						<input id="dock-user" class={inputCls} placeholder="Optional" bind:value={dockerUser} />
					</div>
					<div class="flex flex-col gap-1.5">
						<label for="dock-pass" class="text-sm font-medium text-muted-foreground">Registry Password</label>
						<input id="dock-pass" class={inputCls} type="password" placeholder="Optional" bind:value={dockerPass} />
					</div>
				</div>
				<div class="flex flex-col gap-1.5">
					<label for="dock-reg" class="text-sm font-medium text-muted-foreground">Registry URL</label>
					<input id="dock-reg" class="{inputCls} font-mono" placeholder="registry.example.com" bind:value={registryUrl} />
					<p class="text-sm text-muted-foreground">Leave empty for Docker Hub</p>
				</div>
				{#if srcError}<div class="rounded-md bg-destructive/10 border border-destructive/30 px-3 py-2 text-sm text-destructive">{srcError}</div>{/if}
				<div class="flex justify-end items-center gap-3">
					{#if srcSaved}<span class="text-sm text-green-500">Saved!</span>{/if}
					<button onclick={saveSource} disabled={srcSaving || !dockerImage.trim()} class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50">
						{#if srcSaving}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>Saving…{:else}<Save size={13} />Save{/if}
					</button>
				</div>
			</div>

		<!-- Drop -->
		{:else if provider === 'DROP'}
			<div class="flex flex-col gap-4">
				<div class="border-2 border-dashed border-border rounded-lg p-10 flex flex-col items-center gap-2 text-muted-foreground hover:border-foreground/30 transition-colors cursor-pointer">
					<p class="text-sm font-medium">Drop your zip / tar.gz file here</p>
					<p class="text-xs opacity-60">or click to browse files</p>
				</div>
				<div class="flex justify-end">
					<button class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90">
						Upload
					</button>
				</div>
			</div>
		{/if}
	</section>

	<!-- ── Build Type ── -->
	<section class="bg-card border border-border rounded-lg p-6">
		<div class="flex items-center justify-between mb-1">
			<h2 class="text-base font-semibold">Build Type</h2>
			<Settings size={16} class="text-muted-foreground/40" />
		</div>
		<p class="text-sm text-muted-foreground mb-4">Select how your code is built</p>

		<div class="flex items-start gap-2.5 bg-primary/10 border border-primary/20 rounded-lg px-4 py-3 mb-5">
			<Info size={14} class="text-primary mt-0.5 shrink-0" />
			<p class="text-xs text-primary/90 leading-relaxed">
				Builders consume significant CPU and memory (recommended: 4+ GB RAM, 2+ cores).
				For production environments, review our Production Guide for best practices.
			</p>
		</div>

		<div class="flex flex-col gap-2.5 mb-5">
			{#each BUILD_TYPES as bt}
				<div class="flex items-center gap-3 cursor-pointer group">
					<div
						class="w-4 h-4 rounded-full border-2 flex items-center justify-center shrink-0 transition-colors {buildType === bt.id ? 'border-primary bg-primary' : 'border-border group-hover:border-foreground/40'}"
						onclick={() => (buildType = bt.id)}
						role="radio"
						aria-checked={buildType === bt.id}
						tabindex="0"
						onkeydown={(e) => e.key === 'Enter' && (buildType = bt.id)}
					>
						{#if buildType === bt.id}
							<div class="w-1.5 h-1.5 rounded-full bg-white"></div>
						{/if}
					</div>
					<span class="text-sm {buildType === bt.id ? 'text-foreground font-medium' : 'text-muted-foreground'}">{bt.label}</span>
					{#if bt.isNew}
						<span class="text-[10px] font-bold px-1.5 py-0.5 rounded bg-primary text-primary-foreground">NEW</span>
					{/if}
				</div>
			{/each}
		</div>

		{#if buildType === 'STATIC'}
			<div class="flex flex-col gap-1.5 mb-4">
				<label for="pub-dir" class="text-sm font-medium text-muted-foreground">Publish Directory</label>
				<p class="text-sm text-muted-foreground">Serve this directory via NGINX after the build phase.</p>
				<input id="pub-dir" class={inputCls} placeholder="dist" bind:value={publishDir} />
			</div>
		{/if}

		{#if buildError}<div class="mb-3 rounded-md bg-destructive/10 border border-destructive/30 px-3 py-2 text-sm text-destructive">{buildError}</div>{/if}
		<div class="flex justify-end items-center gap-3">
			{#if buildSaved}<span class="text-sm text-green-500">Saved!</span>{/if}
			<button onclick={saveBuildType} disabled={buildSaving} class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50">
				{#if buildSaving}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>Saving…{:else}<Save size={13} />Save{/if}
			</button>
		</div>
	</section>
</div>
