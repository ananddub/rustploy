<script lang="ts">
	import { FolderOpen, Pencil, Trash2, Check, X } from '@lucide/svelte';
	import type { ProjectResponseDto } from '$lib/client/types.gen';

	type Props = {
		project: ProjectResponseDto;
		editingId: number | null;
		deletingId: number | null;
		saving: boolean;
		editName: string;
		editDesc: string;
		onNavigate: () => void;
		onStartEdit: () => void;
		onSaveEdit: () => void;
		onCancelEdit: () => void;
		onDelete: () => void;
		onEditName: (v: string) => void;
		onEditDesc: (v: string) => void;
	};

	let {
		project,
		editingId,
		deletingId,
		saving,
		editName,
		editDesc,
		onNavigate,
		onStartEdit,
		onSaveEdit,
		onCancelEdit,
		onDelete,
		onEditName,
		onEditDesc
	}: Props = $props();

	const isEditing = $derived(editingId === project.id);
	const isDeleting = $derived(deletingId === project.id);

	function formatDate(ts: number) {
		return new Date(ts * 1000).toLocaleDateString('en-IN', {
			day: '2-digit',
			month: 'short',
			year: 'numeric'
		});
	}
</script>

<div
	role="button"
	tabindex="0"
	class="bg-card border border-border rounded-lg px-4 py-3 flex items-center gap-4 hover:border-foreground/20 transition-colors cursor-pointer group"
	onclick={() => { if (!isEditing) onNavigate(); }}
	onkeydown={(e) => e.key === 'Enter' && !isEditing && onNavigate()}
>
	<div
		class="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center shrink-0"
	>
		<FolderOpen class="w-4 h-4 text-primary" />
	</div>

	<div class="flex-1 min-w-0">
		{#if isEditing}
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<div class="flex flex-col gap-1.5" onclick={(e) => e.stopPropagation()} onkeydown={() => {}}>
				<input
					class="flex h-7 rounded-md border border-input bg-transparent px-2 py-1 text-sm focus:outline-none focus:ring-1 focus:ring-ring max-w-xs"
					value={editName}
					oninput={(e) => onEditName((e.target as HTMLInputElement).value)}
					placeholder="Name"
				/>
				<input
					class="flex h-7 rounded-md border border-input bg-transparent px-2 py-1 text-sm focus:outline-none focus:ring-1 focus:ring-ring max-w-sm"
					value={editDesc}
					oninput={(e) => onEditDesc((e.target as HTMLInputElement).value)}
					placeholder="Description"
				/>
			</div>
		{:else}
			<p class="font-medium text-sm group-hover:text-primary transition-colors">{project.name}</p>
			<p class="text-sm text-muted-foreground mt-0.5">
				{#if project.description}
					{project.description}
				{:else}
					<span class="italic">No description</span>
				{/if}
			</p>
		{/if}
	</div>

	{#if !isEditing}
		<div class="text-right hidden sm:block shrink-0">
			<p class="text-sm font-mono text-muted-foreground/50">{project.env_var}</p>
			<p class="text-sm text-muted-foreground/50 mt-0.5">{formatDate(project.created_at)}</p>
		</div>
	{/if}

	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="flex items-center gap-1 shrink-0" onclick={(e) => e.stopPropagation()} onkeydown={() => {}}>
		{#if isEditing}
			<button
				class="p-1 rounded text-green-600 hover:bg-green-500/10 transition-colors"
				onclick={onSaveEdit}
				disabled={saving}
			>
				{#if saving}
					<div class="w-3.5 h-3.5 border-2 border-current/30 border-t-current rounded-full animate-spin"></div>
				{:else}
					<Check class="w-3.5 h-3.5" />
				{/if}
			</button>
			<button
				class="p-1 rounded text-muted-foreground hover:text-destructive transition-colors"
				onclick={onCancelEdit}
			>
				<X class="w-3.5 h-3.5" />
			</button>
		{:else}
			<button
				class="p-1 rounded text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
				onclick={onStartEdit}
			>
				<Pencil class="w-3.5 h-3.5" />
			</button>
			<button
				class="p-1 rounded text-muted-foreground hover:text-destructive hover:bg-destructive/10 transition-colors"
				onclick={onDelete}
				disabled={isDeleting}
			>
				{#if isDeleting}
					<div class="w-3.5 h-3.5 border-2 border-current/30 border-t-current rounded-full animate-spin"></div>
				{:else}
					<Trash2 class="w-3.5 h-3.5" />
				{/if}
			</button>
		{/if}
	</div>
</div>
