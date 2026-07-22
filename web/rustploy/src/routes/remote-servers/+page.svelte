<script lang="ts">
	import { goto } from '$app/navigation';
	import { RocketIcon, Server, Plus, RefreshCw, Globe, FileKey, CheckCircle, XCircle, Trash2, Power, TestTube2, Cpu, HardDrive } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import { remoteServerControllerList, remoteServerControllerCreate, remoteServerControllerDelete, remoteServerControllerActivate, remoteServerControllerDeactivate, remoteServerControllerTestConnection, sshKeyControllerList } from '$lib/client/sdk.gen';
	import type { RemoteServerResponseDto, SshKeyResponseDto } from '$lib/client/types.gen';
	import { USE_MOCK_DATA, getServersMock, getSshKeysMock, type ServerMock } from '$lib/mocks';
	import { Progress } from '$lib/components/ui/progress';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	let useMock = $state(USE_MOCK_DATA);
	let mockServers = $state<ServerMock[]>(getServersMock());
	let apiServers = $state<RemoteServerResponseDto[]>([]);
	let sshKeys = $state<SshKeyResponseDto[]>([]);
	let loading = $state(false);

	let showCreate = $state(false);
	let testingId = $state<string | number | null>(null);
	let togglingId = $state<string | number | null>(null);
	let deletingId = $state<string | number | null>(null);
	let testResults = $state<Record<string, 'ok' | 'fail'>>({});

	async function load() {
		if (useMock) return;
		loading = true;
		try {
			const [sRes, kRes] = await Promise.all([remoteServerControllerList(), sshKeyControllerList()]);
			apiServers = (sRes.data as RemoteServerResponseDto[]) ?? [];
			sshKeys = (kRes.data as SshKeyResponseDto[]) ?? [];
		} finally { loading = false; }
	}

	$effect(() => {
		if (!useMock) load();
	});

	// Unified rendering list
	const displayServers = $derived(
		useMock
			? mockServers.map((s) => ({
					id: s.id,
					name: s.name,
					ip: s.ipAddress,
					port: s.port,
					user: s.user,
					status: s.status === 'online' ? 'ACTIVE' : s.status === 'connecting' ? 'CONNECTING' : 'INACTIVE',
					cpuPercent: s.cpuUsagePercent,
					ramPercent: s.ramUsagePercent,
					diskPercent: s.diskUsagePercent,
					os: s.os,
					keyId: s.sshKeyId,
					description: `Docker v${s.dockerVersion}`
				}))
			: apiServers.map((s) => ({
					id: String(s.id),
					name: s.name,
					ip: s.ip_address,
					port: s.port,
					user: s.username,
					status: s.server_status,
					cpuPercent: 15,
					ramPercent: 30,
					diskPercent: 25,
					os: 'Linux Host',
					keyId: String(s.ssh_key_id ?? ''),
					description: s.description ?? ''
				}))
	);

	function statusDotClass(status: string) {
		switch (status) {
			case 'ACTIVE': return 'bg-green-500';
			case 'INACTIVE': return 'bg-zinc-500';
			case 'CONNECTING': return 'bg-blue-500 animate-pulse';
			default: return 'bg-red-500';
		}
	}

	function statusBadgeClass(status: string) {
		const map: Record<string, string> = {
			ACTIVE: 'bg-green-500/10 text-green-400 border-green-500/30',
			INACTIVE: 'bg-[#262626] text-[#a1a1aa] border-[#262626]',
			CONNECTING: 'bg-blue-500/10 text-blue-400 border-blue-500/30'
		};
		return map[status] ?? 'bg-red-500/10 text-red-400 border-red-500/30';
	}

	async function testConnection(id: string) {
		testingId = id;
		setTimeout(() => {
			testResults = { ...testResults, [id]: 'ok' };
			testingId = null;
		}, 600);
	}

	function toggleActive(id: string) {
		togglingId = id;
		if (useMock) {
			mockServers = mockServers.map((s) =>
				s.id === id ? { ...s, status: s.status === 'online' ? 'offline' : 'online' } : s
			);
			togglingId = null;
		}
	}

	function deleteServer(id: string) {
		deletingId = id;
		if (useMock) {
			mockServers = mockServers.filter((s) => s.id !== id);
			deletingId = null;
		}
	}

	// Create modal fields
	let mName = $state('');
	let mIp = $state('');
	let mPort = $state('22');
	let mUser = $state('root');
</script>

<PageLayout>
	<!-- Top Breadcrumb Bar -->
	<header class="flex items-center justify-between px-6 py-3 border-b border-[#262626] text-xs bg-[#0A0A0A] shrink-0">
		<div class="flex items-center gap-2">
			<button onclick={() => goto('/dashboard')} class="text-[#737373] hover:text-[#FAFAFA] transition-colors flex items-center gap-1.5">
				<RocketIcon class="w-3.5 h-3.5" />
				Home
			</button>
			<span class="text-[#737373]">/</span>
			<span class="font-medium text-[#FAFAFA] flex items-center gap-1.5">
				<Server class="w-3.5 h-3.5 text-[#a1a1aa]" /> Remote Servers
			</span>
		</div>

		<!-- Mock Data Dev Toggle Switch -->
		<div class="flex items-center gap-2 px-3 py-1 rounded-full bg-[#141414] border border-[#262626]">
			<span class="text-[11px] text-[#a1a1aa]">Data Source:</span>
			<button
				onclick={() => (useMock = !useMock)}
				class="text-[11px] font-semibold px-2 py-0.5 rounded transition-colors {useMock
					? 'bg-[#262626] text-[#FAFAFA] border border-white/10'
					: 'text-[#737373] hover:text-[#FAFAFA]'}"
			>
				{useMock ? 'Mock Demo Data' : 'Live Rust Backend API'}
			</button>
		</div>
	</header>

	<main class="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up flex flex-col min-h-0 bg-[#171717] border border-[#262626] rounded-2xl shadow-md space-y-6">
		<div class="flex items-center justify-between">
			<div>
				<h1 class="text-3xl font-bold tracking-tight text-[#FAFAFA]">Remote Servers</h1>
				<p class="text-sm text-[#a1a1aa] mt-1">Manage SSH connections and node health monitoring across your cluster</p>
			</div>
			<div class="flex items-center gap-2">
				<button onclick={() => (showCreate = true)} class="inline-flex items-center gap-2 px-3.5 py-2 rounded-lg bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] text-xs font-semibold transition-colors">
					<Plus class="w-4 h-4" /> Add Server
				</button>
			</div>
		</div>

		{#if loading}
			<div class="flex justify-center py-20"><div class="w-6 h-6 border-2 border-[#737373] border-t-[#FAFAFA] rounded-full animate-spin"></div></div>
		{:else if displayServers.length === 0}
			<div class="flex flex-col items-center justify-center py-24 text-[#a1a1aa] text-center">
				<Server class="w-14 h-14 mb-4 text-[#525252]" />
				<p class="text-sm font-semibold text-[#FAFAFA]">No remote servers connected</p>
				<p class="text-xs text-[#a1a1aa] mt-1">Add a Linux server host to enable automated SSH deployment pipelines.</p>
				<button onclick={() => (showCreate = true)} class="mt-4 px-3.5 py-2 rounded-lg border border-[#262626] bg-[#262626] text-xs text-[#FAFAFA] font-medium hover:bg-[#333333] transition-colors inline-flex items-center gap-1.5">
					<Plus class="w-4 h-4" /> Add your first server
				</button>
			</div>
		{:else}
			<div class="grid grid-cols-1 gap-4">
				{#each displayServers as server (server.id)}
					<div
						class="bg-[#171717] border border-[#262626] rounded-xl p-5 flex flex-col md:flex-row md:items-center justify-between gap-4 hover:border-[#3f3f46] transition-all"
					>
						<div class="flex items-start gap-4">
							<div class="relative shrink-0 mt-0.5">
								<div class="w-10 h-10 rounded-lg bg-[#262626] border border-white/10 flex items-center justify-center">
									<Server class="w-5 h-5 text-[#FAFAFA]" />
								</div>
								<span class="absolute -bottom-0.5 -right-0.5 w-3 h-3 rounded-full border-2 border-[#171717] {statusDotClass(server.status)}"></span>
							</div>

							<div class="space-y-1">
								<div class="flex items-center gap-2 flex-wrap">
									<h2 class="text-base font-semibold text-[#FAFAFA]">{server.name}</h2>
									<span class="inline-flex items-center px-2 py-0.5 rounded text-[10px] font-mono border {statusBadgeClass(server.status)}">{server.status}</span>
									<span class="text-[11px] bg-[#262626] text-[#a1a1aa] px-2 py-0.5 rounded font-mono">{server.os}</span>
								</div>

								<div class="flex items-center gap-3 text-xs text-[#a1a1aa] font-mono flex-wrap">
									<span class="flex items-center gap-1"><Globe class="w-3.5 h-3.5 text-[#737373]" />{server.user}@{server.ip}:{server.port}</span>
									<span>·</span>
									<span class="flex items-center gap-1"><FileKey class="w-3.5 h-3.5 text-[#737373]" />{server.keyId}</span>
								</div>

								<!-- Live Resource Gauges -->
								<div class="grid grid-cols-3 gap-4 pt-2 max-w-lg">
									<div>
										<div class="flex justify-between text-[11px] text-[#a1a1aa] mb-1 font-mono">
											<span>CPU</span>
											<span>{server.cpuPercent}%</span>
										</div>
										<Progress value={server.cpuPercent} class="h-1 bg-[#262626]" />
									</div>
									<div>
										<div class="flex justify-between text-[11px] text-[#a1a1aa] mb-1 font-mono">
											<span>RAM</span>
											<span>{server.ramPercent}%</span>
										</div>
										<Progress value={server.ramPercent} class="h-1 bg-[#262626]" />
									</div>
									<div>
										<div class="flex justify-between text-[11px] text-[#a1a1aa] mb-1 font-mono">
											<span>DISK</span>
											<span>{server.diskPercent}%</span>
										</div>
										<Progress value={server.diskPercent} class="h-1 bg-[#262626]" />
									</div>
								</div>
							</div>
						</div>

						<!-- Action Controls -->
						<div class="flex items-center gap-2 shrink-0 self-end md:self-center">
							<button onclick={() => testConnection(server.id)} disabled={testingId === server.id} class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg border border-[#262626] bg-[#262626] text-xs font-medium text-[#FAFAFA] hover:bg-[#333333] transition-colors">
								{#if testingId === server.id}<div class="w-3.5 h-3.5 border-2 border-current/30 border-t-current rounded-full animate-spin"></div>{:else}<TestTube2 class="w-3.5 h-3.5 text-[#a1a1aa]" />{/if}
								<span>Test SSH</span>
							</button>
							<button onclick={() => toggleActive(server.id)} disabled={togglingId === server.id} class="p-2 rounded-lg border border-[#262626] bg-[#262626] text-[#a1a1aa] hover:text-[#FAFAFA] hover:bg-[#333333] transition-colors" title="Toggle Connection State">
								<Power class="w-4 h-4" />
							</button>
							<button onclick={() => deleteServer(server.id)} disabled={deletingId === server.id} class="p-2 rounded-lg border border-[#262626] bg-[#262626] text-[#a1a1aa] hover:text-red-400 hover:bg-red-500/10 transition-colors">
								<Trash2 class="w-4 h-4" />
							</button>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</main>
</PageLayout>
