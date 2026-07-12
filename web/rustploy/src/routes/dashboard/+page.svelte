<script lang="ts">
	import { goto } from '$app/navigation';
	import { RocketIcon, Clock, ArrowRight, Zap } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import { deploymentControllerActive } from '$lib/client/sdk.gen';
	import type { ActiveDeploymentDto } from '$lib/client/types.gen';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	const userName = session?.user.first_name || session?.user.email?.split('@')[0] || 'user';
	const now = new Date().toLocaleTimeString('en-GB');

	let activeDeployments = $state<ActiveDeploymentDto[]>([]);
	let deploymentsLoading = $state(true);

	$effect(() => {
		deploymentControllerActive().then((res: any) => {
			activeDeployments = (res.data as ActiveDeploymentDto[]) ?? [];
			deploymentsLoading = false;
		}).catch(() => { deploymentsLoading = false; });
	});

	const stats = [
		{ label: 'PROJECTS', value: '—', sub: 'Your projects' },
		{ label: 'SERVICES', value: '—', sub: 'Apps · Compose · DB' },
		{ label: 'DEPLOYS / 7D', value: '—', sub: 'Deployment activity' }
	];

	const statuses = $derived([
		{ count: activeDeployments.filter(d => d.state === 'running').length, label: 'running', color: 'bg-green-500' },
		{ count: activeDeployments.filter(d => d.state === 'error').length, label: 'errored', color: 'bg-red-500' },
		{ count: activeDeployments.filter(d => d.state === 'idle' || d.state === 'stopped').length, label: 'idle', color: 'bg-muted-foreground/30' }
	]);
</script>

<PageLayout>
	<!-- Top bar -->
	<header class="flex items-center justify-between px-6 py-3 border-b border-border bg-background">
		<div class="flex items-center gap-2 text-sm text-muted-foreground">
			<RocketIcon class="w-4 h-4" />
			<span>Home</span>
		</div>
		<div class="flex items-center gap-1 text-xs text-muted-foreground">
			<Clock class="w-3.5 h-3.5" />
			<span>Server Time: {now} UTC | UTC+00:00</span>
		</div>
	</header>

	<main class="flex-1 p-6">
		<div class="flex items-center justify-between mb-6">
			<h1 class="text-2xl font-semibold">Welcome back, {userName}</h1>
			<button
				class="inline-flex items-center gap-1 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 transition-colors"
				onclick={() => goto('/projects')}
			>
				Go to projects <ArrowRight class="w-3.5 h-3.5" />
			</button>
		</div>

		<!-- Stats -->
		<div class="grid grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
			{#each stats as s}
				<div class="bg-card rounded-lg p-4 border border-border">
					<p class="text-[10px] uppercase tracking-widest text-muted-foreground mb-2">{s.label}</p>
					<p class="text-3xl font-semibold">{s.value}</p>
					<p class="text-xs text-muted-foreground mt-1">{s.sub}</p>
				</div>
			{/each}

			<!-- Status card -->
			<div class="bg-card rounded-lg p-4 border border-border">
				<p class="text-[10px] uppercase tracking-widest text-muted-foreground mb-2">STATUS</p>
				<div class="flex flex-col gap-1.5 mt-1">
					{#each statuses as s}
						<div class="flex items-center gap-2 text-sm">
							<span class="w-2 h-2 rounded-full shrink-0 {s.color}"></span>
							<span class="font-medium">{s.count}</span>
							<span class="text-muted-foreground">{s.label}</span>
						</div>
					{/each}
				</div>
			</div>
		</div>

		<!-- Recent deployments -->
		<div class="bg-card rounded-lg border border-border">
			<div class="flex items-center justify-between px-4 py-3 border-b border-border">
				<div class="flex items-center gap-2 text-sm font-medium">
					<RocketIcon class="w-4 h-4 text-muted-foreground" />
					<span>Active Deployments</span>
				</div>
				<button onclick={() => goto('/projects')} class="text-xs text-muted-foreground hover:text-foreground transition-colors">
					view all →
				</button>
			</div>

			{#if deploymentsLoading}
				<div class="flex justify-center py-8">
					<div class="w-5 h-5 border-2 border-muted-foreground/30 border-t-foreground rounded-full animate-spin"></div>
				</div>
			{:else if activeDeployments.length === 0}
				<div class="flex flex-col items-center justify-center py-10 text-muted-foreground/30">
					<Zap class="w-8 h-8 mb-2" />
					<p class="text-sm">No active deployments</p>
				</div>
			{:else}
				<div class="divide-y divide-border">
					{#each activeDeployments as d (d.id)}
						<div class="flex items-center justify-between px-4 py-3">
							<div class="flex items-center gap-3">
								<span class="w-2 h-2 rounded-full bg-yellow-500 animate-pulse shrink-0"></span>
								<div>
									<p class="text-sm font-medium">Deployment #{d.id}</p>
									<p class="text-xs text-muted-foreground">Project {d.project_id} · Env {d.environment_id}</p>
								</div>
							</div>
							<div class="flex items-center gap-4 text-xs text-muted-foreground">
								<span class="capitalize">{d.kind}</span>
								<span class="px-2 py-0.5 rounded bg-yellow-500/15 text-yellow-500 font-medium">{d.state}</span>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	</main>
</PageLayout>
