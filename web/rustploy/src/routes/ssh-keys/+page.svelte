<script lang="ts">
	import { goto } from '$app/navigation';
	import { RocketIcon, Key, Plus, RefreshCw, FileKey, CheckCircle, Copy, Clock, Pencil, Trash2, Eye, EyeOff } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import { sshKeyControllerList, sshKeyControllerCreate, sshKeyControllerPatch, sshKeyControllerDelete } from '$lib/client/sdk.gen';
	import type { SshKeyResponseDto } from '$lib/client/types.gen';
	import { formatDate } from '$lib/helpers';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	let keys = $state<SshKeyResponseDto[]>([]);
	let loading = $state(true);
	let showCreate = $state(false);
	let editKey = $state<SshKeyResponseDto | null>(null);
	let deletingId = $state<number | null>(null);
	let copied = $state<number | null>(null);

	async function loadKeys() {
		loading = true;
		try {
			const res = await sshKeyControllerList();
			keys = (res.data as SshKeyResponseDto[]) ?? [];
		} finally { loading = false; }
	}

	loadKeys();

	async function deleteKey(id: number) {
		deletingId = id;
		try {
			await sshKeyControllerDelete({ path: { id } });
			keys = keys.filter(k => k.id !== id);
		} finally { deletingId = null; }
	}

	async function copyPublicKey(key: SshKeyResponseDto) {
		await navigator.clipboard.writeText(key.public_key);
		copied = key.id;
		setTimeout(() => (copied = null), 2000);
	}

	function truncateKey(key: string, len = 48) {
		return key.length > len ? key.slice(0, len) + '…' : key;
	}

	function handleSaved(k: SshKeyResponseDto) {
		const exists = keys.find(x => x.id === k.id);
		if (exists) keys = keys.map(x => x.id === k.id ? k : x);
		else keys = [...keys, k];
		showCreate = false;
		editKey = null;
	}

	// Modal state
	let modalName = $state('');
	let modalDesc = $state('');
	let modalPrivateKey = $state('');
	let modalPublicKey = $state('');
	let modalShowPrivate = $state(false);
	let modalSaving = $state(false);
	let modalError = $state('');

	function openCreate() {
		modalName = ''; modalDesc = ''; modalPrivateKey = ''; modalPublicKey = '';
		modalShowPrivate = false; modalError = '';
		showCreate = true;
	}

	function openEdit(key: SshKeyResponseDto) {
		modalName = key.name; modalDesc = key.description ?? '';
		modalPrivateKey = ''; modalPublicKey = key.public_key;
		modalShowPrivate = false; modalError = '';
		editKey = key;
	}

	async function submitModal(e: SubmitEvent) {
		e.preventDefault(); modalError = ''; modalSaving = true;
		try {
			if (editKey) {
				const res = await sshKeyControllerPatch({
					path: { id: editKey.id },
					body: { name: modalName.trim(), description: modalDesc.trim() || undefined, ...(modalPrivateKey.trim() ? { private_key: modalPrivateKey.trim() } : {}), ...(modalPublicKey.trim() ? { public_key: modalPublicKey.trim() } : {}) }
				});
				if (res.data) handleSaved(res.data);
			} else {
				if (!modalPrivateKey.trim() || !modalPublicKey.trim()) { modalError = 'Both private and public key are required'; return; }
				const res = await sshKeyControllerCreate({ body: { name: modalName.trim(), private_key: modalPrivateKey.trim(), public_key: modalPublicKey.trim(), description: modalDesc.trim() || undefined } as any });
				if (res.data) handleSaved(res.data);
			}
		} catch (err: any) { modalError = err?.message ?? 'Failed to save SSH key'; }
		finally { modalSaving = false; }
	}
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<RocketIcon class="w-4 h-4 text-muted-foreground" />
		<button onclick={() => goto('/dashboard')} class="text-muted-foreground hover:text-foreground transition-colors">Dashboard</button>
		<span class="text-muted-foreground/30">/</span>
		<span class="font-medium flex items-center gap-1.5"><Key class="w-4 h-4" /> SSH Keys</span>
	</header>

	<main class="flex-1 px-8 py-8">
		<div class="flex items-center justify-between mb-6">
			<div>
				<h1 class="text-2xl font-semibold">SSH Keys</h1>
				<p class="text-sm text-muted-foreground mt-1">Manage SSH keys used for server and repository authentication</p>
			</div>
			<div class="flex items-center gap-2">
				<button onclick={loadKeys} class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent transition-colors">
					<RefreshCw class="w-3.5 h-3.5" /> Refresh
				</button>
				<button onclick={openCreate} class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 transition-colors">
					<Plus class="w-4 h-4" /> Add SSH Key
				</button>
			</div>
		</div>

		{#if loading}
			<div class="flex justify-center py-20"><div class="w-6 h-6 border-2 border-muted-foreground/30 border-t-foreground rounded-full animate-spin"></div></div>
		{:else if keys.length === 0}
			<div class="flex flex-col items-center justify-center py-24 text-muted-foreground/40">
				<Key class="w-14 h-14 mb-4 opacity-30" />
				<p class="text-sm font-medium">No SSH keys yet</p>
				<p class="text-xs mt-1">Add a key to authenticate with your remote servers</p>
				<button onclick={openCreate} class="mt-5 px-3 py-1.5 rounded-md border border-border text-sm hover:bg-accent transition-colors inline-flex items-center gap-1.5">
					<Plus class="w-4 h-4" /> Add your first key
				</button>
			</div>
		{:else}
			<div class="flex flex-col gap-3">
				{#each keys as key (key.id)}
					<div class="bg-card border border-border rounded-xl p-5 flex items-start gap-4 hover:border-foreground/20 transition-colors">
						<div class="w-10 h-10 rounded-lg bg-muted flex items-center justify-center shrink-0 mt-0.5">
							<FileKey class="w-5 h-5 text-muted-foreground" />
						</div>
						<div class="flex-1 min-w-0">
							<div class="flex items-center gap-2 flex-wrap">
								<p class="text-sm font-semibold">{key.name}</p>
								{#if key.has_private_key}
									<span class="inline-flex items-center gap-1 text-xs bg-green-500/15 text-green-600 px-2 py-0.5 rounded">
										<CheckCircle class="w-3 h-3" /> Private key set
									</span>
								{:else}
									<span class="text-xs bg-yellow-500/15 text-yellow-600 px-2 py-0.5 rounded">No private key</span>
								{/if}
							</div>
							{#if key.description}<p class="text-xs text-muted-foreground mt-0.5">{key.description}</p>{/if}
							<div class="mt-2 flex items-center gap-2 bg-muted/60 rounded-md px-3 py-1.5">
								<p class="font-mono text-xs text-muted-foreground truncate flex-1">{truncateKey(key.public_key)}</p>
								<button onclick={() => copyPublicKey(key)} class="inline-flex items-center gap-1 text-xs text-muted-foreground hover:text-foreground transition-colors shrink-0">
									{#if copied === key.id}<CheckCircle class="w-3.5 h-3.5 text-green-600" /> Copied{:else}<Copy class="w-3.5 h-3.5" /> Copy{/if}
								</button>
							</div>
							<div class="flex items-center gap-4 mt-2 text-[11px] text-muted-foreground/50">
								<span class="flex items-center gap-1"><Clock class="w-3 h-3" /> Added {formatDate(key.created_at)}</span>
								{#if key.last_used_at}<span>Last used {formatDate(key.last_used_at)}</span>{:else}<span>Never used</span>{/if}
							</div>
						</div>
						<div class="flex items-center gap-1 shrink-0">
							<button onclick={() => openEdit(key)} class="p-1.5 rounded text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"><Pencil class="w-4 h-4" /></button>
							<button onclick={() => deleteKey(key.id)} disabled={deletingId === key.id} class="p-1.5 rounded text-muted-foreground hover:text-destructive hover:bg-destructive/10 transition-colors">
								{#if deletingId === key.id}<div class="w-4 h-4 border-2 border-current/30 border-t-current rounded-full animate-spin"></div>{:else}<Trash2 class="w-4 h-4" />{/if}
							</button>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</main>
</PageLayout>

{#if showCreate || editKey}
	<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1" aria-label="Close" onclick={() => { showCreate = false; editKey = null; }} onkeydown={() => {}}></div>
	<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
		<form onsubmit={submitModal} class="bg-card border border-border rounded-xl shadow-2xl w-full max-w-lg mx-4 p-6 flex flex-col gap-4 max-h-[90vh] overflow-y-auto pointer-events-auto">
			<div class="flex items-center justify-between">
				<div>
					<h2 class="text-base font-semibold">{editKey ? 'Edit SSH Key' : 'Add SSH Key'}</h2>
					<p class="text-xs text-muted-foreground mt-0.5">{editKey ? 'Update key details' : 'Add a new SSH key for server authentication'}</p>
				</div>
				<button type="button" class="text-muted-foreground hover:text-foreground hover:bg-accent p-1 rounded" onclick={() => { showCreate = false; editKey = null; }}>✕</button>
			</div>
			<div class="flex flex-col gap-1.5">
				<label class="text-sm font-medium text-muted-foreground" for="page-1">Name <span class="text-destructive">*</span></label>

				<input id="page-1"  class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring" placeholder="my-deploy-key" bind:value={modalName} required />
			</div>
			<div class="flex flex-col gap-1.5">
				<label class="text-sm font-medium text-muted-foreground" for="page-2">Description</label>

				<input id="page-2"  class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring" placeholder="Optional description" bind:value={modalDesc} />
			</div>
			<div class="flex flex-col gap-1.5">
				<div class="flex items-center justify-between">
					<label class="text-sm font-medium text-muted-foreground" for="page-3">Private Key {#if !editKey}<span class="text-destructive">*</span>{/if}</label>
					<button type="button" class="inline-flex items-center gap-1 text-xs text-muted-foreground hover:text-foreground transition-colors" onclick={() => (modalShowPrivate = !modalShowPrivate)}>
						{#if modalShowPrivate}<EyeOff class="w-3.5 h-3.5" /> Hide{:else}<Eye class="w-3.5 h-3.5" /> Show{/if}
					</button>
				</div>
				<textarea class="flex w-full rounded-md border border-input bg-transparent px-3 py-2 text-xs font-mono placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring resize-none" rows={modalShowPrivate ? 6 : 3} placeholder={editKey ? 'Leave blank to keep existing key' : '-----BEGIN OPENSSH PRIVATE KEY-----\n…\n-----END OPENSSH PRIVATE KEY-----'} bind:value={modalPrivateKey} style="filter: {modalShowPrivate ? 'none' : 'blur(3px)'}; transition: filter 0.2s"></textarea>
			</div>
			<div class="flex flex-col gap-1.5">
				<label class="text-sm font-medium text-muted-foreground">Public Key {#if !editKey}<span class="text-destructive">*</span>{/if}</label>

				<textarea id="page-3"  class="flex w-full rounded-md border border-input bg-transparent px-3 py-2 text-xs font-mono placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring resize-none" rows={3} placeholder="ssh-ed25519 AAAA… user@host" bind:value={modalPublicKey}></textarea>
			</div>
			{#if modalError}<p class="text-xs text-destructive">{modalError}</p>{/if}
			<div class="flex justify-end gap-2 pt-1">
				<button type="button" class="px-3 py-1.5 rounded-md text-sm text-muted-foreground hover:bg-accent transition-colors" onclick={() => { showCreate = false; editKey = null; }}>Cancel</button>
				<button type="submit" class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50" disabled={modalSaving}>
					{#if modalSaving}<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>Saving…{:else}{editKey ? 'Save Changes' : 'Add Key'}{/if}
				</button>
			</div>
		</form>
	</div>
{/if}
