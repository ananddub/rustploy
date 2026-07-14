<script lang="ts">
	import { goto } from '$app/navigation';
	import { Globe2, Plus, Server, Trash2 } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	const nodes = [
		{ id: 'abc123', hostname: 'manager-1', role: 'manager', status: 'Ready', availability: 'Active', ip: '10.0.0.1' },
		{ id: 'def456', hostname: 'worker-1', role: 'worker', status: 'Ready', availability: 'Active', ip: '10.0.0.2' },
		{ id: 'ghi789', hostname: 'worker-2', role: 'worker', status: 'Ready', availability: 'Active', ip: '10.0.0.3' }
	];
</script>

<PageLayout>
	<header class="flex items-center justify-between px-6 py-3 border-b border-border text-sm">
		<div class="flex items-center gap-2">
			<Globe2 class="w-4 h-4 text-muted-foreground" />
			<span class="font-medium">Cluster</span>
		</div>
		<div class="flex gap-2">
			<Button variant="outline" size="sm" class="gap-1.5 text-xs"><Plus class="w-3.5 h-3.5" />Add Manager</Button>
			<Button size="sm" class="gap-1.5 text-xs"><Plus class="w-3.5 h-3.5" />Add Worker</Button>
		</div>
	</header>

	<main class="flex-1 p-6 overflow-y-auto animate-fade-up">
		<div class="w-full">
			<Card.Root>
				<Card.Header>
					<Card.Title class="text-base flex items-center gap-2">
						<Server class="w-4 h-4 text-muted-foreground" />
						Swarm Nodes
					</Card.Title>
					<Card.Description>Manage your Docker Swarm cluster nodes</Card.Description>
				</Card.Header>
				<Card.Content class="p-0">
					<Table.Root>
						<Table.Header>
							<Table.Row>
								<Table.Head>Hostname</Table.Head>
								<Table.Head>Role</Table.Head>
								<Table.Head>Status</Table.Head>
								<Table.Head>Availability</Table.Head>
								<Table.Head>IP</Table.Head>
								<Table.Head>Node ID</Table.Head>
								<Table.Head class="w-12"></Table.Head>
							</Table.Row>
						</Table.Header>
						<Table.Body>
							{#each nodes as node (node.id)}
								<Table.Row>
									<Table.Cell class="text-sm font-medium">{node.hostname}</Table.Cell>
									<Table.Cell><Badge variant={node.role === 'manager' ? 'default' : 'outline'} class="text-[10px] capitalize">{node.role}</Badge></Table.Cell>
									<Table.Cell><Badge variant="default" class="text-[10px]">{node.status}</Badge></Table.Cell>
									<Table.Cell class="text-xs">{node.availability}</Table.Cell>
									<Table.Cell class="text-xs font-mono text-muted-foreground">{node.ip}</Table.Cell>
									<Table.Cell class="text-[10px] font-mono text-muted-foreground">{node.id}</Table.Cell>
									<Table.Cell>
										<Button variant="ghost" size="sm" class="h-7 w-7 p-0"><Trash2 class="w-3.5 h-3.5 text-destructive" /></Button>
									</Table.Cell>
								</Table.Row>
							{/each}
						</Table.Body>
					</Table.Root>
				</Card.Content>
			</Card.Root>
		</div>
	</main>
</PageLayout>
