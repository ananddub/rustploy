<script lang="ts">
	import { goto } from '$app/navigation';
	import { GitBranch, Plus, Trash2, Settings, ExternalLink, Users, ShieldCheck } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';

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
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border/60 text-xs">
		<GitBranch class="w-3.5 h-3.5 text-muted-foreground" />
		<span class="font-semibold text-foreground">Git Providers</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto animate-fade-up flex flex-col items-center">
		<div class="w-full max-w-4xl space-y-6">
			<Card.Root class="bg-card border-border/80 shadow-xs">
				<Card.Header class="border-b border-border/40 pb-4">
					<div class="flex items-center justify-between">
						<div>
							<Card.Title class="text-base font-semibold flex items-center gap-2">
								<GitBranch class="w-4 h-4 text-primary" />
								Git Providers
							</Card.Title>
							<Card.Description class="text-xs text-muted-foreground mt-1">
								Connect your Git provider for automated code deployments and webhook triggers.
							</Card.Description>
						</div>
					</div>
				</Card.Header>
				<Card.Content class="pt-5 space-y-6">
					<!-- Branded Add Provider Action Row -->
					<div>
						<p class="text-xs font-medium text-muted-foreground mb-3">Add a new provider</p>
						<div class="grid grid-cols-2 sm:grid-cols-4 gap-3">
							<!-- GitHub -->
							<button class="flex items-center justify-center gap-2 px-3 py-2.5 rounded-lg border border-border/70 bg-card/60 hover:bg-accent/60 hover:border-foreground/20 text-xs font-medium text-foreground transition-all group shadow-2xs">
								<svg class="w-4 h-4 transition-transform group-hover:scale-110" viewBox="0 0 24 24" fill="currentColor">
									<path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0 0 24 12c0-6.63-5.37-12-12-12z"/>
								</svg>
								GitHub
							</button>
							<!-- GitLab -->
							<button class="flex items-center justify-center gap-2 px-3 py-2.5 rounded-lg border border-border/70 bg-card/60 hover:bg-accent/60 hover:border-[#fc6d26]/40 text-xs font-medium text-foreground transition-all group shadow-2xs">
								<svg class="w-4 h-4 transition-transform group-hover:scale-110" viewBox="0 0 24 24">
									<path d="M23.955 13.587l-1.342-4.135-2.664-8.189a.455.455 0 0 0-.867 0L16.418 9.45H7.582L4.918 1.263a.455.455 0 0 0-.867 0L1.386 9.452.044 13.587a.924.924 0 0 0 .331 1.023L12 23.054l11.625-8.443a.92.92 0 0 0 .33-1.024z" fill="#fc6d26"/>
								</svg>
								GitLab
							</button>
							<!-- Bitbucket -->
							<button class="flex items-center justify-center gap-2 px-3 py-2.5 rounded-lg border border-border/70 bg-card/60 hover:bg-accent/60 hover:border-[#2684ff]/40 text-xs font-medium text-foreground transition-all group shadow-2xs">
								<svg class="w-4 h-4 transition-transform group-hover:scale-110" viewBox="0 0 24 24">
									<path d="M.778 1.213a.768.768 0 0 0-.768.892l3.263 19.81c.084.5.515.868 1.022.873H19.95a.772.772 0 0 0 .77-.646l3.27-20.03a.768.768 0 0 0-.768-.891H.778zM14.52 15.53H9.522L8.17 8.466h7.561l-1.211 7.064z" fill="#2684ff"/>
								</svg>
								Bitbucket
							</button>
							<!-- Gitea -->
							<button class="flex items-center justify-center gap-2 px-3 py-2.5 rounded-lg border border-border/70 bg-card/60 hover:bg-accent/60 hover:border-[#609926]/40 text-xs font-medium text-foreground transition-all group shadow-2xs">
								<svg class="w-4 h-4 transition-transform group-hover:scale-110" viewBox="0 0 24 24">
									<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm3.2 14.4H8.8c-.88 0-1.6-.72-1.6-1.6V9.2c0-.88.72-1.6 1.6-1.6h6.4c.88 0 1.6.72 1.6 1.6v5.6c0 .88-.72 1.6-1.6 1.6z" fill="#609926"/>
								</svg>
								Gitea
							</button>
						</div>
					</div>

					<!-- Connected providers list -->
					<div class="space-y-3 pt-2">
						<p class="text-xs font-medium text-muted-foreground">Configured integrations</p>
						{#if providers.length === 0}
							<div class="flex flex-col items-center justify-center py-12 border border-dashed border-border/60 rounded-lg text-center">
								<GitBranch class="w-8 h-8 text-muted-foreground/40 mb-2" />
								<p class="text-xs font-medium text-foreground">No Git Providers connected</p>
								<p class="text-[11px] text-muted-foreground mt-0.5">Select a provider above to connect your account</p>
							</div>
						{:else}
							<div class="grid grid-cols-1 gap-2.5">
								{#each providers as provider, i (provider.id)}
									<div class="flex items-center justify-between p-3.5 rounded-lg border border-border/60 bg-card/40 hover:bg-accent/20 transition-all">
										<div class="flex items-center gap-3">
											<div class="w-8 h-8 rounded-lg bg-card border border-border/60 flex items-center justify-center shrink-0 shadow-2xs">
												{#if provider.type === 'github'}
													<svg class="w-4 h-4" viewBox="0 0 24 24" fill="currentColor">
														<path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0 0 24 12c0-6.63-5.37-12-12-12z"/>
													</svg>
												{:else if provider.type === 'gitlab'}
													<svg class="w-4 h-4" viewBox="0 0 24 24">
														<path d="M23.955 13.587l-1.342-4.135-2.664-8.189a.455.455 0 0 0-.867 0L16.418 9.45H7.582L4.918 1.263a.455.455 0 0 0-.867 0L1.386 9.452.044 13.587a.924.924 0 0 0 .331 1.023L12 23.054l11.625-8.443a.92.92 0 0 0 .33-1.024z" fill="#fc6d26"/>
													</svg>
												{:else if provider.type === 'bitbucket'}
													<svg class="w-4 h-4" viewBox="0 0 24 24">
														<path d="M.778 1.213a.768.768 0 0 0-.768.892l3.263 19.81c.084.5.515.868 1.022.873H19.95a.772.772 0 0 0 .77-.646l3.27-20.03a.768.768 0 0 0-.768-.891H.778zM14.52 15.53H9.522L8.17 8.466h7.561l-1.211 7.064z" fill="#2684ff"/>
													</svg>
												{:else}
													<svg class="w-4 h-4" viewBox="0 0 24 24">
														<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm3.2 14.4H8.8c-.88 0-1.6-.72-1.6-1.6V9.2c0-.88.72-1.6 1.6-1.6h6.4c.88 0 1.6.72 1.6 1.6v5.6c0 .88-.72 1.6-1.6 1.6z" fill="#609926"/>
													</svg>
												{/if}
											</div>
											<div class="flex flex-col gap-0.5">
												<div class="flex items-center gap-2">
													<span class="text-xs font-semibold text-foreground">{provider.name}</span>
													{#if provider.shared}
														<Badge variant="secondary" class="text-[9px] h-4 gap-1 px-1.5 font-normal">
															<Users class="w-2.5 h-2.5" />
															Shared
														</Badge>
													{/if}
												</div>
												<span class="text-[10px] text-muted-foreground">Connected on {provider.createdAt}</span>
											</div>
										</div>
										<div class="flex items-center gap-1.5">
											<Badge variant="outline" class="text-[10px] bg-green-500/10 text-green-400 border-green-500/20 font-medium">Configured</Badge>
											<Button variant="ghost" size="sm" class="h-7 w-7 p-0 text-muted-foreground hover:text-foreground">
												<ExternalLink class="w-3.5 h-3.5" />
											</Button>
											<Button variant="ghost" size="sm" class="h-7 w-7 p-0 text-muted-foreground hover:text-foreground">
												<Settings class="w-3.5 h-3.5" />
											</Button>
											<Button variant="ghost" size="sm" class="h-7 w-7 p-0 text-muted-foreground hover:text-destructive" onclick={() => removeProvider(provider.id)}>
												<Trash2 class="w-3.5 h-3.5" />
											</Button>
										</div>
									</div>
								{/each}
							</div>
						{/if}
					</div>
				</Card.Content>
			</Card.Root>
		</div>
	</main>
</PageLayout>
