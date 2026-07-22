<script lang="ts">
	import { Plus, Calendar, Trash2, Clock, Play, RefreshCw } from '@lucide/svelte';
	import Switch from '$lib/components/Switch.svelte';
	import { formatDate } from '$lib/helpers';

	type Props = { serviceLabel?: string; serviceId?: number };
	let { serviceLabel }: Props = $props();
	const label = $derived(serviceLabel ?? 'service');

	const CRON_PRESETS = [
		{ label: 'Every hour',            value: '0 * * * *'    },
		{ label: 'Every day at midnight', value: '0 0 * * *'    },
		{ label: 'Every Monday at 8 am',  value: '0 8 * * 1'    },
		{ label: 'Every 30 minutes',      value: '*/30 * * * *' },
	];

	let name = $state('');
	let cron = $state('0 0 * * *');
	let saving = $state(false);

	const schedules: any[] = [];

	async function addSchedule() {
		if (!name.trim() || !cron.trim()) return;
		saving = true;
		try {
			await new Promise(r => setTimeout(r, 300));
			name = ''; cron = '0 0 * * *';
		} finally { saving = false; }
	}
</script>

<div class="flex flex-col gap-6">
	<section class="bg-card border border-border rounded-lg p-6">
		<h2 class="text-base font-semibold mb-1">Add Schedule</h2>
		<p class="text-sm text-muted-foreground mb-5">Trigger a deployment automatically on a cron schedule.</p>
		<div class="flex flex-col gap-4">
			<div class="flex flex-col gap-1.5">
				<label for="sched-name" class="text-sm font-medium text-muted-foreground">Schedule Name <span class="text-destructive">*</span></label>
				<input id="sched-name" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring" placeholder="Nightly deploy" bind:value={name} />
			</div>
			<div class="flex flex-col gap-1.5">
				<label for="sched-cron" class="text-sm font-medium text-muted-foreground">Cron Expression <span class="text-destructive">*</span></label>
				<div class="flex gap-2">
					<input id="sched-cron" class="flex h-9 flex-1 rounded-md border border-input bg-transparent px-3 py-1 text-sm font-mono placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring" placeholder="0 0 * * *" bind:value={cron} />
					<select class="h-9 rounded-md border border-input bg-card px-3 text-sm focus:outline-none focus:ring-1 focus:ring-ring" onchange={(e) => (cron = (e.target as HTMLSelectElement).value)}>
						<option value="">Presets…</option>
						{#each CRON_PRESETS as p}
							<option value={p.value}>{p.label}</option>
						{/each}
					</select>
				</div>
				<p class="text-sm text-muted-foreground font-mono">{cron || '—'}</p>
			</div>
			<div class="flex justify-end">
				<button
					class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50 transition-colors"
					onclick={addSchedule}
					disabled={saving || !name.trim() || !cron.trim()}
				>
					{#if saving}
						<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>
					{:else}
						<Plus size={14} />
					{/if}
					Add Schedule
				</button>
			</div>
		</div>
	</section>

	<section class="bg-card border border-border rounded-lg overflow-hidden">
		<div class="px-5 py-3.5 border-b border-border">
			<h2 class="text-sm font-semibold">Active Schedules</h2>
		</div>
		{#if schedules.length === 0}
			<div class="flex flex-col items-center justify-center py-14 text-muted-foreground/30">
				<Calendar size={40} class="mb-3 opacity-40" />
				<p class="text-sm">No schedules configured</p>
				<p class="text-xs mt-1 opacity-70">Add a cron schedule above to automate deploys.</p>
			</div>
		{:else}
			<div class="grid grid-cols-[1fr_160px_140px_140px_80px] gap-4 px-5 py-2 border-b border-border text-xs text-muted-foreground font-medium uppercase tracking-wide">
				<span>Name</span><span>Cron</span><span>Last Run</span><span>Next Run</span><span></span>
			</div>
			{#each schedules as s (s.id)}
				<div class="grid grid-cols-[1fr_160px_140px_140px_80px] gap-4 items-center px-5 py-3 border-b border-border last:border-0 hover:bg-accent/20 transition-colors">
					<div class="flex items-center gap-2 min-w-0">
						<Switch checked={s.enabled} onchange={() => {}} />
						<span class="text-sm font-medium truncate">{s.name}</span>
					</div>
					<span class="text-xs font-mono text-muted-foreground">{s.cron}</span>
					<div class="text-xs text-muted-foreground flex items-center gap-1">
						<Clock size={12} />{s.last_run ? formatDate(s.last_run) : '—'}
					</div>
					<div class="text-xs text-muted-foreground flex items-center gap-1">
						<Play size={12} />{s.next_run ? formatDate(s.next_run) : '—'}
					</div>
					<div class="flex justify-end">
						<button class="p-1.5 rounded-md text-muted-foreground/50 hover:text-destructive hover:bg-destructive/10 transition-all">
							<Trash2 size={13} />
						</button>
					</div>
				</div>
			{/each}
		{/if}
	</section>
</div>
