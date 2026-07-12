<script lang="ts">
	import { goto } from '$app/navigation';
	import { Server, Plus, Trash2, Settings, Activity, Loader2, Wifi, WifiOff } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Progress } from '$lib/components/ui/progress';
	import { Separator } from '$lib/components/ui/separator';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	// Static server data
	const servers = [
		{ id: 1, name: 'Production Server', ip: '192.168.1.100', status: 'active', cpu: 45, memory: 72, disk: 38, os: 'Ubuntu 22.04', uptime: '32 days', cores: 8, ram: '16GB' },
		{ id: 2, name: 'Staging Server', ip: '192.168.1.101', status: 'active', cpu: 22, memory: 45, disk: 25, os: 'Debian 12', uptime: '15 days', cores: 4, ram: '8GB' },
		{ id: 3, name: 'Dev Server', ip: '192.168.1.102', status: 'idle', cpu: 2, memory: 18, disk: 12, os: 'Ubuntu 24.04', uptime: '5 days', cores: 2, ram: '4GB' }
	];
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<Server class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Servers</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto animate-fade-up">
		<div class="w-full">
			<Card.Root>
				<Card.Header>
					<div class="flex items-center justify-between">
						<div>
							<Card.Title class="text-lg flex items-center gap-2">
								<Server class="w-5 h-5 text-muted-foreground" />
								Servers
							</Card.Title>
							<Card.Description>Manage your connected servers and monitor their health</Card.Description>
						</div>
						<Button size="sm" class="gap-1.5 text-xs">
							<Plus class="w-3.5 h-3.5" />
							Add Server
						</Button>
					</div>
				</Card.Header>
				<Card.Content class="pt-4 border-t">
					<div class="flex flex-col gap-2">
						{#each servers as server, i (server.id)}
							<div class="bg-sidebar p-1 rounded-lg animate-fade-up" style="animation-delay: {i * 40}ms">
								<div class="p-4 rounded-lg bg-background border">
									<!-- Header -->
									<div class="flex items-center justify-between mb-3">
										<div class="flex items-center gap-3">
											<div class="w-8 h-8 rounded-md bg-primary/10 flex items-center justify-center">
												<Server class="w-4 h-4 text-primary" />
											</div>
											<div>
												<div class="flex items-center gap-2">
													<span class="text-sm font-medium">{server.name}</span>
													<StatusBadge status={server.status} />
												</div>
												<span class="text-[11px] text-muted-foreground font-mono">{server.ip}</span>
											</div>
										</div>
										<div class="flex items-center gap-1">
											<Button variant="ghost" size="sm" class="h-7 w-7 p-0">
												<Activity class="w-3.5 h-3.5" />
											</Button>
											<Button variant="ghost" size="sm" class="h-7 w-7 p-0">
												<Settings class="w-3.5 h-3.5" />
											</Button>
											<Button variant="ghost" size="sm" class="h-7 w-7 p-0">
												<Trash2 class="w-3.5 h-3.5 text-destructive" />
											</Button>
										</div>
									</div>

									<!-- Stats row -->
									<div class="grid grid-cols-3 gap-3">
										<div class="space-y-1.5">
											<div class="flex items-center justify-between text-[11px]">
												<span class="text-muted-foreground">CPU</span>
												<span class="font-medium">{server.cpu}%</span>
											</div>
											<Progress value={server.cpu} class="h-1.5" />
										</div>
										<div class="space-y-1.5">
											<div class="flex items-center justify-between text-[11px]">
												<span class="text-muted-foreground">Memory</span>
												<span class="font-medium">{server.memory}%</span>
											</div>
											<Progress value={server.memory} class="h-1.5" />
										</div>
										<div class="space-y-1.5">
											<div class="flex items-center justify-between text-[11px]">
												<span class="text-muted-foreground">Disk</span>
												<span class="font-medium">{server.disk}%</span>
											</div>
											<Progress value={server.disk} class="h-1.5" />
										</div>
									</div>

									<!-- Footer info -->
									<div class="flex items-center gap-4 mt-3 pt-3 border-t border-border/50">
										<div class="flex items-center gap-1 text-[10px] text-muted-foreground">
											<span class="font-medium">{server.os}</span>
										</div>
										<div class="flex items-center gap-1 text-[10px] text-muted-foreground">
											<span>{server.cores} cores</span>
										</div>
										<div class="flex items-center gap-1 text-[10px] text-muted-foreground">
											<span>{server.ram} RAM</span>
										</div>
										<div class="ml-auto text-[10px] text-muted-foreground">
											Uptime: {server.uptime}
										</div>
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
