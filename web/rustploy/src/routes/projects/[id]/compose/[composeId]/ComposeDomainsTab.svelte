<script lang="ts">
	import { Globe, Plus, Trash2, Pencil, ExternalLink, Lock, LockOpen, RefreshCw, Info } from '@lucide/svelte';
	import Switch from '$lib/components/Switch.svelte';
	import type { ComposeResponseDto, DomainResponseDto } from '$lib/client/types.gen';
	import { domainControllerListByCompose, domainControllerCreate, domainControllerPatch, domainControllerDelete } from '$lib/client/sdk.gen';
	import { formatDate } from '$lib/helpers';

	type Props = { compose: ComposeResponseDto };
	let { compose }: Props = $props();

	let domains = $state<DomainResponseDto[]>([]);
	let loadingDomains = $state(true);
	let deletingId = $state<number | null>(null);
	let showAdd = $state(false);
	let editing = $state<DomainResponseDto | null>(null);

	async function loadDomains() {
		loadingDomains = true;
		try {
			const res = await domainControllerListByCompose({ path: { compose_id: compose.id } });
			domains = (res.data as DomainResponseDto[]) ?? [];
		} finally { loadingDomains = false; }
	}
	loadDomains();

	async function deleteDomain(id: number) {
		deletingId = id;
		try { await domainControllerDelete({ path: { id } }); domains = domains.filter(d => d.id !== id); }
		finally { deletingId = null; }
	}

	function domainUrl(d: DomainResponseDto) {
		const scheme = d.https ? 'https' : 'http';
		const portSuffix = d.port && ((d.https && d.port !== 443) || (!d.https && d.port !== 80)) ? `:${d.port}` : '';
		return `${scheme}://${d.host}${portSuffix}${d.path && d.path !== '/' ? d.path : ''}`;
	}

	// Add form
	let aHost=$state(''); let aPath=$state('/'); let aIPath=$state('/'); let aPort=$state('80');
	let aHttps=$state(false); let aStrip=$state(false); let aCert=$state('none');
	let aCustomCert=$state(''); let aMw=$state(''); let aService=$state('');
	let aSaving=$state(false); let aError=$state('');

	async function submitAdd(e: SubmitEvent) {
		e.preventDefault(); aError=''; aSaving=true;
		try {
			const res = await domainControllerCreate({ body: { compose_id: compose.id, host: aHost.trim(), https: aHttps, path: aPath||'/', internal_path: aIPath||'/', strip_path: aStrip, domain_type: 'HTTP', certificate_type: aCert, middlewares: aMw, port: aPort?parseInt(aPort):undefined, service_name: aService||undefined, custom_cert_resolver: aCustomCert||undefined } as any });
			if (res.error||!res.data) throw new Error((res.error as any)?.message??'Failed');
			domains=[...domains, res.data as DomainResponseDto]; showAdd=false;
			aHost=''; aPath='/'; aIPath='/'; aPort='80';
		} catch(err) { aError=err instanceof Error?err.message:'Something went wrong'; }
		finally { aSaving=false; }
	}

	// Edit form — same pattern as AppDomainsTab
	let eHost=$state(''); let ePort=$state(''); let ePath=$state('/'); let eIPath=$state('/');
	let eHttps=$state(false); let eStrip=$state(false); let eCert=$state('none');
	let eCustomCert=$state(''); let eMw=$state(''); let eService=$state('');
	let eSaving=$state(false); let eError=$state('');

	function openEdit(d: DomainResponseDto) {
		editing=d; eHost=d.host; ePort=d.port?String(d.port):'80'; ePath=d.path??'/'; eIPath=d.internal_path??'/';
		eHttps=d.https; eStrip=d.strip_path; eCert=d.certificate_type??'none';
		eCustomCert=d.custom_cert_resolver??''; eMw=d.middlewares??''; eService=d.service_name??''; eError='';
	}

	async function submitEdit(e: SubmitEvent) {
		e.preventDefault(); if(!editing) return; eError=''; eSaving=true;
		try {
			const res = await domainControllerPatch({ path:{id:editing.id}, body:{host:eHost.trim()||undefined,https:eHttps,path:ePath||undefined,internal_path:eIPath||undefined,port:ePort?parseInt(ePort):undefined,strip_path:eStrip,certificate_type:eCert,middlewares:eMw,service_name:eService||undefined,custom_cert_resolver:eCustomCert||undefined} as any });
			if (res.error||!res.data) throw new Error('Failed');
			domains=domains.map(d=>d.id===editing!.id?res.data as DomainResponseDto:d); editing=null;
		} catch(err) { eError=err instanceof Error?err.message:'Something went wrong'; }
		finally { eSaving=false; }
	}

	const inputCls='flex h-9 w-full rounded-md border border-input bg-secondary px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring';
	const toggleRowCls='border border-border rounded-lg px-4 py-3 flex items-start justify-between gap-4 bg-secondary/50';
</script>

<div class="flex flex-col gap-6 animate-fade-up">
	<section class="bg-card border border-border rounded-lg p-6">
		<div class="flex items-center justify-between">
			<div>
				<h2 class="text-base font-semibold">Domains</h2>
				<p class="text-sm text-muted-foreground mt-1">Configure custom domains and routing for this compose service.</p>
			</div>
			<div class="flex items-center gap-2">
				<button onclick={loadDomains} disabled={loadingDomains} class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent disabled:opacity-50 transition-colors">
					<RefreshCw size={14} class={loadingDomains?'animate-spin':''} /> Refresh
				</button>
				<button onclick={() => (showAdd=true)} class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 transition-colors">
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
						</div>
						<a href={domainUrl(domain)} target="_blank" rel="noopener noreferrer" class="text-xs text-muted-foreground/40 hover:text-primary transition-colors flex items-center gap-1 mt-0.5 w-fit">
							<ExternalLink size={11} />{domainUrl(domain)}
						</a>
					</div>
					<span class="text-xs text-muted-foreground font-mono">{domain.port??'—'}</span>
					<div>
						{#if domain.https}
							<span class="inline-flex items-center gap-1 text-xs font-medium text-green-500"><Lock size={13} />{domain.certificate_type==='letsencrypt'?"Let's Encrypt":domain.certificate_type==='custom'?'Custom':'HTTPS'}</span>
						{:else}
							<span class="inline-flex items-center gap-1 text-xs text-muted-foreground/40"><LockOpen size={13} />None</span>
						{/if}
					</div>
					<span class="text-xs text-muted-foreground/40">{formatDate(domain.created_at)}</span>
					<div class="flex items-center justify-end gap-0.5">
						<button onclick={() => openEdit(domain)} class="p-1.5 rounded-md text-muted-foreground/40 hover:text-foreground hover:bg-accent transition-all"><Pencil size={13} /></button>
						<button onclick={() => deleteDomain(domain.id)} disabled={deletingId===domain.id} class="p-1.5 rounded-md text-muted-foreground/40 hover:text-destructive hover:bg-destructive/10 transition-all">
							{#if deletingId===domain.id}<div class="w-3.5 h-3.5 border-2 border-current/30 border-t-current rounded-full animate-spin"></div>{:else}<Trash2 size={13} />{/if}
						</button>
					</div>
				</div>
			{/each}
		{/if}
	</section>
</div>

{#if showAdd}
<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1" aria-label="Close" onclick={() => (showAdd=false)} onkeydown={() => {}}></div>
<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
	<div class="bg-card border border-border rounded-lg w-full max-w-lg shadow-2xl flex flex-col max-h-[90vh] pointer-events-auto">
		<div class="flex items-start justify-between px-5 py-4 border-b border-border">
			<div><h2 class="font-semibold">Add Domain</h2><p class="text-sm text-muted-foreground mt-0.5">Configure routing for this compose service</p></div>
			<button onclick={() => (showAdd=false)} class="text-muted-foreground hover:text-foreground p-0.5 rounded hover:bg-accent ml-4">✕</button>
		</div>
		<form onsubmit={submitAdd} class="overflow-y-auto flex-1 px-5 py-4 flex flex-col gap-4">
			<div class="flex items-start gap-3 bg-primary/10 border border-primary/20 rounded-lg px-4 py-3">
				<Info size={15} class="text-primary mt-0.5 shrink-0" />
				<p class="text-sm text-primary/90">Remember to add the service name for compose routing to work correctly.</p>
			</div>
			<div class="flex flex-col gap-1.5"><label for="ca-service" class="text-sm font-medium text-muted-foreground">Service Name <span class="text-destructive">*</span></label><input id="ca-service" class={inputCls} placeholder="nginx" bind:value={aService} required /></div>
			<div class="flex flex-col gap-1.5"><label for="ca-host" class="text-sm font-medium text-muted-foreground">Host <span class="text-destructive">*</span></label><input id="ca-host" class={inputCls} placeholder="app.example.com" bind:value={aHost} required /></div>
			<div class="grid grid-cols-2 gap-4">
				<div class="flex flex-col gap-1.5"><label for="ca-path" class="text-sm font-medium text-muted-foreground">Path</label><input id="ca-path" class={inputCls} placeholder="/" bind:value={aPath} /></div>
				<div class="flex flex-col gap-1.5"><label for="ca-port" class="text-sm font-medium text-muted-foreground">Container Port</label><input id="ca-port" type="number" class={inputCls} placeholder="80" bind:value={aPort} /></div>
			</div>
			<div class={toggleRowCls}><div><p class="text-sm font-medium">HTTPS</p><p class="text-sm text-muted-foreground mt-0.5">Provision SSL certificate</p></div><Switch checked={aHttps} onchange={(v) => (aHttps = v)} /></div>
			{#if aHttps}
				<div class="flex flex-col gap-1.5"><label for="ca-cert" class="text-sm font-medium text-muted-foreground">Certificate Type</label>
				<select id="ca-cert" class="h-9 w-full rounded-md border border-input bg-secondary px-3 text-sm focus:outline-none focus:ring-1 focus:ring-ring" bind:value={aCert}>
					<option value="none">None</option><option value="letsencrypt">Let's Encrypt</option><option value="custom">Custom</option>
				</select></div>
			{/if}
			<div class={toggleRowCls}><div><p class="text-sm font-medium">Strip Path</p></div><Switch checked={aStrip} onchange={(v) => (aStrip = v)} /></div>
			<div class="flex flex-col gap-1.5"><label for="ca-mw" class="text-sm font-medium text-muted-foreground">Middlewares</label><input id="ca-mw" class={inputCls} placeholder="rate-limit@file" bind:value={aMw} /></div>
			{#if aError}<div class="rounded-md bg-destructive/10 border border-destructive/30 px-3 py-2 text-sm text-destructive">{aError}</div>{/if}
			<div class="flex justify-end pt-1">
				<button type="submit" disabled={aSaving||!aHost.trim()} class="inline-flex items-center gap-2 px-5 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50">
					{#if aSaving}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>Creating…{:else}Create{/if}
				</button>
			</div>
		</form>
	</div>
</div>
{/if}

{#if editing}
<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1" aria-label="Close" onclick={() => (editing=null)} onkeydown={() => {}}></div>
<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
	<div class="bg-card border border-border rounded-lg w-full max-w-lg shadow-2xl flex flex-col max-h-[90vh] pointer-events-auto">
		<div class="flex items-start justify-between px-5 py-4 border-b border-border">
			<div><h2 class="font-semibold">Edit Domain</h2></div>
			<button onclick={() => (editing=null)} class="text-muted-foreground hover:text-foreground p-0.5 rounded hover:bg-accent ml-4">✕</button>
		</div>
		<form onsubmit={submitEdit} class="overflow-y-auto flex-1 px-5 py-4 flex flex-col gap-4">
			<div class="flex flex-col gap-1.5"><label for="ce-host" class="text-sm font-medium text-muted-foreground">Host</label><input id="ce-host" class={inputCls} bind:value={eHost} required /></div>
			<div class="grid grid-cols-2 gap-4">
				<div class="flex flex-col gap-1.5"><label for="ce-path" class="text-sm font-medium text-muted-foreground">Path</label><input id="ce-path" class={inputCls} placeholder="/" bind:value={ePath} /></div>
				<div class="flex flex-col gap-1.5"><label for="ce-port" class="text-sm font-medium text-muted-foreground">Port</label><input id="ce-port" type="number" class={inputCls} bind:value={ePort} /></div>
			</div>
			<div class={toggleRowCls}><div><p class="text-sm font-medium">HTTPS</p></div><Switch checked={eHttps} onchange={(v) => (eHttps = v)} /></div>
			{#if eHttps}
				<div class="flex flex-col gap-1.5"><label for="ce-cert" class="text-sm font-medium text-muted-foreground">Certificate Type</label>
				<select id="ce-cert" class="h-9 w-full rounded-md border border-input bg-secondary px-3 text-sm focus:outline-none focus:ring-1 focus:ring-ring" bind:value={eCert}>
					<option value="none">None</option><option value="letsencrypt">Let's Encrypt</option><option value="custom">Custom</option>
				</select></div>
			{/if}
			{#if eError}<div class="rounded-md bg-destructive/10 border border-destructive/30 px-3 py-2 text-sm text-destructive">{eError}</div>{/if}
			<div class="flex justify-end pt-1">
				<button type="submit" disabled={eSaving} class="inline-flex items-center gap-2 px-5 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50">
					{#if eSaving}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>Saving…{:else}Save Changes{/if}
				</button>
			</div>
		</form>
	</div>
</div>
{/if}
