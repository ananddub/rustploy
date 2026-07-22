<script lang="ts">
	import { goto } from '$app/navigation';
	import {
		Plus,
		FolderOpen,
		Search,
		Rocket,
		LayoutGrid,
		List,
		GitBranch,
		ExternalLink,
		MoreVertical,
		Pencil,
		Trash2,
		Check,
		X,
		Layers,
		CheckCircle2,
		AlertTriangle,
		RefreshCw,
		Tag
	} from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import {
		projectControllerListByOrganization,
		projectControllerDelete,
		projectControllerPatch
	} from '$lib/client/sdk.gen';
	import type { ProjectResponseDto } from '$lib/client/types.gen';
	import { USE_MOCK_DATA, getProjectsMock, type ProjectMock } from '$lib/mocks';
	import CreateProjectModal from '$lib/components/projects/CreateProjectModal.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Badge } from '$lib/components/ui/badge';
	import { Card } from '$lib/components/ui/card';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	let useMock = $state(USE_MOCK_DATA);
	let mockProjects = $state<ProjectMock[]>(getProjectsMock());
	let apiProjects = $state<ProjectResponseDto[]>([]);
	let loading = $state(false);

	let showCreate = $state(false);
	let searchQuery = $state('');
	let selectedTag = $state('All');
	let sortBy = $state<'newest' | 'name' | 'active'>('newest');
	let viewMode = $state<'grid' | 'table'>('grid');

	let editingId = $state<string | number | null>(null);
	let editName = $state('');
	let editDesc = $state('');
	let deletingId = $state<string | number | null>(null);
	let saving = $state(false);

	async function loadApiProjects() {
		if (!session || useMock) return;
		loading = true;
		try {
			const res = await projectControllerListByOrganization({
				path: { organization_id: session.user.group_id }
			});
			apiProjects = (res.data as ProjectResponseDto[]) ?? [];
		} finally {
			loading = false;
		}
	}

	$effect(() => {
		if (!useMock) loadApiProjects();
	});

	// Unified list for rendering with explicit ProjectMock shape
	const displayProjects = $derived<ProjectMock[]>(
		useMock
			? mockProjects
			: apiProjects.map((p) => ({
					id: String(p.id),
					name: p.name,
					description: p.description ?? '',
					tags: ['Production'],
					environments: [{ id: '1', name: 'Production', slug: 'production', isProduction: true, servicesCount: 1 }],
					services: [],
					appsCount: 1,
					composeCount: 0,
					databaseCount: 0,
					healthStatus: 'healthy' as const,
					gitBranch: undefined,
					updatedAt: 'recently',
					createdAt: String(p.created_at)
				}))
	);

	const availableTags = $derived(['All', ...new Set(mockProjects.flatMap((p) => p.tags))]);

	const filteredProjects = $derived(
		displayProjects
			.filter((p) => {
				const matchesSearch =
					!searchQuery ||
					p.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
					p.description.toLowerCase().includes(searchQuery.toLowerCase());
				const matchesTag = selectedTag === 'All' || p.tags.includes(selectedTag);
				return matchesSearch && matchesTag;
			})
			.sort((a, b) => {
				if (sortBy === 'name') return a.name.localeCompare(b.name);
				return 0; // Default newest
			})
	);

	function deleteProject(id: string | number) {
		deletingId = id;
		if (useMock) {
			mockProjects = mockProjects.filter((p) => p.id !== id);
			deletingId = null;
		} else {
			projectControllerDelete({ path: { id: Number(id) } }).then(() => {
				apiProjects = apiProjects.filter((p) => p.id !== Number(id));
				deletingId = null;
			});
		}
	}

	function getHealthBadge(status?: string) {
		switch (status) {
			case 'healthy':
				return { label: 'Healthy', color: 'text-green-400 border-green-500/30 bg-green-500/10', dot: 'bg-green-500' };
			case 'deploying':
				return { label: 'Deploying', color: 'text-blue-400 border-blue-500/30 bg-blue-500/10', dot: 'bg-blue-500 animate-pulse' };
			case 'error':
				return { label: 'Error', color: 'text-red-400 border-red-500/30 bg-red-500/10', dot: 'bg-red-500' };
			default:
				return { label: 'Idle', color: 'text-zinc-400 border-[#262626] bg-[#262626]/40', dot: 'bg-zinc-500' };
		}
	}
</script>

<PageLayout>
	<!-- Top Breadcrumb Bar -->
	<header class="flex items-center justify-between px-6 py-3 border-b border-[#262626] text-xs bg-[#0A0A0A] shrink-0">
		<div class="flex items-center gap-2">
			<button onclick={() => goto('/dashboard')} class="text-[#737373] hover:text-[#FAFAFA] transition-colors flex items-center gap-1.5">
				<Rocket class="w-3.5 h-3.5" />
				Home
			</button>
			<span class="text-[#737373]">/</span>
			<span class="font-medium text-[#FAFAFA] flex items-center gap-1.5">
				<FolderOpen class="w-3.5 h-3.5 text-[#a1a1aa]" /> Projects
			</span>
		</div>

		<!-- Mock Data Dev Toggle Switch -->
		<div class="flex items-center gap-2 px-3 py-1 rounded-full bg-[#141414] border border-[#262626]">
			<span class="text-[11px] text-[#a1a1aa]">Data Source:</span>
			<button
				onclick={() => (useMock = !useMock)}
				class="text-[11px] font-semibold px-2 py-0.5 rounded transition-colors {useMock
					? 'bg-[#262626] text-[#FAFAFA] border border-white/10'
					: 'text-[#737373] hover:text-[#FAFAFA]'}"
			>
				{useMock ? 'Mock Demo Data' : 'Live Rust Backend API'}
			</button>
		</div>
	</header>

	<main class="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up flex flex-col min-h-0 bg-[#171717] border border-[#262626] rounded-2xl shadow-md space-y-6">
		<!-- Title & Primary CTA -->
		<div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
			<div>
				<h1 class="text-3xl font-bold tracking-tight text-[#FAFAFA]">Projects</h1>
				<p class="text-sm text-[#a1a1aa] mt-1">Manage your organization's services, deployment environments, and compose stacks</p>
			</div>
			<Button
				variant="default"
				size="default"
				class="gap-2 text-sm font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] px-4 py-2 rounded-lg self-start sm:self-auto"
				onclick={() => (showCreate = true)}
			>
				<Plus class="w-4 h-4" /> Create Project
			</Button>
		</div>

		<!-- Toolbar Filters & Layout Switcher -->
		<div class="flex flex-col md:flex-row md:items-center justify-between gap-3 pt-2">
			<!-- Left: Search & Tag Chips -->
			<div class="flex flex-wrap items-center gap-3 flex-1">
				<div class="relative flex-1 max-w-sm">
					<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-[#737373]" />
					<Input
						bind:value={searchQuery}
						placeholder="Filter projects by name or description..."
						class="pl-9 text-sm h-9 bg-[#141414] border-[#262626] text-[#FAFAFA] placeholder:text-[#737373]"
					/>
				</div>

				<!-- Tag selector chips -->
				<div class="flex items-center gap-1.5 overflow-x-auto py-1">
					{#each availableTags as tag}
						<button
							onclick={() => (selectedTag = tag)}
							class="text-xs font-medium px-2.5 py-1 rounded-md transition-colors whitespace-nowrap {selectedTag === tag
								? 'bg-[#262626] text-[#FAFAFA] border border-white/10'
								: 'text-[#737373] hover:text-[#FAFAFA] hover:bg-[#262626]/40'}"
						>
							{tag}
						</button>
					{/each}
				</div>
			</div>

			<!-- Right: View Mode Toggle -->
			<div class="flex items-center gap-2 shrink-0">
				<div class="flex items-center gap-1 bg-[#141414] p-1 rounded-lg border border-[#262626]">
					<button
						onclick={() => (viewMode = 'grid')}
						class="p-1 rounded text-xs transition-colors {viewMode === 'grid'
							? 'bg-[#262626] text-[#FAFAFA]'
							: 'text-[#737373] hover:text-[#FAFAFA]'}"
						title="Grid View"
					>
						<LayoutGrid class="w-4 h-4" />
					</button>
					<button
						onclick={() => (viewMode = 'table')}
						class="p-1 rounded text-xs transition-colors {viewMode === 'table'
							? 'bg-[#262626] text-[#FAFAFA]'
							: 'text-[#737373] hover:text-[#FAFAFA]'}"
						title="Table View"
					>
						<List class="w-4 h-4" />
					</button>
				</div>
			</div>
		</div>

		<!-- Main Project Content View -->
		{#if loading}
			<div class="flex flex-col items-center justify-center py-24 gap-3">
				<div class="w-6 h-6 border-2 border-[#737373] border-t-[#FAFAFA] rounded-full animate-spin"></div>
				<p class="text-sm text-[#a1a1aa]">Loading projects...</p>
			</div>
		{:else if filteredProjects.length === 0}
			<!-- Empty State -->
			<div class="flex flex-col items-center justify-center py-20 px-4 text-center min-h-[300px]">
				<div class="w-14 h-14 rounded-2xl bg-[#141414] border border-[#262626] flex items-center justify-center mb-4 shadow-xs">
					<FolderOpen class="w-7 h-7 text-[#737373]" />
				</div>
				<h3 class="text-base font-semibold text-[#FAFAFA]">
					{searchQuery || selectedTag !== 'All' ? 'No matching projects found' : 'No projects created yet'}
				</h3>
				<p class="text-xs text-[#a1a1aa] max-w-md mt-1.5 leading-relaxed">
					{searchQuery || selectedTag !== 'All'
						? 'Try clearing your search query or selecting a different tag filter.'
						: 'Projects group your applications, databases, Docker Compose stacks, and environment variables into self-contained deployment units.'}
				</p>
				{#if searchQuery || selectedTag !== 'All'}
					<Button
						variant="secondary"
						size="sm"
						class="mt-4 border border-[#262626] bg-[#262626] text-xs font-medium hover:bg-[#333333] text-[#FAFAFA] px-3.5 py-1.5 rounded-lg"
						onclick={() => {
							searchQuery = '';
							selectedTag = 'All';
						}}
					>
						Clear Filters
					</Button>
				{:else}
					<Button
						variant="secondary"
						size="sm"
						class="mt-4 gap-1.5 border border-[#262626] bg-[#262626] text-xs font-medium hover:bg-[#333333] text-[#FAFAFA] px-3.5 py-1.5 rounded-lg"
						onclick={() => (showCreate = true)}
					>
						<Plus class="w-3.5 h-3.5" /> Create your first project
					</Button>
				{/if}
			</div>
		{:else if viewMode === 'grid'}
			<!-- Grid View -->
			<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
				{#each filteredProjects as project (project.id)}
					{@const health = getHealthBadge(project.healthStatus)}
					<Card
						class="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs hover:border-[#3f3f46] transition-all p-5 flex flex-col justify-between group cursor-pointer"
						onclick={() => goto(`/projects/${project.id}`)}
					>
						<div>
							<!-- Card Top Row: Name + Health Badge -->
							<div class="flex items-start justify-between gap-3 mb-2">
								<div class="flex items-center gap-2.5 min-w-0">
									<div class="w-8 h-8 rounded-lg bg-[#262626] border border-white/10 flex items-center justify-center shrink-0">
										<FolderOpen class="w-4 h-4 text-[#FAFAFA]" />
									</div>
									<h2 class="text-base font-semibold text-[#FAFAFA] truncate group-hover:text-white transition-colors">{project.name}</h2>
								</div>
								<Badge variant="outline" class="text-[10px] gap-1.5 shrink-0 px-2 py-0.5 font-medium {health.color}">
									<span class="w-1.5 h-1.5 rounded-full {health.dot}"></span>
									{health.label}
								</Badge>
							</div>

							<!-- Description -->
							<p class="text-xs text-[#a1a1aa] line-clamp-2 mb-4 leading-relaxed">{project.description || 'No description provided.'}</p>

							<!-- Services Breakdown Stack -->
							<div class="flex items-center gap-1.5 flex-wrap mb-4">
								<span class="px-2 py-0.5 rounded border border-[#262626] bg-[#141414] text-[11px] text-[#FAFAFA] font-mono">{project.appsCount} Apps</span>
								<span class="px-2 py-0.5 rounded border border-[#262626] bg-[#141414] text-[11px] text-[#FAFAFA] font-mono">{project.composeCount} Compose</span>
								<span class="px-2 py-0.5 rounded border border-[#262626] bg-[#141414] text-[11px] text-[#FAFAFA] font-mono">{project.databaseCount} DB</span>
							</div>
						</div>

						<!-- Card Bottom Footer: Git Branch & Timestamp -->
						<div class="pt-3 border-t border-[#262626] flex items-center justify-between text-xs text-[#737373]">
							{#if project.gitBranch}
								<div class="flex items-center gap-1.5 font-mono text-[11px] text-[#a1a1aa]">
									<GitBranch class="w-3.5 h-3.5 text-[#737373]" />
									<span>{project.gitBranch}</span>
								</div>
							{:else}
								<span>No Git repo</span>
							{/if}
							<span class="font-mono text-[11px]">{project.updatedAt}</span>
						</div>
					</Card>
				{/each}
			</div>
		{:else}
			<!-- Table View -->
			<div class="border border-[#262626] rounded-xl overflow-hidden bg-[#171717]">
				<table class="w-full text-left text-xs">
					<thead class="bg-[#141414] border-b border-[#262626] text-[#737373] uppercase tracking-wider font-semibold">
						<tr>
							<th class="px-5 py-3">PROJECT</th>
							<th class="px-5 py-3">SERVICES</th>
							<th class="px-5 py-3">HEALTH</th>
							<th class="px-5 py-3">GIT BRANCH</th>
							<th class="px-5 py-3">UPDATED</th>
							<th class="px-5 py-3 text-right">ACTION</th>
						</tr>
					</thead>
					<tbody class="divide-y divide-[#262626]">
						{#each filteredProjects as project (project.id)}
							{@const health = getHealthBadge(project.healthStatus)}
							<tr
								class="hover:bg-[#262626]/30 transition-colors cursor-pointer"
								onclick={() => goto(`/projects/${project.id}`)}
							>
								<td class="px-5 py-3.5 font-semibold text-[#FAFAFA]">
									<div class="flex items-center gap-2.5">
										<FolderOpen class="w-4 h-4 text-[#a1a1aa]" />
										<span>{project.name}</span>
									</div>
								</td>
								<td class="px-5 py-3.5 text-[#a1a1aa] font-mono">
									{project.appsCount} apps · {project.composeCount} compose · {project.databaseCount} db
								</td>
								<td class="px-5 py-3.5">
									<Badge variant="outline" class="text-[10px] gap-1.5 px-2 py-0.5 font-medium {health.color}">
										<span class="w-1.5 h-1.5 rounded-full {health.dot}"></span>
										{health.label}
									</Badge>
								</td>
								<td class="px-5 py-3.5 text-[#a1a1aa] font-mono">
									{project.gitBranch || '—'}
								</td>
								<td class="px-5 py-3.5 text-[#737373] font-mono">
									{project.updatedAt}
								</td>
								<td class="px-5 py-3.5 text-right">
									<Button variant="ghost" size="sm" class="h-7 text-xs text-[#FAFAFA]" onclick={(e) => { e.stopPropagation(); goto(`/projects/${project.id}`); }}>
										Open →
									</Button>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</main>
</PageLayout>

{#if showCreate}
	<CreateProjectModal
		onClose={() => (showCreate = false)}
		onCreated={(p) => {
			if (useMock) {
				mockProjects = [
					{
						id: `proj-${Date.now()}`,
						name: p.name,
						description: p.description ?? '',
						tags: ['Custom'],
						environments: [{ id: 'env-new', name: 'Production', slug: 'production', isProduction: true, servicesCount: 0 }],
						services: [],
						appsCount: 0,
						composeCount: 0,
						databaseCount: 0,
						healthStatus: 'healthy',
						updatedAt: 'Just now',
						createdAt: 'Just now'
					},
					...mockProjects
				];
			} else {
				apiProjects = [...apiProjects, p];
			}
			showCreate = false;
		}}
	/>
{/if}
