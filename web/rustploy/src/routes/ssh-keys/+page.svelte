<script lang="ts">
	import { goto } from '$app/navigation';
	import { RocketIcon, Key, Plus, RefreshCw, FileKey, CheckCircle, Copy, Clock, Pencil, Trash2, Eye, EyeOff } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import { sshKeyControllerList, sshKeyControllerCreate, sshKeyControllerPatch, sshKeyControllerDelete } from '$lib/client/sdk.gen';
	import type { SshKeyResponseDto } from '$lib/client/types.gen';
	import { USE_MOCK_DATA, getSshKeysMock, type SshKeyMock } from '$lib/mocks';
	import { formatDate } from '$lib/helpers';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	let useMock = $state(USE_MOCK_DATA);
	let mockKeys = $state<SshKeyMock[]>(getSshKeysMock());
	let apiKeys = $state<SshKeyResponseDto[]>([]);
	let loading = $state(false);

	let showCreate = $state(false);
	let editKey = $state<any | null>(null);
	let deletingId = $state<string | number | null>(null);
	let copied = $state<string | number | null>(null);

	async function loadKeys() {
		if (useMock) return;
		loading = true;
		try {
			const res = await sshKeyControllerList();
			apiKeys = (res.data as SshKeyResponseDto[]) ?? [];
		} finally { loading = false; }
	}

	$effect(() => {
		if (!useMock) loadKeys();
	});

	const displayKeys = $derived(
		useMock
			? mockKeys.map((k) => ({
					id: k.id,
					name: k.name,
					fingerprint: k.fingerprint,
					publicKey: k.publicKey,
					keyType: k.keyType,
					createdAt: k.createdAt,
					hasPrivate: true
				}))
			: apiKeys.map((k) => ({
					id: String(k.id),
					name: k.name,
					fingerprint: 'SHA256:generated',
					publicKey: k.public_key,
					keyType: 'ssh-key',
					createdAt: String(k.created_at || 'Recently'),
					hasPrivate: k.has_private_key
				}))
	);

	function deleteKey(id: string) {
		deletingId = id;
		if (useMock) {
			mockKeys = mockKeys.filter((k) => k.id !== id);
			deletingId = null;
		}
	}

	async function copyPublicKey(id: string, publicKey: string) {
		await navigator.clipboard.writeText(publicKey);
		copied = id;
		setTimeout(() => (copied = null), 2000);
	}

	function truncateKey(key: string, len = 54) {
		return key.length > len ? key.slice(0, len) + '…' : key;
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
</script>

<PageLayout>
	<!-- Top Breadcrumb Header Bar -->
	<header class="flex items-center justify-between px-6 py-3 border-b border-[#262626] text-xs bg-[#0A0A0A] shrink-0">
		<div class="flex items-center gap-2">
			<Key class="w-3.5 h-3.5 text-[#a1a1aa]" />
			<span class="font-medium text-[#FAFAFA]">SSH Keys</span>
		</div>

		<!-- Mock Data Dev Toggle Switch -->
		<div class="flex items-center gap-2 px-3 py-1 rounded-full bg-[#141414] border border-[#262626]">
			<span class="text-[11px] text-[#a1a1aa]">Data Source:</span>
			<button
				onclick={() => (useMock = !useMock)}
				class="text-[11px] font-semibold px-2 py-0.5 rounded transition-colors {useMock
					? 'bg-[#262626] text-[#FAFAFA] border border-white/10'
					: 'text-[#737373] hover:text-[#FAFAFA]'}"
			>
				{useMock ? 'Mock Demo Data' : 'Live Rust Backend API'}
			</button>
		</div>
	</header>

	<main class="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up min-h-0">
		<!-- Constrained Max-Width Shell (Dokploy Standard max-w-5xl) -->
		<div class="max-w-5xl space-y-6">
			<div class="flex items-center justify-between">
				<div>
					<h1 class="text-3xl font-bold tracking-tight text-[#FAFAFA]">SSH Keys</h1>
					<p class="text-sm text-[#a1a1aa] mt-1">Manage SSH key pairs for authenticating remote server deployments</p>
				</div>
				<button onclick={openCreate} class="inline-flex items-center gap-2 px-3.5 py-2 rounded-lg bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] text-xs font-semibold transition-colors">
					<Plus class="w-4 h-4" /> Add SSH Key
				</button>
			</div>

			{#if loading}
				<div class="flex justify-center py-20"><div class="w-6 h-6 border-2 border-[#737373] border-t-[#FAFAFA] rounded-full animate-spin"></div></div>
			{:else if displayKeys.length === 0}
				<div class="flex flex-col items-center justify-center py-20 text-[#a1a1aa] text-center bg-[#171717] border border-[#262626] rounded-xl">
					<Key class="w-12 h-12 mb-3 text-[#525252]" />
					<p class="text-sm font-semibold text-[#FAFAFA]">No SSH keys configured</p>
					<p class="text-xs text-[#a1a1aa] mt-1">Add an ED25519 or RSA SSH key to authorize remote deployments.</p>
					<button onclick={openCreate} class="mt-4 px-3.5 py-2 rounded-lg border border-[#262626] bg-[#262626] text-xs text-[#FAFAFA] font-medium hover:bg-[#333333] transition-colors inline-flex items-center gap-1.5">
						<Plus class="w-4 h-4" /> Add your first key
					</button>
				</div>
			{:else}
				<div class="flex flex-col gap-3.5">
					{#each displayKeys as key (key.id)}
						<div class="bg-[#171717] border border-[#262626] rounded-xl p-5 flex items-start gap-4 hover:border-[#3f3f46] transition-all">
							<div class="w-10 h-10 rounded-lg bg-[#262626] border border-white/10 flex items-center justify-center shrink-0 mt-0.5">
								<FileKey class="w-5 h-5 text-[#FAFAFA]" />
							</div>
							<div class="flex-1 min-w-0">
								<div class="flex items-center gap-2 flex-wrap">
									<h2 class="text-base font-semibold text-[#FAFAFA]">{key.name}</h2>
									<span class="inline-flex items-center gap-1 text-[10px] font-mono bg-green-500/10 text-green-400 border border-green-500/30 px-2 py-0.5 rounded">
										<CheckCircle class="w-3 h-3" /> Private Key Active
									</span>
									<span class="text-[10px] font-mono bg-[#262626] text-[#a1a1aa] px-2 py-0.5 rounded uppercase">{key.keyType}</span>
								</div>

								<div class="mt-2.5 flex items-center gap-2 bg-[#141414] border border-[#262626] rounded-lg px-3 py-2">
									<p class="font-mono text-xs text-[#a1a1aa] truncate flex-1">{truncateKey(key.publicKey)}</p>
									<button onclick={() => copyPublicKey(key.id, key.publicKey)} class="inline-flex items-center gap-1 text-xs text-[#a1a1aa] hover:text-[#FAFAFA] transition-colors shrink-0">
										{#if copied === key.id}<CheckCircle class="w-3.5 h-3.5 text-green-400" /> Copied{:else}<Copy class="w-3.5 h-3.5" /> Copy{/if}
									</button>
								</div>

								<div class="flex items-center gap-4 mt-2.5 text-[11px] font-mono text-[#737373]">
									<span class="flex items-center gap-1"><Clock class="w-3 h-3 text-[#737373]" /> Created {key.createdAt}</span>
									<span>·</span>
									<span>Fingerprint: {key.fingerprint}</span>
								</div>
							</div>

							<div class="flex items-center gap-1 shrink-0">
								<button onclick={() => deleteKey(key.id)} disabled={deletingId === key.id} class="p-2 rounded-lg border border-[#262626] bg-[#262626] text-[#a1a1aa] hover:text-red-400 hover:bg-red-500/10 transition-colors">
									<Trash2 class="w-4 h-4" />
								</button>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	</main>
</PageLayout>
