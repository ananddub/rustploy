<script lang="ts">
	import { goto } from '$app/navigation';
	import {
		Server, Plus, Trash2, Terminal, Pencil, Network,
		User, Key, Clock, Loader2, KeyRound, Settings2, Play
	} from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import * as Select from '$lib/components/ui/select';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Switch } from '$lib/components/ui/switch';
	import { toastSuccess, toastError } from '$lib/toast';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	type ServerType   = 'deploy' | 'build';
	type ServerStatus = 'active' | 'inactive';

	type RemoteServer = {
		id: string; name: string; description: string; ipAddress: string;
		port: number; username: string; sshKeyId: string | null;
		serverType: ServerType; serverStatus: ServerStatus;
		enableDockerCleanup: boolean; createdAt: string; totalServices: number;
	};

	type SshKey = { id: string; name: string };

	let servers = $state<RemoteServer[]>([
		{ id:'s1', name:'Production Server', description:'Main production deployment server', ipAddress:'192.168.1.100', port:22, username:'root',   sshKeyId:'k1', serverType:'deploy', serverStatus:'active', enableDockerCleanup:true,  createdAt:'2026-07-01T10:00:00Z', totalServices:5 },
		{ id:'s2', name:'Staging Server',    description:'Staging environment for testing',   ipAddress:'192.168.1.101', port:22, username:'deploy', sshKeyId:'k1', serverType:'deploy', serverStatus:'active', enableDockerCleanup:true,  createdAt:'2026-07-05T08:00:00Z', totalServices:2 },
		{ id:'s3', name:'Build Server',      description:'Dedicated CI/CD build server',      ipAddress:'192.168.1.102', port:2222, username:'root', sshKeyId:'k2', serverType:'build',  serverStatus:'active', enableDockerCleanup:false, createdAt:'2026-07-08T12:00:00Z', totalServices:0 },
	]);

	const sshKeys: SshKey[] = [
		{ id:'k1', name:'deploy-key' },
		{ id:'k2', name:'ci-key' },
	];

	function keyName(id: string | null) {
		return id ? (sshKeys.find(k => k.id === id)?.name ?? id) : 'None';
	}

	function fmtDate(iso: string) {
		return new Date(iso).toLocaleString(undefined, { month:'short', day:'numeric', year:'numeric', hour:'2-digit', minute:'2-digit' });
	}

	// ─── Modal ────────────────────────────────────────────────────────────────────
	let showModal   = $state(false);
	let editingId   = $state<string|null>(null);
	let saving      = $state(false);
	let modalError  = $state('');
	let fName       = $state('');
	let fDesc       = $state('');
	let fIp         = $state('');
	let fPort       = $state('22');
	let fUser       = $state('root');
	let fSshKey     = $state('');
	let fType       = $state<ServerType>('deploy');
	let fCleanup    = $state(true);

	let nextId = $derived(`s${servers.length + 1}`);

	function openCreate() {
		editingId = null; fName=''; fDesc=''; fIp=''; fPort='22'; fUser='root'; fSshKey=''; fType='deploy'; fCleanup=true; modalError='';
		showModal = true;
	}

	function openEdit(s: RemoteServer) {
		editingId = s.id; fName=s.name; fDesc=s.description; fIp=s.ipAddress;
		fPort=String(s.port); fUser=s.username; fSshKey=s.sshKeyId??''; fType=s.serverType; fCleanup=s.enableDockerCleanup; modalError='';
		showModal = true;
	}

	async function submitModal(e: SubmitEvent) {
		e.preventDefault(); modalError='';
		if (!fName.trim()) { modalError='Name is required'; return; }
		if (!fIp.trim())   { modalError='IP Address is required'; return; }
		if (!fSshKey)      { modalError='SSH Key is required'; return; }
		saving = true;
		try {
			await new Promise(r => setTimeout(r, 500));
			if (editingId) {
				servers = servers.map(s => s.id === editingId ? { ...s, name:fName.trim(), description:fDesc.trim(), ipAddress:fIp.trim(), port:Number(fPort)||22, username:fUser||'root', sshKeyId:fSshKey, serverType:fType, enableDockerCleanup:fCleanup } : s);
				toastSuccess('Server updated');
			} else {
				servers = [...servers, { id:nextId, name:fName.trim(), description:fDesc.trim(), ipAddress:fIp.trim(), port:Number(fPort)||22, username:fUser||'root', sshKeyId:fSshKey, serverType:fType, serverStatus:'active', enableDockerCleanup:fCleanup, createdAt:new Date().toISOString(), totalServices:0 }];
				toastSuccess('Server added');
			}
			showModal = false; editingId = null;
		} catch { modalError='Failed to save'; }
		finally { saving=false; }
	}

	// ─── Delete ───────────────────────────────────────────────────────────────────
	let confirmDeleteId = $state<string|null>(null);
	let deletingId      = $state<string|null>(null);

	async function deleteServer(id: string) {
		const s = servers.find(s => s.id === id);
		if (s && s.totalServices > 0) { toastError('Cannot delete — server has active services'); confirmDeleteId=null; return; }
		deletingId = id;
		try {
			await new Promise(r => setTimeout(r, 400));
			servers = servers.filter(s => s.id !== id);
			toastSuccess('Server deleted');
		} finally { deletingId=null; confirmDeleteId=null; }
	}

	// ─── Setup ────────────────────────────────────────────────────────────────────
	let settingUpId = $state<string|null>(null);
	async function setupServer(id: string) {
		settingUpId = id;
		await new Promise(r => setTimeout(r, 1500));
		settingUpId = null;
		toastSuccess('Server setup complete');
	}
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<Server class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Servers</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto">
		<div class="w-full">
			<div class="rounded-xl border border-border bg-card">

				<div class="px-6 pt-5 pb-4 border-b border-border">
					<h2 class="text-xl font-bold flex items-center gap-2">
						<Server class="w-5 h-5 text-muted-foreground" /> Servers
					</h2>
					<p class="text-sm text-muted-foreground mt-0.5">Add servers to deploy your applications remotely</p>
				</div>

				<div class="px-6 py-5">
					{#if servers.length === 0}
						{#if sshKeys.length === 0}
							<div class="flex flex-col items-center justify-center gap-3 min-h-[25vh] text-muted-foreground">
								<KeyRound class="w-8 h-8" />
								<p class="text-base text-center">
									No SSH Keys found. Add an SSH Key to start adding servers.
									<button class="text-primary hover:underline ml-1" onclick={() => goto('/ssh-keys')}>Add SSH Key</button>
								</p>
							</div>
						{:else}
							<div class="flex flex-col items-center justify-center gap-3 min-h-[25vh] text-muted-foreground">
								<Server class="w-8 h-8 opacity-40" />
								<p class="text-base">Start adding servers to deploy your applications remotely</p>
								<Button size="sm" class="gap-1.5 mt-1" onclick={openCreate}>
									<Plus class="w-4 h-4" /> Add Server
								</Button>
							</div>
						{/if}
					{:else}
						<div class="flex flex-col gap-6 min-h-[25vh]">
							<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
								{#each servers as server (server.id)}
									{@const isActive  = server.serverStatus === 'active'}
									{@const isBuild   = server.serverType  === 'build'}
									{@const canDelete = server.totalServices === 0}

									<Card.Root class="relative hover:shadow-lg transition-shadow flex flex-col bg-transparent">
										<Card.Header class="pb-3">
											<div class="flex items-center gap-2 min-w-0">
												<Server class="w-5 h-5 shrink-0 text-muted-foreground" />
												<Card.Title class="text-base truncate">{server.name}</Card.Title>
											</div>
											<div class="flex gap-2 mt-2 flex-wrap">
												<Badge variant={isActive ? 'default' : 'destructive'} class="text-[10px] capitalize">
													{server.serverStatus}
												</Badge>
												<Badge variant={isBuild ? 'secondary' : 'default'} class="text-[10px] capitalize">
													{server.serverType}
												</Badge>
											</div>
										</Card.Header>

										<Card.Content class="space-y-2.5 flex-1 flex flex-col pt-0">
											<div class="flex items-center gap-2 text-sm">
												<Network class="w-4 h-4 text-muted-foreground shrink-0" />
												<span class="text-muted-foreground text-xs">IP:</span>
												<Badge variant="outline" class="text-xs font-mono">{server.ipAddress}</Badge>
												<span class="text-muted-foreground text-xs">Port:</span>
												<span class="font-medium text-xs">{server.port}</span>
											</div>
											<div class="flex items-center gap-2 text-sm">
												<User class="w-4 h-4 text-muted-foreground shrink-0" />
												<span class="text-muted-foreground text-xs">User:</span>
												<span class="font-medium text-xs">{server.username}</span>
											</div>
											<div class="flex items-center gap-2 text-sm">
												<Key class="w-4 h-4 text-muted-foreground shrink-0" />
												<span class="text-muted-foreground text-xs">SSH Key:</span>
												<span class="font-medium text-xs">{keyName(server.sshKeyId)}</span>
											</div>
											<div class="flex items-center gap-2 border-t pt-2.5">
												<Clock class="w-4 h-4 text-muted-foreground shrink-0" />
												<span class="text-xs text-muted-foreground">Created {fmtDate(server.createdAt)}</span>
											</div>

											{#if isActive}
												<div class="flex flex-col gap-2 pt-2.5 border-t mt-auto">
													<Button variant="outline" size="sm" class="w-full gap-1.5 text-xs justify-start h-9"
														onclick={() => setupServer(server.id)} disabled={settingUpId === server.id}>
														{#if settingUpId === server.id}
															<Loader2 class="w-3.5 h-3.5 animate-spin" />
														{:else}
															<Play class="w-3.5 h-3.5" />
														{/if}
														Setup Server
													</Button>
													<div class="flex items-center gap-1.5">
														{#if server.sshKeyId}
															<Button variant="outline" size="icon" class="h-9 w-9" title="Terminal">
																<Terminal class="w-4 h-4" />
															</Button>
														{/if}
														<Button variant="outline" size="icon" class="h-9 w-9" title="Edit"
															onclick={() => openEdit(server)}>
															<Pencil class="w-4 h-4" />
														</Button>
														{#if server.sshKeyId && !isBuild}
															<Button variant="outline" size="icon" class="h-9 w-9" title="Server Actions">
																<Settings2 class="w-4 h-4" />
															</Button>
														{/if}
														<div class="flex-1"></div>
														<Button variant="ghost" size="icon"
															class="h-9 w-9 {canDelete ? 'text-destructive hover:bg-destructive/10' : 'text-muted-foreground'}"
															title={canDelete ? 'Delete Server' : 'Has active services'}
															onclick={() => { if (canDelete) confirmDeleteId = server.id; else toastError('Cannot delete — server has active services'); }}>
															<Trash2 class="w-4 h-4" />
														</Button>
													</div>
												</div>
											{/if}
										</Card.Content>
									</Card.Root>
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

<!-- ── Add / Edit Modal ──────────────────────────────────────────────────── -->
{#if showModal}
	<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1"
		onclick={() => { showModal=false; editingId=null; }} onkeydown={() => {}}></div>
	<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
		<form onsubmit={submitModal}
			class="bg-card border border-border rounded-xl shadow-2xl w-full max-w-lg pointer-events-auto flex flex-col gap-4 p-6 max-h-[90vh] overflow-y-auto">
			<div class="flex items-start justify-between gap-3">
				<div>
					<h2 class="text-base font-semibold">{editingId ? 'Edit' : 'Add'} Server</h2>
					<p class="text-sm text-muted-foreground mt-0.5">{editingId ? 'Update server connection details' : 'Connect a new remote server via SSH'}</p>
				</div>
				<button type="button" onclick={() => { showModal=false; editingId=null; }}
					class="text-muted-foreground hover:text-foreground p-1 rounded hover:bg-accent">✕</button>
			</div>

			<div class="space-y-1.5">
				<Label for="m-name" class="text-sm">Name <span class="text-destructive">*</span></Label>
				<Input id="m-name" bind:value={fName} placeholder="Production Server" required />
			</div>
			<div class="space-y-1.5">
				<Label for="m-desc" class="text-sm">Description</Label>
				<Input id="m-desc" bind:value={fDesc} placeholder="Optional description" />
			</div>
			<div class="grid grid-cols-3 gap-3">
				<div class="col-span-2 space-y-1.5">
					<Label for="m-ip" class="text-sm">IP Address <span class="text-destructive">*</span></Label>
					<Input id="m-ip" bind:value={fIp} placeholder="192.168.1.100" class="font-mono" required />
				</div>
				<div class="space-y-1.5">
					<Label for="m-port" class="text-sm">Port</Label>
					<Input id="m-port" bind:value={fPort} type="number" placeholder="22" class="font-mono" />
				</div>
			</div>
			<div class="space-y-1.5">
				<Label for="m-user" class="text-sm">Username</Label>
				<Input id="m-user" bind:value={fUser} placeholder="root" class="font-mono" />
			</div>
			<div class="space-y-1.5">
				<Label class="text-sm">SSH Key <span class="text-destructive">*</span></Label>
				{#if sshKeys.length === 0}
					<p class="text-sm text-muted-foreground">No SSH keys found.
						<button type="button" class="text-primary hover:underline" onclick={() => goto('/ssh-keys')}>Add one first</button>.
					</p>
				{:else}
					<Select.Root type="single" value={fSshKey} onValueChange={(v) => (fSshKey = v ?? '')}>
						<Select.Trigger class="w-full">
							<span class="text-sm">{sshKeys.find(k=>k.id===fSshKey)?.name ?? 'Select SSH Key'}</span>
						</Select.Trigger>
						<Select.Content>
							{#each sshKeys as k (k.id)}
								<Select.Item value={k.id}>{k.name}</Select.Item>
							{/each}
						</Select.Content>
					</Select.Root>
				{/if}
			</div>
			<div class="space-y-1.5">
				<Label class="text-sm">Server Type</Label>
				<Select.Root type="single" value={fType} onValueChange={(v) => (fType=(v??'deploy') as ServerType)}>
					<Select.Trigger class="w-full">
						<span class="text-sm capitalize">{fType}</span>
					</Select.Trigger>
					<Select.Content>
						<Select.Item value="deploy">Deploy</Select.Item>
						<Select.Item value="build">Build</Select.Item>
					</Select.Content>
				</Select.Root>
			</div>
			<div class="flex items-center justify-between rounded-lg border border-border px-3 py-2.5">
				<div>
					<p class="text-sm font-medium">Enable Docker Cleanup</p>
					<p class="text-[10px] text-muted-foreground">Automatically remove unused Docker resources</p>
				</div>
				<Switch bind:checked={fCleanup} />
			</div>

			{#if modalError}
				<p class="text-xs text-destructive bg-destructive/10 border border-destructive/20 rounded px-3 py-2">{modalError}</p>
			{/if}

			<div class="flex justify-end gap-2 pt-1">
				<Button type="button" variant="outline" size="sm" onclick={() => { showModal=false; editingId=null; }}>Cancel</Button>
				<Button type="submit" size="sm" disabled={saving} class="gap-1.5 min-w-[100px]">
					{#if saving}<Loader2 class="w-3.5 h-3.5 animate-spin" /> Saving…
					{:else}{editingId ? 'Update' : 'Add'} Server{/if}
				</Button>
			</div>
		</form>
	</div>
{/if}

<!-- ── Delete Confirm ────────────────────────────────────────────────────── -->
{#if confirmDeleteId}
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
						This will remove the server and all associated data.
					</p>
				</div>
			</div>
			<div class="flex justify-end gap-2">
				<Button variant="outline" size="sm" onclick={() => (confirmDeleteId=null)}>Cancel</Button>
				<Button variant="destructive" size="sm"
					onclick={() => deleteServer(confirmDeleteId!)}
					disabled={deletingId !== null} class="gap-1.5">
					{#if deletingId}<Loader2 class="w-3.5 h-3.5 animate-spin" /> Deleting…
					{:else}<Trash2 class="w-3.5 h-3.5" /> Delete{/if}
				</Button>
			</div>
		</div>
	</div>
{/if}
