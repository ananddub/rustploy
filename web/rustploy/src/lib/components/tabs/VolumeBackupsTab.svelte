<script lang="ts">
	import { Plus, HardDrive, Trash2, Download, Clock, RefreshCw } from '@lucide/svelte';
	import { formatDate } from '$lib/helpers';

	type Props = { serviceLabel?: string; serviceId?: number };
	let { serviceLabel }: Props = $props();
	const label = $derived(serviceLabel ?? 'service');

	let backupName = $state('');
	let creating = $state(false);
	const backups: any[] = [];

	async function createBackup() {
		if (!backupName.trim()) return;
		creating = true;
		try {
			await new Promise(r => setTimeout(r, 400));
			backupName = '';
		} finally { creating = false; }
	}

	function statusBadgeClass(status: string) {
		const map: Record<string, string> = {
			SUCCESS: 'bg-green-500/15 text-green-500',
			FAILED:  'bg-destructive/15 text-destructive',
			RUNNING: 'bg-yellow-500/15 text-yellow-500',
		};
		return map[status] ?? 'bg-muted text-muted-foreground';
	}
</script>

<div class="flex flex-col gap-6">
	<section class="bg-card border border-border rounded-lg p-6">
		<h2 class="text-base font-semibold mb-1">Volume Backups</h2>
		<p class="text-sm text-muted-foreground mb-5">Create and manage backups of volumes attached to this {label}.</p>
		<div class="flex gap-3 items-end">
			<div class="flex flex-col gap-1.5 flex-1">
				<label for="backup-name" class="text-sm font-medium text-muted-foreground">Backup Name</label>
				<input id="backup-name" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring" placeholder="backup-2026-07-12" bind:value={backupName} />
			</div>
			<button
				class="inline-flex items-center gap-1.5 px-3 py-1.5 h-9 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50 transition-colors"
				onclick={createBackup}
				disabled={creating || !backupName.trim()}
			>
				{#if creating}
					<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>
				{:else}
					<Plus size={14} />
				{/if}
				Create Backup
			</button>
		</div>
	</section>

	<section class="bg-card border border-border rounded-lg overflow-hidden">
		<div class="flex items-center justify-between px-5 py-3.5 border-b border-border">
			<h2 class="text-sm font-semibold">Backup History</h2>
			<button class="inline-flex items-center gap-1 px-2 py-1 rounded text-xs text-muted-foreground hover:text-foreground hover:bg-accent transition-colors">
				<RefreshCw size={12} /> Refresh
			</button>
		</div>
		{#if backups.length === 0}
			<div class="flex flex-col items-center justify-center py-14 text-muted-foreground/30">
				<HardDrive size={40} class="mb-3 opacity-40" />
				<p class="text-sm">No backups yet</p>
				<p class="text-xs mt-1 opacity-70">Create a backup above to protect your volume data.</p>
			</div>
		{:else}
			<div class="grid grid-cols-[1fr_100px_100px_140px_80px] gap-4 px-5 py-2 border-b border-border text-xs text-muted-foreground font-medium uppercase tracking-wide">
				<span>Name</span><span>Size</span><span>Status</span><span>Created</span><span></span>
			</div>
			{#each backups as b (b.id)}
				<div class="grid grid-cols-[1fr_100px_100px_140px_80px] gap-4 items-center px-5 py-3 border-b border-border last:border-0 hover:bg-accent/20 transition-colors">
					<div class="flex items-center gap-2 min-w-0">
						<HardDrive size={13} class="text-muted-foreground/40 shrink-0" />
						<span class="text-sm font-medium truncate">{b.name}</span>
					</div>
					<span class="text-xs text-muted-foreground">{b.size ?? '—'}</span>
					<span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium {statusBadgeClass(b.status)}">
						{b.status.charAt(0) + b.status.slice(1).toLowerCase()}
					</span>
					<div class="text-xs text-muted-foreground flex items-center gap-1">
						<Clock size={12} />{formatDate(b.created_at)}
					</div>
					<div class="flex justify-end gap-0.5">
						<button class="p-1.5 rounded-md text-muted-foreground/40 hover:text-foreground hover:bg-accent transition-all" title="Download">
							<Download size={13} />
						</button>
						<button class="p-1.5 rounded-md text-muted-foreground/40 hover:text-destructive hover:bg-destructive/10 transition-all" title="Delete">
							<Trash2 size={13} />
						</button>
					</div>
				</div>
			{/each}
		{/if}
	</section>
</div>
