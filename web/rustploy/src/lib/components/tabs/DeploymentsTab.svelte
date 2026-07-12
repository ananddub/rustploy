<script lang="ts">
	import { Zap, RefreshCw, Clock, ExternalLink } from '@lucide/svelte';
	import { withToast } from '$lib/toast';
	import { formatDate, formatDuration, deployStatusLabel, deployStatusColor } from '$lib/helpers';
	import {
		deploymentControllerApplicationEvents,
		deploymentControllerComposeEvents,
		deploymentControllerCancel
	} from '$lib/client/sdk.gen';

	type Props = {
		serviceLabel?: string;
		serviceId: number;
		serviceType: 'application' | 'compose';
		onDeploy: () => Promise<void>;
		onRedeploy: () => Promise<void>;
	};

	type DeploymentEvent = {
		id: number;
		title: string;
		description?: string;
		status: string;
		log_path?: string;
		started_at?: number;
		finished_at?: number;
		created_at: number;
	};

	let { serviceLabel, serviceId, serviceType, onDeploy, onRedeploy }: Props = $props();
	const label = $derived(serviceLabel ?? 'service');

	let deployments = $state<DeploymentEvent[]>([]);
	let loadingEvents = $state(true);
	let cancellingId = $state<number | null>(null);

	let deploying = $state(false);
	let redeploying = $state(false);

	async function loadEvents() {
		loadingEvents = true;
		try {
			const res = serviceType === 'application'
				? await deploymentControllerApplicationEvents({ path: { id: serviceId } })
				: await deploymentControllerComposeEvents({ path: { id: serviceId } });
			deployments = (res.data as DeploymentEvent[]) ?? [];
		} catch {
			deployments = [];
		} finally {
			loadingEvents = false;
		}
	}

	loadEvents();

	async function handleDeploy() {
		deploying = true;
		try {
			await withToast(onDeploy, {
				loading: 'Triggering deployment…',
				success: 'Deployment triggered!',
				successDescription: 'Your service is being deployed.'
			});
			await loadEvents();
		} finally { deploying = false; }
	}

	async function handleRedeploy() {
		redeploying = true;
		try {
			await withToast(onRedeploy, {
				loading: 'Redeploying…',
				success: 'Redeploy triggered!',
				successDescription: 'Your service is being redeployed.'
			});
			await loadEvents();
		} finally { redeploying = false; }
	}

	async function cancelDeployment(id: number) {
		cancellingId = id;
		try { await deploymentControllerCancel({ path: { id } }); await loadEvents(); }
		finally { cancellingId = null; }
	}
</script>

<div class="flex flex-col gap-6 animate-fade-up">
	<section class="bg-card border border-border rounded-lg p-6">
		<div class="flex items-center justify-between">
			<div>
				<h2 class="text-base font-semibold">Deployments</h2>
				<p class="text-sm text-muted-foreground mt-1">Deployment history for this {label}.</p>
			</div>
			<div class="flex items-center gap-2">
				<button
					onclick={loadEvents}
					disabled={loadingEvents}
					class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent disabled:opacity-50 transition-colors"
				>
					<RefreshCw size={14} class={loadingEvents ? 'animate-spin' : ''} /> Refresh
				</button>
				<button
					onclick={handleRedeploy}
					disabled={redeploying}
					class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent disabled:opacity-50 transition-colors"
				>
					{#if redeploying}<div class="w-3.5 h-3.5 border-2 border-current/30 border-t-current rounded-full animate-spin"></div>{:else}<RefreshCw size={14} />{/if}
					Redeploy
				</button>
				<button
					onclick={handleDeploy}
					disabled={deploying}
					class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50 transition-colors"
				>
					{#if deploying}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>{:else}<Zap size={14} />{/if}
					Deploy
				</button>
			</div>
		</div>
	</section>

	<section class="bg-card border border-border rounded-lg overflow-hidden">
		{#if loadingEvents}
			<div class="flex justify-center py-14"><div class="w-6 h-6 border-2 border-muted-foreground/30 border-t-foreground rounded-full animate-spin"></div></div>
		{:else if deployments.length === 0}
			<div class="flex flex-col items-center justify-center py-16 text-muted-foreground/40">
				<Zap size={40} class="mb-3 opacity-40" />
				<p class="text-sm">No deployments yet</p>
				<p class="text-xs mt-1 opacity-70">Deploy your {label} to see history here.</p>
			</div>
		{:else}
			<div class="grid grid-cols-[1fr_110px_90px_130px_90px] gap-3 px-5 py-2.5 border-b border-border text-xs text-muted-foreground font-medium uppercase tracking-wide">
				<span>Deployment</span><span>Status</span><span>Duration</span><span>Started</span><span class="text-right">Actions</span>
			</div>
			{#each deployments as d (d.id)}
				<div class="grid grid-cols-[1fr_110px_90px_130px_90px] gap-3 items-center px-5 py-3 border-b border-border last:border-0 hover:bg-accent/20 transition-colors">
					<div class="min-w-0">
						<p class="text-sm font-medium truncate">{d.title}</p>
						{#if d.description}<p class="text-xs text-muted-foreground truncate mt-0.5">{d.description}</p>{/if}
					</div>
					<div>
						<span class="inline-flex items-center gap-1 text-xs font-medium {deployStatusColor(d.status)}">
							<span class="w-1.5 h-1.5 rounded-full bg-current"></span>
							{deployStatusLabel(d.status)}
						</span>
					</div>
					<div class="text-xs text-muted-foreground flex items-center gap-1">
						<Clock size={12} />{formatDuration(d.started_at, d.finished_at)}
					</div>
					<div class="text-xs text-muted-foreground">{d.created_at ? formatDate(d.created_at) : '—'}</div>
					<div class="flex justify-end items-center gap-1">
						{#if d.status === 'RUNNING'}
							<button
								onclick={() => cancelDeployment(d.id)}
								disabled={cancellingId === d.id}
								class="px-2 py-1 rounded text-xs text-destructive/70 hover:text-destructive hover:bg-destructive/10 transition-colors"
								title="Cancel"
							>
								{#if cancellingId === d.id}<div class="w-3 h-3 border-2 border-current/30 border-t-current rounded-full animate-spin"></div>{:else}Cancel{/if}
							</button>
						{/if}
						{#if d.log_path}
							<a href={d.log_path} target="_blank" rel="noopener noreferrer" class="p-1 rounded text-muted-foreground/40 hover:text-foreground hover:bg-accent transition-colors" title="View logs">
								<ExternalLink size={13} />
							</a>
						{/if}
					</div>
				</div>
			{/each}
		{/if}
	</section>
</div>
