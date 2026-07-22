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
		Filter
	} from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
	import { getAuthSession } from '$lib/auth';
	import { deploymentControllerActive } from '$lib/client/sdk.gen';
	import type { ActiveDeploymentDto } from '$lib/client/types.gen';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import * as Tabs from '$lib/components/ui/tabs';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Badge } from '$lib/components/ui/badge';
	import { Skeleton } from '$lib/components/ui/skeleton';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	let deployments = $state<ActiveDeploymentDto[]>([]);
	let loading = $state(true);
	let searchQuery = $state('');
	let filterState = $state<string>('all');
	let activeTab = $state('deployments');

	$effect(() => {
		deploymentControllerActive()
			.then((res: any) => {
				deployments = (res.data as ActiveDeploymentDto[]) ?? [];
			})
			.catch(() => {})
			.finally(() => {
				loading = false;
			});
	});

	const filtered = $derived(
		deployments.filter((d) => {
			const matchSearch =
				!searchQuery ||
				`${d.id}`.includes(searchQuery) ||
				d.kind?.toLowerCase().includes(searchQuery.toLowerCase());
			const matchState = filterState === 'all' || d.state === filterState;
			return matchSearch && matchState;
		})
	);

	const stats = $derived({
		total: deployments.length,
		running: deployments.filter((d) => d.state === 'running').length,
		success: deployments.filter((d) => d.state === 'done').length,
		failed: deployments.filter((d) => d.state === 'error').length
	});

	const stateFilters = ['all', 'running', 'done', 'error'];

	// Static empty queue data for now
	const queuedJobs: { id: string; type: string; state: string; createdAt: string; updatedAt: string }[] = [];

	function getServiceIcon(kind: string | undefined) {
		if (kind === 'application') return Rocket;
		if (kind === 'compose') return Boxes;
		return Rocket;
	}
</script>

<PageLayout>
	<header class="flex items-center justify-between px-6 py-3 border-b border-border/60">
		<div class="flex items-center gap-2 text-xs">
			<Zap class="w-4 h-4 text-primary" />
			<span class="font-semibold text-foreground">Deployments</span>
		</div>
	</header>

	<main class="flex-1 p-6 space-y-6 animate-fade-up">
		<!-- Stats Grid (Dokploy-style stats cards) -->
		<div class="grid grid-cols-2 md:grid-cols-4 gap-3">
			<Card.Root class="bg-card border-border/80">
				<Card.Content class="p-4">
					<div class="flex items-center justify-between">
						<div>
							<p class="text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">Total</p>
							<p class="text-xl font-bold mt-1 text-foreground">{stats.total}</p>
						</div>
						<div class="w-8 h-8 rounded-lg bg-muted/80 flex items-center justify-center border border-border/40">
							<Zap class="w-4 h-4 text-muted-foreground" />
						</div>
					</div>
				</Card.Content>
			</Card.Root>

			<Card.Root class="bg-card border-border/80">
				<Card.Content class="p-4">
					<div class="flex items-center justify-between">
						<div>
							<p class="text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">Running</p>
							<p class="text-xl font-bold mt-1 text-yellow-400">{stats.running}</p>
						</div>
						<div class="w-8 h-8 rounded-lg bg-yellow-500/10 flex items-center justify-center border border-yellow-500/20">
							<Loader2 class="w-4 h-4 text-yellow-400 animate-spin" />
						</div>
					</div>
				</Card.Content>
			</Card.Root>

			<Card.Root class="bg-card border-border/80">
				<Card.Content class="p-4">
					<div class="flex items-center justify-between">
						<div>
							<p class="text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">Succeeded</p>
							<p class="text-xl font-bold mt-1 text-green-400">{stats.success}</p>
						</div>
						<div class="w-8 h-8 rounded-lg bg-green-500/10 flex items-center justify-center border border-green-500/20">
							<CircleCheck class="w-4 h-4 text-green-400" />
						</div>
					</div>
				</Card.Content>
			</Card.Root>

			<Card.Root class="bg-card border-border/80">
				<Card.Content class="p-4">
					<div class="flex items-center justify-between">
						<div>
							<p class="text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">Failed</p>
							<p class="text-xl font-bold mt-1 text-red-400">{stats.failed}</p>
						</div>
						<div class="w-8 h-8 rounded-lg bg-red-500/10 flex items-center justify-center border border-red-500/20">
							<CircleX class="w-4 h-4 text-red-400" />
						</div>
					</div>
				</Card.Content>
			</Card.Root>
		</div>

		<!-- Tabs -->
		<Tabs.Root value={activeTab} onValueChange={(v) => (activeTab = v)}>
			<Tabs.List class="bg-card border border-border/60 p-1 rounded-lg">
				<Tabs.Trigger value="deployments" class="text-xs font-medium px-3 py-1.5 rounded-md">
					<Rocket class="w-3.5 h-3.5 mr-1.5" />
					Deployments
				</Tabs.Trigger>
				<Tabs.Trigger value="queue" class="text-xs font-medium px-3 py-1.5 rounded-md">
					<ListTodo class="w-3.5 h-3.5 mr-1.5" />
					Queue
				</Tabs.Trigger>
			</Tabs.List>

			<!-- Deployments Tab -->
			<Tabs.Content value="deployments" class="mt-4 space-y-4">
				<!-- Show search/filter toolbar ONLY if deployments exist -->
				{#if deployments.length > 0}
					<div class="flex items-center gap-3 flex-wrap">
						<div class="relative flex-1 max-w-xs">
							<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-muted-foreground/60" />
							<Input
								bind:value={searchQuery}
								placeholder="Search by name, project, environment..."
								class="pl-8 text-xs h-9"
							/>
						</div>
						<div class="flex items-center gap-1.5">
							{#each stateFilters as sf}
								<Button
									variant={filterState === sf ? 'secondary' : 'outline'}
									size="sm"
									class="text-xs capitalize h-8 px-2.5"
									onclick={() => (filterState = sf)}
								>
									{sf === 'all' ? 'All' : sf === 'done' ? 'Done' : sf === 'error' ? 'Error' : 'Running'}
								</Button>
							{/each}
						</div>
					</div>
				{/if}

				<!-- Deployments Table or Clean Empty Container -->
				<Card.Root class="bg-card border-border/80">
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
							<!-- Lightweight Centered Empty State Container (No empty table header noise) -->
							<div class="flex flex-col items-center justify-center py-20 px-4 text-center">
								<div class="w-12 h-12 rounded-xl bg-card border border-border/80 flex items-center justify-center mb-3">
									<Rocket class="w-6 h-6 text-muted-foreground/60" />
								</div>
								<h4 class="text-sm font-semibold text-foreground">No deployments found</h4>
								<p class="text-xs text-muted-foreground mt-1 max-w-xs leading-relaxed">
									{#if searchQuery || filterState !== 'all'}
										No deployments match your current filter criteria.
									{:else}
										Deployments will automatically appear here when services or stacks are triggered.
									{/if}
								</p>
							</div>
						{:else}
							<Table.Root>
								<Table.Header>
									<Table.Row class="border-border/60 hover:bg-transparent">
										<Table.Head class="text-xs text-muted-foreground/80">Service</Table.Head>
										<Table.Head class="text-xs text-muted-foreground/80">Status</Table.Head>
										<Table.Head class="text-xs text-muted-foreground/80 text-right">Started</Table.Head>
									</Table.Row>
								</Table.Header>
								<Table.Body>
									{#each filtered as d (d.id)}
										{@const ServiceIcon = getServiceIcon(d.kind)}
										<Table.Row class="border-border/40 cursor-pointer hover:bg-accent/40 transition-colors">
											<Table.Cell>
												<div class="flex items-center gap-3">
													<div class="w-7 h-7 rounded-md bg-muted/80 flex items-center justify-center border border-border/40">
														<ServiceIcon class="w-3.5 h-3.5 text-muted-foreground" />
													</div>
													<div class="flex flex-col">
														<span class="text-xs font-semibold text-foreground">Deployment #{d.id}</span>
														<Badge variant="outline" class="w-fit text-[10px] capitalize mt-0.5 border-border/60">{d.kind ?? 'service'}</Badge>
													</div>
												</div>
											</Table.Cell>
											<Table.Cell>
												<StatusBadge status={d.state ?? 'pending'} pulse={d.state === 'running'} />
											</Table.Cell>
											<Table.Cell class="text-right text-xs text-muted-foreground font-mono">
												—
											</Table.Cell>
										</Table.Row>
									{/each}
								</Table.Body>
							</Table.Root>
						{/if}
					</Card.Content>
				</Card.Root>
			</Tabs.Content>

			<!-- Queue Tab -->
			<Tabs.Content value="queue" class="mt-4 space-y-4">
				<Card.Root class="bg-card border-border/80">
					<Card.Content class="p-0">
						{#if queuedJobs.length === 0}
							<div class="flex flex-col items-center justify-center py-20 text-muted-foreground text-center">
								<div class="w-12 h-12 rounded-xl bg-card border border-border/80 flex items-center justify-center mb-3">
									<ListTodo class="w-6 h-6 text-muted-foreground/60" />
								</div>
								<h4 class="text-sm font-semibold text-foreground">No queued jobs</h4>
								<p class="text-xs text-muted-foreground mt-1 max-w-xs leading-relaxed">
									In-flight and scheduled jobs will appear here as build pipelines execute.
								</p>
							</div>
						{:else}
							<Table.Root>
								<Table.Header>
									<Table.Row class="border-border/60">
										<Table.Head class="w-20 text-xs">ID</Table.Head>
										<Table.Head class="text-xs">Type</Table.Head>
										<Table.Head class="text-xs">State</Table.Head>
										<Table.Head class="text-xs">Created</Table.Head>
										<Table.Head class="text-right text-xs">Updated</Table.Head>
									</Table.Row>
								</Table.Header>
								<Table.Body>
									{#each queuedJobs as job (job.id)}
										<Table.Row class="border-border/40">
											<Table.Cell class="font-mono text-xs">#{job.id}</Table.Cell>
											<Table.Cell class="text-xs capitalize">{job.type}</Table.Cell>
											<Table.Cell>
												<Badge variant="secondary" class="text-[10px] capitalize">{job.state}</Badge>
											</Table.Cell>
											<Table.Cell class="text-xs text-muted-foreground">{job.createdAt}</Table.Cell>
											<Table.Cell class="text-right text-xs text-muted-foreground">{job.updatedAt}</Table.Cell>
										</Table.Row>
									{/each}
								</Table.Body>
							</Table.Root>
						{/if}
					</Card.Content>
				</Card.Root>
			</Tabs.Content>
		</Tabs.Root>
	</main>
</PageLayout>
