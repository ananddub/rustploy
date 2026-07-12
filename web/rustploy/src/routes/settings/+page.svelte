<script lang="ts">
	import { goto } from '$app/navigation';
	import { getAuthSession } from '$lib/auth';
	import { organizationControllerGet, organizationControllerPatch } from '$lib/client/sdk.gen';
	import type { OrganizationResponseDto } from '$lib/client/types.gen';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { Building2, Save, Loader2, Trash2 } from '@lucide/svelte';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Separator } from '$lib/components/ui/separator';
	import { Badge } from '$lib/components/ui/badge';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	let org = $state<OrganizationResponseDto | null>(null);
	let loading = $state(true);
	let formName = $state('');
	let formSlug = $state('');
	let formLogo = $state('');
	let saving = $state(false);
	let saved = $state(false);
	let error = $state('');
	let synced = $state(false);

	$effect(() => {
		if (!session) return;
		organizationControllerGet({ path: { id: session.user.group_id } }).then((res: any) => {
			org = res.data ?? null;
			loading = false;
		});
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
			if (res.data) { org = res.data; saved = true; setTimeout(() => (saved = false), 2000); }
			else throw new Error('Failed to update');
		} catch (e: any) { error = e?.message ?? 'Something went wrong'; }
		finally { saving = false; }
	}

	const formatDate = (ts: number) => new Date(ts * 1000).toLocaleString(undefined, { month: 'short', day: 'numeric', year: 'numeric' });
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<Building2 class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Organization</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto animate-fade-up">
		{#if loading}
			<div class="flex items-center justify-center min-h-[40vh]">
				<Loader2 class="w-5 h-5 animate-spin text-muted-foreground" />
			</div>
		{:else if org}
			<div class="w-full space-y-6">
				<Card.Root>
					<Card.Header>
						<Card.Title class="text-base flex items-center gap-2">
							<Building2 class="w-4 h-4 text-muted-foreground" />
							Organization Settings
						</Card.Title>
						<Card.Description>Manage your organization name, slug, and branding</Card.Description>
					</Card.Header>
					<Card.Content class="space-y-5">
						<!-- Info badges -->
						<div class="flex flex-wrap gap-3 text-xs">
							<div class="flex items-center gap-1.5 text-muted-foreground">
								<span>Created:</span>
								<span class="font-medium text-foreground">{formatDate(org.created_at)}</span>
							</div>
							<div class="flex items-center gap-1.5 text-muted-foreground">
								<span>Owner:</span>
								<Badge variant="outline" class="text-[10px] font-mono">{org.owner_id}</Badge>
							</div>
						</div>

						<Separator />

						<!-- Form -->
						<div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
							<div class="space-y-1.5">
								<Label for="org-name" class="text-xs">Organization Name</Label>
								<Input id="org-name" bind:value={formName} placeholder="My Organization" />
							</div>
							<div class="space-y-1.5">
								<Label for="org-slug" class="text-xs">Slug</Label>
								<Input id="org-slug" bind:value={formSlug} placeholder="my-org" />
								<p class="text-[10px] text-muted-foreground">Used in URLs. Lowercase, no spaces.</p>
							</div>
						</div>
						<div class="space-y-1.5">
							<Label for="org-logo" class="text-xs">Logo URL</Label>
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
					</Card.Content>
					<Card.Footer class="border-t pt-4 flex items-center justify-between">
						{#if saved}
							<span class="text-xs text-green-500 font-medium">✓ Saved successfully</span>
						{:else}
							<span></span>
						{/if}
						<Button onclick={save} disabled={saving || !formName.trim()} size="sm" class="gap-1.5 text-xs">
							{#if saving}
								<Loader2 class="w-3.5 h-3.5 animate-spin" />
								Saving...
							{:else}
								<Save class="w-3.5 h-3.5" />
								Save Changes
							{/if}
						</Button>
					</Card.Footer>
				</Card.Root>

				<!-- Danger Zone -->
				<Card.Root class="border-destructive/20">
					<Card.Header>
						<Card.Title class="text-base flex items-center gap-2 text-destructive">
							<Trash2 class="w-4 h-4" />
							Danger Zone
						</Card.Title>
						<Card.Description>Irreversible actions for your organization</Card.Description>
					</Card.Header>
					<Card.Content>
						<div class="flex items-center justify-between">
							<div>
								<p class="text-sm font-medium">Delete Organization</p>
								<p class="text-[11px] text-muted-foreground">Permanently delete this organization and all its data</p>
							</div>
							<Button variant="destructive" size="sm" class="text-xs gap-1.5">
								<Trash2 class="w-3.5 h-3.5" />
								Delete
							</Button>
						</div>
					</Card.Content>
				</Card.Root>
			</div>
		{/if}
	</main>
</PageLayout>
