<script lang="ts">
	import { Trash2 } from '@lucide/svelte';
	import { applicationControllerPatchResources } from '$lib/client/sdk.gen';
	import type { ApplicationResponseDto } from '$lib/client/types.gen';

	type Props = { app: ApplicationResponseDto; onUpdated: (a: ApplicationResponseDto) => void };
	let { app, onUpdated }: Props = $props();

	let memRes = $state('');
	let memLimit = $state('');
	let cpuRes = $state('');
	let cpuLimit = $state('');
	let replicas = $state('1');
	let saving = $state(false);

	async function save() {
		saving = true;
		try {
			const res = await applicationControllerPatchResources({
				path: { id: app.id },
				body: {
					...(memRes ? { memory_reservation: memRes } : {}),
					...(memLimit ? { memory_limit: memLimit } : {}),
					...(cpuRes ? { cpu_reservation: cpuRes } : {}),
					...(cpuLimit ? { cpu_limit: cpuLimit } : {}),
					replicas: parseInt(replicas) || 1
				}
			});
			if (res.data) onUpdated(res.data);
		} finally {
			saving = false;
		}
	}
</script>

<div class="flex flex-col gap-6">
	<section class="bg-card border border-border rounded-lg p-6">
		<h2 class="text-base font-semibold mb-1">Resource Limits</h2>
		<p class="text-sm text-muted-foreground mb-5">Configure CPU and Memory limits for this application.</p>
		<div class="grid grid-cols-2 gap-4">
			<div class="flex flex-col gap-1.5">
				<label class="text-sm font-medium text-muted-foreground" for="advancedtab-1">Memory Reservation</label>

				<input id="advancedtab-1"  class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring" placeholder="512m" bind:value={memRes} />
			</div>
			<div class="flex flex-col gap-1.5">
				<label class="text-sm font-medium text-muted-foreground" for="advancedtab-2">Memory Limit</label>

				<input id="advancedtab-2"  class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring" placeholder="1g" bind:value={memLimit} />
			</div>
			<div class="flex flex-col gap-1.5">
				<label class="text-sm font-medium text-muted-foreground" for="advancedtab-3">CPU Reservation</label>

				<input id="advancedtab-3"  class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring" placeholder="0.25" bind:value={cpuRes} />
			</div>
			<div class="flex flex-col gap-1.5">
				<label class="text-sm font-medium text-muted-foreground" for="advancedtab-4">CPU Limit</label>

				<input id="advancedtab-4"  class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring" placeholder="0.5" bind:value={cpuLimit} />
			</div>
		</div>
		<div class="flex flex-col gap-1.5 mt-4 w-32">
			<label class="text-sm font-medium text-muted-foreground" for="advancedtab-5">Replicas</label>

			<input id="advancedtab-5"  class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm focus:outline-none focus:ring-1 focus:ring-ring" type="number" min="1" bind:value={replicas} />
		</div>
		<div class="flex justify-end mt-4">
			<button class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50" onclick={save} disabled={saving}>
				{#if saving}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>Saving…{:else}Save{/if}
			</button>
		</div>
	</section>

	<section class="bg-card border border-destructive/30 rounded-lg p-6">
		<h2 class="text-base font-semibold text-destructive mb-1">Danger Zone</h2>
		<p class="text-sm text-muted-foreground mb-4">These actions are irreversible.</p>
		<button class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-destructive text-destructive-foreground text-sm font-medium hover:bg-destructive/90">
			<Trash2 class="w-3.5 h-3.5" /> Delete Application
		</button>
	</section>
</div>
