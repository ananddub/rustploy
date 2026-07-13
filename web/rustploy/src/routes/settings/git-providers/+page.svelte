<script lang="ts">
	import { goto } from '$app/navigation';
	import { GitBranch, Trash2, Users, ExternalLink, Import, Pencil, Loader2 } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Switch } from '$lib/components/ui/switch';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { toastSuccess, toastError } from '$lib/toast';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	const GITHUB_SVG    = `<path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0 0 24 12c0-6.63-5.37-12-12-12z" fill="currentColor"/>`;
	const GITLAB_SVG    = `<path d="M23.955 13.587l-1.342-4.135-2.664-8.189a.455.455 0 0 0-.867 0L16.418 9.45H7.582L4.918 1.263a.455.455 0 0 0-.867 0L1.386 9.452.044 13.587a.924.924 0 0 0 .331 1.023L12 23.054l11.625-8.443a.92.92 0 0 0 .33-1.024z" fill="#fc6d26"/>`;
	const BITBUCKET_SVG = `<path d="M.778 1.213a.768.768 0 0 0-.768.892l3.263 19.81c.084.5.515.868 1.022.873H19.95a.772.772 0 0 0 .77-.646l3.27-20.03a.768.768 0 0 0-.768-.891H.778zM14.52 15.53H9.522L8.17 8.466h7.561l-1.211 7.064z" fill="#2684ff"/>`;
	const GITEA_SVG     = `<path d="M15.876 13.374c-.645.218-1.458.338-2.35.338-1.793 0-3.193-.538-3.193-1.203s1.4-1.203 3.193-1.203c.892 0 1.705.12 2.35.338m-7.74 3.63c0 .88 2.416 1.594 5.39 1.594s5.39-.714 5.39-1.594v-2.61C17.635 15.286 16.01 15.7 13.526 15.7s-4.11-.414-5.39-1.307v2.61zm10.78-7.607c0-.879-2.416-1.594-5.39-1.594S8.136 8.518 8.136 9.397v1.307c1.279-.892 2.906-1.307 5.39-1.307s4.11.415 5.39 1.307V9.397zM12 0C5.373 0 0 5.373 0 12s5.373 12 12 12 12-5.373 12-12S18.627 0 12 0z" fill="#609926"/>`;

	type ProviderType = 'github' | 'gitlab' | 'bitbucket' | 'gitea';
	type GitProvider  = { id: string; name: string; type: ProviderType; isConfigured: boolean; isOwner: boolean; sharedWithOrg: boolean; createdAt: string; actionRequired?: boolean; };

	let providers = $state<GitProvider[]>([
		{ id:'g1', name:'rustploy-org',  type:'github',    isConfigured:true,  isOwner:true,  sharedWithOrg:false, createdAt:'2026-06-15T10:00:00Z' },
		{ id:'g2', name:'rustploy-team', type:'gitlab',    isConfigured:false, isOwner:true,  sharedWithOrg:true,  createdAt:'2026-05-20T08:00:00Z', actionRequired:true },
		{ id:'g3', name:'team-cloud',    type:'bitbucket', isConfigured:true,  isOwner:false, sharedWithOrg:true,  createdAt:'2026-04-10T12:00:00Z' },
		{ id:'g4', name:'self-hosted',   type:'gitea',     isConfigured:true,  isOwner:true,  sharedWithOrg:false, createdAt:'2026-03-22T09:00:00Z' },
	]);

	function svgFor(t: ProviderType) { return { github:GITHUB_SVG, gitlab:GITLAB_SVG, bitbucket:BITBUCKET_SVG, gitea:GITEA_SVG }[t]; }
	function labelFor(t: ProviderType) { return { github:'GitHub', gitlab:'GitLab', bitbucket:'Bitbucket', gitea:'Gitea' }[t]; }
	function fmtDate(iso: string) { return new Date(iso).toLocaleString(undefined,{year:'numeric',month:'2-digit',day:'2-digit',hour:'2-digit',minute:'2-digit',second:'2-digit'}); }

	async function toggleShare(id: string, val: boolean) {
		providers = providers.map(p => p.id === id ? {...p, sharedWithOrg:val} : p);
		toastSuccess(val ? 'Provider shared with organization' : 'Provider unshared');
	}

	// ─── Delete ───────────────────────────────────────────────────────────────────
	let confirmDeleteId = $state<string|null>(null);
	let deletingId      = $state<string|null>(null);

	async function deleteProvider(id: string) {
		deletingId = id;
		await new Promise(r => setTimeout(r, 400));
		providers = providers.filter(p => p.id !== id);
		deletingId = null; confirmDeleteId = null;
		toastSuccess('Git Provider deleted');
	}

	// ─── Add modal ────────────────────────────────────────────────────────────────
	let addType    = $state<ProviderType|null>(null);
	let saving     = $state(false);
	let modalError = $state('');
	let ghOrgMode  = $state(false);
	let ghOrgName  = $state('');
	let glName='', glUrl='https://gitlab.com', glAppId='', glAppSecret='', glRedirect='', glGroup='';
	let bbName='', bbUser='', bbPass='';
	let gtName='', gtUrl='', gtAppId='', gtSecret='', gtRedirect='';

	function openAdd(t: ProviderType) {
		addType=t; modalError=''; saving=false;
		glRedirect = typeof window !== 'undefined' ? `${window.location.origin}/api/providers/gitlab/callback` : '';
		gtRedirect = typeof window !== 'undefined' ? `${window.location.origin}/api/providers/gitea/callback`  : '';
	}

	async function submitAdd(e: SubmitEvent) {
		e.preventDefault(); modalError=''; saving=true;
		try {
			await new Promise(r => setTimeout(r, 500));
			const name = addType==='github' ? 'GitHub App' : addType==='gitlab' ? glName : addType==='bitbucket' ? bbName : gtName;
			providers = [...providers, {id:`g${Date.now()}`, name, type:addType!, isConfigured:true, isOwner:true, sharedWithOrg:false, createdAt:new Date().toISOString()}];
			toastSuccess(`${labelFor(addType!)} provider added`);
			addType = null;
		} catch { modalError='Failed to add provider'; }
		finally { saving=false; }
	}
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<GitBranch class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Git Providers</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto">
		<div class="w-full max-w-5xl mx-auto">
			<div class="rounded-xl border border-border bg-card">

				<div class="px-6 pt-5 pb-4 border-b border-border">
					<h2 class="text-xl font-bold flex items-center gap-2">
						<GitBranch class="w-5 h-5 text-muted-foreground" /> Git Providers
					</h2>
					<p class="text-sm text-muted-foreground mt-0.5">Connect your Git provider for authentication</p>
				</div>

				<div class="px-6 py-5 space-y-4">

					<!-- Available Providers buttons -->
					<div class="flex flex-col gap-2">
						{#if providers.length > 0}
							<span class="text-base font-medium">Available Providers</span>
						{/if}
						<div class="bg-muted/40 p-1 rounded-lg">
							<div class="flex flex-wrap items-center gap-3 p-3.5 rounded-lg bg-background border border-border w-full [&>button]:grow">
								{#each (['github','gitlab','bitbucket','gitea'] as ProviderType[]) as t (t)}
									<Button variant="secondary" class="gap-2 h-9" onclick={() => openAdd(t)}>
										<svg class="w-4 h-4 shrink-0" viewBox="0 0 24 24">{@html svgFor(t)}</svg>
										{labelFor(t)}
									</Button>
								{/each}
							</div>
						</div>
					</div>

					<!-- Empty state -->
					{#if providers.length === 0}
						<div class="flex flex-col items-center gap-2 py-12 text-muted-foreground">
							<GitBranch class="w-8 h-8 opacity-40" />
							<p class="text-base">No Git Providers configured</p>
							<p class="text-sm opacity-60">Click a provider above to get started</p>
						</div>

					{:else}
						<!-- Connected providers list -->
						<div class="flex flex-col gap-3">
							{#each providers as p (p.id)}
								<div class="bg-muted/40 p-1 rounded-lg">
									<div class="flex items-center justify-between p-4 rounded-lg bg-background border border-border gap-4">
										<!-- Left -->
										<div class="flex items-center gap-3 min-w-0">
											<svg class="w-6 h-6 shrink-0" viewBox="0 0 24 24">{@html svgFor(p.type)}</svg>
											<div class="min-w-0">
												<p class="text-sm font-semibold truncate">{p.name}</p>
												<p class="text-xs text-muted-foreground">{fmtDate(p.createdAt)}</p>
											</div>
											{#if !p.isOwner}
												<Badge variant="secondary" class="text-xs gap-1 shrink-0">
													<Users class="w-3 h-3" /> Shared
												</Badge>
											{/if}
										</div>
										<!-- Right -->
										<div class="flex items-center gap-1 shrink-0">
											{#if p.isOwner}
												<div class="flex items-center gap-1.5 px-2 border-r border-border mr-1">
													<Users class="w-4 h-4 text-muted-foreground" />
													<Switch checked={p.sharedWithOrg} onCheckedChange={(v) => toggleShare(p.id, !!v)} />
												</div>
												{#if p.actionRequired}
													<Badge variant="outline" class="text-xs mr-1">Action Required</Badge>
													<Button variant="ghost" size="icon" class="h-8 w-8" title="Complete setup">
														<Import class="w-4 h-4 text-primary" />
													</Button>
												{/if}
												{#if p.isConfigured && p.type === 'github'}
													<Button variant="ghost" size="icon" class="h-8 w-8" title="Open GitHub App">
														<ExternalLink class="w-4 h-4 text-primary" />
													</Button>
												{/if}
												<Button variant="ghost" size="icon" class="h-8 w-8" title="Edit" onclick={() => openAdd(p.type)}>
													<Pencil class="w-4 h-4" />
												</Button>
												<Button variant="ghost" size="icon"
													class="h-8 w-8 group hover:bg-red-500/10"
													onclick={() => (confirmDeleteId = p.id)}
													disabled={deletingId === p.id} title="Delete">
													{#if deletingId === p.id}
														<Loader2 class="w-4 h-4 animate-spin" />
													{:else}
														<Trash2 class="w-4 h-4 text-muted-foreground group-hover:text-red-500 transition-colors" />
													{/if}
												</Button>
											{:else}
												<span class="text-xs text-muted-foreground/60 italic px-2">View only</span>
											{/if}
										</div>
									</div>
								</div>
							{/each}
						</div>
					{/if}
				</div>
			</div>
		</div>
	</main>
</PageLayout>

<!-- ── Add Provider Modal ─────────────────────────────────────────────────── -->
{#if addType}
	<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1"
		onclick={() => (addType=null)} onkeydown={() => {}}></div>
	<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
		<form onsubmit={submitAdd}
			class="bg-card border border-border rounded-xl shadow-2xl w-full max-w-2xl pointer-events-auto flex flex-col gap-5 p-6 max-h-[90vh] overflow-y-auto">
			<div class="flex items-center justify-between">
				<h2 class="text-base font-semibold flex items-center gap-2">
					<svg class="w-5 h-5" viewBox="0 0 24 24">{@html svgFor(addType)}</svg>
					{labelFor(addType)} Provider
				</h2>
				<button type="button" onclick={() => (addType=null)}
					class="text-muted-foreground hover:text-foreground p-1 rounded hover:bg-accent">✕</button>
			</div>

			{#if addType === 'github'}
				<p class="text-sm text-muted-foreground">To integrate GitHub, create and install a GitHub App. This only takes a few minutes.</p>
				<div class="flex items-center justify-between rounded-lg border border-border px-4 py-3">
					<div>
						<p class="text-sm font-medium">GitHub Organization</p>
						<p class="text-xs text-muted-foreground">Enable for an organization account instead of personal</p>
					</div>
					<Switch bind:checked={ghOrgMode} />
				</div>
				{#if ghOrgMode}
					<div class="space-y-1.5">
						<Label for="gh-org" class="text-xs">Organization Name <span class="text-destructive">*</span></Label>
						<Input id="gh-org" bind:value={ghOrgName} placeholder="my-organization" />
					</div>
				{/if}
				<div class="flex justify-end gap-2">
					<Button type="button" variant="outline" size="sm" onclick={() => (addType=null)}>Cancel</Button>
					<Button type="submit" size="sm" disabled={saving} class="gap-1.5">
						{#if saving}<Loader2 class="w-3.5 h-3.5 animate-spin" />{/if}
						Create GitHub App
					</Button>
				</div>

			{:else if addType === 'gitlab'}
				<p class="text-sm text-muted-foreground">Register a GitLab OAuth application and provide the credentials below.</p>
				<div class="grid grid-cols-2 gap-4">
					<div class="space-y-1.5">
						<Label for="gl-name" class="text-xs">Name <span class="text-destructive">*</span></Label>
						<Input id="gl-name" bind:value={glName} placeholder="My GitLab" required />
					</div>
					<div class="space-y-1.5">
						<Label for="gl-url" class="text-xs">GitLab URL <span class="text-destructive">*</span></Label>
						<Input id="gl-url" bind:value={glUrl} placeholder="https://gitlab.com" required />
					</div>
					<div class="space-y-1.5">
						<Label for="gl-appid" class="text-xs">Application ID <span class="text-destructive">*</span></Label>
						<Input id="gl-appid" bind:value={glAppId} placeholder="Application ID" required />
					</div>
					<div class="space-y-1.5">
						<Label for="gl-secret" class="text-xs">Application Secret <span class="text-destructive">*</span></Label>
						<Input id="gl-secret" type="password" bind:value={glAppSecret} placeholder="Secret" required />
					</div>
					<div class="col-span-2 space-y-1.5">
						<Label for="gl-redirect" class="text-xs">Redirect URI</Label>
						<Input id="gl-redirect" bind:value={glRedirect} class="font-mono text-xs" />
					</div>
					<div class="col-span-2 space-y-1.5">
						<Label for="gl-group" class="text-xs">Group Name <span class="text-muted-foreground">(optional)</span></Label>
						<Input id="gl-group" bind:value={glGroup} placeholder="my-gitlab-group" />
					</div>
				</div>
				{#if modalError}<p class="text-xs text-destructive">{modalError}</p>{/if}
				<div class="flex justify-end gap-2">
					<Button type="button" variant="outline" size="sm" onclick={() => (addType=null)}>Cancel</Button>
					<Button type="submit" size="sm" disabled={saving} class="gap-1.5">
						{#if saving}<Loader2 class="w-3.5 h-3.5 animate-spin" />{/if} Save
					</Button>
				</div>

			{:else if addType === 'bitbucket'}
				<p class="text-sm text-muted-foreground">Connect your Bitbucket account using your username and an App Password.</p>
				<div class="space-y-4">
					<div class="space-y-1.5">
						<Label for="bb-name" class="text-xs">Name <span class="text-destructive">*</span></Label>
						<Input id="bb-name" bind:value={bbName} placeholder="My Bitbucket" required />
					</div>
					<div class="space-y-1.5">
						<Label for="bb-user" class="text-xs">Username <span class="text-destructive">*</span></Label>
						<Input id="bb-user" bind:value={bbUser} placeholder="bitbucket-username" required />
					</div>
					<div class="space-y-1.5">
						<Label for="bb-pass" class="text-xs">App Password <span class="text-destructive">*</span></Label>
						<Input id="bb-pass" type="password" bind:value={bbPass} placeholder="App Password" required />
					</div>
				</div>
				{#if modalError}<p class="text-xs text-destructive">{modalError}</p>{/if}
				<div class="flex justify-end gap-2">
					<Button type="button" variant="outline" size="sm" onclick={() => (addType=null)}>Cancel</Button>
					<Button type="submit" size="sm" disabled={saving} class="gap-1.5">
						{#if saving}<Loader2 class="w-3.5 h-3.5 animate-spin" />{/if} Save
					</Button>
				</div>

			{:else if addType === 'gitea'}
				<p class="text-sm text-muted-foreground">Register a Gitea OAuth application and provide the credentials below.</p>
				<div class="grid grid-cols-2 gap-4">
					<div class="space-y-1.5">
						<Label for="gt-name" class="text-xs">Name <span class="text-destructive">*</span></Label>
						<Input id="gt-name" bind:value={gtName} placeholder="My Gitea" required />
					</div>
					<div class="space-y-1.5">
						<Label for="gt-url" class="text-xs">Gitea URL <span class="text-destructive">*</span></Label>
						<Input id="gt-url" bind:value={gtUrl} placeholder="https://gitea.example.com" required />
					</div>
					<div class="space-y-1.5">
						<Label for="gt-appid" class="text-xs">Client ID <span class="text-destructive">*</span></Label>
						<Input id="gt-appid" bind:value={gtAppId} placeholder="Client ID" required />
					</div>
					<div class="space-y-1.5">
						<Label for="gt-secret" class="text-xs">Client Secret <span class="text-destructive">*</span></Label>
						<Input id="gt-secret" type="password" bind:value={gtSecret} placeholder="Secret" required />
					</div>
					<div class="col-span-2 space-y-1.5">
						<Label for="gt-redirect" class="text-xs">Redirect URI</Label>
						<Input id="gt-redirect" bind:value={gtRedirect} class="font-mono text-xs" />
					</div>
				</div>
				{#if modalError}<p class="text-xs text-destructive">{modalError}</p>{/if}
				<div class="flex justify-end gap-2">
					<Button type="button" variant="outline" size="sm" onclick={() => (addType=null)}>Cancel</Button>
					<Button type="submit" size="sm" disabled={saving} class="gap-1.5">
						{#if saving}<Loader2 class="w-3.5 h-3.5 animate-spin" />{/if} Save
					</Button>
				</div>
			{/if}
		</form>
	</div>
{/if}

<!-- ── Delete Confirm ─────────────────────────────────────────────────────── -->
{#if confirmDeleteId}
	{@const target = providers.find(p => p.id === confirmDeleteId)}
	<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1"
		onclick={() => (confirmDeleteId=null)} onkeydown={() => {}}></div>
	<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
		<div class="bg-card border border-border rounded-xl shadow-2xl w-full max-w-sm p-6 pointer-events-auto flex flex-col gap-4">
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 rounded-full bg-destructive/10 flex items-center justify-center shrink-0">
					<Trash2 class="w-5 h-5 text-destructive" />
				</div>
				<div>
					<h2 class="text-sm font-semibold">Delete Git Provider</h2>
					<p class="text-xs text-muted-foreground mt-0.5">
						{#if target?.sharedWithOrg}
							This provider is shared with the organization. Deleting it will remove access for all members.
						{:else}
							Are you sure you want to delete <strong class="text-foreground">{target?.name}</strong>?
						{/if}
						This cannot be undone.
					</p>
				</div>
			</div>
			<div class="flex justify-end gap-2">
				<Button variant="outline" size="sm" onclick={() => (confirmDeleteId=null)}>Cancel</Button>
				<Button variant="destructive" size="sm"
					onclick={() => deleteProvider(confirmDeleteId!)}
					disabled={deletingId !== null} class="gap-1.5">
					{#if deletingId}<Loader2 class="w-3.5 h-3.5 animate-spin" /> Deleting…
					{:else}<Trash2 class="w-3.5 h-3.5" /> Delete{/if}
				</Button>
			</div>
		</div>
	</div>
{/if}
