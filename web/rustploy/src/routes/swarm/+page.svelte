<script lang="ts">
	import { goto } from '$app/navigation';
	import { Globe2, Server, Cpu, MemoryStick } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Progress } from '$lib/components/ui/progress';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	const nodes = [
		{ id: 'node1', hostname: 'manager-1', role: 'manager', status: 'ready', availability: 'active', cpu: 35, memory: 62, containers: 8 },
		{ id: 'node2', hostname: 'worker-1', role: 'worker', status: 'ready', availability: 'active', cpu: 55, memory: 78, containers: 12 },
		{ id: 'node3', hostname: 'worker-2', role: 'worker', status: 'ready', availability: 'active', cpu: 20, memory: 45, containers: 5 }
	];
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<Globe2 class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Swarm</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto animate-fade-up">
		<div class="w-full space-y-6">
			<!-- Overview -->
			<div class="grid grid-cols-3 gap-4">
				<Card.Root>
					<Card.Content class="p-4 flex items-center gap-3">
						<div class="w-10 h-10 rounded-lg bg-primary/10 flex items-center justify-center">
							<Server class="w-5 h-5 text-primary" />
						</div>
						<div>
							<p class="text-xs text-muted-foreground">Nodes</p>
							<p class="text-xl font-bold">{nodes.length}</p>
						</div>
					</Card.Content>
				</Card.Root>
				<Card.Root>
					<Card.Content class="p-4 flex items-center gap-3">
						<div class="w-10 h-10 rounded-lg bg-green-500/10 flex items-center justify-center">
							<Globe2 class="w-5 h-5 text-green-400" />
						</div>
						<div>
							<p class="text-xs text-muted-foreground">Services</p>
							<p class="text-xl font-bold">{nodes.reduce((a, n) => a + n.containers, 0)}</p>
						</div>
					</Card.Content>
				</Card.Root>
				<Card.Root>
					<Card.Content class="p-4 flex items-center gap-3">
						<div class="w-10 h-10 rounded-lg bg-yellow-500/10 flex items-center justify-center">
							<Cpu class="w-5 h-5 text-yellow-400" />
						</div>
						<div>
							<p class="text-xs text-muted-foreground">Avg CPU</p>
							<p class="text-xl font-bold">{Math.round(nodes.reduce((a, n) => a + n.cpu, 0) / nodes.length)}%</p>
						</div>
					</Card.Content>
				</Card.Root>
			</div>

			<!-- Nodes -->
			<Card.Root>
				<Card.Header>
					<Card.Title class="text-base">Cluster Nodes</Card.Title>
					<Card.Description>Docker Swarm nodes in your cluster</Card.Description>
				</Card.Header>
				<Card.Content>
					<div class="flex flex-col gap-3">
						{#each nodes as node (node.id)}
							<div class="p-4 rounded-lg border">
								<div class="flex items-center justify-between mb-3">
									<div class="flex items-center gap-2">
										<Server class="w-4 h-4 text-muted-foreground" />
										<span class="text-sm font-medium">{node.hostname}</span>
										<Badge variant={node.role === 'manager' ? 'default' : 'outline'} class="text-[10px] capitalize">{node.role}</Badge>
										<Badge variant="default" class="text-[10px]">{node.status}</Badge>
									</div>
									<span class="text-xs text-muted-foreground">{node.containers} containers</span>
								</div>
								<div class="grid grid-cols-2 gap-4">
									<div class="space-y-1">
										<div class="flex justify-between text-[11px]"><span class="text-muted-foreground">CPU</span><span>{node.cpu}%</span></div>
										<Progress value={node.cpu} class="h-1.5" />
									</div>
									<div class="space-y-1">
										<div class="flex justify-between text-[11px]"><span class="text-muted-foreground">Memory</span><span>{node.memory}%</span></div>
										<Progress value={node.memory} class="h-1.5" />
									</div>
								</div>
							</div>
						{/each}
					</div>
				</Card.Content>
			</Card.Root>
		</div>
	</main>
</PageLayout>
