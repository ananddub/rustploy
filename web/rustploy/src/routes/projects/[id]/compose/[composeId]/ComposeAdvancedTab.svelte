<script lang="ts">
	import { Trash2, AlertTriangle } from '@lucide/svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { composeControllerDelete } from '$lib/client/sdk.gen';
	import type { ComposeResponseDto } from '$lib/client/types.gen';

	type Props = { compose: ComposeResponseDto; onUpdated: (c: ComposeResponseDto) => void };
	let { compose }: Props = $props();

	const projectId = $derived(page.params.id ?? '');

	let confirmDelete = $state(false);
	let deleting = $state(false);
	let deleteError = $state('');

	async function doDelete() {
		deleting = true; deleteError = '';
		try {
			await composeControllerDelete({ path: { id: compose.id } });
			goto(`/projects/${projectId}`);
		} catch (e: any) { deleteError = e?.message ?? 'Failed to delete'; deleting = false; }
	}
</script>

<div class="flex flex-col gap-6 animate-fade-up">
	<section class="bg-card border border-border rounded-lg p-6">
		<h2 class="text-base font-semibold mb-1">Compose Info</h2>
		<p class="text-sm text-muted-foreground mb-4">Details about this compose service.</p>
		<div class="grid grid-cols-2 gap-4 text-sm">
			<div class="bg-secondary rounded-lg p-3">
				<p class="text-sm text-muted-foreground uppercase tracking-wide mb-1">App Name</p>
				<p class="font-mono">{compose.app_name}</p>
			</div>
			<div class="bg-secondary rounded-lg p-3">
				<p class="text-sm text-muted-foreground uppercase tracking-wide mb-1">Compose Type</p>
				<p>{compose.compose_type}</p>
			</div>
			<div class="bg-secondary rounded-lg p-3">
				<p class="text-sm text-muted-foreground uppercase tracking-wide mb-1">Source Type</p>
				<p>{compose.source_type}</p>
			</div>
			<div class="bg-secondary rounded-lg p-3">
				<p class="text-sm text-muted-foreground uppercase tracking-wide mb-1">Trigger Type</p>
				<p>{compose.trigger_type}</p>
			</div>
		</div>
	</section>

	<section class="bg-card border border-destructive/40 rounded-lg p-6">
		<div class="flex items-center gap-2 mb-1">
			<AlertTriangle size={16} class="text-destructive" />
			<h2 class="text-base font-semibold text-destructive">Danger Zone</h2>
		</div>
		<p class="text-sm text-muted-foreground mb-5">These actions are permanent and cannot be undone.</p>

		{#if !confirmDelete}
			<button onclick={() => (confirmDelete = true)} class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md border border-destructive/50 text-destructive text-sm font-medium hover:bg-destructive hover:text-destructive-foreground transition-colors">
				<Trash2 size={14} /> Delete Compose
			</button>
		{:else}
			<div class="bg-destructive/10 border border-destructive/30 rounded-lg p-4 flex flex-col gap-3">
				<p class="text-sm font-medium text-destructive">
					Are you sure you want to delete <strong>{compose.name}</strong>? All deployments and domains will be permanently removed.
				</p>
				{#if deleteError}<p class="text-xs text-destructive">{deleteError}</p>{/if}
				<div class="flex items-center gap-2">
					<button onclick={doDelete} disabled={deleting} class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-destructive text-destructive-foreground text-sm font-medium hover:bg-destructive/90 disabled:opacity-50">
						{#if deleting}<div class="w-3.5 h-3.5 border-2 border-destructive-foreground/30 border-t-destructive-foreground rounded-full animate-spin"></div>Deleting…{:else}<Trash2 size={14} />Yes, Delete{/if}
					</button>
					<button onclick={() => (confirmDelete = false)} class="px-3 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent transition-colors">Cancel</button>
				</div>
			</div>
		{/if}
	</section>
</div>
