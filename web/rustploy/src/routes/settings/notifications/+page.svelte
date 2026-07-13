<script lang="ts">
	import { goto } from '$app/navigation';
	import { Bell, Plus, Trash2, PenBox, Loader2, Mail } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Select from '$lib/components/ui/select';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { toastSuccess, toastError } from '$lib/toast';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	// ─── Notification type config ─────────────────────────────────────────────────
	type NotifType = 'slack'|'telegram'|'discord'|'email'|'resend'|'gotify'|'ntfy'|'mattermost'|'lark'|'teams'|'custom';

	const TYPE_CONFIG: Record<NotifType, { label: string; color: string }> = {
		slack:      { label:'Slack',       color:'#4A154B' },
		telegram:   { label:'Telegram',    color:'#229ED9' },
		discord:    { label:'Discord',     color:'#5865F2' },
		email:      { label:'Email',       color:'#6366f1' },
		resend:     { label:'Resend',      color:'#000000' },
		gotify:     { label:'Gotify',      color:'#52b788' },
		ntfy:       { label:'ntfy',        color:'#338ad2' },
		mattermost: { label:'Mattermost',  color:'#0058CC' },
		lark:       { label:'Lark',        color:'#3370ff' },
		teams:      { label:'Teams',       color:'#5059C9' },
		custom:     { label:'Custom',      color:'#94a3b8' },
	};

	// SVG icons per type (simplified but recognizable)
	function typeIcon(t: NotifType): string {
		const icons: Record<NotifType, string> = {
			slack:      `<rect width="3" height="3" x="4" y="4" rx="1"/><rect width="3" height="3" x="4" y="10.5" rx="1"/><rect width="3" height="3" x="10.5" y="4" rx="1"/><rect width="3" height="3" x="10.5" y="10.5" rx="1"/>`,
			telegram:   `<path d="m22 2-7 20-4-9-9-4Z"/><path d="M22 2 11 13"/>`,
			discord:    `<circle cx="9" cy="12" r="1"/><circle cx="15" cy="12" r="1"/><path d="M7.5 7.5c3.5-1 5.5-1 9 0"/>`,
			email:      `<rect width="20" height="16" x="2" y="4" rx="2"/><path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"/>`,
			resend:     `<path d="M3 3h18v18H3z"/><path d="M9 9h6v6H9z"/>`,
			gotify:     `<path d="M18 8h1a4 4 0 0 1 0 8h-1"/><path d="M2 8h16v9a4 4 0 0 1-4 4H6a4 4 0 0 1-4-4V8z"/><line x1="6" x2="6" y1="1" y2="4"/><line x1="10" x2="10" y1="1" y2="4"/><line x1="14" x2="14" y1="1" y2="4"/>`,
			ntfy:       `<path d="M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9"/><path d="M10.3 21a1.94 1.94 0 0 0 3.4 0"/>`,
			mattermost: `<circle cx="12" cy="12" r="10"/><path d="M8 12h8"/><path d="M12 8v8"/>`,
			lark:       `<path d="M12 2L2 7l10 5 10-5-10-5z"/><path d="m2 17 10 5 10-5"/><path d="m2 12 10 5 10-5"/>`,
			teams:      `<path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/>`,
			custom:     `<path d="m12 3-1.912 5.813a2 2 0 0 1-1.275 1.275L3 12l5.813 1.912a2 2 0 0 1 1.275 1.275L12 21l1.912-5.813a2 2 0 0 1 1.275-1.275L21 12l-5.813-1.912a2 2 0 0 1-1.275-1.275L12 3z"/>`,
		};
		return icons[t] ?? icons.custom;
	}

	type Notification = { id: string; name: string; type: NotifType; createdAt: string; };

	let notifications = $state<Notification[]>([
		{ id:'n1', name:'Deploy Alerts Slack',  type:'slack',     createdAt:'2026-06-15T10:00:00Z' },
		{ id:'n2', name:'Admin Email Alerts',   type:'email',     createdAt:'2026-07-01T08:00:00Z' },
		{ id:'n3', name:'Discord Ops Channel',  type:'discord',   createdAt:'2026-07-05T12:00:00Z' },
		{ id:'n4', name:'Telegram Bot Alerts',  type:'telegram',  createdAt:'2026-07-08T09:00:00Z' },
		{ id:'n5', name:'Teams Notifications',  type:'teams',     createdAt:'2026-07-10T14:00:00Z' },
	]);

	function fmtDate(iso: string) {
		return new Date(iso).toLocaleDateString(undefined, { year:'numeric', month:'short', day:'numeric' });
	}

	// ─── Modal state ──────────────────────────────────────────────────────────────
	let showModal  = $state(false);
	let editingId  = $state<string|null>(null);
	let notifType  = $state<NotifType>('slack');
	let saving     = $state(false);
	let modalError = $state('');

	// Common
	let fName = $state('');
	// Slack / Discord / Mattermost / Lark / Teams
	let fWebhookUrl = $state('');
	let fChannel    = $state('');
	// Telegram
	let fBotToken   = $state('');
	let fChatId     = $state('');
	let fThreadId   = $state('');
	// Email
	let fSmtp       = $state('');
	let fPort       = $state('587');
	let fUser       = $state('');
	let fPass       = $state('');
	let fFrom       = $state('');
	let fTo         = $state('');
	// Resend
	let fApiKey     = $state('');
	let fResendFrom = $state('');
	let fResendTo   = $state('');
	// Gotify
	let fServerUrl  = $state('');
	let fAppToken   = $state('');
	// ntfy
	let fTopic      = $state('');
	let fAccessToken= $state('');
	// Custom
	let fEndpoint   = $state('');

	function openCreate() {
		editingId=null; fName=''; fWebhookUrl=''; fChannel=''; fBotToken=''; fChatId='';
		fThreadId=''; fSmtp=''; fPort='587'; fUser=''; fPass=''; fFrom=''; fTo='';
		fApiKey=''; fResendFrom=''; fResendTo=''; fServerUrl=''; fAppToken='';
		fTopic=''; fAccessToken=''; fEndpoint=''; modalError=''; notifType='slack';
		showModal=true;
	}

	function openEdit(n: Notification) {
		editingId=n.id; fName=n.name; notifType=n.type; modalError='';
		showModal=true;
	}

	async function submitModal(e: SubmitEvent) {
		e.preventDefault(); modalError='';
		if (!fName.trim()) { modalError='Name is required'; return; }
		saving=true;
		try {
			await new Promise(r => setTimeout(r, 500));
			if (editingId) {
				notifications = notifications.map(n => n.id===editingId ? {...n, name:fName.trim(), type:notifType} : n);
				toastSuccess('Notification updated');
			} else {
				notifications = [...notifications, {id:`n${Date.now()}`, name:fName.trim(), type:notifType, createdAt:new Date().toISOString()}];
				toastSuccess('Notification created');
			}
			showModal=false; editingId=null;
		} catch { modalError='Failed to save'; }
		finally { saving=false; }
	}

	// ─── Delete ───────────────────────────────────────────────────────────────────
	let confirmDeleteId = $state<string|null>(null);
	let deletingId      = $state<string|null>(null);

	async function deleteNotification(id: string) {
		deletingId=id;
		await new Promise(r => setTimeout(r, 400));
		notifications = notifications.filter(n => n.id !== id);
		deletingId=null; confirmDeleteId=null;
		toastSuccess('Notification deleted');
	}
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<Bell class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Notifications</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto">
		<div class="w-full">
			<div class="rounded-xl border border-border bg-card">

				<div class="px-6 pt-5 pb-4 border-b border-border">
					<h2 class="text-xl font-bold flex items-center gap-2">
						<Bell class="w-5 h-5 text-muted-foreground" /> Notifications
					</h2>
					<p class="text-sm text-muted-foreground mt-0.5">
						Add your providers to receive notifications, like Discord, Slack, Telegram, Teams, Email, Resend, Lark.
					</p>
				</div>

				<div class="px-6 py-5">
					{#if notifications.length === 0}
						<div class="flex flex-col items-center gap-3 min-h-[25vh] justify-center text-muted-foreground">
							<Bell class="w-8 h-8 opacity-40" />
							<p class="text-base text-center">To send notifications it is required to set at least 1 provider.</p>
							<Button size="sm" class="gap-1.5 mt-1" onclick={openCreate}>
								<Plus class="w-4 h-4" /> Add Notification
							</Button>
						</div>
					{:else}
						<div class="flex flex-col gap-4 min-h-[25vh]">
							<div class="flex flex-col gap-4">
								{#each notifications as n (n.id)}
									<div class="bg-muted/40 p-1 rounded-lg">
										<div class="flex items-center justify-between p-4 rounded-lg bg-background border border-border">
											<!-- Left: icon + name -->
											<span class="text-sm font-medium flex items-center gap-4">
												<!-- Provider icon circle -->
												<div class="w-8 h-8 rounded-full flex items-center justify-center shrink-0"
													style="background:{TYPE_CONFIG[n.type].color}22">
													<svg class="w-5 h-5" viewBox="0 0 24 24" fill="none"
														stroke={TYPE_CONFIG[n.type].color}
														stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
														{@html typeIcon(n.type)}
													</svg>
												</div>
												{n.name}
											</span>
											<!-- Right: actions -->
											<div class="flex items-center gap-1 shrink-0">
												<Button variant="ghost" size="icon" class="h-8 w-8" title="Edit"
													onclick={() => openEdit(n)}>
													<PenBox class="w-4 h-4" />
												</Button>
												<Button variant="ghost" size="icon"
													class="h-8 w-8 group hover:bg-red-500/10"
													onclick={() => (confirmDeleteId = n.id)}
													disabled={deletingId === n.id} title="Delete">
													{#if deletingId === n.id}
														<Loader2 class="w-4 h-4 animate-spin" />
													{:else}
														<Trash2 class="w-4 h-4 text-muted-foreground group-hover:text-red-500 transition-colors" />
													{/if}
												</Button>
											</div>
										</div>
									</div>
								{/each}
							</div>
							<div class="flex justify-end">
								<Button size="sm" class="gap-1.5" onclick={openCreate}>
									<Plus class="w-4 h-4" /> Add Notification
								</Button>
							</div>
						</div>
					{/if}
				</div>
			</div>
		</div>
	</main>
</PageLayout>

<!-- ── Add / Edit Modal ───────────────────────────────────────────────────── -->
{#if showModal}
	<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1"
		onclick={() => { showModal=false; editingId=null; }} onkeydown={() => {}}></div>
	<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
		<form onsubmit={submitModal}
			class="bg-card border border-border rounded-xl shadow-2xl w-full max-w-lg pointer-events-auto flex flex-col gap-4 p-6 max-h-[90vh] overflow-y-auto">

			<div class="flex items-start justify-between">
				<div>
					<h2 class="text-base font-semibold">{editingId ? 'Edit' : 'Add'} Notification</h2>
					<p class="text-xs text-muted-foreground mt-0.5">{editingId ? 'Update notification provider' : 'Add a new notification provider'}</p>
				</div>
				<button type="button" onclick={() => { showModal=false; editingId=null; }}
					class="text-muted-foreground hover:text-foreground p-1 rounded hover:bg-accent">✕</button>
			</div>

			<!-- Name -->
			<div class="space-y-1.5">
				<Label for="n-name" class="text-xs">Name <span class="text-destructive">*</span></Label>
				<Input id="n-name" bind:value={fName} placeholder="e.g. Deploy Alerts" required />
			</div>

			<!-- Type selector -->
			<div class="space-y-1.5">
				<Label class="text-xs">Provider <span class="text-destructive">*</span></Label>
				<Select.Root type="single" value={notifType} onValueChange={(v) => (notifType = (v??'slack') as NotifType)}>
					<Select.Trigger class="w-full">
						<span class="text-sm">{TYPE_CONFIG[notifType].label}</span>
					</Select.Trigger>
					<Select.Content>
						{#each Object.entries(TYPE_CONFIG) as [key, cfg] (key)}
							<Select.Item value={key}>{cfg.label}</Select.Item>
						{/each}
					</Select.Content>
				</Select.Root>
			</div>

			<!-- Dynamic fields per type -->
			{#if notifType === 'slack' || notifType === 'discord' || notifType === 'lark' || notifType === 'teams'}
				<div class="space-y-1.5">
					<Label for="n-wh" class="text-xs">Webhook URL <span class="text-destructive">*</span></Label>
					<Input id="n-wh" bind:value={fWebhookUrl} placeholder="https://hooks.slack.com/services/..." required />
				</div>
				{#if notifType === 'slack' || notifType === 'mattermost'}
					<div class="space-y-1.5">
						<Label for="n-ch" class="text-xs">Channel <span class="text-muted-foreground">(optional)</span></Label>
						<Input id="n-ch" bind:value={fChannel} placeholder="#deployments" />
					</div>
				{/if}

			{:else if notifType === 'mattermost'}
				<div class="space-y-1.5">
					<Label for="n-wh2" class="text-xs">Webhook URL <span class="text-destructive">*</span></Label>
					<Input id="n-wh2" bind:value={fWebhookUrl} placeholder="https://mattermost.example.com/hooks/..." required />
				</div>
				<div class="space-y-1.5">
					<Label for="n-ch2" class="text-xs">Channel <span class="text-muted-foreground">(optional)</span></Label>
					<Input id="n-ch2" bind:value={fChannel} placeholder="town-square" />
				</div>

			{:else if notifType === 'telegram'}
				<div class="space-y-1.5">
					<Label for="n-bot" class="text-xs">Bot Token <span class="text-destructive">*</span></Label>
					<Input id="n-bot" bind:value={fBotToken} placeholder="123456:ABC-DEF..." required />
				</div>
				<div class="grid grid-cols-2 gap-3">
					<div class="space-y-1.5">
						<Label for="n-chat" class="text-xs">Chat ID <span class="text-destructive">*</span></Label>
						<Input id="n-chat" bind:value={fChatId} placeholder="-100123456789" required />
					</div>
					<div class="space-y-1.5">
						<Label for="n-thread" class="text-xs">Message Thread ID <span class="text-muted-foreground">(optional)</span></Label>
						<Input id="n-thread" bind:value={fThreadId} placeholder="Thread ID" />
					</div>
				</div>

			{:else if notifType === 'email'}
				<div class="grid grid-cols-2 gap-3">
					<div class="space-y-1.5">
						<Label for="n-smtp" class="text-xs">SMTP Server <span class="text-destructive">*</span></Label>
						<Input id="n-smtp" bind:value={fSmtp} placeholder="smtp.gmail.com" required />
					</div>
					<div class="space-y-1.5">
						<Label for="n-port" class="text-xs">Port</Label>
						<Input id="n-port" bind:value={fPort} type="number" placeholder="587" />
					</div>
				</div>
				<div class="grid grid-cols-2 gap-3">
					<div class="space-y-1.5">
						<Label for="n-eu" class="text-xs">Username <span class="text-destructive">*</span></Label>
						<Input id="n-eu" bind:value={fUser} placeholder="user@example.com" required />
					</div>
					<div class="space-y-1.5">
						<Label for="n-ep" class="text-xs">Password <span class="text-destructive">*</span></Label>
						<Input id="n-ep" type="password" bind:value={fPass} placeholder="Password" required />
					</div>
				</div>
				<div class="space-y-1.5">
					<Label for="n-from" class="text-xs">From Address <span class="text-destructive">*</span></Label>
					<Input id="n-from" bind:value={fFrom} placeholder="alerts@example.com" required />
				</div>
				<div class="space-y-1.5">
					<Label for="n-to" class="text-xs">To Addresses <span class="text-destructive">*</span></Label>
					<Input id="n-to" bind:value={fTo} placeholder="admin@example.com, ops@example.com" required />
					<p class="text-[10px] text-muted-foreground">Comma-separated email addresses</p>
				</div>

			{:else if notifType === 'resend'}
				<div class="space-y-1.5">
					<Label for="n-rk" class="text-xs">API Key <span class="text-destructive">*</span></Label>
					<Input id="n-rk" bind:value={fApiKey} placeholder="re_..." required />
				</div>
				<div class="space-y-1.5">
					<Label for="n-rf" class="text-xs">From Address <span class="text-destructive">*</span></Label>
					<Input id="n-rf" bind:value={fResendFrom} placeholder="alerts@example.com" required />
				</div>
				<div class="space-y-1.5">
					<Label for="n-rt" class="text-xs">To Addresses <span class="text-destructive">*</span></Label>
					<Input id="n-rt" bind:value={fResendTo} placeholder="admin@example.com" required />
				</div>

			{:else if notifType === 'gotify'}
				<div class="space-y-1.5">
					<Label for="n-gs" class="text-xs">Server URL <span class="text-destructive">*</span></Label>
					<Input id="n-gs" bind:value={fServerUrl} placeholder="https://gotify.example.com" required />
				</div>
				<div class="space-y-1.5">
					<Label for="n-ga" class="text-xs">App Token <span class="text-destructive">*</span></Label>
					<Input id="n-ga" bind:value={fAppToken} placeholder="App token" required />
				</div>

			{:else if notifType === 'ntfy'}
				<div class="space-y-1.5">
					<Label for="n-ns" class="text-xs">Server URL <span class="text-destructive">*</span></Label>
					<Input id="n-ns" bind:value={fServerUrl} placeholder="https://ntfy.sh" required />
				</div>
				<div class="space-y-1.5">
					<Label for="n-nt" class="text-xs">Topic <span class="text-destructive">*</span></Label>
					<Input id="n-nt" bind:value={fTopic} placeholder="my-alerts" required />
				</div>
				<div class="space-y-1.5">
					<Label for="n-nat" class="text-xs">Access Token <span class="text-muted-foreground">(optional)</span></Label>
					<Input id="n-nat" bind:value={fAccessToken} placeholder="tk_..." />
				</div>

			{:else if notifType === 'custom'}
				<div class="space-y-1.5">
					<Label for="n-ep2" class="text-xs">Endpoint URL <span class="text-destructive">*</span></Label>
					<Input id="n-ep2" bind:value={fEndpoint} placeholder="https://example.com/webhook" required />
				</div>
			{/if}

			{#if modalError}
				<p class="text-xs text-destructive bg-destructive/10 border border-destructive/20 rounded px-3 py-2">{modalError}</p>
			{/if}

			<div class="flex justify-end gap-2 pt-1">
				<Button type="button" variant="outline" size="sm" onclick={() => { showModal=false; editingId=null; }}>Cancel</Button>
				<Button type="submit" size="sm" disabled={saving} class="gap-1.5 min-w-[80px]">
					{#if saving}<Loader2 class="w-3.5 h-3.5 animate-spin" />{/if}
					{editingId ? 'Update' : 'Add'}
				</Button>
			</div>
		</form>
	</div>
{/if}

<!-- ── Delete Confirm ─────────────────────────────────────────────────────── -->
{#if confirmDeleteId}
	{@const target = notifications.find(n => n.id === confirmDeleteId)}
	<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1"
		onclick={() => (confirmDeleteId=null)} onkeydown={() => {}}></div>
	<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
		<div class="bg-card border border-border rounded-xl shadow-2xl w-full max-w-sm p-6 pointer-events-auto flex flex-col gap-4">
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 rounded-full bg-destructive/10 flex items-center justify-center shrink-0">
					<Trash2 class="w-5 h-5 text-destructive" />
				</div>
				<div>
					<h2 class="text-sm font-semibold">Delete Notification</h2>
					<p class="text-xs text-muted-foreground mt-0.5">
						Are you sure you want to delete <strong class="text-foreground">{target?.name}</strong>?
						This cannot be undone.
					</p>
				</div>
			</div>
			<div class="flex justify-end gap-2">
				<Button variant="outline" size="sm" onclick={() => (confirmDeleteId=null)}>Cancel</Button>
				<Button variant="destructive" size="sm"
					onclick={() => deleteNotification(confirmDeleteId!)}
					disabled={deletingId !== null} class="gap-1.5">
					{#if deletingId}<Loader2 class="w-3.5 h-3.5 animate-spin" /> Deleting…
					{:else}<Trash2 class="w-3.5 h-3.5" /> Delete{/if}
				</Button>
			</div>
		</div>
	</div>
{/if}
