<script lang="ts">
	import { goto } from '$app/navigation';
	import { getAuthSession } from '$lib/auth';
	import { organizationControllerGet, organizationControllerPatch } from '$lib/client/sdk.gen';
	import type { OrganizationResponseDto } from '$lib/client/types.gen';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { Building2, Save } from '@lucide/svelte';

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

	const inputCls = 'flex h-9 w-full rounded-md border border-input bg-secondary px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring';
	const formatDate = (ts: number) => new Date(ts * 1000).toLocaleString(undefined, { month: 'short', day: 'numeric', year: 'numeric' });
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<Building2 class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Organization Settings</span>
	</header>

	<main class="flex-1 px-8 py-8 max-w-2xl">
		{#if loading}
			<div class="flex justify-center py-20">
				<div class="w-6 h-6 border-2 border-muted-foreground/30 border-t-foreground rounded-full animate-spin"></div>
			</div>
		{:else if org}
			<div class="flex flex-col gap-6">

				<!-- Info card -->
				<section class="bg-card border border-border rounded-lg p-6">
					<h2 class="text-base font-semibold mb-4">Organization Info</h2>
					<div class="grid grid-cols-2 gap-3">
						<div class="bg-secondary rounded-lg p-3">
							<p class="text-xs text-muted-foreground uppercase tracking-wide mb-1">Created</p>
							<p class="text-sm">{formatDate(org.created_at)}</p>
						</div>
						<div class="bg-secondary rounded-lg p-3">
							<p class="text-xs text-muted-foreground uppercase tracking-wide mb-1">Owner ID</p>
							<p class="text-sm font-mono">{org.owner_id}</p>
						</div>
					</div>
				</section>

				<!-- Edit form -->
				<section class="bg-card border border-border rounded-lg p-6">
					<h2 class="text-base font-semibold mb-1">General</h2>
					<p class="text-sm text-muted-foreground mb-5">Update your organization's display name and slug.</p>
					<div class="flex flex-col gap-4">
						<div class="flex flex-col gap-1.5">
							<label for="org-name" class="text-sm font-medium text-muted-foreground">Organization Name</label>
							<input id="org-name" class={inputCls} bind:value={formName} placeholder="My Organization" />
						</div>
						<div class="flex flex-col gap-1.5">
							<label for="org-slug" class="text-sm font-medium text-muted-foreground">Slug</label>
							<input id="org-slug" class={inputCls} bind:value={formSlug} placeholder="my-org" />
							<p class="text-xs text-muted-foreground">Used in URLs and identifiers. Lowercase, no spaces.</p>
						</div>
						<div class="flex flex-col gap-1.5">
							<label for="org-logo" class="text-sm font-medium text-muted-foreground">Logo URL</label>
							<input id="org-logo" class={inputCls} bind:value={formLogo} placeholder="https://example.com/logo.png" />
							{#if formLogo}
								<img src={formLogo} alt="Logo preview" class="w-12 h-12 rounded-lg object-cover mt-1 border border-border" />
							{/if}
						</div>
						{#if error}
							<div class="rounded-md bg-destructive/10 border border-destructive/30 px-3 py-2 text-sm text-destructive">{error}</div>
						{/if}
						<div class="flex justify-end items-center gap-3">
							{#if saved}<span class="text-sm text-green-500">Saved!</span>{/if}
							<button onclick={save} disabled={saving || !formName.trim()} class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50">
								{#if saving}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>Saving…{:else}<Save size={13} />Save{/if}
							</button>
						</div>
					</div>
				</section>
			</div>
		{/if}
	</main>
</PageLayout>
