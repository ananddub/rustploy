<script lang="ts">
	import { goto } from '$app/navigation';
	import { Tag, Plus, Trash2, PenBox, Loader2, Palette } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { toastSuccess, toastError } from '$lib/toast';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	type TagItem = { id: string; name: string; color: string };

	let tags = $state<TagItem[]>([
		{ id:'t1', name:'production',  color:'#22c55e' },
		{ id:'t2', name:'staging',     color:'#eab308' },
		{ id:'t3', name:'development', color:'#3b82f6' },
		{ id:'t4', name:'deprecated',  color:'#ef4444' },
		{ id:'t5', name:'internal',    color:'#8b5cf6' },
	]);

	// ─── Modal ────────────────────────────────────────────────────────────────────
	let showModal  = $state(false);
	let editingId  = $state<string|null>(null);
	let saving     = $state(false);
	let modalError = $state('');
	let mName      = $state('');
	let mColor     = $state('#3b82f6');
	let colorEl: HTMLInputElement;

	function openCreate() {
		editingId = null; mName = ''; mColor = '#3b82f6'; modalError = '';
		showModal = true;
	}

	function openEdit(tag: TagItem) {
		editingId = tag.id; mName = tag.name; mColor = tag.color ?? '#3b82f6'; modalError = '';
		showModal = true;
	}

	async function submitModal(e: SubmitEvent) {
		e.preventDefault(); modalError = '';
		if (!mName.trim()) { modalError = 'Tag name is required'; return; }
		if (mName.trim().length > 50) { modalError = 'Tag name must be less than 50 characters'; return; }
		saving = true;
		try {
			await new Promise(r => setTimeout(r, 400));
			if (editingId) {
				tags = tags.map(t => t.id === editingId ? { ...t, name: mName.trim(), color: mColor } : t);
				toastSuccess('Tag updated');
			} else {
				tags = [...tags, { id: `t${Date.now()}`, name: mName.trim(), color: mColor }];
				toastSuccess('Tag created');
			}
			showModal = false; editingId = null;
		} catch { modalError = 'Failed to save tag'; }
		finally { saving = false; }
	}

	// ─── Delete confirm ───────────────────────────────────────────────────────────
	let confirmDeleteId = $state<string|null>(null);
	let deletingId      = $state<string|null>(null);

	async function deleteTag(id: string) {
		deletingId = id;
		try {
			await new Promise(r => setTimeout(r, 400));
			tags = tags.filter(t => t.id !== id);
			toastSuccess('Tag deleted');
		} finally { deletingId = null; confirmDeleteId = null; }
	}
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<Tag class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Tags</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto">
		<div class="w-full max-w-5xl mx-auto">
			<div class="rounded-xl border border-border bg-card">

				<!-- Header -->
				<div class="px-6 pt-5 pb-4 border-b border-border">
					<h2 class="text-xl font-bold flex items-center gap-2">
						<Tag class="w-5 h-5 text-muted-foreground" /> Tags
					</h2>
					<p class="text-sm text-muted-foreground mt-0.5">Create and manage tags to organize your projects</p>
				</div>

				<div class="px-6 py-5">
					{#if tags.length === 0}
						<div class="flex flex-col items-center gap-3 min-h-[25vh] justify-center text-muted-foreground">
							<Tag class="w-8 h-8 opacity-40" />
							<p class="text-base text-center">No tags yet. Create your first tag to start organizing projects.</p>
							<Button size="sm" class="gap-1.5 mt-1" onclick={openCreate}>
								<Plus class="w-4 h-4" /> Create Tag
							</Button>
						</div>
					{:else}
						<div class="flex flex-col gap-4 min-h-[25vh]">
							<div class="flex flex-col gap-4 rounded-lg">
								{#each tags as tag (tag.id)}
									<!-- Two-layer card matching dokploy exactly -->
									<div class="flex items-center justify-between bg-muted/40 p-1 w-full rounded-lg">
										<div class="flex items-center justify-between p-3.5 rounded-lg bg-background border border-border w-full">
											<div class="flex items-center gap-3">
												<!-- TagBadge: colored square + name -->
												<div class="flex items-center gap-1.5">
													<span class="inline-block w-3.5 h-3.5 rounded-[3px] shrink-0 border border-black/10" style="background:{tag.color}"></span>
													<span class="text-sm font-medium">{tag.name}</span>
												</div>
												{#if tag.color}
													<span class="text-xs text-muted-foreground font-mono">{tag.color}</span>
												{/if}
											</div>
											<div class="flex items-center gap-1">
												<Button variant="ghost" size="icon" class="h-8 w-8" onclick={() => openEdit(tag)} title="Edit tag">
													<PenBox class="w-4 h-4" />
												</Button>
												<Button
													variant="ghost" size="icon"
													class="h-8 w-8 group hover:bg-red-500/10"
													onclick={() => (confirmDeleteId = tag.id)}
													disabled={deletingId === tag.id}
													title="Delete tag"
												>
													{#if deletingId === tag.id}
														<Loader2 class="w-4 h-4 animate-spin" />
													{:else}
														<Trash2 class="w-4 h-4 text-muted-foreground group-hover:text-red-500 transition-colors" />
													{/if}
												</Button>
											</div>
										</div>
									</div>
								{/each}
							</div>

							<!-- Add more button bottom-right -->
							<div class="flex justify-end">
								<Button size="sm" class="gap-1.5" onclick={openCreate}>
									<Plus class="w-4 h-4" /> Create Tag
								</Button>
							</div>
						</div>
					{/if}
				</div>
			</div>
		</div>
	</main>
</PageLayout>

<!-- ── Create / Edit Modal ────────────────────────────────────────────────── -->
{#if showModal}
	<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1"
		onclick={() => { showModal=false; editingId=null; }} onkeydown={() => {}}></div>
	<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
		<form onsubmit={submitModal}
			class="bg-card border border-border rounded-xl shadow-2xl w-full max-w-lg pointer-events-auto flex flex-col gap-5 p-6">

			<div class="flex items-start justify-between">
				<div>
					<h2 class="text-base font-semibold">{editingId ? 'Update' : 'Create'} Tag</h2>
					<p class="text-sm text-muted-foreground mt-0.5">
						{editingId ? 'Update the tag name and color' : 'Create a new tag to organize your projects'}
					</p>
				</div>
				<button type="button" onclick={() => { showModal=false; editingId=null; }}
					class="text-muted-foreground hover:text-foreground p-1 rounded hover:bg-accent">✕</button>
			</div>

			<!-- Name -->
			<div class="space-y-1.5">
				<Label for="t-name" class="text-sm">Name</Label>
				<Input id="t-name" bind:value={mName} placeholder="e.g., Production, Client, Internal" required />
				{#if modalError && modalError.includes('name')}
					<p class="text-xs text-destructive">{modalError}</p>
				{/if}
			</div>

			<!-- Color -->
			<div class="space-y-1.5">
				<Label class="text-sm">Color <span class="text-muted-foreground">(Optional)</span></Label>
				<div class="flex items-center gap-3">
					<!-- Colored square that opens color picker -->
					<button
						type="button"
						class="w-12 h-12 rounded-md border-2 border-border cursor-pointer hover:opacity-80 transition-opacity shrink-0 flex items-center justify-center relative"
						style="background-color:{mColor}"
						onclick={() => colorEl?.click()}
					>
						<input
							bind:this={colorEl}
							type="color"
							class="absolute opacity-0 pointer-events-none w-0 h-0"
							bind:value={mColor}
						/>
					</button>
					<div class="flex-1 space-y-1">
						<Input
							placeholder="#3b82f6"
							value={mColor}
							oninput={(e) => {
								const v = (e.target as HTMLInputElement).value;
								if (v.startsWith('#') || v === '') mColor = v;
							}}
							class="font-mono text-sm"
						/>
						<p class="text-[10px] text-muted-foreground">Choose a color to easily identify this tag</p>
					</div>
				</div>
			</div>

			<!-- Preview -->
			{#if mColor && mName}
				<div class="flex items-center gap-2 text-sm">
					<span class="text-muted-foreground">Preview:</span>
					<div class="flex items-center gap-1.5">
						<span class="inline-block w-3.5 h-3.5 rounded-[3px] shrink-0 border border-black/10" style="background:{mColor}"></span>
						<span class="font-medium text-sm">{mName}</span>
					</div>
				</div>
			{/if}

			{#if modalError && !modalError.includes('name')}
				<p class="text-xs text-destructive bg-destructive/10 border border-destructive/20 rounded px-3 py-2">{modalError}</p>
			{/if}

			<div class="flex justify-end gap-2 pt-1">
				<Button type="button" variant="outline" size="sm" onclick={() => { showModal=false; editingId=null; }}>Cancel</Button>
				<Button type="submit" size="sm" disabled={saving || !mName.trim()} class="gap-1.5 min-w-[80px]">
					{#if saving}<Loader2 class="w-3.5 h-3.5 animate-spin" />{/if}
					{editingId ? 'Update' : 'Create'}
				</Button>
			</div>
		</form>
	</div>
{/if}

<!-- ── Delete Confirm ─────────────────────────────────────────────────────── -->
{#if confirmDeleteId}
	{@const target = tags.find(t => t.id === confirmDeleteId)}
	<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1"
		onclick={() => (confirmDeleteId=null)} onkeydown={() => {}}></div>
	<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
		<div class="bg-card border border-border rounded-xl shadow-2xl w-full max-w-sm p-6 pointer-events-auto flex flex-col gap-4">
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 rounded-full bg-destructive/10 flex items-center justify-center shrink-0">
					<Trash2 class="w-5 h-5 text-destructive" />
				</div>
				<div>
					<h2 class="text-sm font-semibold">Delete Tag</h2>
					<p class="text-sm text-muted-foreground mt-0.5">
						Are you sure you want to delete the tag
						<strong class="text-foreground"> "{target?.name}"</strong>?
						This will remove the tag from all projects. This cannot be undone.
					</p>
				</div>
			</div>
			<div class="flex justify-end gap-2">
				<Button variant="outline" size="sm" onclick={() => (confirmDeleteId=null)}>Cancel</Button>
				<Button variant="destructive" size="sm"
					onclick={() => deleteTag(confirmDeleteId!)}
					disabled={deletingId !== null} class="gap-1.5">
					{#if deletingId}<Loader2 class="w-3.5 h-3.5 animate-spin" /> Deleting…
					{:else}<Trash2 class="w-3.5 h-3.5" /> Delete{/if}
				</Button>
			</div>
		</div>
	</div>
{/if}
