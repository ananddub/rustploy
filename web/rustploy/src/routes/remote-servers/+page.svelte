<script lang="ts">
	import { goto } from '$app/navigation';
	import {
		Server, Plus, Trash2, Terminal, Pencil, Network, User,
		Key, Clock, Loader2, KeyRound, TestTube2, Power,
		CheckCircle, XCircle, RefreshCw, Settings2, FileKey, Play
	} from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import {
		remoteServerControllerList,
		remoteServerControllerCreate,
		remoteServerControllerDelete,
		remoteServerControllerActivate,
		remoteServerControllerDeactivate,
		remoteServerControllerTestConnection,
		sshKeyControllerList
	} from '$lib/client/sdk.gen';
	import type { RemoteServerResponseDto, SshKeyResponseDto } from '$lib/client/types.gen';
	import * as Select from '$lib/components/ui/select';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { toastSuccess, toastError } from '$lib/toast';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	// ─── Data ─────────────────────────────────────────────────────────────────────
	let servers    = $state<RemoteServerResponseDto[]>([]);
	let sshKeys    = $state<SshKeyResponseDto[]>([]);
	let loading    = $state(true);

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

	function fmtDate(ts?: number) {
		if (!ts) return '—';
		return new Date(ts * 1000).toLocaleString(undefined, { month:'short', day:'numeric', year:'numeric', hour:'2-digit', minute:'2-digit' });
	}

	// ─── Status helpers ───────────────────────────────────────────────────────────
	function statusDot(status: string) {
		if (status === 'ACTIVE')     return 'bg-green-500';
		if (status === 'CONNECTING') return 'bg-yellow-500 animate-pulse';
		if (status === 'INACTIVE')   return 'bg-muted-foreground/30';
		return 'bg-destructive';
	}

	function statusBadge(status: string): string {
		if (status === 'ACTIVE')     return 'default';
		if (status === 'CONNECTING') return 'secondary';
		return 'secondary';
	}

	function statusLabel(status: string) {
		return status.charAt(0) + status.slice(1).toLowerCase();
	}

	// ─── Actions ──────────────────────────────────────────────────────────────────
	let testingId   = $state<number|null>(null);
	let togglingId  = $state<number|null>(null);
	let deletingId  = $state<number|null>(null);
	let testResults = $state<Record<number,'ok'|'fail'>>({});
	let confirmDeleteId = $state<number|null>(null);

	async function testConnection(id: number) {
		testingId = id;
		try {
			const res = await remoteServerControllerTestConnection({ path: { id } });
			testResults = { ...testResults, [id]: res.data ? 'ok' : 'fail' };
			if (res.data) toastSuccess('Connection successful');
			else toastError('Connection failed');
		} catch { testResults = { ...testResults, [id]: 'fail' }; toastError('Connection failed'); }
		finally { testingId = null; }
	}

	async function toggleActive(server: RemoteServerResponseDto) {
		togglingId = server.id;
		try {
			const res = server.server_status === 'ACTIVE'
				? await remoteServerControllerDeactivate({ path: { id: server.id } })
				: await remoteServerControllerActivate({ path: { id: server.id } });
			if (res.data?.server) {
				servers = servers.map(s => s.id === server.id ? res.data!.server : s);
				toastSuccess(server.server_status === 'ACTIVE' ? 'Server deactivated' : 'Server activated');
			}
		} finally { togglingId = null; }
	}

	async function deleteServer(id: number) {
		deletingId = id;
		try {
			await remoteServerControllerDelete({ path: { id } });
			servers = servers.filter(s => s.id !== id);
			toastSuccess('Server deleted');
		} catch { toastError('Failed to delete server'); }
		finally { deletingId = null; confirmDeleteId = null; }
	}

	// ─── Modal ────────────────────────────────────────────────────────────────────
	let showModal  = $state(false);
	let saving     = $state(false);
	let modalError = $state('');
	let mName      = $state('');
	let mDesc      = $state('');
	let mIp        = $state('');
	let mPort      = $state('22');
	let mUser      = $state('root');
	let mType      = $state('REMOTE');
	let mSshKeyId  = $state<number|null>(null);
	const SERVER_TYPES = ['REMOTE','SWARM'];

	function openCreate() {
		mName=''; mDesc=''; mIp=''; mPort='22'; mUser='root'; mType='REMOTE'; mSshKeyId=null; modalError='';
		showModal = true;
	}

	async function submitModal(e: SubmitEvent) {
		e.preventDefault(); modalError=''; saving=true;
		try {
			const res = await remoteServerControllerCreate({
				body: { name:mName.trim(), ip_address:mIp.trim(), port:parseInt(mPort)||22, username:mUser.trim()||'root', server_type:mType, description:mDesc.trim()||undefined, ssh_key_id:mSshKeyId??undefined } as any
			});
			if (res.data) { servers=[...servers, res.data]; showModal=false; toastSuccess('Server added'); }
		} catch(err: any) { modalError = err?.message ?? 'Failed to create server'; }
		finally { saving=false; }
	}
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<Server class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Remote Servers</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto">
		<div class="w-full">
			<div class="rounded-xl border border-border bg-card">

				<!-- Header -->
				<div class="px-6 pt-5 pb-4 border-b border-border">
					<div class="flex items-start justify-between gap-3 flex-wrap">
						<div>
							<h2 class="text-xl font-bold flex items-center gap-2">
								<Server class="w-5 h-5 text-muted-foreground" /> Remote Servers
							</h2>
							<p class="text-sm text-muted-foreground mt-0.5">Manage SSH connections to your deployment servers</p>
						</div>
						<div class="flex items-center gap-2">
							<Button variant="outline" size="sm" class="gap-1.5 h-8" onclick={load} disabled={loading}>
								<RefreshCw class="w-3.5 h-3.5 {loading ? 'animate-spin' : ''}" /> Refresh
							</Button>
							<Button size="sm" class="gap-1.5 h-8" onclick={openCreate}>
								<Plus class="w-4 h-4" /> Add Server
							</Button>
						</div>
					</div>
				</div>

				<!-- Content -->
				<div class="px-6 py-5">
					{#if loading}
						<div class="flex items-center justify-center gap-2 min-h-[25vh] text-muted-foreground">
							<Loader2 class="w-5 h-5 animate-spin" />
							<span class="text-sm">Loading...</span>
						</div>

					{:else if sshKeys.length === 0 && servers.length === 0}
						<div class="flex flex-col items-center justify-center gap-3 min-h-[25vh] text-muted-foreground">
							<KeyRound class="w-8 h-8 opacity-50" />
							<p class="text-base text-center">
								No SSH Keys found. Add an SSH Key to start adding servers.
								<button class="text-primary hover:underline ml-1" onclick={() => goto('/ssh-keys')}>Add SSH Key</button>
							</p>
						</div>

					{:else if servers.length === 0}
						<div class="flex flex-col items-center justify-center gap-3 min-h-[25vh] text-muted-foreground">
							<Server class="w-10 h-10 opacity-30" />
							<p class="text-base">Start adding servers to deploy your applications remotely</p>
							<Button size="sm" class="gap-1.5 mt-1" onclick={openCreate}>
								<Plus class="w-4 h-4" /> Add Server
							</Button>
						</div>

					{:else}
						<div class="flex flex-col gap-6 min-h-[25vh]">
							<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
								{#each servers as server (server.id)}
									{@const isActive = server.server_status === 'ACTIVE'}
									{@const isBuild  = server.server_type === 'SWARM'}

									<div
										class="relative rounded-xl border border-border bg-transparent hover:shadow-lg transition-shadow flex flex-col cursor-pointer"
										role="button" tabindex="0"
										onclick={() => goto(`/remote-servers/${server.id}`)}
										onkeydown={(e) => e.key === 'Enter' && goto(`/remote-servers/${server.id}`)}
									>
										<!-- Card header -->
										<div class="px-4 pt-4 pb-3">
											<div class="flex items-center gap-2 min-w-0 mb-2">
												<div class="relative shrink-0">
													<Server class="w-5 h-5 text-muted-foreground" />
													<span class="absolute -bottom-0.5 -right-0.5 w-2.5 h-2.5 rounded-full border-2 border-card {statusDot(server.server_status)}"></span>
												</div>
												<span class="text-base font-semibold truncate">{server.name}</span>
											</div>
											<div class="flex gap-2 flex-wrap">
												<Badge variant={statusBadge(server.server_status) as any} class="text-[10px]">
													{statusLabel(server.server_status)}
												</Badge>
												<Badge variant="outline" class="text-[10px] font-mono capitalize">
													{server.server_type?.toLowerCase()}
												</Badge>
											</div>
										</div>

										<!-- Card body -->
										<div class="px-4 pb-3 space-y-2 flex-1">
											<div class="flex items-center gap-2 text-sm">
												<Network class="w-4 h-4 text-muted-foreground shrink-0" />
												<span class="text-muted-foreground text-xs">IP:</span>
												<Badge variant="outline" class="text-xs font-mono">{server.ip_address}</Badge>
												<span class="text-muted-foreground text-xs">:{server.port}</span>
											</div>
											<div class="flex items-center gap-2 text-sm">
												<User class="w-4 h-4 text-muted-foreground shrink-0" />
												<span class="text-muted-foreground text-xs">User:</span>
												<span class="font-medium text-xs">{server.username}</span>
											</div>
											<div class="flex items-center gap-2 text-sm">
												<Key class="w-4 h-4 text-muted-foreground shrink-0" />
												<span class="text-muted-foreground text-xs">SSH Key:</span>
												<span class="font-medium text-xs">{keyName(server.ssh_key_id) ?? 'None'}</span>
											</div>
											{#if server.description}
												<p class="text-sm text-muted-foreground truncate">{server.description}</p>
											{/if}

											<!-- Test result -->
											{#if testResults[server.id]}
												<div class="flex items-center gap-1.5 text-xs {testResults[server.id] === 'ok' ? 'text-green-500' : 'text-destructive'}">
													{#if testResults[server.id] === 'ok'}
														<CheckCircle class="w-3.5 h-3.5" /> Connection successful
													{:else}
														<XCircle class="w-3.5 h-3.5" /> Connection failed
													{/if}
												</div>
											{/if}
										</div>

										<!-- Card footer actions -->
										<div class="px-4 pb-4 border-t border-border pt-3 mt-auto"
											onclick={(e) => e.stopPropagation()}
											onkeydown={(e) => e.stopPropagation()}
											role="group"
										>
											<!-- Test Connection full width -->
											<Button variant="outline" size="sm" class="w-full gap-1.5 text-xs justify-start h-9 mb-2"
												onclick={() => testConnection(server.id)} disabled={testingId === server.id}>
												{#if testingId === server.id}
													<Loader2 class="w-3.5 h-3.5 animate-spin" />
												{:else}
													<TestTube2 class="w-3.5 h-3.5" />
												{/if}
												Test Connection
											</Button>

											<!-- Icon buttons -->
											<div class="flex items-center gap-1.5">
												{#if server.ssh_key_id}
													<Button variant="outline" size="icon" class="h-8 w-8" title="Terminal">
														<Terminal class="w-3.5 h-3.5" />
													</Button>
												{/if}
												<Button variant="outline" size="icon" class="h-8 w-8" title="Edit"
													onclick={() => goto(`/remote-servers/${server.id}`)}>
													<Pencil class="w-3.5 h-3.5" />
												</Button>
												<Button variant="outline" size="icon" class="h-8 w-8"
													title={isActive ? 'Deactivate' : 'Activate'}
													onclick={() => toggleActive(server)} disabled={togglingId === server.id}>
													{#if togglingId === server.id}
														<Loader2 class="w-3.5 h-3.5 animate-spin" />
													{:else}
														<Power class="w-3.5 h-3.5" />
													{/if}
												</Button>
												<div class="flex-1"></div>
												<Button variant="ghost" size="icon"
													class="h-8 w-8 text-destructive hover:bg-destructive/10"
													title="Delete Server"
													onclick={() => (confirmDeleteId = server.id)}>
													{#if deletingId === server.id}
														<Loader2 class="w-3.5 h-3.5 animate-spin" />
													{:else}
														<Trash2 class="w-3.5 h-3.5" />
													{/if}
												</Button>
											</div>
										</div>
									</div>
								{/each}
							</div>

							<div class="flex justify-end">
								<Button size="sm" class="gap-1.5" onclick={openCreate}>
									<Plus class="w-4 h-4" /> Add Server
								</Button>
							</div>
						</div>
					{/if}
				</div>
			</div>
		</div>
	</main>
</PageLayout>

<!-- ── Add Server Modal ────────────────────────────────────────────────────── -->
{#if showModal}
	<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1"
		onclick={() => (showModal=false)} onkeydown={() => {}}></div>
	<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
		<form onsubmit={submitModal}
			class="bg-card border border-border rounded-xl shadow-2xl w-full max-w-md pointer-events-auto flex flex-col gap-4 p-6 max-h-[90vh] overflow-y-auto">
			<div class="flex items-start justify-between gap-3">
				<div>
					<h2 class="text-base font-semibold">Add Remote Server</h2>
					<p class="text-sm text-muted-foreground mt-0.5">Connect a new server via SSH</p>
				</div>
				<button type="button" onclick={() => (showModal=false)}
					class="text-muted-foreground hover:text-foreground p-1 rounded hover:bg-accent">✕</button>
			</div>

			<div class="space-y-1.5">
				<Label for="m-name" class="text-sm">Name <span class="text-destructive">*</span></Label>
				<Input id="m-name" bind:value={mName} placeholder="production-1" required />
			</div>
			<div class="space-y-1.5">
				<Label for="m-desc" class="text-sm">Description</Label>
				<Input id="m-desc" bind:value={mDesc} placeholder="Optional description" />
			</div>
			<div class="grid grid-cols-3 gap-3">
				<div class="col-span-2 space-y-1.5">
					<Label for="m-ip" class="text-sm">IP Address <span class="text-destructive">*</span></Label>
					<Input id="m-ip" bind:value={mIp} placeholder="192.168.1.100" class="font-mono" required />
				</div>
				<div class="space-y-1.5">
					<Label for="m-port" class="text-sm">Port</Label>
					<Input id="m-port" bind:value={mPort} type="number" placeholder="22" class="font-mono" />
				</div>
			</div>
			<div class="grid grid-cols-2 gap-3">
				<div class="space-y-1.5">
					<Label for="m-user" class="text-sm">Username</Label>
					<Input id="m-user" bind:value={mUser} placeholder="root" class="font-mono" />
				</div>
				<div class="space-y-1.5">
					<Label class="text-sm">Server Type</Label>
					<Select.Root type="single" value={mType} onValueChange={(v) => (mType = v ?? 'REMOTE')}>
						<Select.Trigger class="w-full h-9">
							<span class="text-sm">{mType}</span>
						</Select.Trigger>
						<Select.Content>
							{#each SERVER_TYPES as t (t)}
								<Select.Item value={t}>{t}</Select.Item>
							{/each}
						</Select.Content>
					</Select.Root>
				</div>
			</div>
			<div class="space-y-1.5">
				<Label class="text-sm flex items-center gap-1.5">
					<FileKey class="w-3.5 h-3.5" /> SSH Key
				</Label>
				<Select.Root type="single" value={mSshKeyId !== null ? String(mSshKeyId) : ''} onValueChange={(v) => (mSshKeyId = v ? Number(v) : null)}>
					<Select.Trigger class="w-full">
						<span class="text-sm">{mSshKeyId ? (sshKeys.find(k=>k.id===mSshKeyId)?.name ?? `Key #${mSshKeyId}`) : 'None (password auth)'}</span>
					</Select.Trigger>
					<Select.Content>
						<Select.Item value="">None (password auth)</Select.Item>
						{#each sshKeys as k (k.id)}
							<Select.Item value={String(k.id)}>{k.name}{k.description ? ` — ${k.description}` : ''}</Select.Item>
						{/each}
					</Select.Content>
				</Select.Root>
			</div>

			{#if modalError}
				<p class="text-xs text-destructive bg-destructive/10 border border-destructive/20 rounded px-3 py-2">{modalError}</p>
			{/if}

			<div class="flex justify-end gap-2 pt-1">
				<Button type="button" variant="outline" size="sm" onclick={() => (showModal=false)}>Cancel</Button>
				<Button type="submit" size="sm" disabled={saving} class="gap-1.5 min-w-[100px]">
					{#if saving}<Loader2 class="w-3.5 h-3.5 animate-spin" /> Adding…{:else}Add Server{/if}
				</Button>
			</div>
		</form>
	</div>
{/if}

<!-- ── Delete Confirm ─────────────────────────────────────────────────────── -->
{#if confirmDeleteId !== null}
	{@const target = servers.find(s => s.id === confirmDeleteId)}
	<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1"
		onclick={() => (confirmDeleteId=null)} onkeydown={() => {}}></div>
	<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
		<div class="bg-card border border-border rounded-xl shadow-2xl w-full max-w-sm p-6 pointer-events-auto flex flex-col gap-4">
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 rounded-full bg-destructive/10 flex items-center justify-center shrink-0">
					<Trash2 class="w-5 h-5 text-destructive" />
				</div>
				<div>
					<h2 class="text-sm font-semibold">Delete Server</h2>
					<p class="text-sm text-muted-foreground mt-0.5">
						Delete <strong class="text-foreground">{target?.name}</strong>?
						This cannot be undone.
					</p>
				</div>
			</div>
			<div class="flex justify-end gap-2">
				<Button variant="outline" size="sm" onclick={() => (confirmDeleteId=null)}>Cancel</Button>
				<Button variant="destructive" size="sm"
					onclick={() => deleteServer(confirmDeleteId!)}
					disabled={deletingId !== null} class="gap-1.5">
					{#if deletingId !== null}<Loader2 class="w-3.5 h-3.5 animate-spin" /> Deleting…
					{:else}<Trash2 class="w-3.5 h-3.5" /> Delete{/if}
				</Button>
			</div>
		</div>
	</div>
{/if}
