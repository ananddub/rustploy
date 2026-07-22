<script lang="ts">
	import { goto } from '$app/navigation';
	import { Plus, FolderOpen, Search, RocketIcon } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import {
		projectControllerListByOrganization,
		projectControllerDelete,
		projectControllerPatch
	} from '$lib/client/sdk.gen';
	import type { ProjectResponseDto } from '$lib/client/types.gen';
	import ProjectCard from '$lib/components/projects/ProjectCard.svelte';
	import CreateProjectModal from '$lib/components/projects/CreateProjectModal.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	let projects = $state<ProjectResponseDto[]>([]);
	let loading = $state(true);
	let showCreate = $state(false);
	let editingId = $state<number | null>(null);
	let editName = $state('');
	let editDesc = $state('');
	let deletingId = $state<number | null>(null);
	let saving = $state(false);

	let searchQuery = $state('');

	async function loadProjects() {
		if (!session) return;
		loading = true;
		try {
			const res = await projectControllerListByOrganization({
				path: { organization_id: session.user.group_id }
			});
			projects = (res.data as ProjectResponseDto[]) ?? [];
		} finally {
			loading = false;
		}
	}

	loadProjects();

	async function deleteProject(id: number) {
		deletingId = id;
		try {
			await projectControllerDelete({ path: { id } });
			projects = projects.filter((p) => p.id !== id);
		} finally {
			deletingId = null;
		}
	}

	function startEdit(p: ProjectResponseDto) {
		editingId = p.id;
		editName = p.name;
		editDesc = p.description ?? '';
	}

	async function saveEdit(id: number) {
		saving = true;
		try {
			const res = await projectControllerPatch({
				path: { id },
				body: { name: editName, description: editDesc || undefined }
			});
			if (res.data) projects = projects.map((p) => (p.id === id ? res.data! : p));
			editingId = null;
		} finally {
			saving = false;
		}
	}

	const filteredProjects = $derived(
		projects.filter((p) => {
			if (!searchQuery) return true;
			const q = searchQuery.toLowerCase();
			return p.name.toLowerCase().includes(q) || (p.description?.toLowerCase().includes(q) ?? false);
		})
	);
</script>

<PageLayout>
	<!-- Top Breadcrumb Bar -->
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border/60 text-sm">
		<button
			onclick={() => goto('/dashboard')}
			class="text-muted-foreground hover:text-foreground transition-colors flex items-center gap-1.5"
		>
			<RocketIcon class="w-4 h-4" />
			Dashboard
		</button>
		<span class="text-muted-foreground/40">/</span>
		<span class="font-medium text-foreground flex items-center gap-1.5">
			<FolderOpen class="w-4 h-4 text-muted-foreground" /> Projects
		</span>
	</header>

	<main class="flex-1 px-8 py-8 animate-fade-up">
		<!-- Title bar -->
		<div class="flex items-center justify-between mb-7">
			<div>
				<h1 class="text-2xl font-bold tracking-tight text-foreground">Projects</h1>
				<p class="text-sm text-muted-foreground mt-1">Manage your organization's deployment stacks and environments</p>
			</div>
			<Button
				variant="default"
				size="default"
				class="gap-2 text-sm font-semibold px-4 py-2"
				onclick={() => (showCreate = true)}
			>
				<Plus class="w-4 h-4" /> Create Project
			</Button>
		</div>

		{#if loading}
			<div class="flex flex-col items-center justify-center py-24 gap-3">
				<div class="w-6 h-6 border-2 border-muted-foreground/30 border-t-foreground rounded-full animate-spin"></div>
				<p class="text-sm text-muted-foreground">Loading projects...</p>
			</div>
		{:else if projects.length === 0}
			<!-- Inviting Empty State (Dokploy scale) -->
			<div class="flex flex-col items-center justify-center py-24 px-4 text-center">
				<div class="w-16 h-16 rounded-2xl bg-card border border-border/80 flex items-center justify-center mb-4 shadow-xs">
					<FolderOpen class="w-8 h-8 text-muted-foreground/70" />
				</div>
				<h3 class="text-lg font-semibold text-foreground">No projects created yet</h3>
				<p class="text-sm text-muted-foreground max-w-md mt-2 leading-relaxed">
					Projects group your applications, databases, Docker Compose stacks, and environment variables together into self-contained units.
				</p>
				<Button
					variant="secondary"
					size="default"
					class="mt-6 gap-2 border border-border/80 text-sm font-medium hover:bg-accent px-4 py-2"
					onclick={() => (showCreate = true)}
				>
					<Plus class="w-4 h-4" /> Create your first project
				</Button>
			</div>
		{:else}
			<!-- Show filter controls ONLY when projects exist -->
			<div class="flex items-center gap-3 mb-6">
				<div class="relative flex-1 max-w-sm">
					<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground/70" />
					<Input
						bind:value={searchQuery}
						placeholder="Filter projects..."
						class="pl-9 text-sm h-10"
					/>
				</div>
			</div>

			<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
				{#each filteredProjects as project (project.id)}
					<ProjectCard
						{project}
						{editingId}
						{deletingId}
						{saving}
						{editName}
						{editDesc}
						onNavigate={() => goto(`/projects/${project.id}`)}
						onStartEdit={() => startEdit(project)}
						onSaveEdit={() => saveEdit(project.id)}
						onCancelEdit={() => (editingId = null)}
						onDelete={() => deleteProject(project.id)}
						onEditName={(v) => (editName = v)}
						onEditDesc={(v) => (editDesc = v)}
					/>
				{/each}
			</div>
		{/if}
	</main>
</PageLayout>

{#if showCreate}
	<CreateProjectModal
		onClose={() => (showCreate = false)}
		onCreated={(p) => {
			projects = [...projects, p];
			showCreate = false;
		}}
	/>
{/if}
