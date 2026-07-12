<script lang="ts">
	import { goto } from '$app/navigation';
	import { Tag, Plus, Trash2, Pencil, Loader2 } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import * as Dialog from '$lib/components/ui/dialog';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	let tags = $state([
		{ id: 1, name: 'production', color: '#22c55e', count: 5 },
		{ id: 2, name: 'staging', color: '#eab308', count: 3 },
		{ id: 3, name: 'development', color: '#3b82f6', count: 8 },
		{ id: 4, name: 'deprecated', color: '#ef4444', count: 2 },
		{ id: 5, name: 'internal', color: '#8b5cf6', count: 4 }
	]);

	let showCreate = $state(false);
	let newTagName = $state('');
	let newTagColor = $state('#6366f1');
	let creating = $state(false);

	function addTag() {
		if (!newTagName.trim()) return;
		creating = true;
		setTimeout(() => {
			tags = [...tags, { id: Date.now(), name: newTagName.trim().toLowerCase(), color: newTagColor, count: 0 }];
			newTagName = '';
			newTagColor = '#6366f1';
			creating = false;
			showCreate = false;
		}, 300);
	}

	function removeTag(id: number) {
		tags = tags.filter((t) => t.id !== id);
	}
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<Tag class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Tags</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto animate-fade-up">
		<div class="w-full">
			<Card.Root>
				<Card.Header>
					<div class="flex items-center justify-between">
						<div>
							<Card.Title class="text-lg flex items-center gap-2">
								<Tag class="w-5 h-5 text-muted-foreground" />
								Tags
							</Card.Title>
							<Card.Description>Create and manage tags to organize your projects</Card.Description>
						</div>
						<Button size="sm" class="gap-1.5 text-xs" onclick={() => (showCreate = true)}>
							<Plus class="w-3.5 h-3.5" />
							Create Tag
						</Button>
					</div>
				</Card.Header>
				<Card.Content class="pt-4 border-t">
					{#if tags.length === 0}
						<div class="flex flex-col items-center gap-3 min-h-[25vh] justify-center">
							<Tag class="w-8 h-8 text-muted-foreground/40" />
							<p class="text-sm text-muted-foreground text-center">
								No tags yet. Create your first tag to start organizing projects.
							</p>
							<Button size="sm" variant="outline" class="gap-1.5 text-xs" onclick={() => (showCreate = true)}>
								<Plus class="w-3.5 h-3.5" />
								Create Tag
							</Button>
						</div>
					{:else}
						<div class="flex flex-col gap-2">
							{#each tags as tag, i (tag.id)}
								<div
									class="flex items-center justify-between p-3 rounded-lg border w-full animate-fade-up"
									style="animation-delay: {i * 30}ms"
								>
									<div class="flex items-center gap-3">
										<div class="w-3.5 h-3.5 rounded-full border border-border/50 shrink-0" style="background-color: {tag.color}"></div>
										<span class="text-sm font-medium">{tag.name}</span>
										{#if tag.color}
											<span class="text-[10px] text-muted-foreground font-mono">{tag.color}</span>
										{/if}
										<Badge variant="outline" class="text-[10px]">{tag.count} services</Badge>
									</div>
									<div class="flex items-center gap-1">
										<Button variant="ghost" size="sm" class="h-7 w-7 p-0">
											<Pencil class="w-3.5 h-3.5" />
										</Button>
										<Button variant="ghost" size="sm" class="h-7 w-7 p-0" onclick={() => removeTag(tag.id)}>
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

<!-- Create Tag Dialog -->
<Dialog.Root bind:open={showCreate}>
	<Dialog.Content class="sm:max-w-md">
		<Dialog.Header>
			<Dialog.Title>Create Tag</Dialog.Title>
			<Dialog.Description>Tags help organize and filter your services</Dialog.Description>
		</Dialog.Header>
		<div class="space-y-4 py-4">
			<div class="space-y-1.5">
				<Label for="tag-name" class="text-xs font-medium">Name</Label>
				<Input id="tag-name" bind:value={newTagName} placeholder="e.g. production" />
			</div>
			<div class="space-y-1.5">
				<Label for="tag-color" class="text-xs font-medium">Color</Label>
				<div class="flex items-center gap-3">
					<input
						id="tag-color"
						type="color"
						bind:value={newTagColor}
						class="w-9 h-9 rounded-md border border-border cursor-pointer bg-transparent p-0.5"
					/>
					<Input value={newTagColor} class="flex-1 font-mono text-xs" disabled />
					{#if newTagName}
						<Badge class="text-[10px]" style="background-color: {newTagColor}; color: white;">{newTagName}</Badge>
					{/if}
				</div>
			</div>
		</div>
		<Dialog.Footer>
			<Button variant="outline" size="sm" onclick={() => (showCreate = false)}>Cancel</Button>
			<Button size="sm" onclick={addTag} disabled={creating || !newTagName.trim()} class="gap-1.5">
				{#if creating}
					<Loader2 class="w-3.5 h-3.5 animate-spin" />
				{:else}
					<Plus class="w-3.5 h-3.5" />
				{/if}
				Create
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
