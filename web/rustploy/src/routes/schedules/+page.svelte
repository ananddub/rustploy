<script lang="ts">
	import { goto } from '$app/navigation';
	import { Calendar, Plus, Trash2, Play, Clock, Terminal, PenLine, Loader2 } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import { USE_MOCK_DATA, getSchedulesMock, type ScheduleMock } from '$lib/mocks';
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

	let useMock = $state(USE_MOCK_DATA);
	let mockSchedules = $state<ScheduleMock[]>(getSchedulesMock());

	let schedules = $derived<Schedule[]>(
		mockSchedules.map((s, idx) => ({
			id: idx + 1,
			name: s.name,
			description: `${s.targetProject} · ${s.targetService}`,
			cron: s.cronExpression,
			shellType: 'bash',
			command: s.command,
			service: s.targetService,
			enabled: s.status === 'active',
			lastRun: s.lastRun,
			nextRun: s.nextRun
		}))
	);

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

	// Modal & state handling
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

	let confirmDeleteId = $state<number | null>(null);
	let deletingId      = $state<number | null>(null);
	let runningId       = $state<number | null>(null);

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

	async function runNow(id: number) {
		runningId = id;
		setTimeout(() => {
			toastSuccess('Schedule triggered successfully');
			runningId = null;
		}, 800);
	}
</script>

<PageLayout>
	<!-- Page header -->
	<header class="flex items-center justify-between px-6 py-3 border-b border-[#262626] text-xs bg-[#0A0A0A] shrink-0">
		<div class="flex items-center gap-2">
			<Calendar class="w-3.5 h-3.5 text-[#a1a1aa]" />
			<span class="font-medium text-[#FAFAFA]">Schedules</span>
		</div>

		<!-- Mock Data Dev Toggle Switch -->
		<div class="flex items-center gap-2 px-3 py-1 rounded-full bg-[#141414] border border-[#262626]">
			<span class="text-[11px] text-[#a1a1aa]">Data Source:</span>
			<button
				onclick={() => (useMock = !useMock)}
				class="text-[11px] font-semibold px-2 py-0.5 rounded transition-colors {useMock
					? 'bg-[#262626] text-[#FAFAFA] border border-white/10'
					: 'text-[#737373] hover:text-[#FAFAFA]'}"
			>
				{useMock ? 'Mock Demo Data' : 'Live Rust Backend API'}
			</button>
		</div>
	</header>

	<main class="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up flex flex-col min-h-0 bg-[#171717] border border-[#262626] rounded-2xl shadow-md space-y-6">
		<div class="flex items-center justify-between">
			<div>
				<h1 class="text-3xl font-bold tracking-tight text-[#FAFAFA]">Scheduled Tasks</h1>
				<p class="text-sm text-[#a1a1aa] mt-1">Schedule cron commands to execute automatically against your services</p>
			</div>
			<Button size="sm" class="gap-1.5 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A]" onclick={openCreate}>
				<Plus class="w-3.5 h-3.5" /> Add Schedule
			</Button>
		</div>

		{#if schedules.length === 0}
			<div class="flex flex-col items-center justify-center py-20 text-[#a1a1aa] text-center">
				<Clock class="w-12 h-12 mb-3 text-[#525252]" />
				<p class="text-base font-semibold text-[#FAFAFA]">No scheduled tasks configured</p>
				<p class="text-xs text-[#a1a1aa] mt-1">Create automated backups, cleanup scripts, or API pingers.</p>
				<Button size="sm" class="gap-1.5 mt-4 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A]" onclick={openCreate}>
					<Plus class="w-4 h-4" /> Add Schedule
				</Button>
			</div>
		{:else}
			<div class="grid xl:grid-cols-2 grid-cols-1 gap-4">
				{#each schedules as s (s.id)}
					<div class="flex flex-col sm:flex-row sm:items-center gap-4 justify-between rounded-xl border border-[#262626] bg-[#171717] p-5 hover:border-[#3f3f46] transition-all">
						<div class="flex items-start gap-3.5 min-w-0 flex-1">
							<div class="shrink-0 w-9 h-9 rounded-lg bg-[#262626] border border-white/10 flex items-center justify-center">
								<Clock class="w-4 h-4 text-[#FAFAFA]" />
							</div>
							<div class="space-y-1.5 min-w-0">
								<div class="flex items-center gap-2 flex-wrap">
									<span class="text-sm font-semibold text-[#FAFAFA]">{s.name}</span>
									<Badge variant="outline" class="text-[10px] px-2 py-0.5 {s.enabled ? 'border-green-500/30 text-green-400 bg-green-500/10' : 'border-[#262626] text-[#737373]'}">
										{s.enabled ? 'Enabled' : 'Disabled'}
									</Badge>
								</div>
								<p class="text-xs text-[#a1a1aa]">{s.description}</p>
								<div class="flex items-center gap-2 flex-wrap font-mono text-[11px] text-[#a1a1aa]">
									<span class="px-2 py-0.5 rounded border border-[#262626] bg-[#141414]">{s.cron}</span>
									<span class="px-2 py-0.5 rounded border border-[#262626] bg-[#141414]">{s.shellType}</span>
								</div>
								<div class="flex items-start gap-1.5 pt-1">
									<Terminal class="w-3.5 h-3.5 text-[#737373] shrink-0 mt-0.5" />
									<code class="font-mono text-[11px] text-[#a1a1aa] break-all">{s.command}</code>
								</div>
								<div class="flex items-center gap-3 text-[11px] font-mono text-[#737373] pt-1">
									<span>Last: {s.lastRun}</span>
									<span>·</span>
									<span>Next: {s.nextRun}</span>
								</div>
							</div>
						</div>

						<div class="flex items-center gap-2 shrink-0 self-end sm:self-center">
							<button
								class="p-2 rounded-lg border border-[#262626] bg-[#262626] text-[#a1a1aa] hover:text-[#FAFAFA] hover:bg-[#333333] transition-colors"
								onclick={() => runNow(s.id)} disabled={runningId === s.id} title="Run now"
							>
								{#if runningId === s.id}<Loader2 class="w-4 h-4 animate-spin" />{:else}<Play class="w-4 h-4" />{/if}
							</button>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</main>
</PageLayout>
