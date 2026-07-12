<script lang="ts">
	import { RefreshCw } from '@lucide/svelte';
	import { publicListBranches } from '$lib/client/sdk.gen';

	type Props = {
		value: string;
		onchange: (v: string) => void;
		repoUrl?: string;
		owner?: string;
		repo?: string;
		provider?: string;
	};

	let { value, onchange, repoUrl, owner, repo, provider = 'GITHUB' }: Props = $props();

	let branches = $state<string[]>([]);
	let loading = $state(false);
	let lastFetched = $state('');

	function buildUrl(): string {
		if (repoUrl) return repoUrl.trim();
		if (owner && repo) {
			const p = provider.toUpperCase();
			if (p === 'GITHUB')    return `https://github.com/${owner}/${repo}.git`;
			if (p === 'GITLAB')    return `https://gitlab.com/${owner}/${repo}.git`;
			if (p === 'GITEA')     return `https://gitea.com/${owner}/${repo}.git`;
			if (p === 'BITBUCKET') return `https://bitbucket.org/${owner}/${repo}.git`;
		}
		return '';
	}

	async function fetchBranches(url: string) {
		if (!url || url === lastFetched) return;
		loading = true;
		try {
			const res = await publicListBranches({ query: { query: { query: url } } as any });
			const raw = (res.data as string[] | undefined) ?? [];
			branches = raw
				.filter(b => b !== 'HEAD' && !b.startsWith('refs/tags/'))
				.map(b => b.replace('refs/heads/', ''));
			lastFetched = url;
		} catch {
			branches = [];
		} finally {
			loading = false;
		}
	}

	// Debounce auto-fetch when URL changes
	let debounceTimer: ReturnType<typeof setTimeout> | null = null;

	$effect(() => {
		const url = buildUrl();
		if (!url) return;
		if (debounceTimer) clearTimeout(debounceTimer);
		debounceTimer = setTimeout(() => fetchBranches(url), 600);
	});

	function manualRefresh(e: MouseEvent) {
		e.preventDefault();
		e.stopPropagation();
		lastFetched = ''; // force re-fetch
		fetchBranches(buildUrl());
	}
</script>

<div class="flex gap-2 items-center">
	<div class="relative flex-1">
		<select
			class="h-9 w-full rounded-md border border-input bg-secondary px-3 text-sm focus:outline-none focus:ring-1 focus:ring-ring disabled:opacity-60"
			{value}
			onchange={(e) => onchange((e.target as HTMLSelectElement).value)}
			disabled={loading || branches.length === 0}
		>
			{#if loading}
				<option value="">Loading branches…</option>
			{:else if branches.length === 0}
				<option value="">{buildUrl() ? 'No branches found' : 'Enter repo URL first'}</option>
			{:else}
				<option value="">Select branch</option>
				{#each branches as b}
					<option value={b} selected={value === b}>{b}</option>
				{/each}
			{/if}
		</select>
		{#if loading}
			<div class="absolute right-8 top-1/2 -translate-y-1/2 w-3.5 h-3.5 border-2 border-muted-foreground/30 border-t-muted-foreground rounded-full animate-spin pointer-events-none"></div>
		{/if}
	</div>
	<button
		type="button"
		onclick={manualRefresh}
		disabled={loading || !buildUrl()}
		class="p-2 rounded-md text-muted-foreground hover:text-foreground hover:bg-accent disabled:opacity-40 transition-colors"
		title="Refresh branches"
	>
		<RefreshCw size={14} class={loading ? 'animate-spin' : ''} />
	</button>
</div>
