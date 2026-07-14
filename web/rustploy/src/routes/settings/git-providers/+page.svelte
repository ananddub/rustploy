<script lang="ts">
	import { goto } from '$app/navigation';
	import { GitBranch, Plus, Trash2, Settings, ExternalLink, Users, Loader2 } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Switch } from '$lib/components/ui/switch';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	let providers = $state([
		{ id: 1, name: 'GitHub - rustploy-org', type: 'github', configured: true, shared: false, createdAt: '2026-06-15' },
		{ id: 2, name: 'GitLab - rustploy', type: 'gitlab', configured: true, shared: true, createdAt: '2026-05-20' },
		{ id: 3, name: 'Bitbucket - team', type: 'bitbucket', configured: true, shared: false, createdAt: '2026-04-10' },
		{ id: 4, name: 'Gitea - self-hosted', type: 'gitea', configured: true, shared: false, createdAt: '2026-03-22' }
	]);

	function removeProvider(id: number) {
		providers = providers.filter((p) => p.id !== id);
	}
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<GitBranch class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Git Providers</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto animate-fade-up">
		<div class="w-full">
			<Card.Root>
				<Card.Header>
					<Card.Title class="text-base flex items-center gap-2">
						<GitBranch class="w-4 h-4 text-muted-foreground" />
						Git Providers
					</Card.Title>
					<Card.Description>Connect your Git provider for authentication and repository access.</Card.Description>
				</Card.Header>
				<Card.Content class="space-y-5">
					<!-- Add provider buttons -->
					<div class="flex flex-wrap items-center gap-2">
						<Button variant="outline" size="sm" class="gap-2 text-xs h-9">
							<svg class="w-4 h-4" viewBox="0 0 24 24" fill="currentColor">
								<path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0 0 24 12c0-6.63-5.37-12-12-12z"/>
							</svg>
							GitHub
						</Button>
						<Button variant="outline" size="sm" class="gap-2 text-xs h-9">
							<svg class="w-4 h-4" viewBox="0 0 24 24">
								<path d="M23.955 13.587l-1.342-4.135-2.664-8.189a.455.455 0 0 0-.867 0L16.418 9.45H7.582L4.918 1.263a.455.455 0 0 0-.867 0L1.386 9.452.044 13.587a.924.924 0 0 0 .331 1.023L12 23.054l11.625-8.443a.92.92 0 0 0 .33-1.024z" fill="#fc6d26"/>
							</svg>
							GitLab
						</Button>
						<Button variant="outline" size="sm" class="gap-2 text-xs h-9">
							<svg class="w-4 h-4" viewBox="0 0 24 24">
								<path d="M.778 1.213a.768.768 0 0 0-.768.892l3.263 19.81c.084.5.515.868 1.022.873H19.95a.772.772 0 0 0 .77-.646l3.27-20.03a.768.768 0 0 0-.768-.891H.778zM14.52 15.53H9.522L8.17 8.466h7.561l-1.211 7.064z" fill="#2684ff"/>
							</svg>
							Bitbucket
						</Button>
						<Button variant="outline" size="sm" class="gap-2 text-xs h-9">
							<svg class="w-4 h-4" viewBox="0 0 24 24">
								<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm3.2 14.4H8.8c-.88 0-1.6-.72-1.6-1.6V9.2c0-.88.72-1.6 1.6-1.6h6.4c.88 0 1.6.72 1.6 1.6v5.6c0 .88-.72 1.6-1.6 1.6z" fill="#609926"/>
							</svg>
							Gitea
						</Button>
					</div>

					<!-- Connected providers -->
					{#if providers.length === 0}
						<div class="flex flex-col items-center gap-3 min-h-[25vh] justify-center">
							<GitBranch class="w-8 h-8 text-muted-foreground/40" />
							<p class="text-sm text-muted-foreground text-center">No Git Providers configured</p>
						</div>
					{:else}
						<div class="flex flex-col gap-2">
							{#each providers as provider, i (provider.id)}
								<div class="flex items-center justify-between p-3.5 rounded-lg border animate-fade-up" style="animation-delay: {i * 40}ms">
									<div class="flex items-center gap-3">
										{#if provider.type === 'github'}
											<svg class="w-5 h-5 shrink-0" viewBox="0 0 24 24" fill="currentColor">
												<path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0 0 24 12c0-6.63-5.37-12-12-12z"/>
											</svg>
										{:else if provider.type === 'gitlab'}
											<svg class="w-5 h-5 shrink-0" viewBox="0 0 24 24">
												<path d="M23.955 13.587l-1.342-4.135-2.664-8.189a.455.455 0 0 0-.867 0L16.418 9.45H7.582L4.918 1.263a.455.455 0 0 0-.867 0L1.386 9.452.044 13.587a.924.924 0 0 0 .331 1.023L12 23.054l11.625-8.443a.92.92 0 0 0 .33-1.024z" fill="#fc6d26"/>
											</svg>
										{:else if provider.type === 'bitbucket'}
											<svg class="w-5 h-5 shrink-0" viewBox="0 0 24 24">
												<path d="M.778 1.213a.768.768 0 0 0-.768.892l3.263 19.81c.084.5.515.868 1.022.873H19.95a.772.772 0 0 0 .77-.646l3.27-20.03a.768.768 0 0 0-.768-.891H.778zM14.52 15.53H9.522L8.17 8.466h7.561l-1.211 7.064z" fill="#2684ff"/>
											</svg>
										{:else}
											<svg class="w-5 h-5 shrink-0" viewBox="0 0 24 24">
												<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm3.2 14.4H8.8c-.88 0-1.6-.72-1.6-1.6V9.2c0-.88.72-1.6 1.6-1.6h6.4c.88 0 1.6.72 1.6 1.6v5.6c0 .88-.72 1.6-1.6 1.6z" fill="#609926"/>
											</svg>
										{/if}
										<div class="flex flex-col gap-0.5">
											<span class="text-sm font-medium">{provider.name}</span>
											<span class="text-[11px] text-muted-foreground">{provider.createdAt}</span>
										</div>
										{#if provider.shared}
											<Badge variant="secondary" class="text-[10px] gap-1">
												<Users class="w-3 h-3" />
												Shared
											</Badge>
										{/if}
									</div>
									<div class="flex items-center gap-2">
										<Badge variant="default" class="text-[10px]">Configured</Badge>
										<Button variant="ghost" size="sm" class="h-7 w-7 p-0">
											<ExternalLink class="w-3.5 h-3.5" />
										</Button>
										<Button variant="ghost" size="sm" class="h-7 w-7 p-0">
											<Settings class="w-3.5 h-3.5" />
										</Button>
										<Button variant="ghost" size="sm" class="h-7 w-7 p-0" onclick={() => removeProvider(provider.id)}>
											<Trash2 class="w-3.5 h-3.5 text-destructive" />
										</Button>
									</div>
								</div>
							{/each}
						</div>
					{/if}
				</Card.Content>
			</Card.Root>
		</div>
	</main>
</PageLayout>
