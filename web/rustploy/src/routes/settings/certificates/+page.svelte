<script lang="ts">
	import { goto } from '$app/navigation';
	import {
		ShieldCheck, Plus, Trash2, PenBox, Loader2,
		AlertTriangle, ChevronDown, ChevronRight, Link as LinkIcon, Server, AlertCircle
	} from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { toastSuccess, toastError } from '$lib/toast';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	type CertStatus = 'valid' | 'expiring' | 'expired';

	type Certificate = {
		id: string; name: string; commonName: string;
		server: string | null; isChain: boolean; chainCount: number;
		status: CertStatus; expiresAt: string; autoRenew: boolean;
		certificateData: string; privateKey: string; createdAt: string;
		chainDetails?: { label: string; commonName: string; message: string; className: string }[];
	};

	let certificates = $state<Certificate[]>([
		{
			id:'c1', name:'Wildcard Production', commonName:'*.example.com',
			server:null, isChain:true, chainCount:3,
			status:'valid', expiresAt:'2027-03-15', autoRenew:true,
			certificateData:'', privateKey:'', createdAt:'2026-01-15T10:00:00Z',
			chainDetails:[
				{ label:'Certificate 1 (Leaf)', commonName:'*.example.com', message:'Expires March 15, 2027', className:'text-muted-foreground' },
				{ label:'Certificate 2 (Intermediate)', commonName:"Let's Encrypt R3", message:'Expires September 15, 2025', className:'text-yellow-500' },
				{ label:'Certificate 3 (Root)', commonName:'ISRG Root X1', message:'Expires June 4, 2035', className:'text-muted-foreground' },
			]
		},
		{
			id:'c2', name:'API Certificate', commonName:'api.example.com',
			server:'prod-server (192.168.1.100)', isChain:false, chainCount:1,
			status:'valid', expiresAt:'2027-01-20', autoRenew:true,
			certificateData:'', privateKey:'', createdAt:'2026-03-01T08:00:00Z',
		},
		{
			id:'c3', name:'Staging Certificate', commonName:'staging.example.com',
			server:null, isChain:false, chainCount:1,
			status:'expiring', expiresAt:'2026-08-01', autoRenew:false,
			certificateData:'', privateKey:'', createdAt:'2026-05-10T12:00:00Z',
		},
	]);

	// ─── Chain expand ─────────────────────────────────────────────────────────────
	let expandedChains = $state<Set<string>>(new Set());
	function toggleChain(id: string) {
		const s = new Set(expandedChains);
		s.has(id) ? s.delete(id) : s.add(id);
		expandedChains = s;
	}

	// ─── Expiration helpers ───────────────────────────────────────────────────────
	function daysUntil(dateStr: string) {
		return Math.ceil((new Date(dateStr).getTime() - Date.now()) / 86400000);
	}

	// Match dokploy exactly: valid=muted-foreground, warning=yellow-500, expired=red-500
	function expirationClass(status: CertStatus) {
		if (status === 'expiring') return 'text-yellow-500';
		if (status === 'expired')  return 'text-red-500';
		return 'text-muted-foreground';
	}

	function expirationMsg(cert: Certificate) {
		const d = daysUntil(cert.expiresAt);
		const dateStr = new Date(cert.expiresAt).toLocaleDateString(undefined, { year:'numeric', month:'long', day:'numeric' });
		if (d < 0)  return `Expired on ${dateStr}`;
		if (d <= 30) return `Expires in ${d} days`;
		return `Expires ${dateStr}`;
	}

	function fmtDate(iso: string) {
		return new Date(iso).toLocaleDateString(undefined, { year:'numeric', month:'short', day:'numeric' });
	}

	// ─── Modal ────────────────────────────────────────────────────────────────────
	const CERT_PLACEHOLDER = `-----BEGIN CERTIFICATE-----\nMIIFRDCCAyygAwIBAgIUEPOR47ys6VDwMVB9tYoeEka83uQwDQYJ...\n-----END CERTIFICATE-----`;
	const KEY_PLACEHOLDER  = `-----BEGIN PRIVATE KEY-----\nMIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQC7...\n-----END PRIVATE KEY-----`;

	let showModal  = $state(false);
	let editingId  = $state<string|null>(null);
	let saving     = $state(false);
	let modalError = $state('');
	let fName      = $state('');
	let fCertData  = $state('');
	let fPrivKey   = $state('');

	function openCreate() {
		editingId=null; fName=''; fCertData=''; fPrivKey=''; modalError='';
		showModal=true;
	}

	function openEdit(c: Certificate) {
		editingId=c.id; fName=c.name; fCertData=c.certificateData; fPrivKey=c.privateKey; modalError='';
		showModal=true;
	}

	async function submitModal(e: SubmitEvent) {
		e.preventDefault(); modalError='';
		if (!fName.trim())    { modalError='Name is required'; return; }
		if (!fCertData.trim()){ modalError='Certificate data is required'; return; }
		if (!fPrivKey.trim()) { modalError='Private key is required'; return; }
		saving=true;
		try {
			await new Promise(r => setTimeout(r, 500));
			if (editingId) {
				certificates = certificates.map(c => c.id===editingId ? {...c, name:fName.trim(), certificateData:fCertData, privateKey:fPrivKey} : c);
				toastSuccess('Certificate updated');
			} else {
				certificates = [...certificates, {
					id:`c${Date.now()}`, name:fName.trim(), commonName:'', server:null,
					isChain:false, chainCount:1, status:'valid',
					expiresAt: new Date(Date.now()+365*86400000).toISOString().slice(0,10),
					autoRenew:false, certificateData:fCertData, privateKey:fPrivKey,
					createdAt:new Date().toISOString()
				}];
				toastSuccess('Certificate added');
			}
			showModal=false; editingId=null;
		} catch { modalError='Failed to save'; }
		finally { saving=false; }
	}

	// ─── Delete ───────────────────────────────────────────────────────────────────
	let confirmDeleteId = $state<string|null>(null);
	let deletingId      = $state<string|null>(null);

	async function deleteCertificate(id: string) {
		deletingId=id;
		await new Promise(r => setTimeout(r, 400));
		certificates = certificates.filter(c => c.id !== id);
		deletingId=null; confirmDeleteId=null;
		toastSuccess('Certificate deleted');
	}
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<ShieldCheck class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Certificates</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto">
		<div class="w-full">
			<div class="rounded-xl border border-border bg-card">

				<!-- Header -->
				<div class="px-6 pt-5 pb-4 border-b border-border">
					<h2 class="text-xl font-bold flex items-center gap-2">
						<ShieldCheck class="w-5 h-5 text-muted-foreground" /> Certificates
					</h2>
					<p class="text-sm text-muted-foreground mt-0.5">Create certificates in the Traefik directory</p>

					<!-- Warning -->
					<div class="flex items-start gap-2 mt-3 rounded-lg border border-yellow-500/30 bg-yellow-500/10 px-3 py-2.5 text-xs text-yellow-600 dark:text-yellow-400">
						<AlertTriangle class="w-3.5 h-3.5 shrink-0 mt-0.5" />
						<span>
							Certificates are created in the Traefik directory. Traefik uses these certificates to secure your applications.
							Using invalid certificates can break your Traefik instance, preventing access to your applications.
						</span>
					</div>
				</div>

				<!-- Content -->
				<div class="px-6 py-5">
					{#if certificates.length === 0}
						<div class="flex flex-col items-center gap-3 min-h-[25vh] justify-center text-muted-foreground">
							<ShieldCheck class="w-8 h-8 opacity-40" />
							<p class="text-base text-center">You don't have any certificates created</p>
							<Button size="sm" class="gap-1.5 mt-1" onclick={openCreate}>
								<Plus class="w-4 h-4" /> Add Certificate
							</Button>
						</div>
					{:else}
						<div class="flex flex-col gap-4 min-h-[25vh]">
							<div class="flex flex-col gap-4">
								{#each certificates as cert, i (cert.id)}
									<div class="bg-muted/40 p-1 rounded-lg">
										<div class="flex items-center justify-between p-4 rounded-lg bg-background border border-border">
											<!-- Left: info -->
											<div class="flex flex-col gap-1.5 min-w-0">
												<span class="text-sm font-semibold">{i + 1}. {cert.name}</span>

												{#if cert.commonName}
													<span class="text-xs text-muted-foreground">CN: {cert.commonName}</span>
												{/if}

												<span class="text-xs text-muted-foreground flex items-center gap-1">
													<Server class="w-3 h-3 shrink-0" />
													{cert.server ?? 'Rustploy (Local)'}
												</span>

												<!-- Chain toggle -->
												{#if cert.isChain}
													<div class="flex flex-col gap-1.5 mt-0.5">
														<button
															type="button"
															class="flex items-center gap-1 px-1.5 py-0.5 rounded bg-muted/50 w-fit hover:bg-muted transition-colors"
															onclick={() => toggleChain(cert.id)}
														>
															{#if expandedChains.has(cert.id)}
																<ChevronDown class="w-3 h-3 text-muted-foreground" />
															{:else}
																<ChevronRight class="w-3 h-3 text-muted-foreground" />
															{/if}
															<LinkIcon class="w-3 h-3 text-muted-foreground" />
															<span class="text-xs text-muted-foreground">Chain ({cert.chainCount} certificates)</span>
														</button>

														{#if expandedChains.has(cert.id) && cert.chainDetails}
															<div class="flex flex-col gap-2 pl-2 border-l-2 border-muted">
																{#each cert.chainDetails as detail, j (j)}
																	<div class="flex flex-col gap-0.5 p-2 rounded-md bg-muted/30">
																		<span class="text-xs font-medium text-muted-foreground">{detail.label}</span>
																		{#if detail.commonName}
																			<span class="text-xs text-muted-foreground/80">CN: {detail.commonName}</span>
																		{/if}
																		<span class="text-xs {detail.className}">{detail.message}</span>
																	</div>
																{/each}
															</div>
														{/if}
													</div>
												{/if}

												<!-- Expiration -->
												<div class="flex items-center gap-1.5 text-xs {expirationClass(cert.status)}">
													{#if cert.status !== 'valid'}
														<AlertCircle class="w-3 h-3 shrink-0" />
													{/if}
													{expirationMsg(cert)}
													{#if cert.autoRenew && cert.status !== 'valid'}
														<span class="text-emerald-500 ml-1">(Auto-renewal enabled)</span>
													{/if}
												</div>
											</div>

											<!-- Right: actions -->
											<div class="flex items-center gap-1 shrink-0 ml-4">
												<Button variant="ghost" size="icon" class="h-8 w-8" title="Edit"
													onclick={() => openEdit(cert)}>
													<PenBox class="w-4 h-4" />
												</Button>
												<Button variant="ghost" size="icon"
													class="h-8 w-8 group hover:bg-red-500/10"
													onclick={() => (confirmDeleteId = cert.id)}
													disabled={deletingId === cert.id} title="Delete">
													{#if deletingId === cert.id}
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
									<Plus class="w-4 h-4" /> Add Certificate
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
					<h2 class="text-base font-semibold">{editingId ? 'Edit' : 'Add'} Certificate</h2>
					<p class="text-sm text-muted-foreground mt-0.5">
						{editingId ? 'Update the certificate configuration' : 'Add a new TLS certificate'}
					</p>
				</div>
				<button type="button" onclick={() => { showModal=false; editingId=null; }}
					class="text-muted-foreground hover:text-foreground p-1 rounded hover:bg-accent">✕</button>
			</div>

			<!-- Name -->
			<div class="space-y-1.5">
				<Label for="c-name" class="text-sm">Name <span class="text-destructive">*</span></Label>
				<Input id="c-name" bind:value={fName} placeholder="My Certificate" required />
			</div>

			<!-- Certificate Data -->
			<div class="space-y-1.5">
				<Label for="c-cert" class="text-sm">
					Certificate Data <span class="text-destructive">*</span>
				</Label>
				<textarea
					id="c-cert"
					bind:value={fCertData}
					placeholder={CERT_PLACEHOLDER}
					rows={6}
					class="flex w-full rounded-md border border-input bg-transparent px-3 py-2 text-xs font-mono placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring resize-none"
					required
				></textarea>
				<p class="text-[10px] text-muted-foreground">Paste your PEM-encoded certificate (chain supported)</p>
			</div>

			<!-- Private Key -->
			<div class="space-y-1.5">
				<Label for="c-key" class="text-sm">
					Private Key <span class="text-destructive">*</span>
				</Label>
				<textarea
					id="c-key"
					bind:value={fPrivKey}
					placeholder={KEY_PLACEHOLDER}
					rows={6}
					class="flex w-full rounded-md border border-input bg-transparent px-3 py-2 text-xs font-mono placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring resize-none"
					required
				></textarea>
				<p class="text-[10px] text-muted-foreground">Paste your PEM-encoded private key</p>
			</div>

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
	{@const target = certificates.find(c => c.id === confirmDeleteId)}
	<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1"
		onclick={() => (confirmDeleteId=null)} onkeydown={() => {}}></div>
	<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
		<div class="bg-card border border-border rounded-xl shadow-2xl w-full max-w-sm p-6 pointer-events-auto flex flex-col gap-4">
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 rounded-full bg-destructive/10 flex items-center justify-center shrink-0">
					<Trash2 class="w-5 h-5 text-destructive" />
				</div>
				<div>
					<h2 class="text-sm font-semibold">Delete Certificate</h2>
					<p class="text-sm text-muted-foreground mt-0.5">
						Are you sure you want to delete <strong class="text-foreground">{target?.name}</strong>?
						This cannot be undone.
					</p>
				</div>
			</div>
			<div class="flex justify-end gap-2">
				<Button variant="outline" size="sm" onclick={() => (confirmDeleteId=null)}>Cancel</Button>
				<Button variant="destructive" size="sm"
					onclick={() => deleteCertificate(confirmDeleteId!)}
					disabled={deletingId !== null} class="gap-1.5">
					{#if deletingId}<Loader2 class="w-3.5 h-3.5 animate-spin" /> Deleting…
					{:else}<Trash2 class="w-3.5 h-3.5" /> Delete{/if}
				</Button>
			</div>
		</div>
	</div>
{/if}
