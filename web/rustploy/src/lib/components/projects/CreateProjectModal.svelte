<script lang="ts">
	import { X } from '@lucide/svelte';
	import { projectControllerCreate } from '$lib/client/sdk.gen';
	import { getAuthSession } from '$lib/auth';
	import type { ProjectResponseDto } from '$lib/client/types.gen';

	type Props = {
		onClose: () => void;
		onCreated: (p: ProjectResponseDto) => void;
	};

	let { onClose, onCreated }: Props = $props();

	let name = $state('');
	let description = $state('');
	let envVar = $state('');
	let loading = $state(false);
	let error = $state('');

	async function submit(e: SubmitEvent) {
		e.preventDefault();
		error = '';
		loading = true;
		try {
			const session = getAuthSession()!;
			const res = await projectControllerCreate({
				body: {
					name,
					description: description || undefined,
					env_var: envVar,
					organization_id: session.user.group_id
				}
			});
			if (res.error || !res.data) throw new Error((res.error as any)?.message ?? 'Failed to create');
			onCreated(res.data);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Something went wrong';
		} finally {
			loading = false;
		}
	}
</script>

<!-- Backdrop -->
<div
	class="fixed inset-0 bg-black/60 z-40"
	role="button"
	tabindex="-1"
	aria-label="Close"
	onclick={onClose}
	onkeydown={() => {}}
></div>

<!-- Dialog -->
<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
	<div
		class="bg-card border border-border rounded-lg w-full max-w-md shadow-2xl flex flex-col pointer-events-auto"
	>
		<div class="flex items-start justify-between px-5 py-4 border-b border-border shrink-0">
			<h2 class="font-semibold">Create Project</h2>
			<button
				class="text-muted-foreground hover:text-foreground transition-colors mt-0.5 ml-4 shrink-0 rounded p-0.5 hover:bg-accent"
				onclick={onClose}
			>
				<X size={16} />
			</button>
		</div>

		<form onsubmit={submit} class="px-5 py-4 flex flex-col gap-4">
			<div class="flex flex-col gap-1.5">
				<label class="text-sm font-medium text-muted-foreground"
					 for="createprojectmodal-1">Name <span class="text-destructive">*</span></label
				>
				<input
					class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring"
					placeholder="my-project"
					bind:value={name}
					required
				/>
			</div>

			<div class="flex flex-col gap-1.5">
				<label class="text-sm font-medium text-muted-foreground">Description</label>

				<textarea id="createprojectmodal-1" 
					class="flex w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring resize-none"
					placeholder="Optional description..."
					rows={3}
					bind:value={description}
				></textarea>
			</div>

			<div class="flex flex-col gap-1.5">
				<label for="cpm-envvar" class="text-sm font-medium text-muted-foreground"
					>Env Var Prefix <span class="text-destructive">*</span></label
				>
				<input id="cpm-envvar"
					class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring"
					placeholder="MY_PROJECT"
					bind:value={envVar}
					required
				/>
				<p class="text-sm text-muted-foreground">Used as prefix for environment variables</p>
			</div>

			{#if error}
				<div class="rounded-md bg-destructive/10 border border-destructive/30 px-3 py-2 text-sm text-destructive">
					{error}
				</div>
			{/if}

			<div class="flex justify-end gap-2 pt-1">
				<button
					type="button"
					class="px-3 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
					onclick={onClose}
					disabled={loading}
				>
					Cancel
				</button>
				<button
					type="submit"
					class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 transition-colors disabled:opacity-50"
					disabled={loading}
				>
					{#if loading}
						<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>
						Creating…
					{:else}
						Create Project
					{/if}
				</button>
			</div>
		</form>
	</div>
</div>
