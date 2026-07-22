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
		}).catch(() => {
			// Fallback mock org for demo
			org = {
				id: 'org-main',
				name: 'My Organization',
				slug: 'my-org',
				owner_id: 'user-01',
				created_at: Math.floor(Date.now() / 1000) - 86400 * 30
			} as any;
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
	<!-- Top Breadcrumb Header Bar -->
	<header class="flex items-center justify-between px-6 py-3 border-b border-[#262626] text-xs bg-[#0A0A0A] shrink-0">
		<div class="flex items-center gap-2">
			<Building2 class="w-3.5 h-3.5 text-[#a1a1aa]" />
			<span class="font-medium text-[#FAFAFA]">Organization Settings</span>
		</div>
	</header>

	<main class="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up min-h-0">
		{#if loading}
			<div class="flex items-center justify-center min-h-[40vh]">
				<Loader2 class="w-5 h-5 animate-spin text-[#a1a1aa]" />
			</div>
		{:else if org}
			<!-- Constrained Max-Width Shell (Dokploy Standard max-w-5xl) -->
			<div class="max-w-5xl space-y-6">
				<Card.Root class="bg-[#171717] border border-[#262626] rounded-xl shadow-md overflow-hidden">
					<Card.Header class="p-6">
						<Card.Title class="text-base font-semibold text-[#FAFAFA] flex items-center gap-2">
							<Building2 class="w-4 h-4 text-[#a1a1aa]" />
							Organization Details
						</Card.Title>
						<Card.Description class="text-xs text-[#a1a1aa] mt-1">Manage your organization name, slug, and branding</Card.Description>
					</Card.Header>
					<Card.Content class="p-6 pt-0 space-y-5">
						<div class="flex flex-wrap gap-3 text-xs">
							<div class="flex items-center gap-1.5 text-[#a1a1aa]">
								<span>Created:</span>
								<span class="font-medium text-[#FAFAFA]">{formatDate(org.created_at)}</span>
							</div>
							<div class="flex items-center gap-1.5 text-[#a1a1aa]">
								<span>Owner:</span>
								<Badge variant="outline" class="text-[10px] font-mono border-[#262626] text-[#FAFAFA] bg-[#262626]">{org.owner_id}</Badge>
							</div>
						</div>

						<Separator class="bg-[#262626]" />

						<div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
							<div class="space-y-1.5">
								<Label for="org-name" class="text-xs text-[#a1a1aa]">Organization Name</Label>
								<Input id="org-name" bind:value={formName} placeholder="My Organization" class="bg-[#141414] border-[#262626] text-[#FAFAFA]" />
							</div>
							<div class="space-y-1.5">
								<Label for="org-slug" class="text-xs text-[#a1a1aa]">Slug</Label>
								<Input id="org-slug" bind:value={formSlug} placeholder="my-org" class="bg-[#141414] border-[#262626] text-[#FAFAFA]" />
								<p class="text-[10px] text-[#737373]">Used in URLs. Lowercase, no spaces.</p>
							</div>
						</div>
						<div class="space-y-1.5">
							<Label for="org-logo" class="text-xs text-[#a1a1aa]">Logo URL</Label>
							<Input id="org-logo" bind:value={formLogo} placeholder="https://example.com/logo.png" class="bg-[#141414] border-[#262626] text-[#FAFAFA]" />
						</div>

						{#if error}
							<div class="rounded-md bg-red-500/10 border border-red-500/30 px-3 py-2 text-xs text-red-400">{error}</div>
						{/if}
					</Card.Content>
					<Card.Footer class="border-t border-[#262626] p-4 flex items-center justify-between bg-[#141414]">
						{#if saved}
							<span class="text-xs text-green-400 font-medium">✓ Saved successfully</span>
						{:else}
							<span></span>
						{/if}
						<Button onclick={save} disabled={saving || !formName.trim()} size="sm" class="gap-1.5 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A]">
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
				<Card.Root class="bg-[#171717] border border-red-500/20 rounded-xl shadow-md overflow-hidden">
					<Card.Header class="p-6">
						<Card.Title class="text-base font-semibold flex items-center gap-2 text-red-400">
							<Trash2 class="w-4 h-4" />
							Danger Zone
						</Card.Title>
						<Card.Description class="text-xs text-[#a1a1aa] mt-1">Irreversible actions for your organization</Card.Description>
					</Card.Header>
					<Card.Content class="p-6 pt-0">
						<div class="flex items-center justify-between">
							<div>
								<p class="text-xs font-semibold text-[#FAFAFA]">Delete Organization</p>
								<p class="text-[11px] text-[#737373]">Permanently delete this organization and all its data</p>
							</div>
							<Button variant="destructive" size="sm" class="text-xs gap-1.5 bg-red-500/20 text-red-400 border border-red-500/30 hover:bg-red-500/30">
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
