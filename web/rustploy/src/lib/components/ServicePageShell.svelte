<script lang="ts">
	import { type Snippet } from 'svelte';
	import { goto } from '$app/navigation';
	import {
		RocketIcon,
		FolderOpen,
		ChevronRight,
		Cpu,
		Pencil,
		Trash2
	} from '@lucide/svelte';
	import PageLayout from './PageLayout.svelte';
	import { Separator } from '$lib/components/ui/separator';

	type Props = {
		projectId: string | number;
		name: string;
		appName: string;
		tabs: readonly string[];
		activeTab: string;
		onTabChange: (tab: string) => void;
		loading: boolean;
		onEdit?: () => void;
		onDelete?: () => void;
		children: Snippet;
	};

	let {
		projectId,
		name,
		appName,
		tabs,
		activeTab,
		onTabChange,
		loading,
		onEdit,
		onDelete,
		children
	}: Props = $props();
</script>

<PageLayout>
	<div class="flex flex-col min-w-0 flex-1">
		<header class="px-6 pt-4 border-b border-border">
			<!-- Breadcrumb -->
			<div class="flex items-center gap-1.5 text-xs mb-3 text-muted-foreground">
				<RocketIcon size={13} />
				<button onclick={() => goto('/dashboard')} class="hover:text-foreground transition-colors">
					Dashboard
				</button>
				<ChevronRight size={11} class="opacity-40" />
				<button
					onclick={() => goto('/projects')}
					class="hover:text-foreground transition-colors flex items-center gap-1"
				>
					<FolderOpen size={13} /> Projects
				</button>
				<ChevronRight size={11} class="opacity-40" />
				<button
					onclick={() => goto(`/projects/${projectId}`)}
					class="hover:text-foreground transition-colors"
				>
					Project
				</button>
				<ChevronRight size={11} class="opacity-40" />
				<span class="text-foreground/70 font-medium">{name || '…'}</span>
			</div>

			<!-- Title row -->
			<div class="flex items-center justify-between mb-3">
				<div>
					<h1 class="text-lg font-semibold leading-tight">{name || '…'}</h1>
					<p class="text-xs text-muted-foreground mt-0.5 font-mono">{appName}</p>
				</div>
				<div class="flex items-center gap-1.5">
					<button
						class="flex items-center gap-1.5 text-xs px-2.5 py-1.5 rounded-md border border-border text-muted-foreground hover:text-foreground hover:bg-accent transition-all"
					>
						<Cpu size={13} /> Rustploy Server
					</button>
					{#if onEdit}
						<button
							class="p-1.5 rounded-md text-muted-foreground hover:text-foreground hover:bg-accent transition-all"
							onclick={onEdit}
							title="Edit"
						>
							<Pencil size={14} />
						</button>
					{/if}
					{#if onDelete}
						<button
							class="p-1.5 rounded-md text-muted-foreground hover:text-destructive hover:bg-destructive/10 transition-all"
							onclick={onDelete}
							title="Delete"
						>
							<Trash2 size={14} />
						</button>
					{/if}
				</div>
			</div>

			<!-- Tab bar -->
			<div class="flex overflow-x-auto -mb-px scrollbar-none">
				{#each tabs as tab}
					<button
						class="px-4 py-2 text-sm whitespace-nowrap border-b-2 transition-all duration-150 {activeTab ===
						tab
							? 'border-foreground text-foreground font-medium'
							: 'border-transparent text-muted-foreground hover:text-foreground/80 hover:border-border'}"
						onclick={() => onTabChange(tab)}
					>
						{tab}
					</button>
				{/each}
			</div>
		</header>

		<main class="flex-1 px-6 py-6 overflow-y-auto">
			{#if loading}
				<div class="flex justify-center py-20">
					<div class="w-6 h-6 border-2 border-muted-foreground/30 border-t-foreground rounded-full animate-spin"></div>
				</div>
			{:else}
				{@render children()}
			{/if}
		</main>
	</div>
</PageLayout>
