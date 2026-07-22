<script lang="ts">
	import { goto } from '$app/navigation';
	import { getAuthSession } from '$lib/auth';
	import { organizationControllerGet, organizationControllerPatch } from '$lib/client/sdk.gen';
	import type { OrganizationResponseDto } from '$lib/client/types.gen';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { Building2, Save, Loader2, Trash2, AlertTriangle } from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Separator } from '$lib/components/ui/separator';
	import { Badge } from '$lib/components/ui/badge';
	import { toastSuccess, toastError } from '$lib/toast';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	let org      = $state<OrganizationResponseDto | null>(null);
	let loading  = $state(true);
	let formName = $state('');
	let formSlug = $state('');
	let formLogo = $state('');
	let saving   = $state(false);
	let saved    = $state(false);
	let error    = $state('');
	let synced   = $state(false);

	// ─── Delete confirm ───────────────────────────────────────────────────────────
	let confirmDelete = $state(false);
	let deleting      = $state(false);

	$effect(() => {
		if (!session) return;
		organizationControllerGet({ path: { id: session.user.group_id } }).then((res: any) => {
			org = res.data ?? null;
			loading = false;
		}).catch(() => { loading = false; });
	});

	$effect(() => {
		if (org && !synced) {
			formName = org.name ?? '';
			formSlug = org.slug ?? '';
			formLogo = org.logo ?? '';
			synced = true;
		}
	});

	async function save() {
		saving = true; error = ''; saved = false;
		try {
			const res = await organizationControllerPatch({
				path: { id: org!.id },
				body: { name: formName || undefined, slug: formSlug || undefined, logo: formLogo || undefined }
			});
			if (res.data) {
				org = res.data; saved = true;
				toastSuccess('Organization settings saved');
				setTimeout(() => (saved = false), 2500);
			} else throw new Error('Failed to update');
		} catch (e: any) {
			error = e?.message ?? 'Something went wrong';
			toastError(error);
		} finally { saving = false; }
	}

	async function deleteOrg() {
		deleting = true;
		await new Promise(r => setTimeout(r, 800));
		deleting = false;
		confirmDelete = false;
		toastError('Delete organization not yet implemented');
	}

	function fmtDate(ts: number) {
		return new Date(ts * 1000).toLocaleString(undefined, { month:'short', day:'numeric', year:'numeric' });
	}
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<Building2 class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Organization</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto">
		{#if loading}
			<div class="flex items-center justify-center min-h-[40vh]">
				<div class="flex items-center gap-2 text-muted-foreground text-sm">
					<Loader2 class="w-5 h-5 animate-spin" />
					<span>Loading...</span>
				</div>
			</div>
		{:else if org}
			<div class="w-full max-w-5xl mx-auto flex flex-col gap-6">

				<!-- ── Organization Settings ────────────────────────────────── -->
				<div class="rounded-xl border border-border bg-card">
					<div class="px-6 pt-5 pb-4 border-b border-border">
						<h2 class="text-xl font-bold flex items-center gap-2">
							<Building2 class="w-5 h-5 text-muted-foreground" /> Organization Settings
						</h2>
						<p class="text-sm text-muted-foreground mt-0.5">Manage your organization name, slug, and branding</p>
					</div>

					<div class="px-6 py-5 space-y-5">
						<!-- Meta info -->
						<div class="flex flex-wrap gap-4 text-xs text-muted-foreground">
							<div class="flex items-center gap-1.5">
								<span>Created:</span>
								<span class="font-medium text-foreground">{fmtDate(org.created_at)}</span>
							</div>
							<div class="flex items-center gap-1.5">
								<span>Owner ID:</span>
								<Badge variant="outline" class="text-[10px] font-mono">{org.owner_id}</Badge>
							</div>
							{#if org.slug}
								<div class="flex items-center gap-1.5">
									<span>Slug:</span>
									<Badge variant="secondary" class="text-[10px] font-mono">{org.slug}</Badge>
								</div>
							{/if}
						</div>

						<Separator />

						<!-- Form -->
						<div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
							<div class="space-y-1.5">
								<Label for="org-name" class="text-sm">Organization Name</Label>
								<Input id="org-name" bind:value={formName} placeholder="My Organization" />
							</div>
							<div class="space-y-1.5">
								<Label for="org-slug" class="text-sm">Slug</Label>
								<Input id="org-slug" bind:value={formSlug} placeholder="my-org" />
								<p class="text-[10px] text-muted-foreground">Used in URLs. Lowercase, no spaces.</p>
							</div>
						</div>

						<div class="space-y-1.5">
							<Label for="org-logo" class="text-sm">Logo URL</Label>
							<Input id="org-logo" bind:value={formLogo} placeholder="https://example.com/logo.png" />
							{#if formLogo}
								<div class="flex items-center gap-3 mt-2">
									<img src={formLogo} alt="Logo preview" class="w-10 h-10 rounded-lg object-cover border border-border" />
									<span class="text-[11px] text-muted-foreground">Preview</span>
								</div>
							{/if}
						</div>

						{#if error}
							<div class="rounded-md bg-destructive/10 border border-destructive/30 px-3 py-2 text-xs text-destructive">{error}</div>
						{/if}
					</div>

					<!-- Footer -->
					<div class="px-6 py-4 border-t border-border flex items-center justify-between">
						{#if saved}
							<span class="text-xs text-green-500 font-medium">✓ Saved successfully</span>
						{:else}
							<span></span>
						{/if}
						<Button onclick={save} disabled={saving || !formName.trim()} size="sm" class="gap-1.5">
							{#if saving}
								<Loader2 class="w-3.5 h-3.5 animate-spin" /> Saving...
							{:else}
								<Save class="w-3.5 h-3.5" /> Save Changes
							{/if}
						</Button>
					</div>
				</div>

				<!-- ── Danger Zone ──────────────────────────────────────────── -->
				<div class="rounded-xl border border-destructive/30 bg-card">
					<div class="px-6 pt-5 pb-4 border-b border-destructive/20">
						<h2 class="text-xl font-bold flex items-center gap-2 text-destructive">
							<Trash2 class="w-5 h-5" /> Danger Zone
						</h2>
						<p class="text-sm text-muted-foreground mt-0.5">Irreversible and destructive actions</p>
					</div>
					<div class="px-6 py-5">
						<div class="flex items-center justify-between">
							<div>
								<p class="text-sm font-medium">Delete Organization</p>
								<p class="text-sm text-muted-foreground mt-0.5">
									Permanently delete this organization, all projects, services, and associated data.
									This action cannot be undone.
								</p>
							</div>
							<Button variant="destructive" size="sm" class="gap-1.5 shrink-0 ml-4"
								onclick={() => (confirmDelete = true)}>
								<Trash2 class="w-3.5 h-3.5" /> Delete Organization
							</Button>
						</div>
					</div>
				</div>

			</div>
		{/if}
	</main>
</PageLayout>

<!-- ── Delete Confirm ─────────────────────────────────────────────────────── -->
{#if confirmDelete}
	<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1"
		onclick={() => (confirmDelete=false)} onkeydown={() => {}}></div>
	<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
		<div class="bg-card border border-border rounded-xl shadow-2xl w-full max-w-md pointer-events-auto flex flex-col gap-4 p-6">
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 rounded-full bg-destructive/10 flex items-center justify-center shrink-0">
					<AlertTriangle class="w-5 h-5 text-destructive" />
				</div>
				<div>
					<h2 class="text-sm font-semibold">Delete Organization</h2>
					<p class="text-sm text-muted-foreground mt-0.5">
						Are you sure you want to delete <strong class="text-foreground">{org?.name}</strong>?
						This will permanently delete all projects, services, and data. This cannot be undone.
					</p>
				</div>
			</div>
			<div class="flex justify-end gap-2">
				<Button variant="outline" size="sm" onclick={() => (confirmDelete=false)}>Cancel</Button>
				<Button variant="destructive" size="sm" onclick={deleteOrg} disabled={deleting} class="gap-1.5">
					{#if deleting}<Loader2 class="w-3.5 h-3.5 animate-spin" /> Deleting…
					{:else}<Trash2 class="w-3.5 h-3.5" /> Delete Organization{/if}
				</Button>
			</div>
		</div>
	</div>
{/if}
