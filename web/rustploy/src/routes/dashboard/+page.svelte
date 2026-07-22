<script lang="ts">
	import { goto } from '$app/navigation';
	import { Zap, FolderOpen, Layers, Activity, ArrowRight, Rocket, Clock } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import { deploymentControllerActive } from '$lib/client/sdk.gen';
	import type { ActiveDeploymentDto } from '$lib/client/types.gen';
	import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Skeleton } from '$lib/components/ui/skeleton';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	const firstName = session?.user.first_name || session?.user.email?.split('@')[0] || 'User';

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

	// Static stats (to be replaced with real API calls later)
	const projectsCount = 3;
	const servicesCount = 12;
	const servicesBreakdown = { apps: 5, compose: 4, db: 3 };
	const deploys7d = 24;
	const deploys7dChange = '+15%';

	// Derived status counts from active deployments
	const statusCounts = $derived({
		running: activeDeployments.filter((d) => d.state === 'running').length,
		errored: activeDeployments.filter((d) => d.state === 'error').length,
		idle: activeDeployments.filter((d) => d.state === 'idle' || d.state === 'stopped').length
	});

	// Recent deployments (last 10)
	const recentDeployments = $derived(activeDeployments.slice(0, 10));

	function getStatusColor(state: string): string {
		switch (state) {
			case 'running':
				return 'bg-yellow-500';
			case 'done':
				return 'bg-green-500';
			case 'error':
				return 'bg-red-500';
			default:
				return 'bg-muted-foreground/50';
		}
	}

	function getStatusPulse(state: string): string {
		return state === 'running' ? 'animate-pulse' : '';
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
	<main class="flex-1 overflow-y-auto p-6 animate-fade-up">
		<!-- Header -->
		<div class="flex items-center justify-between mb-8">
			<div>
				<h1 class="text-2xl font-bold tracking-tight">Welcome back, {firstName}</h1>
				<p class="text-sm text-muted-foreground mt-1">Here's what's happening with your deployments</p>
			</div>
			<Button variant="default" size="sm" onclick={() => goto('/projects')}>
				Go to projects
				<ArrowRight class="ml-2 h-4 w-4" />
			</Button>
		</div>

		<!-- Stats Grid -->
		<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
			<!-- Projects -->
			<Card class="bg-card border-border">
				<CardHeader class="pb-2">
					<div class="flex items-center justify-between">
						<p class="text-[10px] uppercase tracking-wide text-muted-foreground font-medium">Projects</p>
						<FolderOpen class="h-4 w-4 text-muted-foreground" />
					</div>
				</CardHeader>
				<CardContent>
					<p class="text-2xl font-bold">{projectsCount}</p>
					<p class="text-sm text-muted-foreground mt-1">Active projects</p>
				</CardContent>
			</Card>

			<!-- Services -->
			<Card class="bg-card border-border">
				<CardHeader class="pb-2">
					<div class="flex items-center justify-between">
						<p class="text-[10px] uppercase tracking-wide text-muted-foreground font-medium">Services</p>
						<Layers class="h-4 w-4 text-muted-foreground" />
					</div>
				</CardHeader>
				<CardContent>
					<p class="text-2xl font-bold">{servicesCount}</p>
					<p class="text-sm text-muted-foreground mt-1">
						{servicesBreakdown.apps} apps · {servicesBreakdown.compose} compose · {servicesBreakdown.db} db
					</p>
				</CardContent>
			</Card>

			<!-- Deploys in 7 days -->
			<Card class="bg-card border-border">
				<CardHeader class="pb-2">
					<div class="flex items-center justify-between">
						<p class="text-[10px] uppercase tracking-wide text-muted-foreground font-medium">Deploys (7d)</p>
						<Rocket class="h-4 w-4 text-muted-foreground" />
					</div>
				</CardHeader>
				<CardContent>
					<div class="flex items-baseline gap-2">
						<p class="text-2xl font-bold">{deploys7d}</p>
						<Badge variant="secondary" class="text-[10px] font-medium text-green-500">{deploys7dChange}</Badge>
					</div>
					<p class="text-sm text-muted-foreground mt-1">Deployment activity</p>
				</CardContent>
			</Card>

			<!-- Status -->
			<Card class="bg-card border-border">
				<CardHeader class="pb-2">
					<div class="flex items-center justify-between">
						<p class="text-[10px] uppercase tracking-wide text-muted-foreground font-medium">Status</p>
						<Activity class="h-4 w-4 text-muted-foreground" />
					</div>
				</CardHeader>
				<CardContent>
					{#if deploymentsLoading}
						<Skeleton class="h-5 w-20 mb-1" />
						<Skeleton class="h-4 w-32" />
					{:else}
						<div class="flex flex-col gap-1.5">
							<div class="flex items-center gap-2">
								<span class="h-2 w-2 rounded-full bg-green-500"></span>
								<span class="text-sm font-medium">{statusCounts.running}</span>
								<span class="text-xs text-muted-foreground">running</span>
							</div>
							<div class="flex items-center gap-2">
								<span class="h-2 w-2 rounded-full bg-red-500"></span>
								<span class="text-sm font-medium">{statusCounts.errored}</span>
								<span class="text-xs text-muted-foreground">errored</span>
							</div>
							<div class="flex items-center gap-2">
								<span class="h-2 w-2 rounded-full bg-muted-foreground/40"></span>
								<span class="text-sm font-medium">{statusCounts.idle}</span>
								<span class="text-xs text-muted-foreground">idle</span>
							</div>
						</div>
					{/if}
				</CardContent>
			</Card>
		</div>

		<!-- Recent Deployments -->
		<Card class="bg-card border-border">
			<CardHeader>
				<div class="flex items-center justify-between">
					<div class="flex items-center gap-2">
						<Clock class="h-4 w-4 text-muted-foreground" />
						<CardTitle class="text-base">Recent Deployments</CardTitle>
					</div>
					<Button variant="ghost" size="sm" class="text-xs text-muted-foreground" onclick={() => goto('/deployments')}>
						View all
						<ArrowRight class="ml-1 h-3 w-3" />
					</Button>
				</div>
				<CardDescription>Last 10 deployment activities</CardDescription>
			</CardHeader>
			<CardContent class="p-0">
				{#if deploymentsLoading}
					<div class="divide-y divide-border">
						{#each Array(5) as _}
							<div class="flex items-center gap-4 px-6 py-4">
								<Skeleton class="h-2.5 w-2.5 rounded-full" />
								<div class="flex-1 space-y-2">
									<Skeleton class="h-4 w-40" />
									<Skeleton class="h-3 w-24" />
								</div>
								<Skeleton class="h-3 w-16" />
							</div>
						{/each}
					</div>
				{:else if recentDeployments.length === 0}
					<div class="flex flex-col items-center justify-center py-12 text-muted-foreground">
						<Zap class="h-8 w-8 mb-3 opacity-40" />
						<p class="text-sm">No recent deployments</p>
						<p class="text-xs mt-1 opacity-60">Deploy a service to see activity here</p>
					</div>
				{:else}
					<div class="divide-y divide-border">
						{#each recentDeployments as deployment (deployment.id)}
							<div class="flex items-center justify-between px-6 py-3.5 hover:bg-muted/5 transition-colors">
								<div class="flex items-center gap-3">
									<span
										class="h-2.5 w-2.5 rounded-full shrink-0 {getStatusColor(deployment.state)} {getStatusPulse(deployment.state)}"
									></span>
									<div>
										<p class="text-sm font-medium">
											{deployment.kind} service
											<span class="text-muted-foreground font-normal">#{deployment.id}</span>
										</p>
										<p class="text-sm text-muted-foreground mt-0.5">
											Project {deployment.project_id} · Environment {deployment.environment_id}
										</p>
									</div>
								</div>
								<div class="flex items-center gap-3">
									<Badge
										variant="outline"
										class="text-[10px] capitalize {deployment.state === 'running'
											? 'border-yellow-500/30 text-yellow-500'
											: deployment.state === 'error'
												? 'border-red-500/30 text-red-500'
												: deployment.state === 'done'
													? 'border-green-500/30 text-green-500'
													: 'border-border text-muted-foreground'}"
									>
										{deployment.state}
									</Badge>
									<span class="text-[11px] text-muted-foreground">{relativeTime()}</span>
								</div>
							</div>
						{/each}
					</div>
				{/if}
			</CardContent>
		</Card>
	</main>
</PageLayout>
