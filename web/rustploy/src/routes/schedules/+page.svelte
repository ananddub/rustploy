<script lang="ts">
	import { goto } from '$app/navigation';
	import { Calendar, Plus, Trash2, Play, Pause, Clock } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Switch } from '$lib/components/ui/switch';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	let schedules = $state([
		{ id: 1, name: 'Daily Backup', cron: '0 2 * * *', service: 'postgres-main', type: 'backup', enabled: true, lastRun: '2026-07-12 02:00', nextRun: '2026-07-13 02:00' },
		{ id: 2, name: 'Weekly Cleanup', cron: '0 4 * * 0', service: 'rustploy-app', type: 'command', enabled: true, lastRun: '2026-07-07 04:00', nextRun: '2026-07-14 04:00' },
		{ id: 3, name: 'Health Check', cron: '*/5 * * * *', service: 'api-service', type: 'command', enabled: false, lastRun: '2026-07-12 20:55', nextRun: '—' }
	]);

	function toggleSchedule(id: number) {
		schedules = schedules.map(s => s.id === id ? { ...s, enabled: !s.enabled } : s);
	}
</script>

<PageLayout>
	<header class="flex items-center justify-between px-6 py-3 border-b border-border text-sm">
		<div class="flex items-center gap-2">
			<Calendar class="w-4 h-4 text-muted-foreground" />
			<span class="font-medium">Schedules</span>
		</div>
		<Button size="sm" class="gap-1.5 text-xs"><Plus class="w-3.5 h-3.5" />Create Schedule</Button>
	</header>

	<main class="flex-1 p-6 overflow-y-auto animate-fade-up">
		<div class="w-full">
			<Card.Root>
				<Card.Header>
					<Card.Title class="text-base flex items-center gap-2">
						<Calendar class="w-4 h-4 text-muted-foreground" />
						Scheduled Tasks
					</Card.Title>
					<Card.Description>Automate recurring tasks like backups and commands</Card.Description>
				</Card.Header>
				<Card.Content class="p-0">
					<Table.Root>
						<Table.Header>
							<Table.Row>
								<Table.Head>Name</Table.Head>
								<Table.Head>Cron</Table.Head>
								<Table.Head>Service</Table.Head>
								<Table.Head>Type</Table.Head>
								<Table.Head>Last Run</Table.Head>
								<Table.Head>Next Run</Table.Head>
								<Table.Head>Enabled</Table.Head>
								<Table.Head class="w-12"></Table.Head>
							</Table.Row>
						</Table.Header>
						<Table.Body>
							{#each schedules as s (s.id)}
								<Table.Row>
									<Table.Cell class="text-sm font-medium">{s.name}</Table.Cell>
									<Table.Cell><Badge variant="outline" class="text-[10px] font-mono">{s.cron}</Badge></Table.Cell>
									<Table.Cell class="text-xs text-muted-foreground">{s.service}</Table.Cell>
									<Table.Cell><Badge variant="secondary" class="text-[10px] capitalize">{s.type}</Badge></Table.Cell>
									<Table.Cell class="text-xs text-muted-foreground">{s.lastRun}</Table.Cell>
									<Table.Cell class="text-xs text-muted-foreground">{s.nextRun}</Table.Cell>
									<Table.Cell><Switch checked={s.enabled} onCheckedChange={() => toggleSchedule(s.id)} /></Table.Cell>
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
