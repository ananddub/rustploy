<script lang="ts">
	import { X } from '@lucide/svelte';
	import { composeControllerCreate } from '$lib/client/sdk.gen';
	import type { ComposeResponseDto } from '$lib/client/types.gen';

	type Props = { environmentId: number; onClose: () => void; onCreated: (c: ComposeResponseDto) => void };
	let { environmentId, onClose, onCreated }: Props = $props();
	let name = $state('');
	let desc = $state('');
	let loading = $state(false);
	let error = $state('');

	async function submit(e: SubmitEvent) {
		e.preventDefault(); error = ''; loading = true;
		try {
			const res = await composeControllerCreate({ body: { name, description: desc || undefined, compose_file: '', compose_type: 'DOCKER-COMPOSE', source_type: 'RAW', environment_id: environmentId } });
			if (res.error || !res.data) throw new Error((res.error as any)?.message ?? 'Failed to create');
			onCreated(res.data); onClose();
		} catch (err) { error = err instanceof Error ? err.message : 'Something went wrong'; }
		finally { loading = false; }
	}
</script>

<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1" aria-label="Close" onclick={onClose} onkeydown={() => {}}></div>
<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
	<div class="bg-card border border-border rounded-lg w-full max-w-md shadow-2xl pointer-events-auto">
		<div class="flex items-start justify-between px-5 py-4 border-b border-border">
			<h2 class="font-semibold">Create Compose</h2>
			<button class="text-muted-foreground hover:text-foreground p-0.5 rounded hover:bg-accent" onclick={onClose}><X size={16} /></button>
		</div>
		<form onsubmit={submit} class="px-5 py-4 flex flex-col gap-4">
			<div class="flex flex-col gap-1.5">
				<label class="text-sm font-medium text-muted-foreground" for="createcomposemodal-1">Name <span class="text-destructive">*</span></label>

				<input id="createcomposemodal-1"  class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring" placeholder="my-compose" bind:value={name} required />
			</div>
			<div class="flex flex-col gap-1.5">
				<label class="text-sm font-medium text-muted-foreground" for="createcomposemodal-2">Description</label>

				<input id="createcomposemodal-2"  class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring" placeholder="Optional" bind:value={desc} />
			</div>
			{#if error}<div class="rounded-md bg-destructive/10 border border-destructive/30 px-3 py-2 text-sm text-destructive">{error}</div>{/if}
			<div class="flex justify-end gap-2 pt-1">
				<button type="button" class="px-3 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent transition-colors" onclick={onClose} disabled={loading}>Cancel</button>
				<button type="submit" class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50" disabled={loading}>
					{#if loading}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>Creating…{:else}Create Compose{/if}
				</button>
			</div>
		</form>
	</div>
</div>
