<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { RocketIcon, FolderOpen, Box, Layers2, Plus, Settings2, Database } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import {
		projectControllerGet,
		environmentControllerListByProject,
		applicationControllerListByEnvironment,
		composeControllerListByEnvironment,
		databaseControllerListByEnvironment
	} from '$lib/client/sdk.gen';
	import type {
		EnvironmentResponseDto,
		ApplicationResponseDto,
		ComposeResponseDto,
		DatabaseResponseDto
	} from '$lib/client/types.gen';
	import ServiceCard from '$lib/components/projects/ServiceCard.svelte';
	import EnvDropdown from '$lib/components/projects/EnvDropdown.svelte';
	import CreateEnvModal from '$lib/components/projects/CreateEnvModal.svelte';
	import CreateApplicationModal from '$lib/components/projects/CreateApplicationModal.svelte';
	import CreateComposeModal from '$lib/components/projects/CreateComposeModal.svelte';
	import ProjectEnvironmentModal from '$lib/components/projects/ProjectEnvironmentModal.svelte';
	import CreateServiceDropdown from '$lib/components/projects/CreateServiceDropdown.svelte';
	import CreateDatabaseModal from '$lib/components/projects/CreateDatabaseModal.svelte';
	import { appStatusDot } from '$lib/helpers';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	const projectId = $derived(parseInt(page.params.id ?? '0'));

	let projectName = $state('');
	let envs = $state<EnvironmentResponseDto[]>([]);
	let selectedEnvId = $state<number | null>(null);
	let apps = $state<ApplicationResponseDto[]>([]);
	let composes = $state<ComposeResponseDto[]>([]);
	let databases = $state<DatabaseResponseDto[]>([]);
	let appsLoading = $state(true);

	let showCreateEnv = $state(false);
	let showEnvSettings = $state(false);
	let showCreateApp = $state(false);
	let showCreateCompose = $state(false);
	let showCreateDatabase = $state(false);

	const selectedEnv = $derived(envs.find((e) => e.id === selectedEnvId) ?? null);

	async function loadProject() {
		const res = await projectControllerGet({ path: { id: projectId } });
		projectName = res.data?.name ?? '';
	}

	async function loadEnvs() {
		const res = await environmentControllerListByProject({ path: { project_id: projectId } });
		envs = (res.data as EnvironmentResponseDto[]) ?? [];
		if (!selectedEnvId && envs.length > 0) {
			const def = envs.find((e) => e.is_default) ?? envs[0];
			selectedEnvId = def.id;
		}
	}

	async function loadServices(envId: number) {
		appsLoading = true;
		try {
			const [appsRes, composesRes, dbRes] = await Promise.all([
				applicationControllerListByEnvironment({ path: { environment_id: envId } }),
				composeControllerListByEnvironment({ path: { environment_id: envId } }),
				databaseControllerListByEnvironment({ path: { environment_id: envId } })
			]);
			apps = (appsRes.data as ApplicationResponseDto[]) ?? [];
			composes = (composesRes.data as ComposeResponseDto[]) ?? [];
			databases = (dbRes.data as DatabaseResponseDto[]) ?? [];
		} finally {
			appsLoading = false;
		}
	}

	$effect(() => {
		loadProject();
		loadEnvs();
	});

	$effect(() => {
		if (selectedEnvId) loadServices(selectedEnvId);
	});

	// DB kind icon colour
	function dbKindColor(kind: string): string {
		const map: Record<string, string> = {
			postgres: 'text-blue-400',
			mysql: 'text-orange-400',
			mariadb: 'text-orange-400',
			mongo: 'text-green-500',
			redis: 'text-red-400',
			libsql: 'text-purple-400'
		};
		return map[kind?.toLowerCase()] ?? 'text-muted-foreground';
	}

	function dbStatusDot(status: string): string {
		return appStatusDot(status);
	}

	function formatDate(ts: number) {
		return new Date(ts * 1000).toLocaleDateString('en-IN', { day: '2-digit', month: 'short', year: 'numeric' });
	}

	const totalServices = $derived(apps.length + composes.length + databases.length);
</script>

<PageLayout>
	<!-- Top bar -->
	<header class="flex items-center justify-between px-6 py-3 border-b border-border text-sm">
		<div class="flex items-center gap-2">
			<RocketIcon class="w-4 h-4 text-muted-foreground" />
			<button onclick={() => goto('/dashboard')} class="text-muted-foreground hover:text-foreground transition-colors">Dashboard</button>
			<span class="text-muted-foreground/30">/</span>
			<button onclick={() => goto('/projects')} class="text-muted-foreground hover:text-foreground transition-colors flex items-center gap-1">
				<FolderOpen class="w-3.5 h-3.5" />{projectName || '...'}
			</button>
			<span class="text-muted-foreground/30">/</span>
			<EnvDropdown
				{envs}
				selectedId={selectedEnvId}
				onSelect={(id) => (selectedEnvId = id)}
				onCreateNew={() => (showCreateEnv = true)}
			/>
		</div>

		<div class="flex items-center gap-2">
			<button
				class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
				onclick={() => (showEnvSettings = true)}
			>
				<Settings2 class="w-4 h-4" /> Project Environment
			</button>
			<CreateServiceDropdown
				onSelect={(type) => {
					if (type === 'application') showCreateApp = true;
					else if (type === 'compose') showCreateCompose = true;
					else if (type === 'database') showCreateDatabase = true;
				}}
			/>
		</div>
	</header>

	<!-- Env description + stats -->
	<div class="px-8 pt-5 pb-3 flex items-center justify-between">
		<p class="text-sm text-muted-foreground">{selectedEnv?.description ?? 'Production environment'}</p>
		{#if !appsLoading && totalServices > 0}
			<p class="text-sm text-muted-foreground">
				{apps.length} app{apps.length !== 1 ? 's' : ''}
				{#if composes.length} · {composes.length} compose{/if}
				{#if databases.length} · {databases.length} db{/if}
			</p>
		{/if}
	</div>

	<!-- Services -->
	<main class="flex-1 px-8 pb-8">
		{#if appsLoading}
			<div class="flex justify-center py-20">
				<div class="w-6 h-6 border-2 border-muted-foreground/30 border-t-foreground rounded-full animate-spin"></div>
			</div>
		{:else if totalServices === 0}
			<div class="flex flex-col items-center justify-center py-20 text-muted-foreground">
				<Box class="w-12 h-12 mb-3 opacity-30" />
				<p class="text-sm">No services yet</p>
				<button
					class="mt-4 inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md border border-border text-sm hover:bg-accent transition-colors"
					onclick={() => (showCreateApp = true)}
				>
					<Plus class="w-4 h-4" /> Create your first service
				</button>
			</div>
		{:else}
			<div class="flex flex-wrap gap-3">
				<!-- Applications -->
				{#each apps as app (app.id)}
					<ServiceCard {app} {projectId} />
				{/each}

				<!-- Compose services -->
				{#each composes as compose (compose.id)}
					<div
						role="button"
						tabindex="0"
						class="w-56 bg-card border border-border rounded-lg p-4 flex flex-col gap-3 hover:border-foreground/20 transition-colors cursor-pointer group shrink-0"
						onclick={() => goto(`/projects/${projectId}/compose/${compose.id}`)}
						onkeydown={(e) => e.key === 'Enter' && goto(`/projects/${projectId}/compose/${compose.id}`)}
					>
						<div class="flex items-center gap-3">
							<div class="w-9 h-9 rounded-lg bg-secondary flex items-center justify-center shrink-0 relative">
								<Layers2 class="w-4 h-4 text-secondary-foreground" />
								<span class="absolute -top-0.5 -right-0.5 w-2.5 h-2.5 rounded-full border-2 border-card {appStatusDot(compose.compose_status ?? '')}"></span>
							</div>
							<div class="min-w-0">
								<p class="font-medium text-sm truncate group-hover:text-primary transition-colors">{compose.name}</p>
								<p class="text-sm text-muted-foreground">compose</p>
							</div>
						</div>
						<div class="flex items-center justify-between border-t border-border pt-2">
							<span class="flex items-center gap-1.5 text-xs text-muted-foreground">
								<span class="w-1.5 h-1.5 rounded-full {appStatusDot(compose.compose_status ?? '')}"></span>
								{compose.compose_status?.toLowerCase() ?? 'idle'}
							</span>
							<span class="text-xs text-muted-foreground/40">{formatDate(compose.created_at)}</span>
						</div>
					</div>
				{/each}

				<!-- Databases -->
				{#each databases as db (db.id)}
					<div
						role="button"
						tabindex="0"
						class="w-56 bg-card border border-border rounded-lg p-4 flex flex-col gap-3 hover:border-foreground/20 transition-colors cursor-pointer group shrink-0"
						onclick={() => goto(`/projects/${projectId}/database/${db.kind}/${db.id}`)}
						onkeydown={(e) => e.key === 'Enter' && goto(`/projects/${projectId}/database/${db.kind}/${db.id}`)}
					>
						<div class="flex items-center gap-3">
							<div class="w-9 h-9 rounded-lg bg-secondary flex items-center justify-center shrink-0 relative">
								<Database class="w-4 h-4 {dbKindColor(db.kind)}" />
								<span class="absolute -top-0.5 -right-0.5 w-2.5 h-2.5 rounded-full border-2 border-card {dbStatusDot(db.app_status)}"></span>
							</div>
							<div class="min-w-0">
								<p class="font-medium text-sm truncate group-hover:text-primary transition-colors">{db.name}</p>
								<p class="text-sm text-muted-foreground capitalize">{db.kind}</p>
							</div>
						</div>
						<div class="flex items-center justify-between border-t border-border pt-2">
							<span class="flex items-center gap-1.5 text-xs text-muted-foreground">
								<span class="w-1.5 h-1.5 rounded-full {dbStatusDot(db.app_status)}"></span>
								{db.app_status?.toLowerCase() ?? 'idle'}
							</span>
							<span class="text-xs text-muted-foreground/40">{formatDate(db.created_at)}</span>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</main>
</PageLayout>

{#if showCreateEnv}
	<CreateEnvModal
		{projectId}
		onClose={() => (showCreateEnv = false)}
		onCreated={(env) => { envs = [...envs, env]; selectedEnvId = env.id; showCreateEnv = false; }}
	/>
{/if}

{#if showEnvSettings && selectedEnv}
	<ProjectEnvironmentModal
		env={selectedEnv}
		onClose={() => (showEnvSettings = false)}
		onUpdated={(updated) => { envs = envs.map((e) => (e.id === updated.id ? updated : e)); }}
	/>
{/if}

{#if showCreateApp && selectedEnvId}
	<CreateApplicationModal
		environmentId={selectedEnvId}
		onClose={() => (showCreateApp = false)}
		onCreated={(app) => { apps = [...apps, app]; showCreateApp = false; }}
	/>
{/if}

{#if showCreateCompose && selectedEnvId}
	<CreateComposeModal
		environmentId={selectedEnvId}
		onClose={() => (showCreateCompose = false)}
		onCreated={(compose) => { goto(`/projects/${projectId}/compose/${compose.id}`); }}
	/>
{/if}

{#if showCreateDatabase && selectedEnvId}
	<CreateDatabaseModal
		environmentId={selectedEnvId}
		onClose={() => (showCreateDatabase = false)}
		onCreated={(db) => { databases = [...databases, db]; showCreateDatabase = false; }}
	/>
{/if}
