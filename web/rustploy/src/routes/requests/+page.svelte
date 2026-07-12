<script lang="ts">
	import { goto } from '$app/navigation';
	import { Link, BarChart3, Filter } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	const requests = [
		{ id: 1, method: 'GET', path: '/api/v1/projects', status: 200, duration: '45ms', timestamp: '21:00:01', ip: '10.0.0.1' },
		{ id: 2, method: 'POST', path: '/api/v1/deployments', status: 201, duration: '120ms', timestamp: '20:59:45', ip: '10.0.0.2' },
		{ id: 3, method: 'GET', path: '/api/v1/monitoring/stats', status: 200, duration: '32ms', timestamp: '20:59:30', ip: '10.0.0.1' },
		{ id: 4, method: 'DELETE', path: '/api/v1/applications/5', status: 404, duration: '12ms', timestamp: '20:58:10', ip: '10.0.0.3' },
		{ id: 5, method: 'GET', path: '/api/v1/health', status: 200, duration: '5ms', timestamp: '20:57:00', ip: '10.0.0.1' },
		{ id: 6, method: 'POST', path: '/api/v1/auth/login', status: 401, duration: '89ms', timestamp: '20:55:30', ip: '10.0.0.4' }
	];

	function statusColor(s: number): 'default' | 'destructive' | 'secondary' | 'outline' {
		if (s >= 200 && s < 300) return 'default';
		if (s >= 400 && s < 500) return 'secondary';
		if (s >= 500) return 'destructive';
		return 'outline';
	}

	function methodColor(m: string): string {
		const map: Record<string, string> = { GET: 'text-green-400', POST: 'text-blue-400', PUT: 'text-yellow-400', DELETE: 'text-red-400', PATCH: 'text-purple-400' };
		return map[m] ?? 'text-foreground';
	}
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<Link class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Requests</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto animate-fade-up">
		<div class="w-full">
			<Card.Root>
				<Card.Header>
					<Card.Title class="text-base flex items-center gap-2">
						<BarChart3 class="w-4 h-4 text-muted-foreground" />
						Access Logs
					</Card.Title>
					<Card.Description>Recent HTTP requests to your services</Card.Description>
				</Card.Header>
				<Card.Content class="p-0">
					<Table.Root>
						<Table.Header>
							<Table.Row>
								<Table.Head class="w-20">Method</Table.Head>
								<Table.Head>Path</Table.Head>
								<Table.Head class="w-20">Status</Table.Head>
								<Table.Head class="w-20">Duration</Table.Head>
								<Table.Head class="w-28">Time</Table.Head>
								<Table.Head class="w-28">IP</Table.Head>
							</Table.Row>
						</Table.Header>
						<Table.Body>
							{#each requests as req (req.id)}
								<Table.Row>
									<Table.Cell><span class="text-xs font-mono font-bold {methodColor(req.method)}">{req.method}</span></Table.Cell>
									<Table.Cell class="text-xs font-mono text-muted-foreground">{req.path}</Table.Cell>
									<Table.Cell><Badge variant={statusColor(req.status)} class="text-[10px] font-mono">{req.status}</Badge></Table.Cell>
									<Table.Cell class="text-xs text-muted-foreground">{req.duration}</Table.Cell>
									<Table.Cell class="text-xs text-muted-foreground">{req.timestamp}</Table.Cell>
									<Table.Cell class="text-xs font-mono text-muted-foreground">{req.ip}</Table.Cell>
								</Table.Row>
							{/each}
						</Table.Body>
					</Table.Root>
				</Card.Content>
			</Card.Root>
		</div>
	</main>
</PageLayout>
