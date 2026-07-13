<script lang="ts">
	import { goto } from '$app/navigation';
	import { Database, FolderUp, Plus, Trash2, PenBox, Loader2 } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Select from '$lib/components/ui/select';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { toastSuccess, toastError } from '$lib/toast';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	const S3_PROVIDERS = [
		{ key:'AWS',          name:'Amazon Web Services (AWS) S3' },
		{ key:'Cloudflare',   name:'Cloudflare R2 Storage' },
		{ key:'DigitalOcean', name:'DigitalOcean Spaces' },
		{ key:'GCS',          name:'Google Cloud Storage' },
		{ key:'Wasabi',       name:'Wasabi Object Storage' },
		{ key:'Minio',        name:'Minio Object Storage' },
		{ key:'Linode',       name:'Linode Object Storage' },
		{ key:'Scaleway',     name:'Scaleway Object Storage' },
		{ key:'Backblaze',    name:'Backblaze B2' },
		{ key:'Storj',        name:'Storj (S3 Compatible Gateway)' },
		{ key:'IBMCOS',       name:'IBM COS S3' },
		{ key:'HuaweiOBS',    name:'Huawei Object Storage Service' },
		{ key:'TencentCOS',   name:'Tencent Cloud Object Storage (COS)' },
		{ key:'Alibaba',      name:'Alibaba Cloud OSS' },
		{ key:'ArvanCloud',   name:'Arvan Cloud Object Storage' },
		{ key:'IDrive',       name:'IDrive e2' },
		{ key:'IONOS',        name:'IONOS Cloud' },
		{ key:'Synology',     name:'Synology C2 Object Storage' },
		{ key:'SeaweedFS',    name:'SeaweedFS S3' },
		{ key:'Other',        name:'Any other S3 compatible provider' },
	];

	type Destination = {
		id: string; name: string; provider: string;
		accessKeyId: string; secretAccessKey: string;
		bucket: string; region: string; endpoint: string;
		createdAt: string;
		additionalFlags: string[];
	};

	let destinations = $state<Destination[]>([
		{ id:'d1', name:'AWS S3 Backups',       provider:'AWS',          accessKeyId:'AKIA...', secretAccessKey:'',   bucket:'rustploy-backups', region:'us-east-1',    endpoint:'s3.amazonaws.com',          createdAt:'2026-06-15T10:00:00Z', additionalFlags:[] },
		{ id:'d2', name:'Cloudflare R2 Backup', provider:'Cloudflare',   accessKeyId:'abc123',  secretAccessKey:'',   bucket:'r2-backup-bucket', region:'auto',         endpoint:'account.r2.cloudflarestorage.com', createdAt:'2026-07-01T08:00:00Z', additionalFlags:[] },
		{ id:'d3', name:'DigitalOcean Spaces',  provider:'DigitalOcean', accessKeyId:'DO00...', secretAccessKey:'',   bucket:'do-backup-space',  region:'nyc3',         endpoint:'nyc3.digitaloceanspaces.com', createdAt:'2026-07-05T12:00:00Z', additionalFlags:[] },
	]);

	function fmtDate(iso: string) {
		return new Date(iso).toLocaleDateString(undefined, { year:'numeric', month:'short', day:'numeric' });
	}

	function providerName(key: string) {
		return S3_PROVIDERS.find(p => p.key === key)?.name ?? key;
	}

	// ─── Modal ────────────────────────────────────────────────────────────────────
	let showModal       = $state(false);
	let editingId       = $state<string|null>(null);
	let saving          = $state(false);
	let modalError      = $state('');

	let fName           = $state('');
	let fProvider       = $state('AWS');
	let fAccessKey      = $state('');
	let fSecretKey      = $state('');
	let fBucket         = $state('');
	let fRegion         = $state('');
	let fEndpoint       = $state('');
	let fFlags          = $state<string[]>([]);

	function openCreate() {
		editingId=null; fName=''; fProvider='AWS'; fAccessKey=''; fSecretKey='';
		fBucket=''; fRegion=''; fEndpoint=''; fFlags=[]; modalError='';
		showModal=true;
	}

	function openEdit(d: Destination) {
		editingId=d.id; fName=d.name; fProvider=d.provider; fAccessKey=d.accessKeyId;
		fSecretKey=d.secretAccessKey; fBucket=d.bucket; fRegion=d.region;
		fEndpoint=d.endpoint; fFlags=[...d.additionalFlags]; modalError='';
		showModal=true;
	}

	function addFlag() { fFlags=[...fFlags, '']; }
	function removeFlag(i: number) { fFlags=fFlags.filter((_,idx)=>idx!==i); }
	function updateFlag(i: number, val: string) { fFlags=fFlags.map((f,idx)=>idx===i?val:f); }

	async function submitModal(e: SubmitEvent) {
		e.preventDefault(); modalError='';
		if (!fName.trim())     { modalError='Name is required'; return; }
		if (!fProvider)        { modalError='Provider is required'; return; }
		if (!fAccessKey.trim()){ modalError='Access Key ID is required'; return; }
		if (!fBucket.trim())   { modalError='Bucket is required'; return; }
		if (!fEndpoint.trim()) { modalError='Endpoint is required'; return; }
		saving=true;
		try {
			await new Promise(r => setTimeout(r, 500));
			if (editingId) {
				destinations = destinations.map(d => d.id===editingId
					? {...d, name:fName.trim(), provider:fProvider, accessKeyId:fAccessKey.trim(), secretAccessKey:fSecretKey, bucket:fBucket.trim(), region:fRegion.trim(), endpoint:fEndpoint.trim(), additionalFlags:fFlags.filter(f=>f.trim())}
					: d);
				toastSuccess('Destination updated');
			} else {
				destinations = [...destinations, {id:`d${Date.now()}`, name:fName.trim(), provider:fProvider, accessKeyId:fAccessKey.trim(), secretAccessKey:fSecretKey, bucket:fBucket.trim(), region:fRegion.trim(), endpoint:fEndpoint.trim(), createdAt:new Date().toISOString(), additionalFlags:fFlags.filter(f=>f.trim())}];
				toastSuccess('Destination added');
			}
			showModal=false; editingId=null;
		} catch { modalError='Failed to save'; }
		finally { saving=false; }
	}

	// ─── Delete ───────────────────────────────────────────────────────────────────
	let confirmDeleteId = $state<string|null>(null);
	let deletingId      = $state<string|null>(null);

	async function deleteDestination(id: string) {
		deletingId=id;
		await new Promise(r => setTimeout(r, 400));
		destinations = destinations.filter(d => d.id !== id);
		deletingId=null; confirmDeleteId=null;
		toastSuccess('Destination deleted');
	}
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<Database class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">S3 Destinations</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto">
		<div class="w-full max-w-5xl mx-auto">
			<div class="rounded-xl border border-border bg-card">

				<div class="px-6 pt-5 pb-4 border-b border-border">
					<h2 class="text-xl font-bold flex items-center gap-2">
						<Database class="w-5 h-5 text-muted-foreground" /> S3 Destinations
					</h2>
					<p class="text-sm text-muted-foreground mt-0.5">
						Add your providers like AWS S3, Cloudflare R2, Wasabi, DigitalOcean Spaces etc.
					</p>
				</div>

				<div class="px-6 py-5">
					{#if destinations.length === 0}
						<div class="flex flex-col items-center gap-3 min-h-[25vh] justify-center text-muted-foreground">
							<FolderUp class="w-8 h-8 opacity-40" />
							<p class="text-base text-center">To create a backup it is required to set at least 1 provider</p>
							<Button size="sm" class="gap-1.5 mt-1" onclick={openCreate}>
								<Plus class="w-4 h-4" /> Add Destination
							</Button>
						</div>
					{:else}
						<div class="flex flex-col gap-4 min-h-[25vh]">
							<div class="flex flex-col gap-4">
								{#each destinations as dest, i (dest.id)}
									<div class="bg-muted/40 p-1 rounded-lg">
										<div class="flex items-center justify-between p-4 rounded-lg bg-background border border-border">
											<div class="flex flex-col gap-1 min-w-0">
												<span class="text-sm font-semibold">{i + 1}. {dest.name}</span>
												<span class="text-xs text-muted-foreground">
													Created at: {fmtDate(dest.createdAt)}
												</span>
											</div>
											<div class="flex items-center gap-1 shrink-0">
												<Button variant="ghost" size="icon" class="h-8 w-8" title="Edit"
													onclick={() => openEdit(dest)}>
													<PenBox class="w-4 h-4" />
												</Button>
												<Button variant="ghost" size="icon"
													class="h-8 w-8 group hover:bg-red-500/10"
													onclick={() => (confirmDeleteId = dest.id)}
													disabled={deletingId === dest.id} title="Delete">
													{#if deletingId === dest.id}
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
									<Plus class="w-4 h-4" /> Add Destination
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
					<h2 class="text-base font-semibold">{editingId ? 'Update' : 'Add'} Destination</h2>
					<p class="text-xs text-muted-foreground mt-0.5">
						{editingId ? 'Update destination configuration' : 'Add a new S3-compatible storage destination'}
					</p>
				</div>
				<button type="button" onclick={() => { showModal=false; editingId=null; }}
					class="text-muted-foreground hover:text-foreground p-1 rounded hover:bg-accent">✕</button>
			</div>

			<!-- Name -->
			<div class="space-y-1.5">
				<Label for="d-name" class="text-xs">Name <span class="text-destructive">*</span></Label>
				<Input id="d-name" bind:value={fName} placeholder="AWS S3 Backups" required />
			</div>

			<!-- Provider -->
			<div class="space-y-1.5">
				<Label class="text-xs">Provider <span class="text-destructive">*</span></Label>
				<Select.Root type="single" value={fProvider} onValueChange={(v) => (fProvider = v ?? 'AWS')}>
					<Select.Trigger class="w-full">
						<span class="text-sm">{providerName(fProvider)}</span>
					</Select.Trigger>
					<Select.Content class="max-h-60">
						{#each S3_PROVIDERS as p (p.key)}
							<Select.Item value={p.key}>{p.name}</Select.Item>
						{/each}
					</Select.Content>
				</Select.Root>
			</div>

			<!-- Access Key + Secret -->
			<div class="grid grid-cols-2 gap-3">
				<div class="space-y-1.5">
					<Label for="d-ak" class="text-xs">Access Key ID <span class="text-destructive">*</span></Label>
					<Input id="d-ak" bind:value={fAccessKey} placeholder="AKIAIOSFODNN7EXAMPLE" required />
				</div>
				<div class="space-y-1.5">
					<Label for="d-sk" class="text-xs">Secret Access Key</Label>
					<Input id="d-sk" type="password" bind:value={fSecretKey} placeholder="Secret key" />
				</div>
			</div>

			<!-- Bucket -->
			<div class="space-y-1.5">
				<Label for="d-bucket" class="text-xs">Bucket <span class="text-destructive">*</span></Label>
				<Input id="d-bucket" bind:value={fBucket} placeholder="my-backup-bucket" required />
			</div>

			<!-- Region + Endpoint -->
			<div class="grid grid-cols-2 gap-3">
				<div class="space-y-1.5">
					<Label for="d-region" class="text-xs">Region</Label>
					<Input id="d-region" bind:value={fRegion} placeholder="us-east-1" />
				</div>
				<div class="space-y-1.5">
					<Label for="d-endpoint" class="text-xs">Endpoint <span class="text-destructive">*</span></Label>
					<Input id="d-endpoint" bind:value={fEndpoint} placeholder="s3.amazonaws.com" class="font-mono text-sm" required />
				</div>
			</div>

			<!-- Additional Flags -->
			<div class="space-y-2">
				<div class="flex items-center justify-between">
					<Label class="text-xs">Additional Flags <span class="text-muted-foreground font-normal">(optional)</span></Label>
					<Button type="button" variant="outline" size="sm" class="gap-1.5 h-7 text-xs" onclick={addFlag}>
						<Plus class="w-3 h-3" /> Add Flag
					</Button>
				</div>
				{#if fFlags.length > 0}
					<div class="flex flex-col gap-2">
						{#each fFlags as flag, i (i)}
							<div class="flex items-center gap-2">
								<Input
									value={flag}
									oninput={(e) => updateFlag(i, (e.target as HTMLInputElement).value)}
									placeholder="--flag=value"
									class="font-mono text-xs flex-1"
								/>
								<Button type="button" variant="ghost" size="icon" class="h-8 w-8 shrink-0 text-muted-foreground hover:text-destructive"
									onclick={() => removeFlag(i)}>
									<Trash2 class="w-3.5 h-3.5" />
								</Button>
							</div>
						{/each}
					</div>
				{/if}
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
	{@const target = destinations.find(d => d.id === confirmDeleteId)}
	<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1"
		onclick={() => (confirmDeleteId=null)} onkeydown={() => {}}></div>
	<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
		<div class="bg-card border border-border rounded-xl shadow-2xl w-full max-w-sm p-6 pointer-events-auto flex flex-col gap-4">
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 rounded-full bg-destructive/10 flex items-center justify-center shrink-0">
					<Trash2 class="w-5 h-5 text-destructive" />
				</div>
				<div>
					<h2 class="text-sm font-semibold">Delete Destination</h2>
					<p class="text-xs text-muted-foreground mt-0.5">
						Are you sure you want to delete <strong class="text-foreground">{target?.name}</strong>?
						This cannot be undone.
					</p>
				</div>
			</div>
			<div class="flex justify-end gap-2">
				<Button variant="outline" size="sm" onclick={() => (confirmDeleteId=null)}>Cancel</Button>
				<Button variant="destructive" size="sm"
					onclick={() => deleteDestination(confirmDeleteId!)}
					disabled={deletingId !== null} class="gap-1.5">
					{#if deletingId}<Loader2 class="w-3.5 h-3.5 animate-spin" /> Deleting…
					{:else}<Trash2 class="w-3.5 h-3.5" /> Delete{/if}
				</Button>
			</div>
		</div>
	</div>
{/if}
