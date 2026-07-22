<script lang="ts">
	import { goto } from '$app/navigation';
	import {
		Zap,
		Search,
		Rocket,
		Boxes,
		Loader2,
		CircleCheck,
		CircleX,
		ListTodo,
		Filter,
		GitBranch,
		Clock
	} from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
	import { getAuthSession } from '$lib/auth';
	import { deploymentControllerActive } from '$lib/client/sdk.gen';
	import type { ActiveDeploymentDto } from '$lib/client/types.gen';
	import { USE_MOCK_DATA, getDeploymentsMock, type DeploymentMock } from '$lib/mocks';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import * as Tabs from '$lib/components/ui/tabs';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Badge } from '$lib/components/ui/badge';
	import { Skeleton } from '$lib/components/ui/skeleton';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	let useMock = $state(USE_MOCK_DATA);
	let mockDeployments = $state<DeploymentMock[]>(getDeploymentsMock());
	let apiDeployments = $state<ActiveDeploymentDto[]>([]);
	let loading = $state(false);

	let searchQuery = $state('');
	let filterState = $state<string>('all');
	let activeTab = $state('deployments');

	$effect(() => {
		if (!useMock) {
			loading = true;
			deploymentControllerActive()
				.then((res: any) => {
					apiDeployments = (res.data as ActiveDeploymentDto[]) ?? [];
				})
				.finally(() => {
					loading = false;
				});
		}
	});

	// Unified rendering list
	const displayDeployments = $derived(
		useMock
			? mockDeployments.map((d) => ({
					id: d.id,
					title: `${d.projectName} · ${d.serviceName}`,
					kind: d.kind,
					state: d.state,
					branch: d.branch,
					commitHash: d.commitHash,
					commitMsg: d.commitMessage,
					duration: `${d.durationSeconds}s`,
					createdAt: d.createdAt
				}))
			: apiDeployments.map((d) => ({
					id: String(d.id),
					title: `${d.kind || 'service'} #${d.id}`,
					kind: d.kind || 'application',
					state: d.state || 'idle',
					branch: 'main',
					commitHash: 'head',
					commitMsg: 'Manual trigger',
					duration: '30s',
					createdAt: 'recently'
				}))
	);

	const filtered = $derived(
		displayDeployments.filter((d) => {
			const matchSearch =
				!searchQuery ||
				d.id.toLowerCase().includes(searchQuery.toLowerCase()) ||
				d.title.toLowerCase().includes(searchQuery.toLowerCase());
			const matchState = filterState === 'all' || d.state === filterState;
			return matchSearch && matchState;
		})
	);

	const stats = $derived({
		total: displayDeployments.length,
		running: displayDeployments.filter((d) => d.state === 'running' || d.state === 'building').length,
		success: displayDeployments.filter((d) => d.state === 'done').length,
		failed: displayDeployments.filter((d) => d.state === 'error').length
	});

	const stateFilters = ['all', 'running', 'done', 'error'];

	function getServiceIcon(kind: string | undefined) {
		if (kind === 'application') return Rocket;
		if (kind === 'compose') return Boxes;
		return Rocket;
	}
</script>

<PageLayout>
	<!-- Top Breadcrumb Bar -->
	<header class="flex items-center justify-between px-6 py-3 border-b border-[#262626] text-xs bg-[#0A0A0A] shrink-0">
		<div class="flex items-center gap-2">
			<Zap class="w-3.5 h-3.5 text-[#a1a1aa]" />
			<span class="font-medium text-[#FAFAFA]">Deployments</span>
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
		<!-- Stats Grid (Dokploy-style stats cards) -->
		<div class="grid grid-cols-2 md:grid-cols-4 gap-3.5">
			<Card.Root class="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs">
				<Card.Content class="p-4">
					<div class="flex items-center justify-between">
						<div>
							<p class="text-[10px] font-semibold text-[#a1a1aa] uppercase tracking-wider">Total</p>
							<p class="text-2xl font-bold mt-1 text-[#FAFAFA]">{stats.total}</p>
						</div>
						<div class="w-8 h-8 rounded-lg bg-[#262626] flex items-center justify-center border border-white/10">
							<Zap class="w-4 h-4 text-[#a1a1aa]" />
						</div>
					</div>
				</Card.Content>
			</Card.Root>

			<Card.Root class="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs">
				<Card.Content class="p-4">
					<div class="flex items-center justify-between">
						<div>
							<p class="text-[10px] font-semibold text-[#a1a1aa] uppercase tracking-wider">Running</p>
							<p class="text-2xl font-bold mt-1 text-blue-400">{stats.running}</p>
						</div>
						<div class="w-8 h-8 rounded-lg bg-blue-500/10 flex items-center justify-center border border-blue-500/20">
							<Loader2 class="w-4 h-4 text-blue-400 animate-spin" />
						</div>
					</div>
				</Card.Content>
			</Card.Root>

			<Card.Root class="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs">
				<Card.Content class="p-4">
					<div class="flex items-center justify-between">
						<div>
							<p class="text-[10px] font-semibold text-[#a1a1aa] uppercase tracking-wider">Succeeded</p>
							<p class="text-2xl font-bold mt-1 text-green-400">{stats.success}</p>
						</div>
						<div class="w-8 h-8 rounded-lg bg-green-500/10 flex items-center justify-center border border-green-500/20">
							<CircleCheck class="w-4 h-4 text-green-400" />
						</div>
					</div>
				</Card.Content>
			</Card.Root>

			<Card.Root class="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs">
				<Card.Content class="p-4">
					<div class="flex items-center justify-between">
						<div>
							<p class="text-[10px] font-semibold text-[#a1a1aa] uppercase tracking-wider">Failed</p>
							<p class="text-2xl font-bold mt-1 text-red-400">{stats.failed}</p>
						</div>
						<div class="w-8 h-8 rounded-lg bg-red-500/10 flex items-center justify-center border border-red-500/20">
							<CircleX class="w-4 h-4 text-red-400" />
						</div>
					</div>
				</Card.Content>
			</Card.Root>
		</div>

		<!-- Deployments Filter & Table -->
		<div class="space-y-4">
			<div class="flex items-center gap-3 flex-wrap justify-between">
				<div class="relative flex-1 max-w-sm">
					<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-[#737373]" />
					<Input
						bind:value={searchQuery}
						placeholder="Search by deployment ID or project..."
						class="pl-9 text-xs h-9 bg-[#141414] border-[#262626] text-[#FAFAFA] placeholder:text-[#737373]"
					/>
				</div>
				<div class="flex items-center gap-1 bg-[#141414] p-1 rounded-lg border border-[#262626]">
					{#each stateFilters as sf}
						<button
							class="px-2.5 py-1 text-xs font-medium rounded-md transition-colors capitalize {filterState === sf
								? 'bg-[#262626] text-[#FAFAFA]'
								: 'text-[#737373] hover:text-[#FAFAFA]'}"
							onclick={() => (filterState = sf)}
						>
							{sf}
						</button>
					{/each}
				</div>
			</div>

			<Card.Root class="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs overflow-hidden">
				<Card.Content class="p-0">
					{#if loading}
						<div class="p-6 space-y-4">
							{#each Array(4) as _}
								<div class="flex items-center gap-4">
									<Skeleton class="h-4 w-8" />
									<Skeleton class="h-4 w-32" />
									<Skeleton class="h-6 w-20 rounded-full" />
									<Skeleton class="h-4 flex-1" />
								</div>
							{/each}
						</div>
					{:else if filtered.length === 0}
						<div class="flex flex-col items-center justify-center py-20 px-4 text-center">
							<div class="w-12 h-12 rounded-xl bg-[#141414] border border-[#262626] flex items-center justify-center mb-3">
								<Rocket class="w-6 h-6 text-[#737373]" />
							</div>
							<h4 class="text-sm font-semibold text-[#FAFAFA]">No deployments found</h4>
							<p class="text-xs text-[#a1a1aa] mt-1 max-w-xs leading-relaxed">
								No deployments match your current filter criteria.
							</p>
						</div>
					{:else}
						<Table.Root class="w-full text-left text-xs">
							<Table.Header class="bg-[#141414] border-b border-[#262626] text-[#737373]">
								<Table.Row class="hover:bg-transparent border-[#262626]">
									<Table.Head class="text-xs text-[#737373]">SERVICE / DEPLOYMENT</Table.Head>
									<Table.Head class="text-xs text-[#737373]">BRANCH & COMMIT</Table.Head>
									<Table.Head class="text-xs text-[#737373]">STATUS</Table.Head>
									<Table.Head class="text-xs text-[#737373]">DURATION</Table.Head>
									<Table.Head class="text-xs text-[#737373] text-right">TIME</Table.Head>
								</Table.Row>
							</Table.Header>
							<Table.Body class="divide-y divide-[#262626]">
								{#each filtered as d (d.id)}
									{@const ServiceIcon = getServiceIcon(d.kind)}
									<Table.Row class="border-[#262626] hover:bg-[#262626]/30 transition-colors">
										<Table.Cell>
											<div class="flex items-center gap-3">
												<div class="w-8 h-8 rounded-lg bg-[#262626] border border-white/10 flex items-center justify-center shrink-0">
													<ServiceIcon class="w-4 h-4 text-[#FAFAFA]" />
												</div>
												<div class="flex flex-col">
													<span class="text-xs font-semibold text-[#FAFAFA]">{d.title}</span>
													<span class="text-[10px] font-mono text-[#737373]">ID: #{d.id}</span>
												</div>
											</div>
										</Table.Cell>
										<Table.Cell class="font-mono text-xs text-[#a1a1aa]">
											<div class="flex items-center gap-1.5">
												<GitBranch class="w-3.5 h-3.5 text-[#737373]" />
												<span>{d.branch}</span>
												<span class="text-[#737373]">({d.commitHash})</span>
											</div>
											<p class="text-[10px] text-[#737373] mt-0.5 truncate max-w-xs">{d.commitMsg}</p>
										</Table.Cell>
										<Table.Cell>
											<StatusBadge status={d.state ?? 'idle'} pulse={d.state === 'running' || d.state === 'building'} />
										</Table.Cell>
										<Table.Cell class="text-xs text-[#a1a1aa] font-mono">
											{d.duration}
										</Table.Cell>
										<Table.Cell class="text-right text-xs text-[#737373] font-mono">
											{d.createdAt}
										</Table.Cell>
									</Table.Row>
								{/each}
							</Table.Body>
						</Table.Root>
					{/if}
				</Card.Content>
			</Card.Root>
		</div>
	</main>
</PageLayout>
