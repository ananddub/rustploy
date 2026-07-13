<script lang="ts">
	import { goto } from '$app/navigation';
	import {
		ClipboardList, ArrowUpDown, ChevronDown, X, FileJson,
		PlusCircle, RefreshCw, Trash2, Upload, XCircle,
		RotateCcw, LogIn, LogOut, Calendar as CalendarIcon
	} from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Table from '$lib/components/ui/table';
	import * as Select from '$lib/components/ui/select';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	// ─── Action config ────────────────────────────────────────────────────────────
	const ACTION_CONFIG: Record<string, { label: string; class: string }> = {
		create:   { label: 'Created',    class: 'bg-emerald-500/10 text-emerald-500 border-emerald-500/20' },
		update:   { label: 'Updated',    class: 'bg-blue-500/10 text-blue-500 border-blue-500/20' },
		delete:   { label: 'Deleted',    class: 'bg-red-500/10 text-red-500 border-red-500/20' },
		deploy:   { label: 'Deployed',   class: 'bg-violet-500/10 text-violet-500 border-violet-500/20' },
		cancel:   { label: 'Cancelled',  class: 'bg-orange-500/10 text-orange-500 border-orange-500/20' },
		redeploy: { label: 'Redeployed', class: 'bg-violet-500/10 text-violet-500 border-violet-500/20' },
		login:    { label: 'Login',      class: 'bg-teal-500/10 text-teal-500 border-teal-500/20' },
		logout:   { label: 'Logout',     class: 'bg-slate-500/10 text-muted-foreground border-border' },
	};

	const RESOURCE_LABELS: Record<string, string> = {
		project: 'Project', service: 'Service', environment: 'Environment',
		deployment: 'Deployment', user: 'User', customRole: 'Custom Role',
		domain: 'Domain', certificate: 'Certificate', registry: 'Registry',
		server: 'Server', sshKey: 'SSH Key', gitProvider: 'Git Provider',
		notification: 'Notification', settings: 'Settings', session: 'Session',
	};

	const ACTION_OPTIONS = [
		{ value:'create', label:'Created' }, { value:'update', label:'Updated' },
		{ value:'delete', label:'Deleted' }, { value:'deploy', label:'Deployed' },
		{ value:'cancel', label:'Cancelled' }, { value:'redeploy', label:'Redeployed' },
		{ value:'login', label:'Login' }, { value:'logout', label:'Logout' },
	];

	const RESOURCE_OPTIONS = [
		{ value:'project', label:'Projects' }, { value:'service', label:'Applications / Services' },
		{ value:'environment', label:'Environments' }, { value:'deployment', label:'Deployments' },
		{ value:'user', label:'Users' }, { value:'customRole', label:'Custom Roles' },
		{ value:'domain', label:'Domains' }, { value:'certificate', label:'Certificates' },
		{ value:'registry', label:'Registries' }, { value:'server', label:'Remote Servers' },
		{ value:'sshKey', label:'SSH Keys' }, { value:'gitProvider', label:'Git Providers' },
		{ value:'notification', label:'Notifications' }, { value:'settings', label:'Settings' },
		{ value:'session', label:'Sessions (Login/Logout)' },
	];

	// ─── Static log data ──────────────────────────────────────────────────────────
	type AuditLog = {
		id: string; createdAt: string; userEmail: string; userRole: string;
		action: string; resourceType: string; resourceName: string;
		metadata: string | null;
	};

	const ALL_LOGS: AuditLog[] = [
		{ id:'1', createdAt:'2026-07-13T14:30:00Z', userEmail:'admin@example.com', userRole:'owner',  action:'create',   resourceType:'project',     resourceName:'my-app',          metadata:'{"projectId":"p1"}' },
		{ id:'2', createdAt:'2026-07-13T13:45:00Z', userEmail:'dev@example.com',   userRole:'admin',  action:'deploy',   resourceType:'service',     resourceName:'frontend',        metadata:'{"serviceId":"s1","branch":"main"}' },
		{ id:'3', createdAt:'2026-07-13T12:00:00Z', userEmail:'admin@example.com', userRole:'owner',  action:'login',    resourceType:'session',     resourceName:'admin@example.com',metadata:null },
		{ id:'4', createdAt:'2026-07-13T11:20:00Z', userEmail:'ops@example.com',   userRole:'member', action:'create',   resourceType:'sshKey',      resourceName:'deploy-key',      metadata:'{"keyId":"k1"}' },
		{ id:'5', createdAt:'2026-07-13T10:10:00Z', userEmail:'admin@example.com', userRole:'owner',  action:'update',   resourceType:'settings',    resourceName:'Organization',    metadata:'{"field":"name"}' },
		{ id:'6', createdAt:'2026-07-13T09:00:00Z', userEmail:'dev@example.com',   userRole:'admin',  action:'deploy',   resourceType:'service',     resourceName:'backend-stack',   metadata:'{"serviceId":"s2"}' },
		{ id:'7', createdAt:'2026-07-12T22:30:00Z', userEmail:'ops@example.com',   userRole:'member', action:'delete',   resourceType:'service',     resourceName:'old-worker',      metadata:'{"serviceId":"s3"}' },
		{ id:'8', createdAt:'2026-07-12T20:00:00Z', userEmail:'dev@example.com',   userRole:'admin',  action:'redeploy', resourceType:'deployment',  resourceName:'deploy-#42',      metadata:'{"deployId":"d42"}' },
		{ id:'9', createdAt:'2026-07-12T18:45:00Z', userEmail:'admin@example.com', userRole:'owner',  action:'create',   resourceType:'certificate', resourceName:'*.example.com',   metadata:'{"certId":"c1"}' },
		{ id:'10',createdAt:'2026-07-12T17:00:00Z', userEmail:'admin@example.com', userRole:'owner',  action:'logout',   resourceType:'session',     resourceName:'admin@example.com',metadata:null },
		{ id:'11',createdAt:'2026-07-12T15:30:00Z', userEmail:'dev@example.com',   userRole:'admin',  action:'cancel',   resourceType:'deployment',  resourceName:'deploy-#41',      metadata:'{"deployId":"d41"}' },
		{ id:'12',createdAt:'2026-07-12T14:00:00Z', userEmail:'ops@example.com',   userRole:'member', action:'update',   resourceType:'server',      resourceName:'prod-server',     metadata:'{"serverId":"srv1"}' },
	];

	// ─── Filters & pagination ─────────────────────────────────────────────────────
	let filterUser     = $state('');
	let filterName     = $state('');
	let filterAction   = $state('');
	let filterResource = $state('');
	let pageIndex      = $state(0);
	let pageSize       = $state(50);

	// Column visibility
	let visibleCols = $state({ date: true, user: true, action: true, resource: true, name: true, role: true, metadata: true });

	// Sort
	let sortKey = $state('createdAt');
	let sortDir = $state<'asc'|'desc'>('desc');

	function toggleSort(key: string) {
		if (sortKey === key) sortDir = sortDir === 'asc' ? 'desc' : 'asc';
		else { sortKey = key; sortDir = 'asc'; }
	}

	const hasFilters = $derived(!!(filterUser || filterName || filterAction || filterResource));

	function clearFilters() { filterUser=''; filterName=''; filterAction=''; filterResource=''; pageIndex=0; }

	$effect(() => { filterUser; filterName; filterAction; filterResource; pageIndex = 0; });

	const filtered = $derived.by(() => {
		let list = ALL_LOGS.filter(l => {
			const mu = !filterUser     || l.userEmail.toLowerCase().includes(filterUser.toLowerCase());
			const mn = !filterName     || l.resourceName.toLowerCase().includes(filterName.toLowerCase());
			const ma = !filterAction   || l.action === filterAction;
			const mr = !filterResource || l.resourceType === filterResource;
			return mu && mn && ma && mr;
		});
		list = [...list].sort((a, b) => {
			const av = a[sortKey as keyof AuditLog] ?? '';
			const bv = b[sortKey as keyof AuditLog] ?? '';
			return sortDir === 'asc' ? String(av).localeCompare(String(bv)) : String(bv).localeCompare(String(av));
		});
		return list;
	});

	const total      = $derived(filtered.length);
	const pageCount  = $derived(Math.max(1, Math.ceil(total / pageSize)));
	const paginated  = $derived(filtered.slice(pageIndex * pageSize, (pageIndex + 1) * pageSize));

	function fmtDate(iso: string) {
		return new Date(iso).toLocaleString(undefined, { month:'short', day:'numeric', year:'numeric', hour:'2-digit', minute:'2-digit' });
	}

	// ─── Metadata detail ──────────────────────────────────────────────────────────
	let selectedMetadata = $state<string|null>(null);

	function fmtJson(s: string) {
		try { return JSON.stringify(JSON.parse(s), null, 2); } catch { return s; }
	}
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<ClipboardList class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Audit Logs</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto">
		<div class="w-full">
			<div class="rounded-xl border border-border bg-card">

				<!-- Header -->
				<div class="px-6 pt-5 pb-4 border-b border-border">
					<h2 class="text-xl font-bold flex items-center gap-2">
						<ClipboardList class="w-5 h-5 text-muted-foreground" /> Audit Logs
					</h2>
					<p class="text-sm text-muted-foreground mt-0.5">Track all actions performed by members in your organization</p>
				</div>

				<div class="px-6 py-4 space-y-4">

					<!-- Filters toolbar -->
					<div class="flex items-center gap-2 flex-wrap">
						<Input bind:value={filterUser}     placeholder="Filter by user…"   class="max-w-xs h-9" />
						<Input bind:value={filterName}     placeholder="Filter by name…"   class="max-w-xs h-9" />

						<Select.Root type="single" value={filterAction || '__all__'} onValueChange={(v) => (filterAction = v === '__all__' ? '' : (v ?? ''))}>
							<Select.Trigger class="w-40 h-9">
								<span class="text-sm">{filterAction ? ACTION_OPTIONS.find(o=>o.value===filterAction)?.label : 'All actions'}</span>
							</Select.Trigger>
							<Select.Content>
								<Select.Item value="__all__">All actions</Select.Item>
								{#each ACTION_OPTIONS as opt (opt.value)}
									<Select.Item value={opt.value}>{opt.label}</Select.Item>
								{/each}
							</Select.Content>
						</Select.Root>

						<Select.Root type="single" value={filterResource || '__all__'} onValueChange={(v) => (filterResource = v === '__all__' ? '' : (v ?? ''))}>
							<Select.Trigger class="w-52 h-9">
								<span class="text-sm">{filterResource ? RESOURCE_OPTIONS.find(o=>o.value===filterResource)?.label : 'All resources'}</span>
							</Select.Trigger>
							<Select.Content>
								<Select.Item value="__all__">All resources</Select.Item>
								{#each RESOURCE_OPTIONS as opt (opt.value)}
									<Select.Item value={opt.value}>{opt.label}</Select.Item>
								{/each}
							</Select.Content>
						</Select.Root>

						{#if hasFilters}
							<Button variant="ghost" size="sm" class="gap-1 text-muted-foreground h-9" onclick={clearFilters}>
								<X class="w-4 h-4" /> Clear
							</Button>
						{/if}

						<!-- Column visibility -->
						<DropdownMenu.Root>
							<DropdownMenu.Trigger>
								<Button variant="outline" class="ml-auto h-9 gap-1.5 text-sm">
									Columns <ChevronDown class="w-4 h-4" />
								</Button>
							</DropdownMenu.Trigger>
							<DropdownMenu.Content align="end">
								{#each [['date','Date'],['user','User'],['action','Action'],['resource','Resource'],['name','Name'],['role','Role'],['metadata','Metadata']] as [k,l] (k)}
									<DropdownMenu.CheckboxItem
										checked={visibleCols[k as keyof typeof visibleCols]}
										onCheckedChange={(v) => (visibleCols[k as keyof typeof visibleCols] = !!v)}
										class="text-xs capitalize"
									>{l}</DropdownMenu.CheckboxItem>
								{/each}
							</DropdownMenu.Content>
						</DropdownMenu.Root>
					</div>

					<!-- Table -->
					<div class="rounded-md border border-border overflow-x-auto">
						<Table.Root>
							<Table.Header>
								<Table.Row>
									{#if visibleCols.date}
										<Table.Head>
											<button class="flex items-center gap-1.5 text-xs font-medium hover:text-foreground transition-colors" onclick={() => toggleSort('createdAt')}>
												Date <ArrowUpDown class="w-3 h-3 opacity-40" />
											</button>
										</Table.Head>
									{/if}
									{#if visibleCols.user}
										<Table.Head>
											<button class="flex items-center gap-1.5 text-xs font-medium hover:text-foreground transition-colors" onclick={() => toggleSort('userEmail')}>
												User <ArrowUpDown class="w-3 h-3 opacity-40" />
											</button>
										</Table.Head>
									{/if}
									{#if visibleCols.action}
										<Table.Head>
											<button class="flex items-center gap-1.5 text-xs font-medium hover:text-foreground transition-colors" onclick={() => toggleSort('action')}>
												Action <ArrowUpDown class="w-3 h-3 opacity-40" />
											</button>
										</Table.Head>
									{/if}
									{#if visibleCols.resource}
										<Table.Head class="text-xs font-medium">Resource</Table.Head>
									{/if}
									{#if visibleCols.name}
										<Table.Head class="text-xs font-medium">Name</Table.Head>
									{/if}
									{#if visibleCols.role}
										<Table.Head class="text-xs font-medium">Role</Table.Head>
									{/if}
									{#if visibleCols.metadata}
										<Table.Head class="text-xs font-medium">Metadata</Table.Head>
									{/if}
								</Table.Row>
							</Table.Header>
							<Table.Body>
								{#if paginated.length === 0}
									<Table.Row>
										<Table.Cell colspan={7} class="h-24 text-center text-muted-foreground text-sm">
											No audit logs found
										</Table.Cell>
									</Table.Row>
								{:else}
									{#each paginated as log (log.id)}
										{@const ac = ACTION_CONFIG[log.action]}
										<Table.Row class="hover:bg-muted/20 transition-colors">
											{#if visibleCols.date}
												<Table.Cell class="text-xs text-muted-foreground whitespace-nowrap">{fmtDate(log.createdAt)}</Table.Cell>
											{/if}
											{#if visibleCols.user}
												<Table.Cell class="text-sm">{log.userEmail}</Table.Cell>
											{/if}
											{#if visibleCols.action}
												<Table.Cell>
													{#if ac}
														<span class="inline-flex items-center gap-1.5 rounded-full border px-2.5 py-0.5 text-xs font-medium {ac.class}">
															{ac.label}
														</span>
													{:else}
														<span class="text-xs text-muted-foreground">{log.action}</span>
													{/if}
												</Table.Cell>
											{/if}
											{#if visibleCols.resource}
												<Table.Cell class="text-sm text-muted-foreground">
													{RESOURCE_LABELS[log.resourceType] ?? log.resourceType}
												</Table.Cell>
											{/if}
											{#if visibleCols.name}
												<Table.Cell class="text-sm font-medium">{log.resourceName}</Table.Cell>
											{/if}
											{#if visibleCols.role}
												<Table.Cell class="text-sm text-muted-foreground capitalize">{log.userRole}</Table.Cell>
											{/if}
											{#if visibleCols.metadata}
												<Table.Cell>
													{#if log.metadata}
														<button
															class="inline-flex items-center gap-1.5 text-xs text-muted-foreground hover:text-foreground transition-colors px-2 py-1 rounded hover:bg-accent"
															onclick={() => (selectedMetadata = log.metadata)}
														>
															<FileJson class="w-3.5 h-3.5" /> View
														</button>
													{:else}
														<span class="text-muted-foreground text-sm">—</span>
													{/if}
												</Table.Cell>
											{/if}
										</Table.Row>
									{/each}
								{/if}
							</Table.Body>
						</Table.Root>
					</div>

					<!-- Pagination footer -->
					<div class="flex items-center justify-between text-sm text-muted-foreground flex-wrap gap-3">
						<span>{total} {total === 1 ? 'entry' : 'entries'} total</span>
						<div class="flex items-center gap-3 flex-wrap">
							<div class="flex items-center gap-2">
								<span class="text-sm whitespace-nowrap">Rows per page</span>
								<Select.Root type="single" value={String(pageSize)} onValueChange={(v) => { pageSize=Number(v??50); pageIndex=0; }}>
									<Select.Trigger class="w-20 h-8"><span class="text-sm">{pageSize}</span></Select.Trigger>
									<Select.Content>
										{#each [25,50,100,200] as s (s)}
											<Select.Item value={String(s)}>{s}</Select.Item>
										{/each}
									</Select.Content>
								</Select.Root>
							</div>
							<span class="whitespace-nowrap">Page {pageIndex+1} of {pageCount}</span>
							<div class="flex gap-2">
								<Button variant="outline" size="sm" class="h-8" disabled={pageIndex===0} onclick={() => pageIndex--}>Previous</Button>
								<Button variant="outline" size="sm" class="h-8" disabled={pageIndex+1>=pageCount} onclick={() => pageIndex++}>Next</Button>
							</div>
						</div>
					</div>

				</div>
			</div>
		</div>
	</main>
</PageLayout>

<!-- ── Metadata side panel ─────────────────────────────────────────────────── -->
{#if selectedMetadata}
	<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1"
		onclick={() => (selectedMetadata=null)} onkeydown={() => {}}></div>
	<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
		<div class="bg-card border border-border rounded-xl shadow-2xl w-full max-w-lg pointer-events-auto flex flex-col gap-4 p-6 max-h-[80vh]">
			<div class="flex items-center justify-between">
				<h2 class="text-base font-semibold flex items-center gap-2">
					<FileJson class="w-4 h-4 text-muted-foreground" /> Metadata
				</h2>
				<button onclick={() => (selectedMetadata=null)}
					class="text-muted-foreground hover:text-foreground p-1 rounded hover:bg-accent">✕</button>
			</div>
			<pre class="bg-muted rounded-lg p-4 text-xs font-mono overflow-auto max-h-[50vh] whitespace-pre-wrap break-all">{fmtJson(selectedMetadata)}</pre>
		</div>
	</div>
{/if}
