<script lang="ts">
	import { Zap, RefreshCw, Clock } from '@lucide/svelte';
	import { formatDate, formatDuration, deployStatusLabel, deployStatusColor } from '$lib/helpers';

	type Props = {
		serviceLabel?: string;
		onDeploy: () => Promise<void>;
		onRedeploy: () => Promise<void>;
	};

	type DeploymentRecord = {
		id: number;
		title: string;
		description?: string;
		status: 'RUNNING' | 'SUCCESS' | 'FAILED' | 'CANCELLED';
		log_path: string;
		started_at?: number;
		finished_at?: number;
		created_at: number;
	};

	let { serviceLabel, onDeploy, onRedeploy }: Props = $props();
	const label = $derived(serviceLabel ?? 'service');
	const deployments: DeploymentRecord[] = [];

	let deploying = $state(false);
	let redeploying = $state(false);

	async function handleDeploy() {
		deploying = true;
		try { await onDeploy(); } finally { deploying = false; }
	}

	async function handleRedeploy() {
		redeploying = true;
		try { await onRedeploy(); } finally { redeploying = false; }
	}
</script>

<div class="flex flex-col gap-6">
	<section class="bg-card border border-border rounded-lg p-6">
		<div class="flex items-center justify-between">
			<div>
				<h2 class="text-base font-semibold">Deployments</h2>
				<p class="text-sm text-muted-foreground mt-1">History of all deployments for this {label}.</p>
			</div>
			<div class="flex items-center gap-2">
				<button
					class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
					onclick={handleRedeploy}
					disabled={redeploying}
				>
					{#if redeploying}
						<div class="w-3.5 h-3.5 border-2 border-current/30 border-t-current rounded-full animate-spin"></div>
					{:else}
						<RefreshCw size={14} />
					{/if}
					Redeploy
				</button>
				<button
					class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 transition-colors disabled:opacity-50"
					onclick={handleDeploy}
					disabled={deploying}
				>
					{#if deploying}
						<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>
					{:else}
						<Zap size={14} />
					{/if}
					Deploy
				</button>
			</div>
		</div>
	</section>

	<section class="bg-card border border-border rounded-lg overflow-hidden">
		{#if deployments.length === 0}
			<div class="flex flex-col items-center justify-center py-16 text-muted-foreground/40">
				<Zap size={40} class="mb-3 opacity-40" />
				<p class="text-sm">No deployments yet</p>
				<p class="text-xs mt-1 opacity-70">Deploy your {label} to see history here.</p>
			</div>
		{:else}
			<div class="grid grid-cols-[1fr_120px_100px_110px_80px] gap-4 px-5 py-2.5 border-b border-border text-xs text-muted-foreground font-medium uppercase tracking-wide">
				<span>Deployment</span><span>Status</span><span>Duration</span><span>Started</span><span></span>
			</div>
			{#each deployments as d (d.id)}
				<div class="grid grid-cols-[1fr_120px_100px_110px_80px] gap-4 items-center px-5 py-3 border-b border-border last:border-0 hover:bg-accent/30 transition-colors">
					<div class="min-w-0">
						<p class="text-sm font-medium truncate">{d.title}</p>
						{#if d.description}<p class="text-xs text-muted-foreground truncate mt-0.5">{d.description}</p>{/if}
					</div>
					<div class="text-xs font-medium {deployStatusColor(d.status)}">{deployStatusLabel(d.status)}</div>
					<div class="text-xs text-muted-foreground flex items-center gap-1">
						<Clock size={12} />{formatDuration(d.started_at, d.finished_at)}
					</div>
					<div class="text-xs text-muted-foreground">{formatDate(d.created_at)}</div>
					<div class="flex justify-end">
						<button class="px-2 py-1 rounded text-xs text-muted-foreground hover:text-foreground hover:bg-accent transition-colors">Logs</button>
					</div>
				</div>
			{/each}
		{/if}
	</section>
</div>
