<script lang="ts">
	import { goto } from '$app/navigation';
	import { Cpu, RefreshCw, Play, Square, Trash2, Terminal, FileText } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Separator } from '$lib/components/ui/separator';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Switch } from '$lib/components/ui/switch';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	let traefikDashboard = $state(true);
	let letsEncryptEmail = $state('admin@example.com');
	let httpPort = $state('80');
	let httpsPort = $state('443');
	let autoCleanup = $state(true);
	let cleanupDays = $state('7');
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<Cpu class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Web Server</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto animate-fade-up">
		<div class="w-full space-y-6">
			<!-- Traefik Config -->
			<Card.Root>
				<Card.Header>
					<Card.Title class="text-base">Traefik Configuration</Card.Title>
					<Card.Description>Configure your reverse proxy settings</Card.Description>
				</Card.Header>
				<Card.Content class="space-y-4">
					<div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
						<div class="space-y-1.5">
							<Label class="text-xs">HTTP Port</Label>
							<Input bind:value={httpPort} placeholder="80" />
						</div>
						<div class="space-y-1.5">
							<Label class="text-xs">HTTPS Port</Label>
							<Input bind:value={httpsPort} placeholder="443" />
						</div>
					</div>
					<div class="space-y-1.5">
						<Label class="text-xs">Let's Encrypt Email</Label>
						<Input bind:value={letsEncryptEmail} placeholder="admin@example.com" />
					</div>
					<Separator />
					<div class="flex items-center justify-between">
						<div>
							<p class="text-sm font-medium">Traefik Dashboard</p>
							<p class="text-[11px] text-muted-foreground">Enable the Traefik web dashboard</p>
						</div>
						<Switch bind:checked={traefikDashboard} />
					</div>
				</Card.Content>
			</Card.Root>

			<!-- Server Actions -->
			<Card.Root>
				<Card.Header>
					<Card.Title class="text-base">Server Actions</Card.Title>
					<Card.Description>Manage Traefik and server processes</Card.Description>
				</Card.Header>
				<Card.Content class="space-y-3">
					<div class="flex flex-wrap gap-2">
						<Button variant="outline" size="sm" class="gap-1.5 text-xs"><RefreshCw class="w-3.5 h-3.5" />Reload Traefik</Button>
						<Button variant="outline" size="sm" class="gap-1.5 text-xs"><Terminal class="w-3.5 h-3.5" />Terminal</Button>
						<Button variant="outline" size="sm" class="gap-1.5 text-xs"><FileText class="w-3.5 h-3.5" />View Logs</Button>
					</div>
				</Card.Content>
			</Card.Root>

			<!-- Docker Cleanup -->
			<Card.Root>
				<Card.Header>
					<Card.Title class="text-base">Docker Cleanup</Card.Title>
					<Card.Description>Automatically clean up unused Docker resources</Card.Description>
				</Card.Header>
				<Card.Content class="space-y-4">
					<div class="flex items-center justify-between">
						<div>
							<p class="text-sm font-medium">Auto Cleanup</p>
							<p class="text-[11px] text-muted-foreground">Remove unused images, containers, and volumes</p>
						</div>
						<Switch bind:checked={autoCleanup} />
					</div>
					{#if autoCleanup}
						<div class="space-y-1.5">
							<Label class="text-xs">Cleanup interval (days)</Label>
							<Input bind:value={cleanupDays} type="number" class="w-32" />
						</div>
					{/if}
				</Card.Content>
			</Card.Root>
		</div>
	</main>
</PageLayout>
