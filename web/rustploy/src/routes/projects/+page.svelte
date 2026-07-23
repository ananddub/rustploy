<script lang="ts">
	import { goto } from '$app/navigation';
	import { Plus, FolderOpen, RocketIcon } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import {
		projectControllerListByOrganization,
		projectControllerDelete,
		projectControllerPatch,
		projectControllerCreate
	} from '$lib/client/sdk.gen';
	import type { ProjectResponseDto } from '$lib/client/types.gen';
	import ProjectCard from '$lib/components/projects/dashboard/ProjectCard.svelte';
	import CreateProjectModal from '$lib/components/projects/dashboard/CreateProjectModal.svelte';

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
</script>

<PageLayout>
	<!-- Breadcrumb -->
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<RocketIcon class="w-4 h-4 text-muted-foreground" />
		<button
			onclick={() => goto('/dashboard')}
			class="text-muted-foreground hover:text-foreground transition-colors"
		>
			Dashboard
		</button>
		<span class="text-muted-foreground/30">/</span>
		<span class="font-medium flex items-center gap-1.5">
			<FolderOpen class="w-4 h-4" /> Projects
		</span>
	</header>

	<main class="flex-1 px-8 py-8">
		<div class="flex items-center justify-between mb-6">
			<div>
				<h1 class="text-2xl font-semibold">Projects</h1>
				<p class="text-sm text-muted-foreground mt-1">Manage your organization's projects</p>
			</div>
			<button
				class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 transition-colors"
				onclick={() => (showCreate = true)}
			>
				<Plus class="w-4 h-4" /> New Project
			</button>
		</div>

		{#if loading}
			<div class="flex justify-center py-20">
				<div class="w-6 h-6 border-2 border-muted-foreground/30 border-t-foreground rounded-full animate-spin"></div>
			</div>
		{:else if projects.length === 0}
			<div class="flex flex-col items-center justify-center py-20 text-muted-foreground">
				<FolderOpen class="w-12 h-12 mb-3 opacity-30" />
				<p class="text-sm">No projects yet</p>
				<button
					class="mt-4 px-3 py-1.5 rounded-md border border-border text-sm hover:bg-accent transition-colors"
					onclick={() => (showCreate = true)}
				>
					Create your first project
				</button>
			</div>
		{:else}
			<div class="flex flex-col gap-2">
				{#each projects as project (project.id)}
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
