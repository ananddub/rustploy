<script lang="ts">
	import { Globe, Plus, Trash2, Pencil, ExternalLink, Lock, LockOpen, RefreshCw, Info, Copy } from '@lucide/svelte';
	import Switch from '$lib/components/Switch.svelte';
	import type { ApplicationResponseDto, DomainResponseDto } from '$lib/client/types.gen';
	import {
		domainControllerListByApplication,
		domainControllerCreate,
		domainControllerPatch,
		domainControllerDelete
	} from '$lib/client/sdk.gen';
	import { formatDate } from '$lib/helpers';

	type Props = { app: ApplicationResponseDto };
	let { app }: Props = $props();

	let domains = $state<DomainResponseDto[]>([]);
	let loadingDomains = $state(true);
	let deletingId = $state<number | null>(null);
	let showAdd = $state(false);
	let editing = $state<DomainResponseDto | null>(null);

	async function loadDomains() {
		loadingDomains = true;
		try {
			const res = await domainControllerListByApplication({ path: { application_id: app.id } });
			domains = (res.data as DomainResponseDto[]) ?? [];
		} finally { loadingDomains = false; }
	}

	loadDomains();

	async function deleteDomain(id: number) {
		deletingId = id;
		try {
			await domainControllerDelete({ path: { id } });
			domains = domains.filter(d => d.id !== id);
		} finally { deletingId = null; }
	}

	function domainUrl(d: DomainResponseDto): string {
		const scheme = d.https ? 'https' : 'http';
		const portSuffix = d.port && ((d.https && d.port !== 443) || (!d.https && d.port !== 80)) ? `:${d.port}` : '';
		const pathSuffix = d.path && d.path !== '/' ? d.path : '';
		return `${scheme}://${d.host}${portSuffix}${pathSuffix}`;
	}

	// ── Add form ──────────────────────────────────────────────
	let aHost = $state('');
	let aPath = $state('/');
	let aInternalPath = $state('/');
	let aPort = $state('3000');
	let aHttps = $state(false);
	let aStripPath = $state(false);
	let aCustomEpEnabled = $state(false);
	let aCustomEntrypoint = $state('');
	let aCertType = $state('none');
	let aCustomCertResolver = $state('');
	let aMiddlewares = $state('');
	let aServiceName = $state('');
	let aSaving = $state(false);
	let aError = $state('');

	async function submitAdd(e: SubmitEvent) {
		e.preventDefault(); aError = ''; aSaving = true;
		try {
			const res = await domainControllerCreate({
				body: {
					application_id: app.id,
					host: aHost.trim(),
					https: aHttps,
					path: aPath || '/',
					internal_path: aInternalPath || '/',
					strip_path: aStripPath,
					domain_type: 'HTTP',
					certificate_type: aCertType,
					middlewares: aMiddlewares,
					port: aPort ? parseInt(aPort) : undefined,
					service_name: aServiceName || undefined,
					custom_cert_resolver: aCustomCertResolver || undefined,
					custom_entrypoint: aCustomEpEnabled ? (aCustomEntrypoint || undefined) : undefined
				} as any
			});
			if (res.error || !res.data) throw new Error((res.error as any)?.message ?? 'Failed to create domain');
			domains = [...domains, res.data as DomainResponseDto];
			showAdd = false;
			aHost = ''; aPath = '/'; aInternalPath = '/'; aPort = '3000';
			aHttps = false; aStripPath = false; aCertType = 'none';
		} catch (err) {
			aError = err instanceof Error ? err.message : 'Something went wrong';
		} finally { aSaving = false; }
	}

	// ── Edit form ─────────────────────────────────────────────
	let eHost = $state('');
	let ePort = $state('');
	let ePath = $state('/');
	let eInternalPath = $state('/');
	let eHttps = $state(false);
	let eStripPath = $state(false);
	let eCustomEpEnabled = $state(false);
	let eCustomEntrypoint = $state('');
	let eCertType = $state('none');
	let eCustomCertResolver = $state('');
	let eMiddlewares = $state('');
	let eServiceName = $state('');
	let eSaving = $state(false);
	let eError = $state('');

	function openEdit(d: DomainResponseDto) {
		editing = d;
		eHost = d.host; ePort = d.port ? String(d.port) : '3000';
		ePath = d.path ?? '/'; eInternalPath = d.internal_path ?? '/';
		eHttps = d.https; eStripPath = d.strip_path;
		eCustomEpEnabled = !!d.custom_entrypoint; eCustomEntrypoint = d.custom_entrypoint ?? '';
		eCertType = d.certificate_type ?? 'none'; eCustomCertResolver = d.custom_cert_resolver ?? '';
		eMiddlewares = d.middlewares ?? ''; eServiceName = d.service_name ?? '';
		eError = '';
	}

	async function submitEdit(e: SubmitEvent) {
		e.preventDefault(); if (!editing) return; eError = ''; eSaving = true;
		try {
			const res = await domainControllerPatch({
				path: { id: editing.id },
				body: {
					host: eHost.trim() || undefined,
					https: eHttps,
					path: ePath || undefined,
					internal_path: eInternalPath || undefined,
					port: ePort ? parseInt(ePort) : undefined,
					strip_path: eStripPath,
					certificate_type: eCertType,
					middlewares: eMiddlewares,
					service_name: eServiceName || undefined,
					custom_cert_resolver: eCustomCertResolver || undefined,
					custom_entrypoint: eCustomEpEnabled ? (eCustomEntrypoint || undefined) : undefined
				} as any
			});
			if (res.error || !res.data) throw new Error((res.error as any)?.message ?? 'Failed to update domain');
			domains = domains.map(d => d.id === editing!.id ? res.data as DomainResponseDto : d);
			editing = null;
		} catch (err) {
			eError = err instanceof Error ? err.message : 'Something went wrong';
		} finally { eSaving = false; }
	}

	const inputCls = 'flex h-9 w-full rounded-md border border-input bg-secondary px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring';
	const toggleRowCls = 'border border-border rounded-lg px-4 py-3 flex items-start justify-between gap-4 bg-secondary/50';
</script>

<!-- Main section -->
<div class="flex flex-col gap-6 animate-fade-up">
	<section class="bg-card border border-border rounded-lg p-6">
		<div class="flex items-center justify-between">
			<div>
				<h2 class="text-base font-semibold">Domains</h2>
				<p class="text-sm text-muted-foreground mt-1">Configure custom domains and routing for this application.</p>
			</div>
			<div class="flex items-center gap-2">
				<button
					onclick={loadDomains}
					class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
					disabled={loadingDomains}
				>
					<RefreshCw size={14} class={loadingDomains ? 'animate-spin' : ''} /> Refresh
				</button>
				<button
					onclick={() => (showAdd = true)}
					class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 transition-colors"
				>
					<Plus size={14} /> Add Domain
				</button>
			</div>
		</div>
	</section>

	<section class="bg-card border border-border rounded-lg overflow-hidden">
		{#if loadingDomains}
			<div class="flex justify-center py-14"><div class="w-6 h-6 border-2 border-muted-foreground/30 border-t-foreground rounded-full animate-spin"></div></div>
		{:else if domains.length === 0}
			<div class="flex flex-col items-center justify-center py-16 text-muted-foreground/30">
				<Globe size={40} class="mb-3 opacity-40" />
				<p class="text-sm">No domains configured</p>
				<p class="text-xs mt-1 opacity-70">Add a domain above to expose this application.</p>
			</div>
		{:else}
			<div class="grid grid-cols-[1fr_80px_130px_140px_72px] gap-4 px-5 py-2.5 border-b border-border text-xs text-muted-foreground font-medium uppercase tracking-wide">
				<span>Host</span><span>Port</span><span>TLS</span><span>Added</span><span></span>
			</div>
			{#each domains as domain (domain.id)}
				<div class="grid grid-cols-[1fr_80px_130px_140px_72px] gap-4 items-center px-5 py-3 border-b border-border last:border-0 hover:bg-accent/20 transition-colors">
					<div class="min-w-0">
						<div class="flex items-center gap-1.5">
							<Globe size={13} class="text-muted-foreground/40 shrink-0" />
							<span class="text-sm font-medium truncate">{domain.host}</span>
							{#if domain.path && domain.path !== '/'}<span class="text-xs text-muted-foreground font-mono">{domain.path}</span>{/if}
						</div>
						<a href={domainUrl(domain)} target="_blank" rel="noopener noreferrer"
							class="text-xs text-muted-foreground/40 hover:text-primary transition-colors flex items-center gap-1 mt-0.5 w-fit">
							<ExternalLink size={11} />{domainUrl(domain)}
						</a>
					</div>
					<span class="text-xs text-muted-foreground font-mono">{domain.port ?? '—'}</span>
					<div>
						{#if domain.https}
							<span class="inline-flex items-center gap-1 text-xs font-medium text-green-500">
								<Lock size={13} /> {domain.certificate_type === 'letsencrypt' ? "Let's Encrypt" : domain.certificate_type === 'custom' ? 'Custom' : 'HTTPS'}
							</span>
						{:else}
							<span class="inline-flex items-center gap-1 text-xs text-muted-foreground/40">
								<LockOpen size={13} /> None
							</span>
						{/if}
					</div>
					<span class="text-xs text-muted-foreground/40">{formatDate(domain.created_at)}</span>
					<div class="flex items-center justify-end gap-0.5">
						<button onclick={() => openEdit(domain)} class="p-1.5 rounded-md text-muted-foreground/40 hover:text-foreground hover:bg-accent transition-all" title="Edit"><Pencil size={13} /></button>
						<button onclick={() => deleteDomain(domain.id)} disabled={deletingId === domain.id} class="p-1.5 rounded-md text-muted-foreground/40 hover:text-destructive hover:bg-destructive/10 transition-all" title="Delete">
							{#if deletingId === domain.id}<div class="w-3.5 h-3.5 border-2 border-current/30 border-t-current rounded-full animate-spin"></div>{:else}<Trash2 size={13} />{/if}
						</button>
					</div>
				</div>
			{/each}
		{/if}
	</section>
</div>

<!-- Add Domain Modal -->
{#if showAdd}
<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1" aria-label="Close" onclick={() => (showAdd = false)} onkeydown={() => {}}></div>
<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
	<div class="bg-card border border-border rounded-lg w-full max-w-lg shadow-2xl flex flex-col max-h-[90vh] pointer-events-auto">
		<div class="flex items-start justify-between px-5 py-4 border-b border-border shrink-0">
			<div><h2 class="font-semibold">Add Domain</h2><p class="text-sm text-muted-foreground mt-0.5">Configure routing for this application</p></div>
			<button onclick={() => (showAdd = false)} class="text-muted-foreground hover:text-foreground p-0.5 rounded hover:bg-accent ml-4">✕</button>
		</div>
		<form onsubmit={submitAdd} class="overflow-y-auto flex-1 px-5 py-4 flex flex-col gap-4">
			<div class="flex items-start gap-3 bg-primary/10 border border-primary/20 rounded-lg px-4 py-3">
				<Info size={15} class="text-primary mt-0.5 shrink-0" />
				<p class="text-sm text-primary/90 leading-snug">Remember to redeploy your application after making domain changes.</p>
			</div>

			<div class="flex flex-col gap-1.5">
				<label for="add-service" class="text-sm font-medium text-muted-foreground">Service Name</label>
				<input id="add-service" class={inputCls} placeholder="Select a service name" bind:value={aServiceName} />
			</div>
			<div class="flex flex-col gap-1.5">
				<label for="add-host" class="text-sm font-medium text-muted-foreground">Host <span class="text-destructive">*</span></label>
				<input id="add-host" class={inputCls} placeholder="api.example.com" bind:value={aHost} required />
			</div>
			<div class="flex flex-col gap-1.5">
				<label for="add-path" class="text-sm font-medium text-muted-foreground">Path</label>
				<input id="add-path" class={inputCls} placeholder="/" bind:value={aPath} />
			</div>
			<div class="flex flex-col gap-1.5">
				<label for="add-ipath" class="text-sm font-medium text-muted-foreground">Internal Path</label>
				<p class="text-xs text-muted-foreground">The path where your application expects requests internally</p>
				<input id="add-ipath" class={inputCls} placeholder="/" bind:value={aInternalPath} />
			</div>
			<div class={toggleRowCls}>
				<div><p class="text-sm font-medium">Strip Path</p><p class="text-xs text-muted-foreground mt-0.5">Remove the external path before forwarding to the app</p></div>
				<Switch checked={aStripPath} onchange={(v) => (aStripPath = v)} />
			</div>
			<div class="flex flex-col gap-1.5">
				<label for="add-port" class="text-sm font-medium text-muted-foreground">Container Port</label>
				<p class="text-xs text-muted-foreground">Port your application listens on inside the container</p>
				<input id="add-port" type="number" min="1" max="65535" class={inputCls} placeholder="3000" bind:value={aPort} />
			</div>
			<div class={toggleRowCls}>
				<div><p class="text-sm font-medium">Custom Entrypoint</p><p class="text-xs text-muted-foreground mt-0.5">Use custom entrypoint instead of default "web"/"websecure"</p></div>
				<Switch checked={aCustomEpEnabled} onchange={(v) => (aCustomEpEnabled = v)} />
			</div>
			{#if aCustomEpEnabled}
				<div class="flex flex-col gap-1.5">
					<label for="add-ep" class="text-sm font-medium text-muted-foreground">Entrypoint</label>
					<input id="add-ep" class={inputCls} placeholder="websecure" bind:value={aCustomEntrypoint} />
				</div>
			{/if}
			<div class={toggleRowCls}>
				<div><p class="text-sm font-medium">HTTPS</p><p class="text-xs text-muted-foreground mt-0.5">Automatically provision SSL certificate</p></div>
				<Switch checked={aHttps} onchange={(v) => (aHttps = v)} />
			</div>
			{#if aHttps}
				<div class="flex flex-col gap-1.5">
					<label for="add-cert" class="text-sm font-medium text-muted-foreground">Certificate Type</label>
					<select id="add-cert" class="h-9 w-full rounded-md border border-input bg-secondary px-3 text-sm focus:outline-none focus:ring-1 focus:ring-ring" bind:value={aCertType}>
						<option value="none">None</option>
						<option value="letsencrypt">Let's Encrypt</option>
						<option value="custom">Custom</option>
					</select>
				</div>
				{#if aCertType === 'custom'}
					<div class="flex flex-col gap-1.5">
						<label for="add-certresolver" class="text-sm font-medium text-muted-foreground">Custom Cert Resolver</label>
						<input id="add-certresolver" class={inputCls} placeholder="myresolver" bind:value={aCustomCertResolver} />
					</div>
				{/if}
			{/if}
			<div class="flex flex-col gap-1.5">
				<label for="add-mw" class="text-sm font-medium text-muted-foreground">Middlewares</label>
				<input id="add-mw" class={inputCls} placeholder="e.g., rate-limit@file, auth@file" bind:value={aMiddlewares} />
			</div>
			{#if aError}<div class="rounded-md bg-destructive/10 border border-destructive/30 px-3 py-2 text-sm text-destructive">{aError}</div>{/if}
			<div class="flex justify-end pt-1">
				<button type="submit" class="inline-flex items-center gap-2 px-5 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50" disabled={aSaving || !aHost.trim()}>
					{#if aSaving}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>Creating…{:else}Create{/if}
				</button>
			</div>
		</form>
	</div>
</div>
{/if}

<!-- Edit Domain Modal -->
{#if editing}
<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1" aria-label="Close" onclick={() => (editing = null)} onkeydown={() => {}}></div>
<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
	<div class="bg-card border border-border rounded-lg w-full max-w-lg shadow-2xl flex flex-col max-h-[90vh] pointer-events-auto">
		<div class="flex items-start justify-between px-5 py-4 border-b border-border shrink-0">
			<div><h2 class="font-semibold">Edit Domain</h2><p class="text-sm text-muted-foreground mt-0.5">Update domain routing configuration</p></div>
			<button onclick={() => (editing = null)} class="text-muted-foreground hover:text-foreground p-0.5 rounded hover:bg-accent ml-4">✕</button>
		</div>
		<form onsubmit={submitEdit} class="overflow-y-auto flex-1 px-5 py-4 flex flex-col gap-4">
			<div class="flex items-start gap-3 bg-primary/10 border border-primary/20 rounded-lg px-4 py-3">
				<Info size={15} class="text-primary mt-0.5 shrink-0" />
				<p class="text-sm text-primary/90 leading-snug">Remember to redeploy your application after making domain changes.</p>
			</div>
			<div class="flex flex-col gap-1.5">
				<label for="edit-host" class="text-sm font-medium text-muted-foreground">Host <span class="text-destructive">*</span></label>
				<input id="edit-host" class={inputCls} placeholder="api.example.com" bind:value={eHost} required />
			</div>
			<div class="grid grid-cols-2 gap-4">
				<div class="flex flex-col gap-1.5">
					<label for="edit-port" class="text-sm font-medium text-muted-foreground">Container Port</label>
					<input id="edit-port" type="number" min="1" max="65535" class={inputCls} placeholder="3000" bind:value={ePort} />
				</div>
				<div class="flex flex-col gap-1.5">
					<label for="e-dtype" class="text-sm font-medium text-muted-foreground">Domain Type</label>
					<input id="e-dtype" class="{inputCls} opacity-50 cursor-not-allowed" value={editing.domain_type} disabled />
				</div>
			</div>
			<div class="grid grid-cols-2 gap-4">
				<div class="flex flex-col gap-1.5">
					<label for="edit-path" class="text-sm font-medium text-muted-foreground">Path</label>
					<input id="edit-path" class={inputCls} placeholder="/" bind:value={ePath} />
				</div>
				<div class="flex flex-col gap-1.5">
					<label for="edit-ipath" class="text-sm font-medium text-muted-foreground">Internal Path</label>
					<input id="edit-ipath" class={inputCls} placeholder="/" bind:value={eInternalPath} />
				</div>
			</div>
			<div class={toggleRowCls}>
				<div><p class="text-sm font-medium">Strip Path</p><p class="text-xs text-muted-foreground mt-0.5">Remove external path before forwarding</p></div>
				<Switch checked={eStripPath} onchange={(v) => (eStripPath = v)} />
			</div>
			<div class={toggleRowCls}>
				<div><p class="text-sm font-medium">Custom Entrypoint</p><p class="text-xs text-muted-foreground mt-0.5">Use custom entrypoint instead of default</p></div>
				<Switch checked={eCustomEpEnabled} onchange={(v) => (eCustomEpEnabled = v)} />
			</div>
			{#if eCustomEpEnabled}
				<div class="flex flex-col gap-1.5">
					<label for="edit-ep" class="text-sm font-medium text-muted-foreground">Entrypoint</label>
					<input id="edit-ep" class={inputCls} placeholder="websecure" bind:value={eCustomEntrypoint} />
				</div>
			{/if}
			<div class={toggleRowCls}>
				<div><p class="text-sm font-medium">HTTPS</p><p class="text-xs text-muted-foreground mt-0.5">Automatically provision SSL certificate</p></div>
				<Switch checked={eHttps} onchange={(v) => (eHttps = v)} />
			</div>
			{#if eHttps}
				<div class="flex flex-col gap-1.5">
					<label for="edit-cert" class="text-sm font-medium text-muted-foreground">Certificate Type</label>
					<select id="edit-cert" class="h-9 w-full rounded-md border border-input bg-secondary px-3 text-sm focus:outline-none focus:ring-1 focus:ring-ring" bind:value={eCertType}>
						<option value="none">None</option>
						<option value="letsencrypt">Let's Encrypt</option>
						<option value="custom">Custom</option>
					</select>
				</div>
				{#if eCertType === 'custom'}
					<div class="flex flex-col gap-1.5">
						<label for="edit-certresolver" class="text-sm font-medium text-muted-foreground">Custom Cert Resolver</label>
						<input id="edit-certresolver" class={inputCls} placeholder="myresolver" bind:value={eCustomCertResolver} />
					</div>
				{/if}
			{/if}
			<div class="flex flex-col gap-1.5">
				<label for="edit-mw" class="text-sm font-medium text-muted-foreground">Middlewares</label>
				<input id="edit-mw" class={inputCls} placeholder="e.g., rate-limit@file, auth@file" bind:value={eMiddlewares} />
			</div>
			{#if eError}<div class="rounded-md bg-destructive/10 border border-destructive/30 px-3 py-2 text-sm text-destructive">{eError}</div>{/if}
			<div class="flex justify-end pt-1">
				<button type="submit" class="inline-flex items-center gap-2 px-5 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50" disabled={eSaving}>
					{#if eSaving}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>Saving…{:else}Save Changes{/if}
				</button>
			</div>
		</form>
	</div>
</div>
{/if}
