<script lang="ts">
	import { goto } from '$app/navigation';
	import {
		Zap,
		FolderOpen,
		Layers,
		Activity,
		ArrowRight,
		Rocket,
		PanelLeft,
		Search,
		Cpu,
		HardDrive,
		MemoryStick,
		Plus,
		CheckCircle2,
		XCircle,
		Clock
	} from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import { sidebarState } from '$lib/sidebar.svelte';
	import { deploymentControllerActive } from '$lib/client/sdk.gen';
	import type { ActiveDeploymentDto } from '$lib/client/types.gen';
	import { Card, CardContent, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { Progress } from '$lib/components/ui/progress';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	const userName = session?.user.first_name || session?.user.email?.split('@')[0] || 'Aditya Sahu';
	const fullName = `${userName} (addy)`;

	let activeDeployments = $state<ActiveDeploymentDto[]>([]);
	let deploymentsLoading = $state(true);
	let activeFilter = $state<'all' | 'running' | 'errored'>('all');

	$effect(() => {
		deploymentControllerActive()
			.then((res: any) => {
				activeDeployments = (res.data as ActiveDeploymentDto[]) ?? [];
				deploymentsLoading = false;
			})
			.catch(() => {
				deploymentsLoading = false;
			});
	});

	// Stats
	const projectsCount = 0;
	const servicesCount = 0;
	const servicesBreakdown = { apps: 0, compose: 0, db: 0 };
	const deploys7d = 0;

	// Status counts
	const statusCounts = $derived({
		running: activeDeployments.filter((d) => d.state === 'running').length,
		errored: activeDeployments.filter((d) => d.state === 'error').length,
		idle: activeDeployments.filter((d) => d.state === 'idle' || d.state === 'stopped').length
	});

	const filteredDeployments = $derived(
		activeDeployments.filter((d) => {
			if (activeFilter === 'running') return d.state === 'running';
			if (activeFilter === 'errored') return d.state === 'error';
			return true;
		})
	);

	function openCommandPalette() {
		window.dispatchEvent(new CustomEvent('open-command-palette'));
	}

	function getStatusColor(state: string): string {
		switch (state) {
			case 'running':
				return 'bg-[#22c55e]';
			case 'done':
				return 'bg-[#22c55e]';
			case 'error':
				return 'bg-[#ef4444]';
			default:
				return 'bg-[#71717a]';
		}
	}

	function relativeTime(timestamp?: number): string {
		if (!timestamp) return 'just now';
		const diff = Date.now() - timestamp;
		const minutes = Math.floor(diff / 60000);
		if (minutes < 1) return 'just now';
		if (minutes < 60) return `${minutes}m ago`;
		const hours = Math.floor(minutes / 60);
		if (hours < 24) return `${hours}h ago`;
		const days = Math.floor(hours / 24);
		return `${days}d ago`;
	}

	// 7-day sparkline sample heights (monotone)
	const sparklineBars = [15, 25, 10, 40, 30, 20, 0];
</script>

<PageLayout>
	<!-- Top Breadcrumb & Live Command Bar -->
	<header class="flex items-center justify-between px-6 py-3 border-b border-[#262626] text-sm bg-[#0A0A0A] shrink-0 gap-4">
		<div class="flex items-center gap-3">
			<button
				class="p-1.5 rounded-lg text-[#737373] hover:text-[#FAFAFA] hover:bg-[#262626]/60 transition-colors cursor-pointer"
				title="Toggle Sidebar"
				onclick={() => sidebarState.toggle()}
			>
				<PanelLeft class="w-4 h-4" />
			</button>
			<span class="text-[#a1a1aa] font-medium text-sm">Home</span>
		</div>

		<!-- Center Quick Command Trigger (⌘K) -->
		<button
			onclick={openCommandPalette}
			class="hidden md:flex items-center gap-2.5 px-3.5 py-1.5 rounded-lg border border-[#262626] bg-[#141414] hover:bg-[#262626]/60 text-xs text-[#a1a1aa] transition-colors cursor-pointer w-64 justify-between"
		>
			<div class="flex items-center gap-2">
				<Search class="w-3.5 h-3.5 text-[#737373]" />
				<span>Search or command...</span>
			</div>
			<kbd class="px-1.5 py-0.5 text-[10px] font-mono rounded bg-[#262626] border border-white/10 text-[#FAFAFA]">⌘K</kbd>
		</button>

		<!-- Right System Pulse Tag & Action Button -->
		<div class="flex items-center gap-3">
			<div class="hidden sm:flex items-center gap-2 px-2.5 py-1 rounded-full border border-green-500/20 bg-green-500/5 text-xs text-green-400 font-medium">
				<span class="relative flex h-2 w-2">
					<span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75"></span>
					<span class="relative inline-flex rounded-full h-2 w-2 bg-green-500"></span>
				</span>
				<span>System Normal</span>
			</div>
			<Button size="sm" class="gap-1.5 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] px-3.5 py-1.5 rounded-lg" onclick={() => goto('/projects')}>
				<Plus class="h-3.5 w-3.5" />
				<span>New Project</span>
			</Button>
		</div>
	</header>

	<!-- Main Workspace Content Card -->
	<div class="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up flex flex-col min-h-0 bg-[#171717] border border-[#262626] rounded-2xl shadow-md space-y-7">
		<!-- Greeting Header -->
		<div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
			<div>
				<h1 class="text-3xl font-bold tracking-tight text-[#FAFAFA]">Welcome back, {fullName}</h1>
				<p class="text-xs text-[#a1a1aa] mt-1">Rustploy Dashboard · System Version v0.29.12</p>
			</div>
			<Button variant="secondary" size="default" class="gap-2 text-sm font-medium bg-[#262626] hover:bg-[#333333] text-[#FAFAFA] border border-[#3f3f46]/60 px-4 py-2 rounded-lg shadow-2xs self-start sm:self-auto" onclick={() => goto('/projects')}>
				Go to projects
				<ArrowRight class="h-4 w-4" />
			</Button>
		</div>

		<!-- Stats Cards (4 Grid with Sparklines & Breakdown Chips) -->
		<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
			<!-- Projects -->
			<Card class="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs hover:border-[#3f3f46] transition-colors">
				<CardHeader class="pb-1 pt-4 px-5">
					<p class="text-xs uppercase tracking-wider text-[#a1a1aa] font-semibold">PROJECTS</p>
				</CardHeader>
				<CardContent class="pb-4 px-5">
					<p class="text-4xl font-bold text-[#FAFAFA] tracking-tight">{projectsCount}</p>
					<div class="flex items-center gap-2 mt-2">
						<span class="px-2 py-0.5 rounded border border-[#262626] bg-[#262626]/40 text-[11px] text-[#a1a1aa]">0 Prod</span>
						<span class="px-2 py-0.5 rounded border border-[#262626] bg-[#262626]/40 text-[11px] text-[#a1a1aa]">0 Staging</span>
					</div>
				</CardContent>
			</Card>

			<!-- Services -->
			<Card class="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs hover:border-[#3f3f46] transition-colors">
				<CardHeader class="pb-1 pt-4 px-5">
					<p class="text-xs uppercase tracking-wider text-[#a1a1aa] font-semibold">SERVICES</p>
				</CardHeader>
				<CardContent class="pb-4 px-5">
					<p class="text-4xl font-bold text-[#FAFAFA] tracking-tight">{servicesCount}</p>
					<div class="flex items-center gap-1.5 mt-2">
						<span class="px-2 py-0.5 rounded border border-[#262626] bg-[#262626]/40 text-[11px] text-[#FAFAFA]">{servicesBreakdown.apps} Apps</span>
						<span class="px-2 py-0.5 rounded border border-[#262626] bg-[#262626]/40 text-[11px] text-[#FAFAFA]">{servicesBreakdown.compose} Compose</span>
						<span class="px-2 py-0.5 rounded border border-[#262626] bg-[#262626]/40 text-[11px] text-[#FAFAFA]">{servicesBreakdown.db} DB</span>
					</div>
				</CardContent>
			</Card>

			<!-- Deploys / 7D -->
			<Card class="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs hover:border-[#3f3f46] transition-colors">
				<CardHeader class="pb-1 pt-4 px-5">
					<p class="text-xs uppercase tracking-wider text-[#a1a1aa] font-semibold">DEPLOYS / 7D</p>
				</CardHeader>
				<CardContent class="pb-4 px-5">
					<p class="text-4xl font-bold text-[#FAFAFA] tracking-tight">{deploys7d}</p>
					<!-- 7-day sparkline bar chart -->
					<div class="flex items-end gap-1.5 h-5 mt-2">
						{#each sparklineBars as barHeight}
							<div
								style="height: {Math.max(4, barHeight)}px"
								class="flex-1 bg-[#3f3f46]/60 rounded-xs hover:bg-[#FAFAFA] transition-colors"
								title="Day deploy activity"
							></div>
						{/each}
					</div>
				</CardContent>
			</Card>

			<!-- Status -->
			<Card class="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs hover:border-[#3f3f46] transition-colors">
				<CardHeader class="pb-1 pt-4 px-5">
					<div class="flex items-center justify-between">
						<p class="text-xs uppercase tracking-wider text-[#a1a1aa] font-semibold">STATUS</p>
						<span class="text-[11px] font-mono text-[#a1a1aa]">100% Ready</span>
					</div>
				</CardHeader>
				<CardContent class="pb-4 px-5">
					{#if deploymentsLoading}
						<Skeleton class="h-5 w-24 mb-1" />
						<Skeleton class="h-4 w-32" />
					{:else}
						<div class="flex flex-col gap-1.5 text-sm mt-1">
							<div class="flex items-center gap-2">
								<span class="w-2 h-2 rounded-full bg-[#22c55e]"></span>
								<span class="font-bold text-[#FAFAFA] text-sm">{statusCounts.running}</span>
								<span class="text-[#a1a1aa] text-sm">running</span>
							</div>
							<div class="flex items-center gap-2">
								<span class="w-2 h-2 rounded-full bg-[#ef4444]"></span>
								<span class="font-bold text-[#FAFAFA] text-sm">{statusCounts.errored}</span>
								<span class="text-[#a1a1aa] text-sm">errored</span>
							</div>
							<div class="flex items-center gap-2">
								<span class="w-2 h-2 rounded-full bg-[#71717a]"></span>
								<span class="font-bold text-[#FAFAFA] text-sm">{statusCounts.idle}</span>
								<span class="text-[#a1a1aa] text-sm">idle</span>
							</div>
						</div>
					{/if}
				</CardContent>
			</Card>
		</div>

		<!-- Server Resources Monitoring Widget -->
		<Card class="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs p-5">
			<div class="flex items-center justify-between mb-4">
				<div class="flex items-center gap-2.5">
					<Cpu class="h-4 w-4 text-[#a1a1aa]" />
					<h2 class="text-sm font-semibold text-[#FAFAFA]">Server Resources & Performance</h2>
				</div>
				<span class="text-xs font-mono text-[#737373]">Host: localhost · Linux 6.6</span>
			</div>
			<div class="grid grid-cols-1 md:grid-cols-3 gap-5">
				<!-- CPU -->
				<div class="space-y-2">
					<div class="flex items-center justify-between text-xs">
						<span class="text-[#a1a1aa] font-medium">CPU Load</span>
						<span class="text-[#FAFAFA] font-mono font-semibold">12%</span>
					</div>
					<Progress value={12} class="h-1.5 bg-[#262626]" />
				</div>
				<!-- Memory -->
				<div class="space-y-2">
					<div class="flex items-center justify-between text-xs">
						<span class="text-[#a1a1aa] font-medium">RAM Usage</span>
						<span class="text-[#FAFAFA] font-mono font-semibold">2.4 GB / 8.0 GB (30%)</span>
					</div>
					<Progress value={30} class="h-1.5 bg-[#262626]" />
				</div>
				<!-- Disk -->
				<div class="space-y-2">
					<div class="flex items-center justify-between text-xs">
						<span class="text-[#a1a1aa] font-medium">Disk Storage</span>
						<span class="text-[#FAFAFA] font-mono font-semibold">18.2 GB / 64 GB (28%)</span>
					</div>
					<Progress value={28} class="h-1.5 bg-[#262626]" />
				</div>
			</div>
		</Card>

		<!-- Recent Deployments & Activity Timeline Card -->
		<Card class="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs overflow-hidden flex-1 flex flex-col">
			<CardHeader class="border-b border-[#262626] py-3.5 px-5 bg-[#171717]">
				<div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
					<div class="flex items-center gap-2.5">
						<Rocket class="h-4 w-4 text-[#a1a1aa] stroke-[1.5]" />
						<CardTitle class="text-sm font-semibold text-[#FAFAFA]">Recent deployments & activity</CardTitle>
					</div>

					<!-- Filter Tabs -->
					<div class="flex items-center gap-1 bg-[#141414] p-0.5 rounded-lg border border-[#262626]">
						<button
							class="px-2.5 py-1 text-xs font-medium rounded-md transition-colors {activeFilter === 'all'
								? 'bg-[#262626] text-[#FAFAFA]'
								: 'text-[#737373] hover:text-[#FAFAFA]'}"
							onclick={() => (activeFilter = 'all')}
						>
							All
						</button>
						<button
							class="px-2.5 py-1 text-xs font-medium rounded-md transition-colors {activeFilter === 'running'
								? 'bg-[#262626] text-[#FAFAFA]'
								: 'text-[#737373] hover:text-[#FAFAFA]'}"
							onclick={() => (activeFilter = 'running')}
						>
							Running
						</button>
						<button
							class="px-2.5 py-1 text-xs font-medium rounded-md transition-colors {activeFilter === 'errored'
								? 'bg-[#262626] text-[#FAFAFA]'
								: 'text-[#737373] hover:text-[#FAFAFA]'}"
							onclick={() => (activeFilter = 'errored')}
						>
							Errored
						</button>
					</div>
				</div>
			</CardHeader>
			<CardContent class="p-0 flex-1 flex flex-col justify-center bg-[#171717]">
				{#if deploymentsLoading}
					<div class="divide-y divide-[#262626] p-6 space-y-4">
						{#each Array(3) as _}
							<div class="flex items-center gap-4">
								<Skeleton class="h-3 w-3 rounded-full" />
								<div class="flex-1 space-y-2">
									<Skeleton class="h-5 w-44" />
									<Skeleton class="h-4 w-28" />
								</div>
								<Skeleton class="h-4 w-20" />
							</div>
						{/each}
					</div>
				{:else if filteredDeployments.length === 0}
					<div class="flex flex-col items-center justify-center py-20 text-[#a1a1aa] text-center min-h-[300px]">
						<Rocket class="h-8 w-8 mb-2.5 text-[#525252] stroke-[1.25]" />
						<p class="text-sm text-[#FAFAFA] font-semibold">No activity recorded yet</p>
						<p class="text-xs text-[#a1a1aa] mt-1 max-w-sm">Create a project or trigger a deployment to monitor real-time build logs and container states.</p>
						<Button size="sm" class="mt-4 gap-1.5 text-xs font-medium bg-[#262626] hover:bg-[#333333] text-[#FAFAFA] border border-[#3f3f46]/60 px-3.5 py-1.5 rounded-lg" onclick={() => goto('/projects')}>
							<Plus class="h-3.5 w-3.5" />
							Create Project
						</Button>
					</div>
				{:else}
					<div class="divide-y divide-[#262626]">
						{#each filteredDeployments as deployment (deployment.id)}
							<div class="flex items-center justify-between px-6 py-3.5 hover:bg-[#262626]/30 transition-colors">
								<div class="flex items-center gap-3.5">
									<span
										class="h-2.5 w-2.5 rounded-full shrink-0 {getStatusColor(deployment.state)}"
									></span>
									<div>
										<p class="text-sm font-semibold text-[#FAFAFA]">
											{deployment.kind} service
											<span class="text-[#737373] font-normal">#{deployment.id}</span>
										</p>
										<p class="text-xs text-[#a1a1aa] mt-0.5">
											Project {deployment.project_id} · Environment {deployment.environment_id}
										</p>
									</div>
								</div>
								<div class="flex items-center gap-3.5">
									<Badge
										variant="outline"
										class="text-xs capitalize {deployment.state === 'running'
											? 'border-green-500/30 text-green-400 bg-green-500/10'
											: deployment.state === 'error'
												? 'border-red-500/30 text-red-400 bg-red-500/10'
												: 'border-[#262626] text-[#a1a1aa]'}"
									>
										{deployment.state}
									</Badge>
									<span class="text-xs text-[#737373] font-mono">{relativeTime()}</span>
								</div>
							</div>
						{/each}
					</div>
				{/if}
			</CardContent>
		</Card>
	</div>
</PageLayout>
