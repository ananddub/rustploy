<script lang="ts">
	import { goto } from '$app/navigation';
	import { Calendar, Plus, Trash2, Play, Clock, Terminal, PenLine, Loader2 } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Switch } from '$lib/components/ui/switch';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import * as Select from '$lib/components/ui/select';
	import { toastSuccess, toastError } from '$lib/toast';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	type Schedule = {
		id: number;
		name: string;
		description: string;
		cron: string;
		shellType: 'bash' | 'sh';
		command: string;
		service: string;
		enabled: boolean;
		lastRun: string;
		nextRun: string;
	};

	let schedules = $state<Schedule[]>([
		{ id: 1, name: 'Daily Backup', description: 'Backs up the database every day at 2am', cron: '0 2 * * *', shellType: 'bash', command: 'pg_dump -U postgres mydb > /backups/db.sql', service: 'postgres-main', enabled: true, lastRun: '2026-07-12 02:00', nextRun: '2026-07-13 02:00' },
		{ id: 2, name: 'Weekly Cleanup', description: 'Removes old log files every Sunday', cron: '0 4 * * 0', shellType: 'bash', command: 'find /logs -mtime +7 -delete', service: 'rustploy-app', enabled: true, lastRun: '2026-07-07 04:00', nextRun: '2026-07-14 04:00' },
		{ id: 3, name: 'Health Check', description: 'Checks API health every 5 minutes', cron: '*/5 * * * *', shellType: 'sh', command: 'curl -sf http://localhost:3000/health', service: 'api-service', enabled: false, lastRun: '2026-07-12 20:55', nextRun: '—' },
	]);

	const cronPresets = [
		{ label: 'Every minute',              value: '* * * * *'    },
		{ label: 'Every hour',                value: '0 * * * *'    },
		{ label: 'Every day at midnight',     value: '0 0 * * *'    },
		{ label: 'Every day at 2am',          value: '0 2 * * *'    },
		{ label: 'Every Sunday at midnight',  value: '0 0 * * 0'    },
		{ label: 'Every 5 minutes',           value: '*/5 * * * *'  },
		{ label: 'Every 15 minutes',          value: '*/15 * * * *' },
		{ label: 'Every weekday at midnight', value: '0 0 * * 1-5'  },
		{ label: 'Custom',                    value: 'custom'        },
	];

	// ─── Modal ────────────────────────────────────────────────────────────────────
	let showModal   = $state(false);
	let editingId   = $state<number | null>(null);
	let saving      = $state(false);
	let modalError  = $state('');
	let fName       = $state('');
	let fDesc       = $state('');
	let fCron       = $state('');
	let fPreset     = $state('custom');
	let fShell      = $state<'bash' | 'sh'>('bash');
	let fCommand    = $state('');
	let fService    = $state('');
	let fEnabled    = $state(true);

	// ─── Delete confirm ───────────────────────────────────────────────────────────
	let confirmDeleteId = $state<number | null>(null);
	let deletingId      = $state<number | null>(null);
	let runningId       = $state<number | null>(null);

	let nextId = $derived(Math.max(0, ...schedules.map(s => s.id)) + 1);

	function openCreate() {
		editingId = null;
		fName = ''; fDesc = ''; fCron = ''; fPreset = 'custom';
		fShell = 'bash'; fCommand = ''; fService = ''; fEnabled = true;
		modalError = '';
		showModal = true;
	}

	function openEdit(s: Schedule) {
		editingId = s.id;
		fName = s.name; fDesc = s.description; fCron = s.cron;
		fPreset = cronPresets.find(p => p.value === s.cron && p.value !== 'custom')?.value ?? 'custom';
		fShell = s.shellType; fCommand = s.command; fService = s.service; fEnabled = s.enabled;
		modalError = '';
		showModal = true;
	}

	function closeModal() { showModal = false; editingId = null; }

	function handlePreset(val: string) {
		fPreset = val;
		if (val !== 'custom') fCron = val;
	}

	function handleCronInput(e: Event) {
		const val = (e.target as HTMLInputElement).value;
		fCron = val;
		fPreset = cronPresets.find(p => p.value === val && p.value !== 'custom')?.value ?? 'custom';
	}

	async function submitModal(e: SubmitEvent) {
		e.preventDefault();
		modalError = '';
		if (!fName.trim()) { modalError = 'Name is required'; return; }
		if (!fCron.trim()) { modalError = 'Cron expression is required'; return; }
		if (!fCommand.trim()) { modalError = 'Command is required'; return; }
		saving = true;
		try {
			await new Promise(r => setTimeout(r, 400));
			if (editingId !== null) {
				schedules = schedules.map(s => s.id === editingId
					? { ...s, name: fName.trim(), description: fDesc.trim(), cron: fCron.trim(), shellType: fShell, command: fCommand.trim(), service: fService.trim(), enabled: fEnabled }
					: s);
				toastSuccess('Schedule updated');
			} else {
				schedules = [...schedules, { id: nextId, name: fName.trim(), description: fDesc.trim(), cron: fCron.trim(), shellType: fShell, command: fCommand.trim(), service: fService.trim(), enabled: fEnabled, lastRun: '—', nextRun: '—' }];
				toastSuccess('Schedule created');
			}
			closeModal();
		} catch { modalError = 'Failed to save'; }
		finally { saving = false; }
	}

	async function deleteSchedule(id: number) {
		deletingId = id;
		try {
			await new Promise(r => setTimeout(r, 400));
			schedules = schedules.filter(s => s.id !== id);
			toastSuccess('Schedule deleted');
		} finally { deletingId = null; confirmDeleteId = null; }
	}

	async function runNow(id: number) {
		runningId = id;
		try {
			await new Promise(r => setTimeout(r, 1000));
			toastSuccess('Schedule ran successfully');
		} catch { toastError('Failed to run schedule'); }
		finally { runningId = null; }
	}
</script>

<PageLayout>
	<!-- Page header -->
	<header class="flex items-center justify-between px-6 py-3 border-b border-border">
		<div class="flex items-center gap-2 text-sm">
			<Calendar class="w-4 h-4 text-muted-foreground" />
			<span class="font-medium">Schedules</span>
		</div>
		<Button size="sm" class="gap-1.5 text-xs" onclick={openCreate}>
			<Plus class="w-3.5 h-3.5" /> Add Schedule
		</Button>
	</header>

	<main class="flex-1 p-6 overflow-y-auto">
		<!-- Section title -->
		<div class="mb-6">
			<h1 class="text-xl font-bold">Scheduled Tasks</h1>
			<p class="text-sm text-muted-foreground mt-1">Schedule tasks to run automatically at specified intervals</p>
		</div>

		<!-- Empty state -->
		{#if schedules.length === 0}
			<div class="flex flex-col items-center justify-center py-20 text-muted-foreground gap-3 rounded-xl border border-dashed border-border">
				<Clock class="w-10 h-10 opacity-30" />
				<p class="text-base font-medium">No scheduled tasks</p>
				<p class="text-sm opacity-60">Create your first scheduled task to automate your workflows</p>
				<Button size="sm" class="gap-1.5 mt-2" onclick={openCreate}>
					<Plus class="w-4 h-4" /> Add Schedule
				</Button>
			</div>

		<!-- Schedule cards grid -->
		{:else}
			<div class="grid xl:grid-cols-2 grid-cols-1 gap-4">
				{#each schedules as s (s.id)}
					<div class="flex flex-col sm:flex-row sm:items-center gap-3 justify-between rounded-lg border border-border bg-card p-4 hover:bg-accent/30 transition-colors">

						<!-- Left: icon + info -->
						<div class="flex items-start gap-3 min-w-0 flex-1">
							<div class="shrink-0 w-9 h-9 rounded-full bg-muted flex items-center justify-center">
								<Clock class="w-4 h-4 text-muted-foreground" />
							</div>
							<div class="space-y-1.5 min-w-0">
								<div class="flex items-center gap-2 flex-wrap">
									<span class="text-sm font-semibold">{s.name}</span>
									<Badge variant={s.enabled ? 'default' : 'secondary'} class="text-[10px] px-1.5 py-0 h-4">
										{s.enabled ? 'Enabled' : 'Disabled'}
									</Badge>
								</div>
								{#if s.description}
									<p class="text-xs text-muted-foreground line-clamp-1">{s.description}</p>
								{/if}
								<div class="flex items-center gap-2 flex-wrap">
									<Badge variant="outline" class="font-mono text-[10px] bg-transparent">{s.cron}</Badge>
									<span class="text-muted-foreground/40 text-xs">•</span>
									<Badge variant="outline" class="font-mono text-[10px] bg-transparent">{s.shellType}</Badge>
								</div>
								{#if s.command}
									<div class="flex items-start gap-1.5">
										<Terminal class="w-3 h-3 text-muted-foreground/50 shrink-0 mt-0.5" />
										<code class="font-mono text-[10px] text-muted-foreground/70 break-all">{s.command}</code>
									</div>
								{/if}
								<div class="flex items-center gap-2 text-[10px] text-muted-foreground/50 pt-0.5">
									<span>Last: {s.lastRun}</span>
									<span>•</span>
									<span>Next: {s.nextRun}</span>
								</div>
							</div>
						</div>

						<!-- Right: actions -->
						<div class="flex items-center gap-1 shrink-0 self-end sm:self-center">
							<Switch checked={s.enabled} onCheckedChange={() => (schedules = schedules.map(x => x.id === s.id ? { ...x, enabled: !x.enabled } : x))} />
							<button
								class="p-1.5 rounded-md text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
								onclick={() => runNow(s.id)} disabled={runningId === s.id} title="Run now"
							>
								{#if runningId === s.id}<Loader2 class="w-4 h-4 animate-spin" />{:else}<Play class="w-4 h-4" />{/if}
							</button>
							<button
								class="p-1.5 rounded-md text-muted-foreground hover:text-blue-400 hover:bg-blue-500/10 transition-colors"
								onclick={() => openEdit(s)} title="Edit"
							>
								<PenLine class="w-4 h-4" />
							</button>
							<button
								class="p-1.5 rounded-md text-muted-foreground hover:text-destructive hover:bg-destructive/10 transition-colors"
								onclick={() => (confirmDeleteId = s.id)} disabled={deletingId === s.id} title="Delete"
							>
								{#if deletingId === s.id}<Loader2 class="w-4 h-4 animate-spin" />{:else}<Trash2 class="w-4 h-4" />{/if}
							</button>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</main>
</PageLayout>

<!-- ─── Create / Edit Modal ───────────────────────────────────────────────── -->
{#if showModal}
	<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1" onclick={closeModal} onkeydown={() => {}}></div>
	<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
		<form onsubmit={submitModal} class="bg-card border border-border rounded-xl shadow-2xl w-full max-w-lg flex flex-col gap-4 p-6 max-h-[90vh] overflow-y-auto pointer-events-auto">

			<!-- Modal header -->
			<div class="flex items-start justify-between gap-3">
				<div>
					<h2 class="text-base font-semibold">{editingId !== null ? 'Edit' : 'Create'} Schedule</h2>
					<p class="text-xs text-muted-foreground mt-0.5">
						{editingId !== null ? 'Update the schedule settings' : 'Set up a new automated task'}
					</p>
				</div>
				<button type="button" onclick={closeModal} class="text-muted-foreground hover:text-foreground p-1 rounded hover:bg-accent">✕</button>
			</div>

			<!-- Name -->
			<div class="flex flex-col gap-1.5">
				<Label for="m-name" class="text-xs text-muted-foreground">Name <span class="text-destructive">*</span></Label>
				<Input id="m-name" placeholder="Daily Database Backup" bind:value={fName} required />
			</div>

			<!-- Description -->
			<div class="flex flex-col gap-1.5">
				<Label for="m-desc" class="text-xs text-muted-foreground">Description</Label>
				<Input id="m-desc" placeholder="Optional description" bind:value={fDesc} />
			</div>

			<!-- Cron preset picker -->
			<div class="flex flex-col gap-1.5">
				<Label class="text-xs text-muted-foreground">Schedule <span class="text-destructive">*</span></Label>
				<Select.Root type="single" value={fPreset} onValueChange={handlePreset}>
					<Select.Trigger class="w-full">
						<span class="text-sm">{cronPresets.find(p => p.value === fPreset)?.label ?? 'Custom'}</span>
					</Select.Trigger>
					<Select.Content>
						{#each cronPresets as p (p.value)}
							<Select.Item value={p.value}>{p.label}{p.value !== 'custom' ? ` — ${p.value}` : ''}</Select.Item>
						{/each}
					</Select.Content>
				</Select.Root>
				<Input placeholder="e.g. 0 0 * * *" value={fCron} oninput={handleCronInput} class="font-mono text-sm" />
				<p class="text-[10px] text-muted-foreground">Cron format: minute hour day month weekday</p>
			</div>

			<!-- Shell + Command side by side -->
			<div class="grid grid-cols-3 gap-3">
				<div class="flex flex-col gap-1.5">
					<Label class="text-xs text-muted-foreground">Shell</Label>
					<Select.Root type="single" value={fShell} onValueChange={(v) => (fShell = (v ?? 'bash') as 'bash' | 'sh')}>
						<Select.Trigger><span class="text-sm font-mono">{fShell}</span></Select.Trigger>
						<Select.Content>
							<Select.Item value="bash">bash</Select.Item>
							<Select.Item value="sh">sh</Select.Item>
						</Select.Content>
					</Select.Root>
				</div>
				<div class="col-span-2 flex flex-col gap-1.5">
					<Label for="m-cmd" class="text-xs text-muted-foreground">Command <span class="text-destructive">*</span></Label>
					<Input id="m-cmd" placeholder="npm run backup" bind:value={fCommand} class="font-mono text-sm" />
				</div>
			</div>

			<!-- Service -->
			<div class="flex flex-col gap-1.5">
				<Label for="m-svc" class="text-xs text-muted-foreground">Service <span class="text-muted-foreground/50">(optional)</span></Label>
				<Input id="m-svc" placeholder="my-service-name" bind:value={fService} />
			</div>

			<!-- Enabled -->
			<div class="flex items-center justify-between rounded-lg border border-border px-3 py-2.5">
				<div>
					<p class="text-sm font-medium">Enabled</p>
					<p class="text-[10px] text-muted-foreground">Run automatically on schedule</p>
				</div>
				<Switch bind:checked={fEnabled} />
			</div>

			{#if modalError}
				<p class="text-xs text-destructive bg-destructive/10 border border-destructive/20 rounded px-3 py-2">{modalError}</p>
			{/if}

			<!-- Submit -->
			<div class="flex justify-end gap-2 pt-1">
				<Button type="button" variant="outline" size="sm" onclick={closeModal}>Cancel</Button>
				<Button type="submit" size="sm" disabled={saving} class="min-w-[110px] gap-1.5">
					{#if saving}<Loader2 class="w-3.5 h-3.5 animate-spin" /> Saving…
					{:else}{editingId !== null ? 'Update' : 'Create'} Schedule{/if}
				</Button>
			</div>
		</form>
	</div>
{/if}

<!-- ─── Delete Confirmation ───────────────────────────────────────────────── -->
{#if confirmDeleteId !== null}
	{@const target = schedules.find(s => s.id === confirmDeleteId)}
	<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1" onclick={() => (confirmDeleteId = null)} onkeydown={() => {}}></div>
	<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
		<div class="bg-card border border-border rounded-xl shadow-2xl w-full max-w-sm p-6 pointer-events-auto flex flex-col gap-4">
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 rounded-full bg-destructive/10 flex items-center justify-center shrink-0">
					<Trash2 class="w-5 h-5 text-destructive" />
				</div>
				<div>
					<h2 class="text-sm font-semibold">Delete Schedule</h2>
					<p class="text-xs text-muted-foreground mt-0.5">
						Delete <strong class="text-foreground">{target?.name}</strong>? This cannot be undone.
					</p>
				</div>
			</div>
			<div class="flex justify-end gap-2">
				<Button variant="outline" size="sm" onclick={() => (confirmDeleteId = null)}>Cancel</Button>
				<Button variant="destructive" size="sm" onclick={() => deleteSchedule(confirmDeleteId!)} disabled={deletingId !== null} class="gap-1.5">
					{#if deletingId !== null}<Loader2 class="w-3.5 h-3.5 animate-spin" /> Deleting…
					{:else}<Trash2 class="w-3.5 h-3.5" /> Delete{/if}
				</Button>
			</div>
		</div>
	</div>
{/if}
