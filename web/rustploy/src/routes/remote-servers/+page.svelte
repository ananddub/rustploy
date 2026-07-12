<script lang="ts">
	import { goto } from '$app/navigation';
	import { RocketIcon, Server, Plus, RefreshCw, Globe, FileKey, CheckCircle, XCircle, Trash2, Power, TestTube2 } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import { remoteServerControllerList, remoteServerControllerCreate, remoteServerControllerDelete, remoteServerControllerActivate, remoteServerControllerDeactivate, remoteServerControllerTestConnection, sshKeyControllerList } from '$lib/client/sdk.gen';
	import type { RemoteServerResponseDto, SshKeyResponseDto } from '$lib/client/types.gen';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	let servers = $state<RemoteServerResponseDto[]>([]);
	let sshKeys = $state<SshKeyResponseDto[]>([]);
	let loading = $state(true);
	let showCreate = $state(false);
	let testingId = $state<number | null>(null);
	let togglingId = $state<number | null>(null);
	let deletingId = $state<number | null>(null);
	let testResults = $state<Record<number, 'ok' | 'fail'>>({});

	async function load() {
		loading = true;
		try {
			const [sRes, kRes] = await Promise.all([remoteServerControllerList(), sshKeyControllerList()]);
			servers = (sRes.data as RemoteServerResponseDto[]) ?? [];
			sshKeys = (kRes.data as SshKeyResponseDto[]) ?? [];
		} finally { loading = false; }
	}

	load();

	function keyName(id?: number) {
		if (!id) return null;
		return sshKeys.find(k => k.id === id)?.name ?? `Key #${id}`;
	}

	function statusDotClass(status: string) {
		switch (status) {
			case 'ACTIVE': return 'bg-green-500';
			case 'INACTIVE': return 'bg-muted-foreground/25';
			case 'CONNECTING': return 'bg-yellow-500 animate-pulse';
			default: return 'bg-red-500';
		}
	}

	function statusBadgeClass(status: string) {
		const map: Record<string, string> = {
			ACTIVE: 'bg-green-500/15 text-green-600',
			INACTIVE: 'bg-muted/50 text-muted-foreground',
			CONNECTING: 'bg-yellow-500/15 text-yellow-600'
		};
		return map[status] ?? 'bg-red-500/15 text-red-600';
	}

	async function testConnection(id: number) {
		testingId = id;
		try {
			const res = await remoteServerControllerTestConnection({ path: { id } });
			testResults = { ...testResults, [id]: res.data ? 'ok' : 'fail' };
		} catch { testResults = { ...testResults, [id]: 'fail' }; }
		finally { testingId = null; }
	}

	async function toggleActive(server: RemoteServerResponseDto) {
		togglingId = server.id;
		try {
			const res = server.server_status === 'ACTIVE'
				? await remoteServerControllerDeactivate({ path: { id: server.id } })
				: await remoteServerControllerActivate({ path: { id: server.id } });
			if (res.data?.server) servers = servers.map(s => s.id === server.id ? res.data!.server : s);
		} finally { togglingId = null; }
	}

	async function deleteServer(id: number) {
		deletingId = id;
		try {
			await remoteServerControllerDelete({ path: { id } });
			servers = servers.filter(s => s.id !== id);
		} finally { deletingId = null; }
	}

	// Create modal
	let mName = $state('');
	let mIp = $state('');
	let mPort = $state('22');
	let mUser = $state('root');
	let mType = $state('REMOTE');
	let mSshKeyId = $state<number | null>(null);
	let mDesc = $state('');
	let mSaving = $state(false);
	let mError = $state('');
	const SERVER_TYPES = ['REMOTE', 'SWARM'];

	async function submitCreate(e: SubmitEvent) {
		e.preventDefault(); mError = ''; mSaving = true;
		try {
			const res = await remoteServerControllerCreate({ body: { name: mName.trim(), ip_address: mIp.trim(), port: parseInt(mPort) || 22, username: mUser.trim() || 'root', server_type: mType, description: mDesc.trim() || undefined, ssh_key_id: mSshKeyId ?? undefined } as any });
			if (res.data) { servers = [...servers, res.data]; showCreate = false; }
		} catch (err: any) { mError = err?.message ?? 'Failed to create server'; }
		finally { mSaving = false; }
	}
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<RocketIcon class="w-4 h-4 text-muted-foreground" />
		<button onclick={() => goto('/dashboard')} class="text-muted-foreground hover:text-foreground transition-colors">Dashboard</button>
		<span class="text-muted-foreground/30">/</span>
		<span class="font-medium flex items-center gap-1.5"><Server class="w-4 h-4" /> Remote Servers</span>
	</header>

	<main class="flex-1 px-8 py-8">
		<div class="flex items-center justify-between mb-6">
			<div>
				<h1 class="text-2xl font-semibold">Remote Servers</h1>
				<p class="text-sm text-muted-foreground mt-1">Manage SSH connections to your deployment servers</p>
			</div>
			<div class="flex items-center gap-2">
				<button onclick={load} class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent transition-colors">
					<RefreshCw class="w-3.5 h-3.5" /> Refresh
				</button>
				<button onclick={() => (showCreate = true)} class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 transition-colors">
					<Plus class="w-4 h-4" /> Add Server
				</button>
			</div>
		</div>

		{#if loading}
			<div class="flex justify-center py-20"><div class="w-6 h-6 border-2 border-muted-foreground/30 border-t-foreground rounded-full animate-spin"></div></div>
		{:else if servers.length === 0}
			<div class="flex flex-col items-center justify-center py-24 text-muted-foreground/40">
				<Server class="w-14 h-14 mb-4 opacity-30" />
				<p class="text-sm font-medium">No remote servers yet</p>
				<p class="text-xs mt-1">Add a server to start deploying your applications</p>
				<button onclick={() => (showCreate = true)} class="mt-5 px-3 py-1.5 rounded-md border border-border text-sm hover:bg-accent transition-colors inline-flex items-center gap-1.5">
					<Plus class="w-4 h-4" /> Add your first server
				</button>
			</div>
		{:else}
			<div class="flex flex-col gap-3">
				{#each servers as server (server.id)}
					<div class="bg-card border border-border rounded-xl p-5 flex items-center gap-4 hover:border-foreground/20 transition-colors">
						<div class="relative shrink-0">
							<div class="w-10 h-10 rounded-lg bg-muted flex items-center justify-center">
								<Server class="w-5 h-5 text-muted-foreground" />
							</div>
							<span class="absolute -bottom-0.5 -right-0.5 w-3 h-3 rounded-full border-2 border-card {statusDotClass(server.server_status)}"></span>
						</div>
						<div class="flex-1 min-w-0">
							<div class="flex items-center gap-2 flex-wrap">
								<p class="text-sm font-semibold truncate">{server.name}</p>
								<span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium {statusBadgeClass(server.server_status)}">{server.server_status.charAt(0) + server.server_status.slice(1).toLowerCase()}</span>
								<span class="text-xs bg-muted text-muted-foreground px-1.5 py-0.5 rounded font-mono">{server.server_type}</span>
							</div>
							<div class="flex items-center gap-3 mt-1 text-xs text-muted-foreground">
								<span class="font-mono flex items-center gap-1"><Globe class="w-3 h-3" />{server.ip_address}:{server.port}</span>
								<span class="font-mono">{server.username}</span>
								{#if keyName(server.ssh_key_id)}<span class="flex items-center gap-1"><FileKey class="w-3 h-3" />{keyName(server.ssh_key_id)}</span>{/if}
								{#if server.description}<span class="truncate max-w-xs">{server.description}</span>{/if}
							</div>
							{#if testResults[server.id]}
								<p class="text-xs mt-1 flex items-center gap-1 {testResults[server.id] === 'ok' ? 'text-green-600' : 'text-destructive'}">
									{#if testResults[server.id] === 'ok'}<CheckCircle class="w-3 h-3" /> Connection successful{:else}<XCircle class="w-3 h-3" /> Connection failed{/if}
								</p>
							{/if}
						</div>
						<div class="flex items-center gap-1 shrink-0">
							<button onclick={() => testConnection(server.id)} disabled={testingId === server.id} class="inline-flex items-center gap-1.5 px-2 py-1.5 rounded text-sm text-muted-foreground hover:text-foreground hover:bg-accent transition-colors">
								{#if testingId === server.id}<div class="w-4 h-4 border-2 border-current/30 border-t-current rounded-full animate-spin"></div>{:else}<TestTube2 class="w-4 h-4" />{/if}
								<span class="hidden sm:inline">Test</span>
							</button>
							<button onclick={() => toggleActive(server)} disabled={togglingId === server.id} class="p-1.5 rounded text-muted-foreground hover:text-foreground hover:bg-accent transition-colors" title={server.server_status === 'ACTIVE' ? 'Deactivate' : 'Activate'}>
								{#if togglingId === server.id}<div class="w-4 h-4 border-2 border-current/30 border-t-current rounded-full animate-spin"></div>{:else}<Power class="w-4 h-4" />{/if}
							</button>
							<button onclick={() => deleteServer(server.id)} disabled={deletingId === server.id} class="p-1.5 rounded text-muted-foreground hover:text-destructive hover:bg-destructive/10 transition-colors">
								{#if deletingId === server.id}<div class="w-4 h-4 border-2 border-current/30 border-t-current rounded-full animate-spin"></div>{:else}<Trash2 class="w-4 h-4" />{/if}
							</button>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</main>
</PageLayout>

{#if showCreate}
	<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1" aria-label="Close" onclick={() => (showCreate = false)} onkeydown={() => {}}></div>
	<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
		<form onsubmit={submitCreate} class="bg-card border border-border rounded-xl shadow-2xl w-full max-w-md mx-4 p-6 flex flex-col gap-4 pointer-events-auto">
			<div class="flex items-center justify-between">
				<div>
					<h2 class="text-base font-semibold">Add Remote Server</h2>
					<p class="text-xs text-muted-foreground mt-0.5">Connect a new server via SSH</p>
				</div>
				<button type="button" class="text-muted-foreground hover:text-foreground hover:bg-accent p-1 rounded" onclick={() => (showCreate = false)}>✕</button>
			</div>
			<div class="flex flex-col gap-1.5">
				<label class="text-sm font-medium text-muted-foreground" for="page-1">Name <span class="text-destructive">*</span></label>

				<input id="page-1"  class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring" placeholder="production-1" bind:value={mName} required />
			</div>
			<div class="grid grid-cols-2 gap-3">
				<div class="flex flex-col gap-1.5">
					<label class="text-sm font-medium text-muted-foreground" for="page-2">IP Address <span class="text-destructive">*</span></label>

					<input id="page-2"  class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm font-mono placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring" placeholder="192.168.1.100" bind:value={mIp} required />
				</div>
				<div class="flex flex-col gap-1.5">
					<label class="text-sm font-medium text-muted-foreground" for="page-3">Port</label>

					<input id="page-3"  class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm font-mono placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring" type="number" placeholder="22" bind:value={mPort} />
				</div>
			</div>
			<div class="grid grid-cols-2 gap-3">
				<div class="flex flex-col gap-1.5">
					<label class="text-sm font-medium text-muted-foreground" for="page-4">Username</label>

					<input id="page-4"  class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm font-mono placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring" placeholder="root" bind:value={mUser} />
				</div>
				<div class="flex flex-col gap-1.5">
					<label class="text-sm font-medium text-muted-foreground" for="page-5">Server Type</label>

					<select id="page-5"  class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm focus:outline-none focus:ring-1 focus:ring-ring" bind:value={mType}>
						{#each SERVER_TYPES as t}<option value={t}>{t}</option>{/each}
					</select>
				</div>
			</div>
			<div class="flex flex-col gap-1.5">
				<label class="text-sm font-medium text-muted-foreground flex items-center gap-1.5" for="page-6"><FileKey class="w-3.5 h-3.5" /> SSH Key</label>

				<select id="page-6"  class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm focus:outline-none focus:ring-1 focus:ring-ring" bind:value={mSshKeyId}>
					<option value={null}>None (password auth)</option>
					{#each sshKeys as k}<option value={k.id}>{k.name}{k.description ? ` — ${k.description}` : ''}</option>{/each}
				</select>
			</div>
			<div class="flex flex-col gap-1.5">
				<label class="text-sm font-medium text-muted-foreground" for="page-7">Description</label>

				<input id="page-7"  class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring" placeholder="Optional description" bind:value={mDesc} />
			</div>
			{#if mError}<p class="text-xs text-destructive">{mError}</p>{/if}
			<div class="flex justify-end gap-2 pt-1">
				<button type="button" class="px-3 py-1.5 rounded-md text-sm text-muted-foreground hover:bg-accent transition-colors" onclick={() => (showCreate = false)}>Cancel</button>
				<button type="submit" class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50" disabled={mSaving}>
					{#if mSaving}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>Adding…{:else}Add Server{/if}
				</button>
			</div>
		</form>
	</div>
{/if}
