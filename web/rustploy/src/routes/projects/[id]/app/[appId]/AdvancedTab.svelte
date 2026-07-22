<script lang="ts">
	import { Trash2, AlertTriangle } from '@lucide/svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import {
		applicationControllerPatchResources,
		applicationControllerPatch,
		applicationControllerDelete
	} from '$lib/client/sdk.gen';
	import type { ApplicationResponseDto } from '$lib/client/types.gen';

	type Props = { app: ApplicationResponseDto; onUpdated: (a: ApplicationResponseDto) => void };
	let { app, onUpdated }: Props = $props();

	const projectId = $derived(page.params.id ?? '');

	// Resources
	let memRes = $state(app.build_type ? '' : '');
	let memLimit = $state('');
	let cpuRes = $state('');
	let cpuLimit = $state('');
	let replicas = $state('1');
	let resSaving = $state(false);
	let resError = $state('');
	let resSaved = $state(false);

	async function saveResources() {
		resSaving = true; resError = ''; resSaved = false;
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
			if (res.data) { onUpdated(res.data); resSaved = true; setTimeout(() => (resSaved = false), 2000); }
		} catch (e: any) { resError = e?.message ?? 'Failed to save'; }
		finally { resSaving = false; }
	}

	// Delete
	let confirmDelete = $state(false);
	let deleting = $state(false);
	let deleteError = $state('');

	async function doDelete() {
		deleting = true; deleteError = '';
		try {
			await applicationControllerDelete({ path: { id: app.id } });
			goto(`/projects/${projectId}`);
		} catch (e: any) { deleteError = e?.message ?? 'Failed to delete'; deleting = false; }
	}

	const inputCls = 'flex h-9 w-full rounded-md border border-input bg-secondary px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring';
</script>

<div class="flex flex-col gap-6 animate-fade-up">

	<!-- Resource limits -->
	<section class="bg-card border border-border rounded-lg p-6">
		<h2 class="text-base font-semibold mb-1">Resource Limits</h2>
		<p class="text-sm text-muted-foreground mb-5">
			Configure CPU and Memory reservations/limits for this application.
			Use Docker format: <code class="font-mono text-xs bg-secondary px-1 rounded">512m</code>,
			<code class="font-mono text-xs bg-secondary px-1 rounded">1g</code>,
			<code class="font-mono text-xs bg-secondary px-1 rounded">0.5</code>
		</p>
		<div class="grid grid-cols-2 gap-4">
			<div class="flex flex-col gap-1.5">
				<label for="mem-res" class="text-sm font-medium text-muted-foreground">Memory Reservation</label>
				<input id="mem-res" class={inputCls} placeholder="512m" bind:value={memRes} />
				<p class="text-sm text-muted-foreground">Soft limit — guaranteed minimum</p>
			</div>
			<div class="flex flex-col gap-1.5">
				<label for="mem-lim" class="text-sm font-medium text-muted-foreground">Memory Limit</label>
				<input id="mem-lim" class={inputCls} placeholder="1g" bind:value={memLimit} />
				<p class="text-sm text-muted-foreground">Hard limit — container will be OOM-killed if exceeded</p>
			</div>
			<div class="flex flex-col gap-1.5">
				<label for="cpu-res" class="text-sm font-medium text-muted-foreground">CPU Reservation</label>
				<input id="cpu-res" class={inputCls} placeholder="0.25" bind:value={cpuRes} />
				<p class="text-sm text-muted-foreground">Fractional CPU units (e.g. 0.5 = half a core)</p>
			</div>
			<div class="flex flex-col gap-1.5">
				<label for="cpu-lim" class="text-sm font-medium text-muted-foreground">CPU Limit</label>
				<input id="cpu-lim" class={inputCls} placeholder="0.5" bind:value={cpuLimit} />
			</div>
		</div>
		<div class="flex flex-col gap-1.5 mt-4 w-40">
			<label for="replicas" class="text-sm font-medium text-muted-foreground">Replicas</label>
			<input id="replicas" class={inputCls} type="number" min="1" max="99" bind:value={replicas} />
			<p class="text-sm text-muted-foreground">Number of container instances</p>
		</div>
		{#if resError}<div class="mt-4 rounded-md bg-destructive/10 border border-destructive/30 px-3 py-2 text-sm text-destructive">{resError}</div>{/if}
		<div class="flex justify-end items-center gap-3 mt-4">
			{#if resSaved}<span class="text-sm text-green-500">Saved!</span>{/if}
			<button onclick={saveResources} disabled={resSaving} class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50">
				{#if resSaving}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>Saving…{:else}Save{/if}
			</button>
		</div>
	</section>

	<!-- Danger Zone -->
	<section class="bg-card border border-destructive/40 rounded-lg p-6">
		<div class="flex items-center gap-2 mb-1">
			<AlertTriangle size={16} class="text-destructive" />
			<h2 class="text-base font-semibold text-destructive">Danger Zone</h2>
		</div>
		<p class="text-sm text-muted-foreground mb-5">These actions are permanent and cannot be undone.</p>

		{#if !confirmDelete}
			<button
				onclick={() => (confirmDelete = true)}
				class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md border border-destructive/50 text-destructive text-sm font-medium hover:bg-destructive hover:text-destructive-foreground transition-colors"
			>
				<Trash2 size={14} /> Delete Application
			</button>
		{:else}
			<div class="bg-destructive/10 border border-destructive/30 rounded-lg p-4 flex flex-col gap-3">
				<p class="text-sm font-medium text-destructive">
					Are you sure you want to delete <strong>{app.name}</strong>? All deployments, domains, and data will be permanently removed.
				</p>
				{#if deleteError}<p class="text-xs text-destructive">{deleteError}</p>{/if}
				<div class="flex items-center gap-2">
					<button
						onclick={doDelete}
						disabled={deleting}
						class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-destructive text-destructive-foreground text-sm font-medium hover:bg-destructive/90 disabled:opacity-50"
					>
						{#if deleting}<div class="w-3.5 h-3.5 border-2 border-destructive-foreground/30 border-t-destructive-foreground rounded-full animate-spin"></div>Deleting…{:else}<Trash2 size={14} />Yes, Delete{/if}
					</button>
					<button onclick={() => (confirmDelete = false)} class="px-3 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent transition-colors">
						Cancel
					</button>
				</div>
			</div>
		{/if}
	</section>
</div>
