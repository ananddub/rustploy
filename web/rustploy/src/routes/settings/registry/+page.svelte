<script lang="ts">
	import { goto } from '$app/navigation';
	import { Database, Plus, Trash2, ExternalLink, CheckCircle, Settings, Loader2 } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	let showCreate = $state(false);

	let registries = $state([
		{ id: 1, name: 'Docker Hub', url: 'https://registry.hub.docker.com', username: 'rustploy', status: 'connected', images: 12 },
		{ id: 2, name: 'GitHub GHCR', url: 'https://ghcr.io', username: 'org/rustploy', status: 'connected', images: 8 },
		{ id: 3, name: 'Private Registry', url: 'https://registry.internal.io', username: 'admin', status: 'connected', images: 3 }
	]);

	function removeRegistry(id: number) {
		registries = registries.filter((r) => r.id !== id);
	}
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<Database class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Docker Registry</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto animate-fade-up">
		<div class="w-full">
			<Card.Root>
				<Card.Header>
					<div class="flex items-center justify-between">
						<div>
							<Card.Title class="text-base flex items-center gap-2">
								<Database class="w-4 h-4 text-muted-foreground" />
								Docker Registry
							</Card.Title>
							<Card.Description>Manage container registries for pulling and pushing images</Card.Description>
						</div>
						<Button size="sm" class="gap-1.5 text-xs" onclick={() => (showCreate = true)}>
							<Plus class="w-3.5 h-3.5" />
							Add Registry
						</Button>
					</div>
				</Card.Header>
				<Card.Content>
					{#if registries.length === 0}
						<div class="flex flex-col items-center gap-3 min-h-[25vh] justify-center">
							<Database class="w-8 h-8 text-muted-foreground/40" />
							<p class="text-sm text-muted-foreground">No registries configured</p>
						</div>
					{:else}
						<div class="flex flex-col gap-2">
							{#each registries as reg, i (reg.id)}
								<div class="flex items-center justify-between p-3.5 rounded-lg border animate-fade-up" style="animation-delay: {i * 40}ms">
									<div class="flex items-center gap-3">
										<div class="w-8 h-8 rounded-md bg-primary/10 flex items-center justify-center shrink-0">
											<Database class="w-4 h-4 text-primary" />
										</div>
										<div class="flex flex-col gap-0.5">
											<div class="flex items-center gap-2">
												<span class="text-sm font-medium">{reg.name}</span>
												<Badge variant="default" class="text-[9px] gap-1">
													<CheckCircle class="w-2.5 h-2.5" />
													Connected
												</Badge>
											</div>
											<span class="text-[11px] text-muted-foreground font-mono">{reg.url}</span>
										</div>
									</div>
									<div class="flex items-center gap-3">
										<div class="text-right hidden sm:block">
											<p class="text-xs font-medium">{reg.images} images</p>
											<p class="text-[10px] text-muted-foreground">@{reg.username}</p>
										</div>
										<Button variant="ghost" size="sm" class="h-7 w-7 p-0">
											<ExternalLink class="w-3.5 h-3.5" />
										</Button>
										<Button variant="ghost" size="sm" class="h-7 w-7 p-0">
											<Settings class="w-3.5 h-3.5" />
										</Button>
										<Button variant="ghost" size="sm" class="h-7 w-7 p-0" onclick={() => removeRegistry(reg.id)}>
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

<Dialog.Root bind:open={showCreate}>
	<Dialog.Content class="sm:max-w-lg">
		<Dialog.Header>
			<Dialog.Title>Add Docker Registry</Dialog.Title>
			<Dialog.Description>Connect a container registry to pull/push images</Dialog.Description>
		</Dialog.Header>
		<div class="space-y-4 py-4">
			<div class="space-y-1.5">
				<Label class="text-xs">Registry Name</Label>
				<Input placeholder="e.g. Docker Hub" />
			</div>
			<div class="space-y-1.5">
				<Label class="text-xs">Registry URL</Label>
				<Input placeholder="https://registry.hub.docker.com" />
			</div>
			<div class="grid grid-cols-2 gap-3">
				<div class="space-y-1.5">
					<Label class="text-xs">Username</Label>
					<Input placeholder="username" />
				</div>
				<div class="space-y-1.5">
					<Label class="text-xs">Password / Token</Label>
					<Input type="password" placeholder="••••••••" />
				</div>
			</div>
		</div>
		<Dialog.Footer>
			<Button variant="outline" size="sm" onclick={() => (showCreate = false)}>Cancel</Button>
			<Button size="sm" class="gap-1.5">
				<Plus class="w-3.5 h-3.5" />
				Add Registry
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
