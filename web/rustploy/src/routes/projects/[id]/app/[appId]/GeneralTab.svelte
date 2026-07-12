<script lang="ts">
	import { RocketIcon, RefreshCw, Hammer, Play, Terminal } from '@lucide/svelte';
	import type { ApplicationResponseDto } from '$lib/client/types.gen';

	type Props = { app: ApplicationResponseDto; onUpdated: (a: ApplicationResponseDto) => void };
	let { app }: Props = $props();

	const PROVIDERS = [
		{ id: 'DOCKER', label: 'Docker' },
		{ id: 'GIT', label: 'Git' },
		{ id: 'DROP', label: 'Drop' }
	];

	const BUILD_TYPES = [
		{ id: 'DOCKERFILE', label: 'Dockerfile' },
		{ id: 'RAILPACK', label: 'Railpack' },
		{ id: 'NIXPACKS', label: 'Nixpacks' },
		{ id: 'HEROKU_BUILDPACKS', label: 'Heroku Buildpacks' },
		{ id: 'PAKETO_BUILDPACKS', label: 'Paketo Buildpacks' },
		{ id: 'STATIC', label: 'Static' }
	];

	let provider = $state(app.source_type ?? 'DOCKER');
	let buildType = $state(app.build_type ?? 'NIXPACKS');
	let dockerImage = $state(app.docker_image ?? '');
	let registryUrl = $state(app.registry_url ?? '');
	let gitUrl = $state(app.custom_git_url ?? '');
	let gitBranch = $state(app.custom_git_branch ?? '');
	let autoDeploy = $state(true);
	let cleanCache = $state(false);
</script>

<div class="flex flex-col gap-6">
	<!-- Provider + Deploy Actions -->
	<section class="bg-card border border-border rounded-lg p-6">
		<div class="flex items-center justify-between mb-4">
			<div>
				<h2 class="text-base font-semibold">Source & Build</h2>
				<p class="text-sm text-muted-foreground mt-0.5">Configure how this application is built and deployed.</p>
			</div>
			<div class="flex flex-wrap items-center gap-2">
				<button class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90">
					<RocketIcon class="w-3.5 h-3.5" /> Deploy
				</button>
				<button class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent">
					<RefreshCw class="w-3.5 h-3.5" /> Reload
				</button>
				<button class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent">
					<Hammer class="w-3.5 h-3.5" /> Rebuild
				</button>
				<button class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent">
					<Play class="w-3.5 h-3.5" /> Start
				</button>
				<button class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent">
					<Terminal class="w-3.5 h-3.5" /> Terminal
				</button>
				<label class="flex items-center gap-2 text-sm text-muted-foreground" for="generaltab-1">
					Autodeploy
					<input type="checkbox" bind:checked={autoDeploy} class="rounded" />
				</label>
				<label class="flex items-center gap-2 text-sm text-muted-foreground">
					Clean Cache
					<input type="checkbox" bind:checked={cleanCache} class="rounded" />
				</label>
			</div>
		</div>

		<!-- Provider tabs -->
		<div class="flex gap-1 mb-4">
			{#each PROVIDERS as p}
				<button
					class="px-3 py-1.5 rounded-md text-sm transition-colors {provider === p.id ? 'bg-accent text-accent-foreground font-medium' : 'text-muted-foreground hover:text-foreground hover:bg-accent/50'}"
					onclick={() => (provider = p.id)}
				>
					{p.label}
				</button>
			{/each}
		</div>

		{#if provider === 'DOCKER'}
			<div class="grid grid-cols-2 gap-4">
				<div class="flex flex-col gap-1.5">
					<label class="text-sm font-medium text-muted-foreground">Docker Image</label>

					<input id="generaltab-1"  class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring" placeholder="nginx:latest" bind:value={dockerImage} />
				</div>
				<div class="flex flex-col gap-1.5">
					<label class="text-sm font-medium text-muted-foreground" for="generaltab-2">Registry URL</label>

					<input id="generaltab-2"  class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring" placeholder="registry.example.com" bind:value={registryUrl} />
				</div>
			</div>
		{:else if provider === 'GIT'}
			<div class="grid grid-cols-2 gap-4">
				<div class="flex flex-col gap-1.5">
					<label class="text-sm font-medium text-muted-foreground" for="generaltab-3">Git URL</label>

					<input id="generaltab-3"  class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring" placeholder="https://github.com/user/repo.git" bind:value={gitUrl} />
				</div>
				<div class="flex flex-col gap-1.5">
					<label class="text-sm font-medium text-muted-foreground" for="generaltab-4">Branch</label>

					<input id="generaltab-4"  class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring" placeholder="main" bind:value={gitBranch} />
				</div>
			</div>
		{:else}
			<div class="text-sm text-muted-foreground py-4 text-center">Drop source — drag & drop your files here.</div>
		{/if}
	</section>

	<!-- Build type -->
	<section class="bg-card border border-border rounded-lg p-6">
		<h2 class="text-base font-semibold mb-4">Build Type</h2>
		<div class="flex flex-wrap gap-2">
			{#each BUILD_TYPES as bt}
				<button
					class="px-3 py-1.5 rounded-md text-sm border transition-colors {buildType === bt.id ? 'border-primary bg-primary/10 text-primary font-medium' : 'border-border text-muted-foreground hover:text-foreground hover:bg-accent'}"
					onclick={() => (buildType = bt.id)}
				>
					{bt.label}
				</button>
			{/each}
		</div>
	</section>
</div>
