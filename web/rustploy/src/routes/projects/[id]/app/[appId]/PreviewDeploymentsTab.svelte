<script lang="ts">
	import { Eye, GitBranch, Info } from '@lucide/svelte';
	import type { ApplicationResponseDto } from '$lib/client/types.gen';

	type Props = { app: ApplicationResponseDto; onUpdated: (a: ApplicationResponseDto) => void };
	let { app }: Props = $props();

	let enabled = $state(false);
	let wildcard = $state('');
	let port = $state('3000');
	let https = $state(false);
	let path = $state('/');
	let limit = $state('3');
	let saving = $state(false);

	const isGitProvider = $derived(
		['GITHUB', 'GITLAB', 'GITEA', 'BITBUCKET', 'GIT'].includes(app.source_type ?? '')
	);
</script>

<div class="flex flex-col gap-6">
	<!-- Enable toggle -->
	<section class="bg-card border border-border rounded-lg p-6">
		<div class="flex items-start justify-between">
			<div>
				<h2 class="text-base font-semibold">Preview Deployments</h2>
				<p class="text-sm text-muted-foreground mt-1 max-w-lg">
					Automatically deploy pull-request / merge-request branches as isolated preview environments.
					Requires a Git provider source.
				</p>
			</div>
			<input
				type="checkbox"
				bind:checked={enabled}
				class="w-10 h-6 rounded-full cursor-pointer accent-primary"
			/>
		</div>

		{#if !isGitProvider}
			<div class="flex items-start gap-2 mt-4 bg-yellow-500/10 border border-yellow-500/20 rounded-md px-3 py-2.5">
				<Info class="w-4 h-4 text-yellow-500 shrink-0 mt-0.5" />
				<p class="text-xs text-yellow-500/90 leading-relaxed">
					Preview deployments require a GitHub, GitLab, Gitea, Bitbucket, or custom Git source.
					Switch the provider in the General tab first.
				</p>
			</div>
		{/if}
	</section>

	<!-- Config — only when enabled + git provider -->
	{#if enabled && isGitProvider}
		<section class="bg-card border border-border rounded-lg p-6">
			<h2 class="text-base font-semibold mb-4">Preview Configuration</h2>
			<div class="flex flex-col gap-4">
				<div class="grid grid-cols-2 gap-4">
					<div class="flex flex-col gap-1.5">
						<label for="prev-wildcard" class="text-sm font-medium text-muted-foreground">Wildcard Domain</label>
						<input id="prev-wildcard" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring" placeholder="*.preview.example.com" bind:value={wildcard} />
						<p class="text-xs text-muted-foreground">Subdomains will be generated per branch.</p>
					</div>
					<div class="flex flex-col gap-1.5">
						<label for="prev-port" class="text-sm font-medium text-muted-foreground">Exposed Port</label>
						<input id="prev-port" type="number" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring" placeholder="3000" bind:value={port} />
					</div>
				</div>
				<div class="grid grid-cols-2 gap-4">
					<div class="flex flex-col gap-1.5">
						<label for="prev-path" class="text-sm font-medium text-muted-foreground">Path Prefix</label>
						<input id="prev-path" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring" placeholder="/" bind:value={path} />
					</div>
					<div class="flex flex-col gap-1.5">
						<label for="prev-limit" class="text-sm font-medium text-muted-foreground">Max Active Previews</label>
						<input id="prev-limit" type="number" min="1" max="20" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring" bind:value={limit} />
					</div>
				</div>
				<div class="flex items-center gap-3">
					<input id="prev-https" type="checkbox" bind:checked={https} class="rounded cursor-pointer accent-primary" />
					<label for="prev-https" class="text-sm cursor-pointer">Enable HTTPS / SSL</label>
				</div>
				<div class="flex justify-end">
					<button
						class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50"
						disabled={saving}
					>
						{#if saving}
							<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>Saving…
						{:else}
							Save
						{/if}
					</button>
				</div>
			</div>
		</section>

		<!-- Active previews -->
		<section class="bg-card border border-border rounded-lg p-6">
			<h2 class="text-base font-semibold mb-1">Active Previews</h2>
			<p class="text-sm text-muted-foreground mb-6">Running preview environments for open pull requests.</p>
			<div class="flex flex-col items-center justify-center py-10 text-muted-foreground/30">
				<Eye class="w-10 h-10 mb-3" />
				<p class="text-sm">No active preview environments</p>
				<p class="text-xs mt-1">Open a pull request on your repository to trigger a preview.</p>
			</div>
		</section>
	{/if}
</div>
