<script lang="ts">
	import { goto } from '$app/navigation';
	import { Zap, FolderOpen, Layers, Activity, ArrowRight, Rocket, PanelLeft } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import { sidebarState } from '$lib/sidebar.svelte';
	import { deploymentControllerActive } from '$lib/client/sdk.gen';
	import type { ActiveDeploymentDto } from '$lib/client/types.gen';
	import { Card, CardContent, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Skeleton } from '$lib/components/ui/skeleton';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	const userName = session?.user.first_name || session?.user.email?.split('@')[0] || 'Aditya Sahu';
	const fullName = `${userName} (addy)`;

	let activeDeployments = $state<ActiveDeploymentDto[]>([]);
	let deploymentsLoading = $state(true);

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

	const recentDeployments = $derived(activeDeployments.slice(0, 10));

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
</script>

<PageLayout>
	<!-- Top Breadcrumb Bar (14px text & 16px icon) -->
	<header class="flex items-center gap-3 px-6 py-3 border-b border-[#262626] text-sm bg-[#0A0A0A] shrink-0">
		<button
			class="p-1 rounded-md text-[#737373] hover:text-[#FAFAFA] hover:bg-[#262626]/60 transition-colors cursor-pointer"
			title="Toggle Sidebar"
			onclick={() => sidebarState.toggle()}
		>
			<PanelLeft class="w-4 h-4" />
		</button>
		<span class="text-[#a1a1aa] font-medium text-sm">Home</span>
	</header>

	<!-- Main Workspace Content Card -->
	<div class="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up flex flex-col min-h-0 bg-[#171717] border border-[#262626] rounded-2xl shadow-md">
		<!-- 30px Heading & Primary Action Button -->
		<div class="flex items-center justify-between mb-7">
			<div>
				<h1 class="text-3xl font-bold tracking-tight text-[#FAFAFA]">Welcome back, {fullName}</h1>
			</div>
			<Button variant="secondary" size="default" class="gap-2 text-sm font-medium bg-[#262626] hover:bg-[#333333] text-[#FAFAFA] border border-[#3f3f46]/60 px-4 py-2 rounded-lg shadow-2xs" onclick={() => goto('/projects')}>
				Go to projects
				<ArrowRight class="h-4 w-4" />
			</Button>
		</div>

		<!-- Stats Cards (36px Numbers & 12px/14px Text) -->
		<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-7">
			<!-- Projects -->
			<Card class="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs">
				<CardHeader class="pb-1 pt-4 px-5">
					<p class="text-xs uppercase tracking-wider text-[#a1a1aa] font-semibold">PROJECTS</p>
				</CardHeader>
				<CardContent class="pb-4 px-5">
					<p class="text-4xl font-bold text-[#FAFAFA] tracking-tight">{projectsCount}</p>
					<p class="text-sm text-[#a1a1aa] mt-1.5 font-normal">0 environments</p>
				</CardContent>
			</Card>

			<!-- Services -->
			<Card class="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs">
				<CardHeader class="pb-1 pt-4 px-5">
					<p class="text-xs uppercase tracking-wider text-[#a1a1aa] font-semibold">SERVICES</p>
				</CardHeader>
				<CardContent class="pb-4 px-5">
					<p class="text-4xl font-bold text-[#FAFAFA] tracking-tight">{servicesCount}</p>
					<p class="text-sm text-[#a1a1aa] mt-1.5 font-normal">
						{servicesBreakdown.apps} apps · {servicesBreakdown.compose} compose · {servicesBreakdown.db} db
					</p>
				</CardContent>
			</Card>

			<!-- Deploys / 7D -->
			<Card class="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs">
				<CardHeader class="pb-1 pt-4 px-5">
					<p class="text-xs uppercase tracking-wider text-[#a1a1aa] font-semibold">DEPLOYS / 7D</p>
				</CardHeader>
				<CardContent class="pb-4 px-5">
					<p class="text-4xl font-bold text-[#FAFAFA] tracking-tight">{deploys7d}</p>
					<p class="text-sm text-[#a1a1aa] mt-1.5 font-normal">no activity yet</p>
				</CardContent>
			</Card>

			<!-- Status -->
			<Card class="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs">
				<CardHeader class="pb-1 pt-4 px-5">
					<p class="text-xs uppercase tracking-wider text-[#a1a1aa] font-semibold">STATUS</p>
				</CardHeader>
				<CardContent class="pb-4 px-5">
					{#if deploymentsLoading}
						<Skeleton class="h-5 w-24 mb-1" />
						<Skeleton class="h-4 w-32" />
					{:else}
						<div class="flex flex-col gap-1.5 text-sm">
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

		<!-- Recent Deployments Inner Card (14px Header & Icons) -->
		<Card class="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs overflow-hidden flex-1 flex flex-col">
			<CardHeader class="border-b border-[#262626] py-3.5 px-5 bg-[#171717]">
				<div class="flex items-center justify-between">
					<div class="flex items-center gap-2.5">
						<Rocket class="h-4 w-4 text-[#a1a1aa] stroke-[1.5]" />
						<CardTitle class="text-sm font-semibold text-[#FAFAFA]">Recent deployments</CardTitle>
					</div>
					<Button variant="ghost" size="sm" class="text-xs text-[#737373] hover:text-[#FAFAFA] h-7 px-2" onclick={() => goto('/deployments')}>
						view all ...
					</Button>
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
				{:else if recentDeployments.length === 0}
					<div class="flex flex-col items-center justify-center py-20 text-[#a1a1aa] text-center min-h-[350px]">
						<Rocket class="h-8 w-8 mb-2.5 text-[#525252] stroke-[1.25]" />
						<p class="text-sm text-[#a1a1aa] font-medium">No deployments yet.</p>
					</div>
				{:else}
					<div class="divide-y divide-[#262626]">
						{#each recentDeployments as deployment (deployment.id)}
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
