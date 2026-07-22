<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { RocketIcon, FolderOpen, Layers, Box, Plus } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import {
		projectControllerGet,
		environmentControllerGet,
		applicationControllerListByEnvironment
	} from '$lib/client/sdk.gen';
	import type { ApplicationResponseDto } from '$lib/client/types.gen';
	import { appStatusDot } from '$lib/helpers';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	const projectId = $derived(parseInt(page.params.id ?? '0'));
	const envId = $derived(parseInt(page.params.envId ?? '0'));

	let projectName = $state('');
	let env = $state<{ id: number; name: string; description?: string; is_default?: boolean } | null>(null);
	let apps = $state<ApplicationResponseDto[]>([]);
	let loading = $state(true);

	$effect(() => {
		Promise.all([
			projectControllerGet({ path: { id: projectId } }),
			environmentControllerGet({ path: { id: envId } }),
			applicationControllerListByEnvironment({ path: { environment_id: envId } })
		]).then(([pRes, eRes, aRes]: any[]) => {
			projectName = pRes.data?.name ?? '';
			env = eRes.data ?? null;
			apps = (aRes.data as ApplicationResponseDto[]) ?? [];
			loading = false;
		});
	});

	function formatDate(ts: number) {
		return new Date(ts * 1000).toLocaleDateString('en-IN', {
			day: '2-digit', month: 'short', year: 'numeric'
		});
	}
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm flex-wrap">
		<RocketIcon class="w-4 h-4 text-muted-foreground" />
		<button onclick={() => goto('/dashboard')} class="text-muted-foreground hover:text-foreground transition-colors">Dashboard</button>
		<span class="text-muted-foreground/30">/</span>
		<button onclick={() => goto('/projects')} class="text-muted-foreground hover:text-foreground transition-colors flex items-center gap-1">
			<FolderOpen class="w-3.5 h-3.5" /> Projects
		</button>
		<span class="text-muted-foreground/30">/</span>
		<button onclick={() => goto(`/projects/${projectId}`)} class="text-muted-foreground hover:text-foreground transition-colors flex items-center gap-1">
			<FolderOpen class="w-3.5 h-3.5" /> {projectName || '...'}
		</button>
		<span class="text-muted-foreground/30">/</span>
		<span class="font-medium flex items-center gap-1">
			<Layers class="w-3.5 h-3.5" /> {env?.name ?? '...'}
		</span>
	</header>

	<main class="flex-1 px-8 py-8">
		<div class="flex items-center justify-between mb-8">
			<div>
				<div class="flex items-center gap-2">
					<h1 class="text-2xl font-semibold">{env?.name ?? '...'}</h1>
					{#if env?.is_default}
						<span class="text-xs px-2 py-0.5 rounded bg-muted text-muted-foreground">default</span>
					{/if}
				</div>
				<p class="text-sm text-muted-foreground mt-1">{env?.description ?? projectName}</p>
			</div>
			<button
				onclick={() => goto(`/projects/${projectId}`)}
				class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 transition-colors"
			>
				<Plus class="w-4 h-4" /> Create Service
			</button>
		</div>

		<p class="text-sm font-medium text-muted-foreground uppercase tracking-widest mb-4">Services</p>

		{#if loading}
			<div class="flex justify-center py-20">
				<div class="w-6 h-6 border-2 border-muted-foreground/30 border-t-foreground rounded-full animate-spin"></div>
			</div>
		{:else if apps.length === 0}
			<div class="flex flex-col items-center justify-center py-20 text-muted-foreground/30">
				<Box class="w-12 h-12 mb-3" />
				<p class="text-sm">No services yet</p>
				<button
					onclick={() => goto(`/projects/${projectId}`)}
					class="mt-4 px-3 py-1.5 rounded-md border border-border text-sm hover:bg-accent transition-colors inline-flex items-center gap-1.5"
				>
					<Plus class="w-4 h-4" /> Create your first service
				</button>
			</div>
		{:else}
			<div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-3">
				{#each apps as app (app.id)}
					<div
						role="button"
						tabindex="0"
						class="bg-card border border-border rounded-lg p-4 flex flex-col gap-3 hover:border-foreground/20 transition-colors cursor-pointer group"
						onclick={() => goto(`/projects/${projectId}/app/${app.id}`)}
						onkeydown={(e) => e.key === 'Enter' && goto(`/projects/${projectId}/app/${app.id}`)}
					>
						<div class="flex items-start justify-between">
							<div class="flex items-center gap-3 min-w-0">
								<div class="w-9 h-9 rounded-lg bg-primary/10 flex items-center justify-center shrink-0 relative">
									<Box class="w-4 h-4 text-primary" />
									<span class="absolute -top-0.5 -right-0.5 w-2.5 h-2.5 rounded-full border-2 border-card {appStatusDot(app.app_status)}"></span>
								</div>
								<div class="min-w-0">
									<p class="font-medium text-sm truncate group-hover:text-primary transition-colors">{app.app_name}</p>
									<p class="text-sm text-muted-foreground truncate">{app.build_type}</p>
								</div>
							</div>
						</div>
						<div class="flex items-center justify-between pt-2 border-t border-border">
							<span class="flex items-center gap-1.5 text-xs">
								<span class="w-1.5 h-1.5 rounded-full {appStatusDot(app.app_status)}"></span>
								<span class="text-muted-foreground">{app.app_status ?? 'unknown'}</span>
							</span>
							<span class="text-xs text-muted-foreground/50">{formatDate(app.created_at)}</span>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</main>
</PageLayout>
