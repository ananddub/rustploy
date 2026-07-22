<script lang="ts">
	import { goto } from '$app/navigation';
	import {
		Cpu, Globe, Server, RefreshCw, Terminal, FileText,
		HardDriveDownload, Copy, Trash2, Plus, ArrowRightLeft,
		Download, Loader2, CheckCircle2, XCircle, AlertTriangle,
		Sparkles, RefreshCcw, Info, Package, Database, Play, Square
	} from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import * as Select from '$lib/components/ui/select';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Switch } from '$lib/components/ui/switch';
	import { Separator } from '$lib/components/ui/separator';
	import { toastSuccess, toastError } from '$lib/toast';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	// ─── Server Domain state ──────────────────────────────────────────────────────
	let domain          = $state('rustploy.example.com');
	let letsEncryptEmail= $state('admin@example.com');
	let https           = $state(true);
	let certType        = $state<'none'|'letsencrypt'|'custom'>('letsencrypt');
	let domainSaving    = $state(false);

	const serverIp      = '192.168.1.100';
	const serverVersion = 'v0.1.0';

	// ─── Docker cleanup ───────────────────────────────────────────────────────────
	let dockerCleanup   = $state(true);
	let autoCheckUpdates= $state(true);

	// ─── Update Server modal ──────────────────────────────────────────────────────
	type ModalState = 'idle'|'checking'|'results'|'updating';
	type HealthResult = { postgres: {status:'healthy'|'unhealthy'}, redis: {status:'healthy'|'unhealthy'}, traefik: {status:'healthy'|'unhealthy'} };

	let updateOpen      = $state(false);
	let updateState     = $state<ModalState>('idle');
	let healthResult    = $state<HealthResult|null>(null);
	let hasChecked      = $state(false);
	let isUpdateAvail   = $state(false);
	let latestVersion   = $state('');

	const allHealthy = $derived(
		!!healthResult &&
		healthResult.postgres.status === 'healthy' &&
		healthResult.redis.status   === 'healthy' &&
		healthResult.traefik.status === 'healthy'
	);

	async function checkUpdates() {
		hasChecked = true;
		await new Promise(r => setTimeout(r, 1200));
		isUpdateAvail = false;
		toastSuccess('No updates available');
	}

	async function verifyHealth() {
		updateState = 'checking';
		healthResult = null;
		await new Promise(r => setTimeout(r, 1000));
		healthResult = { postgres:{ status:'healthy' }, redis:{ status:'healthy' }, traefik:{ status:'healthy' } };
		updateState = 'results';
	}

	async function confirmUpdate() {
		updateState = 'updating';
		await new Promise(r => setTimeout(r, 2000));
		updateState = 'idle';
		updateOpen = false;
		toastSuccess('Server updated successfully');
	}

	// ─── Traefik Ports modal ──────────────────────────────────────────────────────
	type Port = { targetPort: number|''; publishedPort: number|''; protocol: 'tcp'|'udp'|'sctp' };
	let portsOpen = $state(false);
	let ports = $state<Port[]>([
		{ targetPort: 80,  publishedPort: 80,  protocol: 'tcp' },
		{ targetPort: 443, publishedPort: 443, protocol: 'tcp' },
	]);
	let portsSaving = $state(false);

	function addPort() { ports = [...ports, { targetPort: '', publishedPort: '', protocol: 'tcp' }]; }
	function removePort(i: number) { ports = ports.filter((_, idx) => idx !== i); }

	async function savePorts() {
		portsSaving = true;
		await new Promise(r => setTimeout(r, 500));
		portsSaving = false;
		portsOpen = false;
		toastSuccess('Ports updated successfully');
	}

	// ─── Action buttons ───────────────────────────────────────────────────────────
	let reloading = $state(false);
	async function reloadTraefik() {
		reloading = true;
		await new Promise(r => setTimeout(r, 1000));
		reloading = false;
		toastSuccess('Traefik reloaded');
	}

	async function saveDomain() {
		domainSaving = true;
		await new Promise(r => setTimeout(r, 600));
		domainSaving = false;
		toastSuccess('Domain saved');
	}

	function copyIp() {
		navigator.clipboard.writeText(serverIp);
		toastSuccess('Copied to clipboard');
	}
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<Cpu class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Web Server</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto">
		<div class="max-w-5xl mx-auto flex flex-col gap-6">

			<!-- ── 1. Server Domain ───────────────────────────────────────────── -->
			<div class="rounded-xl border border-border bg-card">
				<div class="px-6 pt-5 pb-4 border-b border-border flex items-center justify-between flex-wrap gap-3">
					<div>
						<h2 class="text-xl font-bold flex items-center gap-2">
							<Globe class="w-5 h-5 text-muted-foreground" /> Server Domain
						</h2>
						<p class="text-sm text-muted-foreground mt-0.5">Add a domain to your server application</p>
					</div>
				</div>
				<div class="px-6 py-5 space-y-4">
					{#if domain !== 'rustploy.example.com'}
						<div class="flex items-start gap-2 rounded-lg border border-yellow-500/30 bg-yellow-500/10 px-3 py-2.5 text-xs text-yellow-600 dark:text-yellow-400">
							<AlertTriangle class="w-3.5 h-3.5 shrink-0 mt-0.5" />
							<div>
								<p class="font-semibold">⚠️ Important: URL Change Impact</p>
								<p class="mt-0.5">If you change the Server URL make sure to update your GitHub Apps to keep auto-deploy and preview deployments working.</p>
							</div>
						</div>
					{/if}

					<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
						<div class="space-y-1.5">
							<Label for="domain" class="text-sm">Domain</Label>
							<Input id="domain" bind:value={domain} placeholder="example.com" />
						</div>
						<div class="space-y-1.5">
							<Label for="email" class="text-sm">Let's Encrypt Email</Label>
							<Input id="email" bind:value={letsEncryptEmail} placeholder="admin@example.com" type="email" />
						</div>
					</div>

					<div class="flex items-center justify-between rounded-lg border border-border px-4 py-3">
						<div>
							<p class="text-sm font-medium">HTTPS</p>
							<p class="text-sm text-muted-foreground">Automatically provision SSL Certificate</p>
						</div>
						<Switch bind:checked={https} />
					</div>

					{#if https}
						<div class="space-y-1.5">
							<Label class="text-sm">Certificate Provider</Label>
							<Select.Root type="single" value={certType} onValueChange={(v) => (certType = (v ?? 'none') as typeof certType)}>
								<Select.Trigger class="w-full">
									<span class="text-sm capitalize">{certType === 'letsencrypt' ? "Let's Encrypt" : certType}</span>
								</Select.Trigger>
								<Select.Content>
									<Select.Item value="none">None</Select.Item>
									<Select.Item value="letsencrypt">Let's Encrypt</Select.Item>
									<Select.Item value="custom">Custom</Select.Item>
								</Select.Content>
							</Select.Root>
						</div>
					{/if}

					<div class="flex justify-end">
						<Button size="sm" onclick={saveDomain} disabled={domainSaving} class="gap-1.5">
							{#if domainSaving}<Loader2 class="w-3.5 h-3.5 animate-spin" /> Saving…{:else}Save{/if}
						</Button>
					</div>
				</div>
			</div>

			<!-- ── 2. Web Server ──────────────────────────────────────────────── -->
			<div class="rounded-xl border border-border bg-card">
				<div class="px-6 pt-5 pb-4 border-b border-border">
					<h2 class="text-xl font-bold flex items-center gap-2">
						<Server class="w-5 h-5 text-muted-foreground" /> Web Server
					</h2>
					<p class="text-sm text-muted-foreground mt-0.5">Reload or clean the web server</p>
				</div>
				<div class="px-6 py-5 space-y-6">

					<!-- Action buttons grid -->
					<div class="grid md:grid-cols-2 gap-3">
						<!-- Rustploy actions -->
						<Button variant="secondary" size="sm" class="w-full gap-2 justify-start h-10">
							<Play class="w-4 h-4" /> Start Rustploy
						</Button>
						<Button variant="secondary" size="sm" class="w-full gap-2 justify-start h-10">
							<Square class="w-4 h-4" /> Stop Rustploy
						</Button>

						<!-- Traefik actions -->
						<Button variant="secondary" size="sm" class="w-full gap-2 justify-start h-10"
							onclick={reloadTraefik} disabled={reloading}>
							{#if reloading}<Loader2 class="w-4 h-4 animate-spin" />{:else}<RefreshCw class="w-4 h-4" />{/if}
							Reload Traefik
						</Button>

						<!-- Manage ports -->
						<Button variant="secondary" size="sm" class="w-full gap-2 justify-start h-10"
							onclick={() => (portsOpen = true)}>
							<ArrowRightLeft class="w-4 h-4" /> Manage Traefik Ports
						</Button>

						<!-- Logs -->
						<Button variant="secondary" size="sm" class="w-full gap-2 justify-start h-10">
							<FileText class="w-4 h-4" /> View Traefik Logs
						</Button>

						<!-- Terminal -->
						<Button variant="secondary" size="sm" class="w-full gap-2 justify-start h-10">
							<Terminal class="w-4 h-4" /> Terminal
						</Button>

						<!-- Storage clean -->
						<Button variant="secondary" size="sm" class="w-full gap-2 justify-start h-10">
							<Trash2 class="w-4 h-4" /> Clean Docker Storage
						</Button>

						<!-- Update server -->
						<Button variant="secondary" size="sm" class="w-full gap-2 justify-start h-10 relative"
							onclick={() => (updateOpen = true)}>
							<HardDriveDownload class="w-4 h-4" />
							Check for Updates
						</Button>
					</div>

					<Separator />

					<!-- Info row -->
					<div class="flex items-center justify-between flex-wrap gap-4">
						<div class="flex items-center gap-1.5 text-sm text-muted-foreground">
							Server IP: <span class="font-mono">{serverIp}</span>
							<button onclick={copyIp} class="hover:text-foreground transition-colors">
								<Copy class="w-3.5 h-3.5" />
							</button>
						</div>
						<span class="text-sm text-muted-foreground">Version: {serverVersion}</span>
						<div class="flex items-center gap-3">
							<div class="flex items-center justify-between rounded-lg border border-border px-3 py-2 gap-4">
								<div>
									<p class="text-xs font-medium">Docker Cleanup</p>
									<p class="text-[10px] text-muted-foreground">Remove unused images and containers</p>
								</div>
								<Switch bind:checked={dockerCleanup} />
							</div>
						</div>
					</div>
				</div>
			</div>

		</div>
	</main>
</PageLayout>

<!-- ── Update Server Modal ─────────────────────────────────────────────────── -->
{#if updateOpen}
	<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1"
		onclick={() => { if (updateState !== 'updating') { updateOpen = false; updateState = 'idle'; healthResult = null; }}} onkeydown={() => {}}></div>
	<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
		<div class="bg-card border border-border rounded-xl shadow-2xl w-full max-w-lg pointer-events-auto p-6 flex flex-col gap-4">

			<!-- Title -->
			<div class="flex items-center justify-between">
				<h2 class="text-xl font-semibold">Web Server Update</h2>
				<div class="flex items-center gap-1.5 rounded-full px-3 py-1 bg-muted text-xs text-muted-foreground">
					<Server class="w-3.5 h-3.5" /> {serverVersion}
				</div>
			</div>

			<!-- Body -->
			{#if updateState === 'idle' && !hasChecked}
				<p class="text-sm text-muted-foreground">
					Check for new releases and update Rustploy.<br /><br />
					We recommend checking for updates regularly to ensure you have the latest features and security improvements.
				</p>
			{/if}

			{#if updateState === 'idle' && hasChecked && !isUpdateAvail}
				<div class="flex flex-col items-center gap-4 py-4">
					<div class="rounded-full p-4 bg-green-500/20">
						<Sparkles class="w-8 h-8 text-green-400" />
					</div>
					<div class="text-center">
						<h3 class="font-medium">You are using the latest version</h3>
						<p class="text-sm text-muted-foreground mt-1">Your server is up to date.</p>
					</div>
				</div>
			{/if}

			{#if updateState === 'checking'}
				<div class="flex items-center gap-2 text-sm text-muted-foreground py-4">
					<Loader2 class="w-4 h-4 animate-spin" /> Checking PostgreSQL, Redis and Traefik…
				</div>
			{/if}

			{#if updateState === 'results' && healthResult}
				<div class="flex flex-col gap-3">
					<div class="space-y-2">
						{#each [['PostgreSQL', healthResult.postgres], ['Redis', healthResult.redis], ['Traefik', healthResult.traefik]] as [name, svc] (name)}
							<div class="flex items-center gap-2 text-sm">
								{#if svc.status === 'healthy'}
									<CheckCircle2 class="w-4 h-4 text-green-500" />
								{:else}
									<XCircle class="w-4 h-4 text-destructive" />
								{/if}
								<span class="font-medium">{name}</span>
							</div>
						{/each}
					</div>
					{#if allHealthy}
						<p class="text-sm text-muted-foreground">All services are running. You can proceed.</p>
					{:else}
						<div class="flex items-start gap-2 rounded-lg border border-yellow-500/30 bg-yellow-500/10 px-3 py-2 text-xs text-yellow-600 dark:text-yellow-400">
							<AlertTriangle class="w-3.5 h-3.5 shrink-0 mt-0.5" />
							Some services are not healthy. You can still proceed with the update.
						</div>
					{/if}
				</div>
			{/if}

			{#if updateState === 'updating'}
				<div class="flex items-center gap-2 text-sm text-muted-foreground py-4">
					<Loader2 class="w-4 h-4 animate-spin" /> The server is being updated, please wait…
				</div>
			{/if}

			<!-- Auto-check toggle -->
			<div class="flex items-center justify-between rounded-lg border border-border px-3 py-2.5">
				<p class="text-xs font-medium">Auto-check updates</p>
				<Switch bind:checked={autoCheckUpdates} />
			</div>

			<!-- Footer -->
			<div class="flex items-center justify-end gap-2 pt-1">
				{#if updateState !== 'updating'}
					<Button variant="outline" size="sm"
						onclick={() => { updateOpen = false; updateState = 'idle'; healthResult = null; }}>
						Cancel
					</Button>
				{/if}
				{#if updateState === 'idle'}
					<Button variant="secondary" size="sm" class="gap-1.5" onclick={verifyHealth}>
						<RefreshCw class="w-3.5 h-3.5" /> Verify Status
					</Button>
					<Button size="sm" onclick={checkUpdates} class="gap-1.5">
						<Download class="w-3.5 h-3.5" /> Check for Updates
					</Button>
				{/if}
				{#if updateState === 'results'}
					<Button variant="secondary" size="sm" class="gap-1.5" onclick={verifyHealth}>
						<RefreshCw class="w-3.5 h-3.5" /> Re-check
					</Button>
					<Button size="sm" onclick={confirmUpdate}>
						{allHealthy ? 'Confirm' : 'Confirm Anyway'}
					</Button>
				{/if}
			</div>
		</div>
	</div>
{/if}

<!-- ── Traefik Ports Modal ──────────────────────────────────────────────────── -->
{#if portsOpen}
	<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1"
		onclick={() => (portsOpen = false)} onkeydown={() => {}}></div>
	<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
		<div class="bg-card border border-border rounded-xl shadow-2xl w-full max-w-3xl pointer-events-auto flex flex-col max-h-[90vh]">
			<!-- Header -->
			<div class="px-6 pt-5 pb-4 border-b border-border">
				<h2 class="text-lg font-semibold">Additional Port Mappings</h2>
				<div class="flex items-center justify-between mt-1">
					<p class="text-sm text-muted-foreground">
						Add or remove additional ports for Traefik
						<span class="ml-1">· {ports.length} port mapping{ports.length !== 1 ? 's' : ''} configured</span>
					</p>
					<Button size="sm" class="gap-1.5" onclick={addPort}>
						<Plus class="w-3.5 h-3.5" /> Add Mapping
					</Button>
				</div>
			</div>

			<!-- Body -->
			<div class="flex-1 overflow-y-auto px-6 py-4 space-y-3">
				{#if ports.length === 0}
					<div class="flex flex-col items-center justify-center py-12 text-muted-foreground gap-2">
						<ArrowRightLeft class="w-8 h-8 opacity-30" />
						<p class="font-medium">No port mappings configured</p>
						<p class="text-xs opacity-60">Add one to get started</p>
					</div>
				{:else}
					{#each ports as port, i (i)}
						<div class="grid grid-cols-4 gap-3 items-end rounded-lg border border-border p-4 bg-muted/20">
							<div class="space-y-1.5">
								<Label class="text-sm text-muted-foreground">Target Port</Label>
								<Input type="number" bind:value={port.targetPort} placeholder="e.g. 8080" class="h-8 text-sm" />
							</div>
							<div class="space-y-1.5">
								<Label class="text-sm text-muted-foreground">Published Port</Label>
								<Input type="number" bind:value={port.publishedPort} placeholder="e.g. 80" class="h-8 text-sm" />
							</div>
							<div class="space-y-1.5">
								<Label class="text-sm text-muted-foreground">Protocol</Label>
								<Select.Root type="single" value={port.protocol} onValueChange={(v) => (port.protocol = (v ?? 'tcp') as typeof port.protocol)}>
									<Select.Trigger class="h-8"><span class="text-sm uppercase">{port.protocol}</span></Select.Trigger>
									<Select.Content>
										<Select.Item value="tcp">TCP</Select.Item>
										<Select.Item value="udp">UDP</Select.Item>
										<Select.Item value="sctp">SCTP</Select.Item>
									</Select.Content>
								</Select.Root>
							</div>
							<button onclick={() => removePort(i)}
								class="h-8 w-8 rounded-md flex items-center justify-center text-muted-foreground hover:text-destructive hover:bg-destructive/10 transition-colors">
								<Trash2 class="w-4 h-4" />
							</button>
						</div>
					{/each}

					{#if ports.length > 0}
						<div class="flex items-start gap-2 rounded-lg border border-blue-500/20 bg-blue-500/10 px-3 py-2.5 text-xs text-blue-500">
							<Info class="w-3.5 h-3.5 shrink-0 mt-0.5" />
							<div>
								<strong>Target Port:</strong> the port inside your container.
								<strong class="ml-1">Published Port:</strong> the port on your host machine.
							</div>
						</div>
					{/if}

					<div class="flex items-start gap-2 rounded-lg border border-yellow-500/30 bg-yellow-500/10 px-3 py-2.5 text-xs text-yellow-600 dark:text-yellow-400">
						<AlertTriangle class="w-3.5 h-3.5 shrink-0 mt-0.5" />
						The Traefik container will be recreated from scratch. This may cause downtime in your applications.
					</div>
				{/if}
			</div>

			<!-- Footer -->
			<div class="px-6 py-4 border-t border-border flex justify-end gap-2">
				<Button variant="outline" size="sm" onclick={() => (portsOpen = false)}>Cancel</Button>
				<Button size="sm" onclick={savePorts} disabled={portsSaving} class="gap-1.5">
					{#if portsSaving}<Loader2 class="w-3.5 h-3.5 animate-spin" /> Saving…{:else}Save{/if}
				</Button>
			</div>
		</div>
	</div>
{/if}
