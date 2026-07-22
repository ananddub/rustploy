<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { getAuthSession } from '$lib/auth';
	import {
		databaseControllerGet,
		databaseControllerPatch,
		databaseControllerDeploy,
		databaseControllerStart,
		databaseControllerStop,
		databaseControllerRedeploy,
		databaseControllerDelete
	} from '$lib/client/sdk.gen';
	import type { DatabaseResponseDto } from '$lib/client/types.gen';
	import ServicePageShell from '$lib/components/ServicePageShell.svelte';
	import { RocketIcon, Play, Square, Hammer, Trash2, Save, AlertTriangle } from '@lucide/svelte';
	import { withToast } from '$lib/toast';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	const dbId = $derived(parseInt(page.params.dbId ?? '0'));
	const kind = $derived(page.params.kind ?? '');
	const projectId = $derived(page.params.id ?? '');

	const TABS = ['General', 'Logs', 'Monitoring', 'Advanced'] as const;

	let activeTab = $state('General');
	let db = $state<DatabaseResponseDto | null>(null);
	let loading = $state(true);

	$effect(() => {
		databaseControllerGet({ path: { kind, id: dbId } }).then((res: { data?: DatabaseResponseDto | null }) => {
			db = res.data ?? null;
			loading = false;
		});
	});

	// Form fields — synced once db loads
	let formName = $state('');
	let formDesc = $state('');
	let formImage = $state('');
	let formDbName = $state('');
	let formDbUser = $state('');
	let formExtPort = $state('');
	let generalSaving = $state(false);
	let generalError = $state('');
	let generalSaved = $state(false);
	let synced = $state(false);

	$effect(() => {
		if (db && !synced) {
			formName = db.name ?? '';
			formDesc = db.description ?? '';
			formImage = db.docker_image ?? '';
			formDbName = db.database_name ?? '';
			formDbUser = db.database_user ?? '';
			formExtPort = db.external_port ? String(db.external_port) : '';
			synced = true;
		}
	});

	async function saveGeneral() {
		generalSaving = true; generalError = ''; generalSaved = false;
		try {
			const res = await databaseControllerPatch({
				path: { kind, id: dbId },
				body: {
					name: formName || undefined,
					description: formDesc || undefined,
					docker_image: formImage || undefined,
					external_port: formExtPort ? parseInt(formExtPort) : undefined
				}
			});
			if (res.data) {
				db = res.data as DatabaseResponseDto;
				generalSaved = true;
				setTimeout(() => (generalSaved = false), 2000);
			}
		} catch (e: any) { generalError = e?.message ?? 'Failed to save'; }
		finally { generalSaving = false; }
	}

	// Action buttons
	let deploying = $state(false);
	let starting = $state(false);
	let stopping = $state(false);
	let redeploying = $state(false);

	async function run(setter: (v: boolean) => void, fn: () => Promise<any>, label = 'Action') {
		setter(true);
		try {
			await withToast(fn, { loading: label + 'ing…', success: label + ' triggered!' });
		} finally { setter(false); }
	}

	// Delete
	let confirmDelete = $state(false);
	let deleting = $state(false);
	let deleteError = $state('');

	async function doDelete() {
		deleting = true; deleteError = '';
		try {
			await databaseControllerDelete({ path: { kind, id: dbId } });
			goto(`/projects/${projectId}`);
		} catch (e: any) { deleteError = e?.message ?? 'Failed'; deleting = false; }
	}

	const inputCls = 'flex h-9 w-full rounded-md border border-input bg-secondary px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring';

	function statusDot(status: string) {
		switch (status?.toLowerCase()) {
			case 'running': return 'bg-green-500';
			case 'stopped': case 'exited': return 'bg-red-500';
			default: return 'bg-muted-foreground/40';
		}
	}
</script>

<ServicePageShell
	{projectId}
	name={db?.name ?? ''}
	appName={db?.app_name ?? ''}
	tabs={TABS}
	{activeTab}
	onTabChange={(t) => (activeTab = t)}
	{loading}
>
	{#if db}

		<!-- GENERAL TAB -->
		{#if activeTab === 'General'}
			<div class="flex flex-col gap-6 animate-fade-up">

				<!-- Action bar -->
				<section class="bg-card border border-border rounded-lg p-4">
					<div class="flex flex-wrap items-center gap-2">
						<button
							onclick={() => run(d => (deploying = d), () => databaseControllerDeploy({ path: { kind, id: dbId } }), 'Deploy')}
							disabled={deploying}
							class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50 transition-colors"
						>
							{#if deploying}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>{:else}<RocketIcon size={14} />{/if}
							Deploy
						</button>
						<button
							onclick={() => run(d => (starting = d), () => databaseControllerStart({ path: { kind, id: dbId } }), 'Start')}
							disabled={starting}
							class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent disabled:opacity-50 transition-colors"
						>
							{#if starting}<div class="w-3.5 h-3.5 border-2 border-current/30 border-t-current rounded-full animate-spin"></div>{:else}<Play size={14} />{/if}
							Start
						</button>
						<button
							onclick={() => run(d => (stopping = d), () => databaseControllerStop({ path: { kind, id: dbId } }), 'Stop')}
							disabled={stopping}
							class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent disabled:opacity-50 transition-colors"
						>
							{#if stopping}<div class="w-3.5 h-3.5 border-2 border-current/30 border-t-current rounded-full animate-spin"></div>{:else}<Square size={14} />{/if}
							Stop
						</button>
						<button
							onclick={() => run(d => (redeploying = d), () => databaseControllerRedeploy({ path: { kind, id: dbId } }), 'Redeploy')}
							disabled={redeploying}
							class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent disabled:opacity-50 transition-colors"
						>
							{#if redeploying}<div class="w-3.5 h-3.5 border-2 border-current/30 border-t-current rounded-full animate-spin"></div>{:else}<Hammer size={14} />{/if}
							Redeploy
						</button>
					</div>
				</section>

				<!-- Status summary -->
				<section class="bg-card border border-border rounded-lg p-6">
					<h2 class="text-base font-semibold mb-4">Database Info</h2>
					<div class="grid grid-cols-2 md:grid-cols-4 gap-3">
						<div class="bg-secondary rounded-lg p-3">
							<p class="text-sm text-muted-foreground uppercase tracking-wide mb-1">Type</p>
							<p class="text-sm font-medium capitalize">{db.kind}</p>
						</div>
						<div class="bg-secondary rounded-lg p-3">
							<p class="text-sm text-muted-foreground uppercase tracking-wide mb-1">Status</p>
							<div class="flex items-center gap-1.5">
								<span class="w-2 h-2 rounded-full {statusDot(db.app_status)}"></span>
								<p class="text-sm font-medium">{db.app_status}</p>
							</div>
						</div>
						<div class="bg-secondary rounded-lg p-3">
							<p class="text-sm text-muted-foreground uppercase tracking-wide mb-1">Replicas</p>
							<p class="text-sm font-medium">{db.replicas}</p>
						</div>
						<div class="bg-secondary rounded-lg p-3">
							<p class="text-sm text-muted-foreground uppercase tracking-wide mb-1">App Name</p>
							<p class="text-sm font-mono truncate">{db.app_name}</p>
						</div>
						{#if db.external_port}
							<div class="bg-secondary rounded-lg p-3">
								<p class="text-sm text-muted-foreground uppercase tracking-wide mb-1">External Port</p>
								<p class="text-sm font-mono">{db.external_port}</p>
							</div>
						{/if}
						{#if db.memory_limit || db.cpu_limit}
							<div class="bg-secondary rounded-lg p-3">
								<p class="text-sm text-muted-foreground uppercase tracking-wide mb-1">Memory Limit</p>
								<p class="text-sm font-mono">{db.memory_limit ?? '—'}</p>
							</div>
							<div class="bg-secondary rounded-lg p-3">
								<p class="text-sm text-muted-foreground uppercase tracking-wide mb-1">CPU Limit</p>
								<p class="text-sm font-mono">{db.cpu_limit ?? '—'}</p>
							</div>
						{/if}
					</div>
				</section>

				<!-- Config form -->
				<section class="bg-card border border-border rounded-lg p-6">
					<h2 class="text-base font-semibold mb-1">Configuration</h2>
					<p class="text-sm text-muted-foreground mb-5">Changes apply on next deploy.</p>
					<div class="flex flex-col gap-4">
						<div class="grid grid-cols-2 gap-4">
							<div class="flex flex-col gap-1.5">
								<label for="f-name" class="text-sm font-medium text-muted-foreground">Service Name</label>
								<input id="f-name" class={inputCls} bind:value={formName} />
							</div>
							<div class="flex flex-col gap-1.5">
								<label for="f-image" class="text-sm font-medium text-muted-foreground">Docker Image</label>
								<input id="f-image" class={inputCls} bind:value={formImage} />
							</div>
						</div>
						<div class="flex flex-col gap-1.5">
							<label for="f-desc" class="text-sm font-medium text-muted-foreground">Description</label>
							<input id="f-desc" class={inputCls} placeholder="Optional" bind:value={formDesc} />
						</div>
						{#if db.kind !== 'redis' && db.kind !== 'libsql'}
							<div class="grid grid-cols-2 gap-4">
								<div class="flex flex-col gap-1.5">
									<label for="f-dbname" class="text-sm font-medium text-muted-foreground">Database Name</label>
									<input id="f-dbname" class={inputCls} bind:value={formDbName} />
								</div>
								<div class="flex flex-col gap-1.5">
									<label for="f-dbuser" class="text-sm font-medium text-muted-foreground">Database User</label>
									<input id="f-dbuser" class={inputCls} bind:value={formDbUser} />
								</div>
							</div>
						{/if}
						<div class="flex flex-col gap-1.5 w-48">
							<label for="f-port" class="text-sm font-medium text-muted-foreground">External Port</label>
							<input id="f-port" type="number" class={inputCls} placeholder="Auto-assign" bind:value={formExtPort} />
						</div>
						{#if generalError}
							<div class="rounded-md bg-destructive/10 border border-destructive/30 px-3 py-2 text-sm text-destructive">{generalError}</div>
						{/if}
						<div class="flex justify-end items-center gap-3">
							{#if generalSaved}<span class="text-sm text-green-500">Saved!</span>{/if}
							<button onclick={saveGeneral} disabled={generalSaving}
								class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50">
								{#if generalSaving}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>Saving…{:else}<Save size={13} />Save{/if}
							</button>
						</div>
					</div>
				</section>
			</div>
		{/if}

		<!-- LOGS TAB -->
		{#if activeTab === 'Logs'}
			<div class="bg-card border border-border rounded-lg p-6 flex flex-col gap-4 animate-fade-up">
				<h2 class="text-base font-semibold">Logs</h2>
				<p class="text-sm text-muted-foreground">Container logs for this database.</p>
				<div class="rounded-md bg-[#0d0d0d] border border-border p-4 font-mono text-xs min-h-48 flex items-center justify-center text-muted-foreground/30">
					Deploy the database first to see logs here.
				</div>
			</div>
		{/if}

		<!-- MONITORING TAB -->
		{#if activeTab === 'Monitoring'}
			<div class="bg-card border border-border rounded-lg p-6 animate-fade-up">
				<h2 class="text-base font-semibold mb-1">Monitoring</h2>
				<p class="text-sm text-muted-foreground mb-4">Resource usage for this database.</p>
				<div class="flex items-center justify-center py-12 text-muted-foreground/30">
					Deploy the database first to see stats here.
				</div>
			</div>
		{/if}

		<!-- ADVANCED TAB -->
		{#if activeTab === 'Advanced'}
			<div class="flex flex-col gap-6 animate-fade-up">
				<section class="bg-card border border-destructive/40 rounded-lg p-6">
					<div class="flex items-center gap-2 mb-1">
						<AlertTriangle size={16} class="text-destructive" />
						<h2 class="text-base font-semibold text-destructive">Danger Zone</h2>
					</div>
					<p class="text-sm text-muted-foreground mb-5">Permanently delete this database and all its data.</p>
					{#if !confirmDelete}
						<button onclick={() => (confirmDelete = true)}
							class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md border border-destructive/50 text-destructive text-sm font-medium hover:bg-destructive hover:text-destructive-foreground transition-colors">
							<Trash2 size={14} /> Delete Database
						</button>
					{:else}
						<div class="bg-destructive/10 border border-destructive/30 rounded-lg p-4 flex flex-col gap-3">
							<p class="text-sm font-medium text-destructive">
								Are you sure? <strong>{db.name}</strong> and all its data will be permanently deleted.
							</p>
							{#if deleteError}<p class="text-xs text-destructive">{deleteError}</p>{/if}
							<div class="flex items-center gap-2">
								<button onclick={doDelete} disabled={deleting}
									class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-destructive text-destructive-foreground text-sm font-medium hover:bg-destructive/90 disabled:opacity-50">
									{#if deleting}<div class="w-3.5 h-3.5 border-2 border-destructive-foreground/30 border-t-destructive-foreground rounded-full animate-spin"></div>Deleting…{:else}<Trash2 size={14} />Yes, Delete{/if}
								</button>
								<button onclick={() => (confirmDelete = false)} class="px-3 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent transition-colors">Cancel</button>
							</div>
						</div>
					{/if}
				</section>
			</div>
		{/if}

	{/if}
</ServicePageShell>
