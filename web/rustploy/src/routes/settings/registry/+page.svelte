<script lang="ts">
	import { goto } from '$app/navigation';
	import { Package, Plus, Trash2, PenBox, Loader2, AlertTriangle } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { toastSuccess, toastError } from '$lib/toast';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	type Registry = { id: string; registryName: string; username: string; password: string; registryUrl: string; imagePrefix: string; };

	let registries = $state<Registry[]>([
		{ id:'r1', registryName:'Docker Hub',       username:'rustploy',     password:'', registryUrl:'',                     imagePrefix:'rustploy' },
		{ id:'r2', registryName:'GitHub GHCR',      username:'org-rustploy', password:'', registryUrl:'ghcr.io',              imagePrefix:'ghcr.io/org' },
		{ id:'r3', registryName:'Private Registry', username:'admin',        password:'', registryUrl:'registry.internal.io', imagePrefix:'registry.internal.io' },
	]);

	// ─── Modal ────────────────────────────────────────────────────────────────────
	let showModal  = $state(false);
	let editingId  = $state<string|null>(null);
	let saving     = $state(false);
	let testing    = $state(false);
	let testResult = $state<'ok'|'fail'|null>(null);
	let modalError = $state('');
	let fName='', fUser='', fPass='', fUrl='', fPrefix='';

	function openCreate() {
		editingId=null; fName=''; fUser=''; fPass=''; fUrl=''; fPrefix=''; modalError=''; testResult=null;
		showModal=true;
	}

	function openEdit(r: Registry) {
		editingId=r.id; fName=r.registryName; fUser=r.username; fPass=r.password;
		fUrl=r.registryUrl; fPrefix=r.imagePrefix; modalError=''; testResult=null;
		showModal=true;
	}

	function validateUrl(url: string) {
		if (!url?.trim()) return true;
		const t = url.trim();
		if (/^https?:\/\//i.test(t) || t.includes('/')) return false;
		return /^(?:\[[^\]]+\]|[a-zA-Z0-9](?:[a-zA-Z0-9._-]{0,253}[a-zA-Z0-9])?)(?::\d+)?$/.test(t);
	}

	async function testRegistry() {
		if (!fUser.trim() || !fPass.trim()) { modalError='Username and password are required to test'; return; }
		testing=true; testResult=null; modalError='';
		await new Promise(r => setTimeout(r, 1000));
		testResult='ok'; testing=false;
		toastSuccess('Registry connection successful');
	}

	async function submitModal(e: SubmitEvent) {
		e.preventDefault(); modalError='';
		if (!fName.trim()) { modalError='Registry name is required'; return; }
		if (!fUser.trim()) { modalError='Username is required'; return; }
		if (!validateUrl(fUrl)) { modalError='Invalid URL. Enter only the hostname (e.g. ghcr.io). No https:// or paths.'; return; }
		saving=true;
		try {
			await new Promise(r => setTimeout(r, 500));
			if (editingId) {
				registries = registries.map(r => r.id===editingId ? {...r, registryName:fName.trim(), username:fUser.trim(), password:fPass, registryUrl:fUrl.trim(), imagePrefix:fPrefix.trim()} : r);
				toastSuccess('Registry updated');
			} else {
				registries = [...registries, {id:`r${Date.now()}`, registryName:fName.trim(), username:fUser.trim(), password:fPass, registryUrl:fUrl.trim(), imagePrefix:fPrefix.trim()}];
				toastSuccess('Registry added');
			}
			showModal=false; editingId=null;
		} catch { modalError='Failed to save'; }
		finally { saving=false; }
	}

	// ─── Delete ───────────────────────────────────────────────────────────────────
	let confirmDeleteId = $state<string|null>(null);
	let deletingId      = $state<string|null>(null);

	async function deleteRegistry(id: string) {
		deletingId=id;
		await new Promise(r => setTimeout(r, 400));
		registries = registries.filter(r => r.id !== id);
		deletingId=null; confirmDeleteId=null;
		toastSuccess('Registry deleted');
	}
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<Package class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Docker Registry</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto">
		<div class="w-full max-w-5xl mx-auto">
			<div class="rounded-xl border border-border bg-card">

				<div class="px-6 pt-5 pb-4 border-b border-border">
					<h2 class="text-xl font-bold flex items-center gap-2">
						<Package class="w-5 h-5 text-muted-foreground" /> Docker Registry
					</h2>
					<p class="text-sm text-muted-foreground mt-0.5">Manage your Docker Registry configurations</p>
				</div>

				<div class="px-6 py-5">
					{#if registries.length === 0}
						<div class="flex flex-col items-center gap-3 min-h-[25vh] justify-center text-muted-foreground">
							<Package class="w-8 h-8 opacity-40" />
							<p class="text-base text-center">You don't have any registry configurations</p>
							<Button size="sm" class="gap-1.5 mt-1" onclick={openCreate}>
								<Plus class="w-4 h-4" /> Add Registry
							</Button>
						</div>
					{:else}
						<div class="flex flex-col gap-4 min-h-[25vh]">
							<div class="flex flex-col gap-4">
								{#each registries as reg, i (reg.id)}
									<div class="bg-muted/40 p-1 rounded-lg">
										<div class="flex items-center justify-between p-4 rounded-lg bg-background border border-border">
											<div class="flex flex-col gap-1 min-w-0">
												<span class="text-sm font-semibold">{i + 1}. {reg.registryName}</span>
												{#if reg.registryUrl}
													<span class="text-xs text-muted-foreground font-mono">{reg.registryUrl}</span>
												{/if}
											</div>
											<div class="flex items-center gap-1 shrink-0">
												<Button variant="ghost" size="icon" class="h-8 w-8" title="Edit"
													onclick={() => openEdit(reg)}>
													<PenBox class="w-4 h-4" />
												</Button>
												<Button variant="ghost" size="icon"
													class="h-8 w-8 group hover:bg-red-500/10"
													onclick={() => (confirmDeleteId = reg.id)}
													disabled={deletingId === reg.id} title="Delete">
													{#if deletingId === reg.id}
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
									<Plus class="w-4 h-4" /> Add Registry
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
					<h2 class="text-base font-semibold">{editingId ? 'Update' : 'Add'} Docker Registry</h2>
					<p class="text-xs text-muted-foreground mt-0.5">{editingId ? 'Update registry configuration' : 'Add a new Docker registry configuration'}</p>
				</div>
				<button type="button" onclick={() => { showModal=false; editingId=null; }}
					class="text-muted-foreground hover:text-foreground p-1 rounded hover:bg-accent">✕</button>
			</div>

			<div class="space-y-1.5">
				<Label for="r-name" class="text-xs">Registry Name <span class="text-destructive">*</span></Label>
				<Input id="r-name" bind:value={fName} placeholder="e.g. Docker Hub" required />
			</div>

			<div class="space-y-1.5">
				<Label for="r-user" class="text-xs">Username <span class="text-destructive">*</span></Label>
				<Input id="r-user" bind:value={fUser} placeholder="registry-username" required />
			</div>

			<div class="space-y-1.5">
				<Label for="r-pass" class="text-xs">Password</Label>
				<Input id="r-pass" type="password" bind:value={fPass} placeholder="Password or access token" />
			</div>

			<div class="space-y-1.5">
				<Label for="r-url" class="text-xs">Registry URL <span class="text-muted-foreground font-normal">(optional)</span></Label>
				<Input id="r-url" bind:value={fUrl} placeholder="e.g. ghcr.io" class="font-mono text-sm" />
				<p class="text-[10px] text-muted-foreground">
					Hostname only — no <code class="bg-muted px-1 rounded">https://</code> and no paths. Leave empty for Docker Hub.
				</p>
			</div>

			<div class="space-y-1.5">
				<Label for="r-prefix" class="text-xs">Image Prefix <span class="text-muted-foreground font-normal">(optional)</span></Label>
				<Input id="r-prefix" bind:value={fPrefix} placeholder="e.g. ghcr.io/my-org" class="font-mono text-sm" />
				<p class="text-[10px] text-muted-foreground">Used as a prefix when pulling images from this registry</p>
			</div>

			{#if testResult === 'ok'}
				<div class="rounded-md bg-green-500/10 border border-green-500/20 px-3 py-2 text-xs text-green-500">
					✓ Registry connection successful
				</div>
			{:else if testResult === 'fail'}
				<div class="rounded-md bg-destructive/10 border border-destructive/20 px-3 py-2 text-xs text-destructive">
					✗ Registry connection failed
				</div>
			{/if}

			{#if modalError}
				<div class="flex items-start gap-2 rounded-md bg-yellow-500/10 border border-yellow-500/20 px-3 py-2 text-xs text-yellow-600 dark:text-yellow-400">
					<AlertTriangle class="w-3.5 h-3.5 shrink-0 mt-0.5" />{modalError}
				</div>
			{/if}

			<div class="flex items-center justify-between gap-2 pt-1">
				<Button type="button" variant="outline" size="sm" onclick={testRegistry} disabled={testing} class="gap-1.5">
					{#if testing}<Loader2 class="w-3.5 h-3.5 animate-spin" />{/if}
					Test Registry
				</Button>
				<div class="flex gap-2">
					<Button type="button" variant="outline" size="sm" onclick={() => { showModal=false; editingId=null; }}>Cancel</Button>
					<Button type="submit" size="sm" disabled={saving} class="gap-1.5 min-w-[80px]">
						{#if saving}<Loader2 class="w-3.5 h-3.5 animate-spin" />{/if}
						{editingId ? 'Update' : 'Add'}
					</Button>
				</div>
			</div>
		</form>
	</div>
{/if}

<!-- ── Delete Confirm ─────────────────────────────────────────────────────── -->
{#if confirmDeleteId}
	{@const target = registries.find(r => r.id === confirmDeleteId)}
	<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1"
		onclick={() => (confirmDeleteId=null)} onkeydown={() => {}}></div>
	<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
		<div class="bg-card border border-border rounded-xl shadow-2xl w-full max-w-sm p-6 pointer-events-auto flex flex-col gap-4">
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 rounded-full bg-destructive/10 flex items-center justify-center shrink-0">
					<Trash2 class="w-5 h-5 text-destructive" />
				</div>
				<div>
					<h2 class="text-sm font-semibold">Delete Registry</h2>
					<p class="text-xs text-muted-foreground mt-0.5">
						Are you sure you want to delete <strong class="text-foreground">{target?.registryName}</strong>?
						This cannot be undone.
					</p>
				</div>
			</div>
			<div class="flex justify-end gap-2">
				<Button variant="outline" size="sm" onclick={() => (confirmDeleteId=null)}>Cancel</Button>
				<Button variant="destructive" size="sm"
					onclick={() => deleteRegistry(confirmDeleteId!)}
					disabled={deletingId !== null} class="gap-1.5">
					{#if deletingId}<Loader2 class="w-3.5 h-3.5 animate-spin" /> Deleting…
					{:else}<Trash2 class="w-3.5 h-3.5" /> Delete{/if}
				</Button>
			</div>
		</div>
	</div>
{/if}
