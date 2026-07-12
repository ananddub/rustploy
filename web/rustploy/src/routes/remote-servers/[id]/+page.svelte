<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { getAuthSession } from '$lib/auth';
	import {
		remoteServerControllerGet,
		remoteServerControllerPatch,
		remoteServerControllerDelete,
		remoteServerControllerActivate,
		remoteServerControllerDeactivate,
		remoteServerControllerTestConnection,
		serverControllerSessions,
		serverControllerClearSessions,
		serverControllerAudit,
		serverControllerSetup,
		serverControllerTestConnection,
		sshKeyControllerList
	} from '$lib/client/sdk.gen';
	import type {
		RemoteServerResponseDto,
		SshKeyResponseDto,
		ServerConnectionResponseDto,
		ServerAuditDto,
		SetupOutcomeDto
	} from '$lib/client/types.gen';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import Switch from '$lib/components/Switch.svelte';
	import { withToast } from '$lib/toast';
	import {
		Server, Save, Trash2, AlertTriangle, TestTube2,
		Power, PowerOff, RefreshCw, CheckCircle, XCircle,
		Globe, FileKey, Clock, Shield, Wrench
	} from '@lucide/svelte';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	const serverId = $derived(parseInt(page.params.id ?? '0'));

	const TABS = ['General', 'Sessions', 'Audit', 'Setup'] as const;
	let activeTab = $state('General');

	let server = $state<RemoteServerResponseDto | null>(null);
	let sshKeys = $state<SshKeyResponseDto[]>([]);
	let loading = $state(true);
	let synced = $state(false);

	// General form fields
	let fName = $state('');
	let fDesc = $state('');
	let fIp = $state('');
	let fPort = $state('22');
	let fUser = $state('root');
	let fType = $state('REMOTE');
	let fSshKeyId = $state<number | null>(null);
	let fCleanup = $state(false);
	let fCleanupCron = $state('');
	let generalSaving = $state(false);
	let generalError = $state('');
	let generalSaved = $state(false);

	$effect(() => {
		Promise.all([
			remoteServerControllerGet({ path: { id: serverId } }),
			sshKeyControllerList()
		]).then(([sRes, kRes]: any[]) => {
			server = sRes.data ?? null;
			sshKeys = (kRes.data as SshKeyResponseDto[]) ?? [];
			loading = false;
		});
	});

	$effect(() => {
		if (server && !synced) {
			fName = server.name;
			fDesc = server.description ?? '';
			fIp = server.ip_address;
			fPort = String(server.port);
			fUser = server.username;
			fType = server.server_type;
			fSshKeyId = server.ssh_key_id ?? null;
			fCleanup = server.enable_docker_cleanup === 1;
			fCleanupCron = server.log_cleanup_cron ?? '';
			synced = true;
		}
	});

	async function saveGeneral() {
		generalSaving = true; generalError = ''; generalSaved = false;
		try {
			const res = await remoteServerControllerPatch({
				path: { id: serverId },
				body: {
					name: fName || undefined,
					description: fDesc || undefined,
					ip_address: fIp || undefined,
					port: fPort ? parseInt(fPort) : undefined,
					username: fUser || undefined,
					server_type: fType,
					ssh_key_id: fSshKeyId ?? undefined,
					enable_docker_cleanup: fCleanup ? 1 : 0,
					log_cleanup_cron: fCleanupCron || undefined
				}
			});
			if (res.data) { server = res.data; generalSaved = true; setTimeout(() => (generalSaved = false), 2000); }
		} catch (e: any) { generalError = e?.message ?? 'Failed to save'; }
		finally { generalSaving = false; }
	}

	// Activate / Deactivate
	let toggling = $state(false);
	async function toggleActive() {
		if (!server) return;
		toggling = true;
		try {
			const res = server.server_status === 'ACTIVE'
				? await remoteServerControllerDeactivate({ path: { id: serverId } })
				: await remoteServerControllerActivate({ path: { id: serverId } });
			if ((res.data as any)?.server) server = (res.data as any).server;
		} finally { toggling = false; }
	}

	// Test connection
	let testing = $state(false);
	let testResult = $state<{ connected: boolean; connections: number } | null>(null);
	async function testConn() {
		testing = true; testResult = null;
		try {
			const res = await remoteServerControllerTestConnection({ path: { id: serverId } });
			testResult = res.data as any ?? null;
		} catch { testResult = { connected: false, connections: 0 }; }
		finally { testing = false; }
	}

	// Delete
	let confirmDelete = $state(false);
	let deleting = $state(false);
	async function doDelete() {
		deleting = true;
		try {
			await remoteServerControllerDelete({ path: { id: serverId } });
			goto('/remote-servers');
		} catch { deleting = false; }
	}

	// ── Sessions tab ───────────────────────────────────────────────
	let sessions = $state<ServerConnectionResponseDto | null>(null);
	let sessionsLoading = $state(false);
	let clearingSession = $state(false);

	async function loadSessions() {
		sessionsLoading = true;
		try {
			const res = await serverControllerSessions({ path: { id: serverId } });
			sessions = res.data as ServerConnectionResponseDto ?? null;
		} finally { sessionsLoading = false; }
	}

	async function clearSessions() {
		clearingSession = true;
		try {
			await serverControllerClearSessions({ path: { id: serverId } });
			sessions = null;
		} finally { clearingSession = false; }
	}

	// ── Audit tab ──────────────────────────────────────────────────
	let audit = $state<ServerAuditDto | null>(null);
	let auditLoading = $state(false);
	let auditError = $state('');
	let auditSudoPass = $state('');

	async function runAudit() {
		auditLoading = true; auditError = '';
		try {
			const res = await serverControllerAudit({
				path: { id: serverId },
				body: { sudo_password: auditSudoPass || undefined }
			});
			audit = res.data as ServerAuditDto ?? null;
		} catch (e: any) { auditError = e?.message ?? 'Audit failed'; }
		finally { auditLoading = false; }
	}

	// ── Setup tab ──────────────────────────────────────────────────
	let setup = $state<SetupOutcomeDto | null>(null);
	let setupLoading = $state(false);
	let setupError = $state('');
	let setupSudoPass = $state('');
	let setupAcmeEmail = $state('');
	let setupInstallDeps = $state(true);
	let setupPoolSize = $state('10');

	async function runSetup() {
		setupLoading = true; setupError = '';
		try {
			const res = await serverControllerSetup({
				path: { id: serverId },
				body: {
					install_dependencies: setupInstallDeps,
					sudo_password: setupSudoPass || undefined,
					acme_email: setupAcmeEmail || undefined,
					pool_size: setupPoolSize ? parseInt(setupPoolSize) : undefined
				}
			});
			setup = res.data as SetupOutcomeDto ?? null;
		} catch (e: any) { setupError = e?.message ?? 'Setup failed'; }
		finally { setupLoading = false; }
	}

	const inputCls = 'flex h-9 w-full rounded-md border border-input bg-secondary px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring';

	function statusDot(status: string) {
		const m: Record<string,string> = { ACTIVE: 'bg-green-500', INACTIVE: 'bg-muted-foreground/30', CONNECTING: 'bg-yellow-500 animate-pulse' };
		return m[status] ?? 'bg-red-500';
	}
	function statusBadgeClass(status: string) {
		const m: Record<string,string> = { ACTIVE: 'bg-green-500/15 text-green-500', INACTIVE: 'bg-muted/50 text-muted-foreground', CONNECTING: 'bg-yellow-500/15 text-yellow-500' };
		return m[status] ?? 'bg-destructive/15 text-destructive';
	}
	function formatDate(ts: number) {
		return new Date(ts * 1000).toLocaleString(undefined, { month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' });
	}
</script>

<PageLayout>
	<header class="flex items-center justify-between px-6 py-3 border-b border-border text-sm">
		<div class="flex items-center gap-2">
			<Server class="w-4 h-4 text-muted-foreground" />
			<button onclick={() => goto('/remote-servers')} class="text-muted-foreground hover:text-foreground transition-colors">Remote Servers</button>
			<span class="text-muted-foreground/30">/</span>
			<span class="font-medium">{server?.name ?? '...'}</span>
		</div>
		{#if server}
			<div class="flex items-center gap-2">
				<span class="inline-flex items-center gap-1.5 px-2 py-0.5 rounded text-xs font-medium {statusBadgeClass(server.server_status)}">
					<span class="w-1.5 h-1.5 rounded-full {statusDot(server.server_status)}"></span>
					{server.server_status}
				</span>
				<button onclick={testConn} disabled={testing} class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent disabled:opacity-50 transition-colors">
					<TestTube2 size={14} class={testing ? 'animate-spin' : ''} /> Test
				</button>
				<button onclick={toggleActive} disabled={toggling} class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent disabled:opacity-50 transition-colors">
					{#if toggling}<div class="w-3.5 h-3.5 border-2 border-current/30 border-t-current rounded-full animate-spin"></div>
					{:else if server.server_status === 'ACTIVE'}<PowerOff size={14} />{:else}<Power size={14} />{/if}
					{server.server_status === 'ACTIVE' ? 'Deactivate' : 'Activate'}
				</button>
			</div>
		{/if}
	</header>

	{#if loading}
		<div class="flex justify-center py-20"><div class="w-6 h-6 border-2 border-muted-foreground/30 border-t-foreground rounded-full animate-spin"></div></div>
	{:else if server}
		<!-- Tab bar -->
		<div class="flex border-b border-border px-6">
			{#each TABS as tab}
				<button
					onclick={() => { activeTab = tab; if (tab === 'Sessions') loadSessions(); }}
					class="px-4 py-2.5 text-sm border-b-2 -mb-px transition-colors {activeTab === tab ? 'border-foreground text-foreground font-medium' : 'border-transparent text-muted-foreground hover:text-foreground'}"
				>{tab}</button>
			{/each}
		</div>

		<main class="flex-1 px-8 py-6 max-w-3xl">

			<!-- ── GENERAL TAB ── -->
			{#if activeTab === 'General'}
				<div class="flex flex-col gap-6 animate-fade-up">

					<!-- Test result banner -->
					{#if testResult}
						<div class="flex items-center gap-2 px-4 py-3 rounded-lg border {testResult.connected ? 'bg-green-500/10 border-green-500/30 text-green-500' : 'bg-destructive/10 border-destructive/30 text-destructive'} text-sm">
							{#if testResult.connected}<CheckCircle size={15} /> Connection successful — {testResult.connections} active session{testResult.connections !== 1 ? 's' : ''}
							{:else}<XCircle size={15} /> Connection failed{/if}
						</div>
					{/if}

					<!-- Server info summary -->
					<section class="bg-card border border-border rounded-lg p-6">
						<h2 class="text-base font-semibold mb-4">Server Info</h2>
						<div class="grid grid-cols-2 md:grid-cols-3 gap-3">
							<div class="bg-secondary rounded-lg p-3">
								<p class="text-xs text-muted-foreground uppercase tracking-wide mb-1">Address</p>
								<p class="text-sm font-mono">{server.ip_address}:{server.port}</p>
							</div>
							<div class="bg-secondary rounded-lg p-3">
								<p class="text-xs text-muted-foreground uppercase tracking-wide mb-1">User</p>
								<p class="text-sm font-mono">{server.username}</p>
							</div>
							<div class="bg-secondary rounded-lg p-3">
								<p class="text-xs text-muted-foreground uppercase tracking-wide mb-1">Type</p>
								<p class="text-sm">{server.server_type}</p>
							</div>
							<div class="bg-secondary rounded-lg p-3">
								<p class="text-xs text-muted-foreground uppercase tracking-wide mb-1">App Name</p>
								<p class="text-sm font-mono truncate">{server.app_name}</p>
							</div>
							<div class="bg-secondary rounded-lg p-3">
								<p class="text-xs text-muted-foreground uppercase tracking-wide mb-1">Created</p>
								<p class="text-sm">{formatDate(server.created_at)}</p>
							</div>
							<div class="bg-secondary rounded-lg p-3">
								<p class="text-xs text-muted-foreground uppercase tracking-wide mb-1">Updated</p>
								<p class="text-sm">{formatDate(server.updated_at)}</p>
							</div>
						</div>
					</section>

					<!-- Edit form -->
					<section class="bg-card border border-border rounded-lg p-6">
						<h2 class="text-base font-semibold mb-1">Configuration</h2>
						<p class="text-sm text-muted-foreground mb-5">Update server connection settings.</p>
						<div class="flex flex-col gap-4">
							<div class="grid grid-cols-2 gap-4">
								<div class="flex flex-col gap-1.5">
									<label for="s-name" class="text-sm font-medium text-muted-foreground">Name</label>
									<input id="s-name" class={inputCls} bind:value={fName} />
								</div>
								<div class="flex flex-col gap-1.5">
									<label for="s-type" class="text-sm font-medium text-muted-foreground">Server Type</label>
									<select id="s-type" class="h-9 w-full rounded-md border border-input bg-secondary px-3 text-sm focus:outline-none focus:ring-1 focus:ring-ring" bind:value={fType}>
										<option value="REMOTE">REMOTE</option>
										<option value="SWARM">SWARM</option>
									</select>
								</div>
							</div>
							<div class="grid grid-cols-3 gap-4">
								<div class="col-span-2 flex flex-col gap-1.5">
									<label for="s-ip" class="text-sm font-medium text-muted-foreground">IP Address</label>
									<input id="s-ip" class="{inputCls} font-mono" bind:value={fIp} placeholder="192.168.1.100" />
								</div>
								<div class="flex flex-col gap-1.5">
									<label for="s-port" class="text-sm font-medium text-muted-foreground">Port</label>
									<input id="s-port" type="number" class={inputCls} bind:value={fPort} />
								</div>
							</div>
							<div class="grid grid-cols-2 gap-4">
								<div class="flex flex-col gap-1.5">
									<label for="s-user" class="text-sm font-medium text-muted-foreground">Username</label>
									<input id="s-user" class="{inputCls} font-mono" bind:value={fUser} />
								</div>
								<div class="flex flex-col gap-1.5">
									<label for="s-sshkey" class="text-sm font-medium text-muted-foreground">SSH Key</label>
									<select id="s-sshkey" class="h-9 w-full rounded-md border border-input bg-secondary px-3 text-sm focus:outline-none focus:ring-1 focus:ring-ring" bind:value={fSshKeyId}>
										<option value={null}>None (password auth)</option>
										{#each sshKeys as k}<option value={k.id}>{k.name}</option>{/each}
									</select>
								</div>
							</div>
							<div class="flex flex-col gap-1.5">
								<label for="s-desc" class="text-sm font-medium text-muted-foreground">Description</label>
								<input id="s-desc" class={inputCls} bind:value={fDesc} placeholder="Optional" />
							</div>
							<Switch checked={fCleanup} onchange={(v) => (fCleanup = v)} label="Enable Docker Cleanup" description="Automatically clean up unused Docker images and containers" />
							{#if fCleanup}
								<div class="flex flex-col gap-1.5">
									<label for="s-cron" class="text-sm font-medium text-muted-foreground">Cleanup Cron Schedule</label>
									<input id="s-cron" class="{inputCls} font-mono" bind:value={fCleanupCron} placeholder="0 3 * * *" />
								</div>
							{/if}
							{#if generalError}<div class="rounded-md bg-destructive/10 border border-destructive/30 px-3 py-2 text-sm text-destructive">{generalError}</div>{/if}
							<div class="flex justify-end items-center gap-3">
								{#if generalSaved}<span class="text-sm text-green-500">Saved!</span>{/if}
								<button onclick={saveGeneral} disabled={generalSaving} class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50">
									{#if generalSaving}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>Saving…{:else}<Save size={13} />Save{/if}
								</button>
							</div>
						</div>
					</section>

					<!-- Danger zone -->
					<section class="bg-card border border-destructive/40 rounded-lg p-6">
						<div class="flex items-center gap-2 mb-1"><AlertTriangle size={16} class="text-destructive" /><h2 class="text-base font-semibold text-destructive">Danger Zone</h2></div>
						<p class="text-sm text-muted-foreground mb-4">Permanently remove this server from Rustploy.</p>
						{#if !confirmDelete}
							<button onclick={() => (confirmDelete = true)} class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md border border-destructive/50 text-destructive text-sm font-medium hover:bg-destructive hover:text-destructive-foreground transition-colors">
								<Trash2 size={14} /> Delete Server
							</button>
						{:else}
							<div class="bg-destructive/10 border border-destructive/30 rounded-lg p-4 flex flex-col gap-3">
								<p class="text-sm font-medium text-destructive">Delete <strong>{server.name}</strong>? All associated data will be removed.</p>
								<div class="flex gap-2">
									<button onclick={doDelete} disabled={deleting} class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-destructive text-destructive-foreground text-sm font-medium hover:bg-destructive/90 disabled:opacity-50">
										{#if deleting}<div class="w-3.5 h-3.5 border-2 border-destructive-foreground/30 border-t-destructive-foreground rounded-full animate-spin"></div>Deleting…{:else}<Trash2 size={14} />Yes, Delete{/if}
									</button>
									<button onclick={() => (confirmDelete = false)} class="px-3 py-1.5 rounded-md text-sm text-muted-foreground hover:bg-accent transition-colors">Cancel</button>
								</div>
							</div>
						{/if}
					</section>
				</div>

			<!-- ── SESSIONS TAB ── -->
			{:else if activeTab === 'Sessions'}
				<div class="flex flex-col gap-6 animate-fade-up">
					<section class="bg-card border border-border rounded-lg p-6">
						<div class="flex items-center justify-between mb-4">
							<div>
								<h2 class="text-base font-semibold">Active Sessions</h2>
								<p class="text-sm text-muted-foreground mt-1">Current SSH connection pool status for this server.</p>
							</div>
							<div class="flex items-center gap-2">
								<button onclick={loadSessions} disabled={sessionsLoading} class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent disabled:opacity-50">
									<RefreshCw size={14} class={sessionsLoading ? 'animate-spin' : ''} /> Refresh
								</button>
								<button onclick={clearSessions} disabled={clearingSession || !sessions} class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-destructive/70 hover:text-destructive hover:bg-destructive/10 disabled:opacity-50 transition-colors">
									{#if clearingSession}<div class="w-3.5 h-3.5 border-2 border-current/30 border-t-current rounded-full animate-spin"></div>{/if}
									Clear Sessions
								</button>
							</div>
						</div>
						{#if sessionsLoading}
							<div class="flex justify-center py-10"><div class="w-5 h-5 border-2 border-muted-foreground/30 border-t-foreground rounded-full animate-spin"></div></div>
						{:else if !sessions}
							<div class="flex flex-col items-center justify-center py-10 text-muted-foreground/30"><Globe size={36} class="mb-2" /><p class="text-sm">Click Refresh to load sessions</p></div>
						{:else}
							<div class="grid grid-cols-2 md:grid-cols-3 gap-3">
								<div class="bg-secondary rounded-lg p-4">
									<p class="text-xs text-muted-foreground uppercase tracking-wide mb-1">Connected</p>
									<p class="text-2xl font-semibold {sessions.connected ? 'text-green-500' : 'text-destructive'}">{sessions.connected ? 'Yes' : 'No'}</p>
								</div>
								<div class="bg-secondary rounded-lg p-4">
									<p class="text-xs text-muted-foreground uppercase tracking-wide mb-1">Active Sessions</p>
									<p class="text-2xl font-semibold">{sessions.connections}</p>
								</div>
								<div class="bg-secondary rounded-lg p-4">
									<p class="text-xs text-muted-foreground uppercase tracking-wide mb-1">Active Channels</p>
									<p class="text-2xl font-semibold">{sessions.active_channels}</p>
								</div>
								<div class="bg-secondary rounded-lg p-4">
									<p class="text-xs text-muted-foreground uppercase tracking-wide mb-1">Max Pool Size</p>
									<p class="text-2xl font-semibold">{sessions.max_pool_size}</p>
								</div>
								<div class="bg-secondary rounded-lg p-4">
									<p class="text-xs text-muted-foreground uppercase tracking-wide mb-1">Max Channels/Session</p>
									<p class="text-2xl font-semibold">{sessions.max_channels_per_session}</p>
								</div>
								<div class="bg-secondary rounded-lg p-4">
									<p class="text-xs text-muted-foreground uppercase tracking-wide mb-1">Reused Sessions</p>
									<p class="text-2xl font-semibold">{sessions.reused_sessions}</p>
								</div>
							</div>
						{/if}
					</section>
				</div>

			<!-- ── AUDIT TAB ── -->
			{:else if activeTab === 'Audit'}
				<div class="flex flex-col gap-6 animate-fade-up">
					<section class="bg-card border border-border rounded-lg p-6">
						<h2 class="text-base font-semibold mb-1">Server Audit</h2>
						<p class="text-sm text-muted-foreground mb-5">Check installed tools, network, and system readiness on this server.</p>
						<div class="flex flex-col gap-3 mb-4">
							<div class="flex flex-col gap-1.5">
								<label for="audit-sudo" class="text-sm font-medium text-muted-foreground">Sudo Password <span class="text-muted-foreground/50 font-normal">(optional)</span></label>
								<input id="audit-sudo" type="password" class={inputCls} bind:value={auditSudoPass} placeholder="Only needed if required" />
							</div>
						</div>
						{#if auditError}<div class="mb-3 rounded-md bg-destructive/10 border border-destructive/30 px-3 py-2 text-sm text-destructive">{auditError}</div>{/if}
						<button onclick={runAudit} disabled={auditLoading} class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50">
							{#if auditLoading}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>Running…{:else}<Shield size={13} />Run Audit{/if}
						</button>
					</section>
					{#if audit}
						<section class="bg-card border border-border rounded-lg p-6 animate-fade-up">
							<h2 class="text-base font-semibold mb-4">Audit Results</h2>
							<div class="grid grid-cols-2 gap-3 mb-4">
								<div class="bg-secondary rounded-lg p-3 flex items-center justify-between">
									<span class="text-sm">OS</span>
									<span class="text-sm font-mono">{audit.os_id}</span>
								</div>
								<div class="bg-secondary rounded-lg p-3 flex items-center justify-between">
									<span class="text-sm">Architecture</span>
									<span class="text-sm font-mono">{audit.architecture}</span>
								</div>
								<div class="bg-secondary rounded-lg p-3 flex items-center justify-between">
									<span class="text-sm">Base Directory</span>
									{#if audit.base_directory_exists}<CheckCircle size={15} class="text-green-500" />{:else}<XCircle size={15} class="text-destructive" />{/if}
								</div>
								<div class="bg-secondary rounded-lg p-3 flex items-center justify-between">
									<span class="text-sm">Network</span>
									{#if audit.network_exists}<CheckCircle size={15} class="text-green-500" />{:else}<XCircle size={15} class="text-destructive" />{/if}
								</div>
								<div class="bg-secondary rounded-lg p-3 flex items-center justify-between">
									<span class="text-sm">Docker Group</span>
									{#if audit.docker_group_member}<CheckCircle size={15} class="text-green-500" />{:else}<XCircle size={15} class="text-destructive" />{/if}
								</div>
								<div class="bg-secondary rounded-lg p-3 flex items-center justify-between">
									<span class="text-sm">Swarm Active</span>
									{#if audit.swarm_active}<CheckCircle size={15} class="text-green-500" />{:else}<XCircle size={15} class="text-muted-foreground/40" />{/if}
								</div>
							</div>
							<p class="text-xs text-muted-foreground uppercase tracking-wide mb-2">Tools</p>
							<div class="grid grid-cols-2 gap-2">
								{#each [['Docker', audit.docker], ['Git', audit.git], ['Nixpacks', audit.nixpacks], ['Railpack', audit.railpack], ['Buildpacks', audit.buildpacks], ['Rclone', audit.rclone]] as [name, tool]}
									<div class="bg-secondary rounded-lg px-3 py-2 flex items-center justify-between">
										<span class="text-sm">{name}</span>
										<div class="flex items-center gap-2">
											{#if (tool as any).version}<span class="text-xs font-mono text-muted-foreground">{(tool as any).version}</span>{/if}
											{#if (tool as any).installed}<CheckCircle size={14} class="text-green-500" />{:else}<XCircle size={14} class="text-muted-foreground/40" />{/if}
										</div>
									</div>
								{/each}
							</div>
						</section>
					{/if}
				</div>

			<!-- ── SETUP TAB ── -->
			{:else if activeTab === 'Setup'}
				<div class="flex flex-col gap-6 animate-fade-up">
					<section class="bg-card border border-border rounded-lg p-6">
						<h2 class="text-base font-semibold mb-1">Server Setup</h2>
						<p class="text-sm text-muted-foreground mb-5">Install Rustploy dependencies (Docker, Traefik, etc.) on this server.</p>
						<div class="flex flex-col gap-4 mb-4">
							<Switch checked={setupInstallDeps} onchange={(v) => (setupInstallDeps = v)} label="Install Dependencies" description="Install Docker, Git, Nixpacks, and other required tools" />
							<div class="flex flex-col gap-1.5">
								<label for="setup-sudo" class="text-sm font-medium text-muted-foreground">Sudo Password <span class="text-muted-foreground/50 font-normal">(optional)</span></label>
								<input id="setup-sudo" type="password" class={inputCls} bind:value={setupSudoPass} placeholder="Only needed if required" />
							</div>
							<div class="flex flex-col gap-1.5">
								<label for="setup-email" class="text-sm font-medium text-muted-foreground">ACME Email <span class="text-muted-foreground/50 font-normal">(for Let's Encrypt)</span></label>
								<input id="setup-email" type="email" class={inputCls} bind:value={setupAcmeEmail} placeholder="admin@example.com" />
							</div>
							<div class="flex flex-col gap-1.5 w-40">
								<label for="setup-pool" class="text-sm font-medium text-muted-foreground">SSH Pool Size</label>
								<input id="setup-pool" type="number" min="1" max="50" class={inputCls} bind:value={setupPoolSize} />
							</div>
						</div>
						{#if setupError}<div class="mb-3 rounded-md bg-destructive/10 border border-destructive/30 px-3 py-2 text-sm text-destructive">{setupError}</div>{/if}
						<button onclick={runSetup} disabled={setupLoading} class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50">
							{#if setupLoading}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>Running…{:else}<Wrench size={13} />Run Setup{/if}
						</button>
					</section>
					{#if setup}
						<section class="bg-card border border-border rounded-lg p-6 animate-fade-up">
							<h2 class="text-base font-semibold mb-4">Setup Results</h2>
							{#if setup.completed.length > 0}
								<div class="mb-4">
									<p class="text-xs text-muted-foreground uppercase tracking-wide mb-2">Completed Steps</p>
									<div class="flex flex-col gap-1.5">
										{#each setup.completed as step}
											<div class="flex items-center gap-2 text-sm text-green-500">
												<CheckCircle size={13} />{step}
											</div>
										{/each}
									</div>
								</div>
							{/if}
							{#if setup.audit}
								<p class="text-xs text-muted-foreground uppercase tracking-wide mb-2 mt-4">Post-setup Audit</p>
								<div class="grid grid-cols-2 gap-2">
									{#each [['Docker', setup.audit.docker], ['Git', setup.audit.git], ['Nixpacks', setup.audit.nixpacks], ['Railpack', setup.audit.railpack]] as [name, tool]}
										<div class="bg-secondary rounded-lg px-3 py-2 flex items-center justify-between">
											<span class="text-sm">{name}</span>
											{#if (tool as any).installed}<CheckCircle size={14} class="text-green-500" />{:else}<XCircle size={14} class="text-muted-foreground/40" />{/if}
										</div>
									{/each}
								</div>
							{/if}
						</section>
					{/if}
				</div>
			{/if}
		</main>
	{/if}
</PageLayout>
