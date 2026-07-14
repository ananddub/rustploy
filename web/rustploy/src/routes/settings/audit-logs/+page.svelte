<script lang="ts">
	import { goto } from '$app/navigation';
	import { FileText, Search, Filter } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Input } from '$lib/components/ui/input';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	const logs = [
		{ id: 1, action: 'project.create', user: 'admin@example.com', target: 'Project: my-app', timestamp: '2026-07-12 20:30:00', ip: '192.168.1.1' },
		{ id: 2, action: 'application.deploy', user: 'dev@example.com', target: 'App: frontend', timestamp: '2026-07-12 19:45:00', ip: '192.168.1.2' },
		{ id: 3, action: 'user.login', user: 'admin@example.com', target: '—', timestamp: '2026-07-12 18:00:00', ip: '192.168.1.1' },
		{ id: 4, action: 'database.create', user: 'ops@example.com', target: 'DB: postgres-main', timestamp: '2026-07-12 16:20:00', ip: '192.168.1.3' },
		{ id: 5, action: 'settings.update', user: 'admin@example.com', target: 'Organization settings', timestamp: '2026-07-12 14:10:00', ip: '192.168.1.1' },
		{ id: 6, action: 'compose.deploy', user: 'dev@example.com', target: 'Compose: backend-stack', timestamp: '2026-07-12 12:00:00', ip: '192.168.1.2' }
	];

	let searchQuery = $state('');
	const filtered = $derived(
		logs.filter(l => !searchQuery || l.action.includes(searchQuery) || l.user.includes(searchQuery) || l.target.includes(searchQuery))
	);
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<FileText class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Audit Logs</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto animate-fade-up">
		<div class="w-full">
			<Card.Root>
				<Card.Header>
					<div class="flex items-center justify-between">
						<div>
							<Card.Title class="text-base flex items-center gap-2">
								<FileText class="w-4 h-4 text-muted-foreground" />
								Audit Logs
							</Card.Title>
							<Card.Description>Track all actions performed in your organization</Card.Description>
						</div>
						<div class="relative w-64">
							<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-muted-foreground" />
							<Input bind:value={searchQuery} placeholder="Search logs..." class="pl-9 h-8 text-xs" />
						</div>
					</div>
				</Card.Header>
				<Card.Content class="p-0">
					<Table.Root>
						<Table.Header>
							<Table.Row>
								<Table.Head>Action</Table.Head>
								<Table.Head>User</Table.Head>
								<Table.Head>Target</Table.Head>
								<Table.Head>IP</Table.Head>
								<Table.Head>Timestamp</Table.Head>
							</Table.Row>
						</Table.Header>
						<Table.Body>
							{#each filtered as log (log.id)}
								<Table.Row>
									<Table.Cell><Badge variant="outline" class="text-[10px] font-mono">{log.action}</Badge></Table.Cell>
									<Table.Cell class="text-xs">{log.user}</Table.Cell>
									<Table.Cell class="text-xs text-muted-foreground">{log.target}</Table.Cell>
									<Table.Cell class="text-xs font-mono text-muted-foreground">{log.ip}</Table.Cell>
									<Table.Cell class="text-xs text-muted-foreground">{log.timestamp}</Table.Cell>
								</Table.Row>
							{/each}
						</Table.Body>
					</Table.Root>
				</Card.Content>
			</Card.Root>
		</div>
	</main>
</PageLayout>
