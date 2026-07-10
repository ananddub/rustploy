import { createResource, createSignal, For, Show } from 'solid-js';
import { useNavigate } from '@solidjs/router';
import {
  Plus, FileKey, Rocket, Copy, Trash2, Pencil, RefreshCw,
  CheckCircle, Eye, EyeOff, KeyRound, Clock,
} from 'lucide-solid';
import { authSession } from '../../lib/auth';
import {
  sshKeyControllerList,
  sshKeyControllerCreate,
  sshKeyControllerPatch,
  sshKeyControllerDelete,
} from '../../client/sdk.gen';
import type { SshKeyResponseDto, CreateSshKeyDto } from '../../client/types.gen';
import { Sidebar } from '../../components';

// ── helpers ────────────────────────────────────────────────────────────────

function formatDate(ts: number) {
  return new Date(ts * 1000).toLocaleString(undefined, {
    month: 'short', day: 'numeric',
    hour: '2-digit', minute: '2-digit',
  });
}

function truncateKey(key: string, len = 48) {
  return key.length > len ? key.slice(0, len) + '…' : key;
}

// ── Create / Edit modal ─────────────────────────────────────────────────────

function KeyModal(props: {
  initial?: SshKeyResponseDto;
  onClose: () => void;
  onSaved: (k: SshKeyResponseDto) => void;
}) {
  const editing = !!props.initial;
  const [name, setName] = createSignal(props.initial?.name ?? '');
  const [desc, setDesc] = createSignal(props.initial?.description ?? '');
  const [privateKey, setPrivateKey] = createSignal('');
  const [publicKey, setPublicKey] = createSignal(props.initial?.public_key ?? '');
  const [showPrivate, setShowPrivate] = createSignal(false);
  const [saving, setSaving] = createSignal(false);
  const [err, setErr] = createSignal('');

  const submit = async (e: SubmitEvent) => {
    e.preventDefault();
    if (!name().trim()) return;
    setSaving(true);
    setErr('');
    try {
      if (editing) {
        const res = await sshKeyControllerPatch({
          path: { id: props.initial!.id },
          body: {
            name: name().trim(),
            description: desc().trim() || undefined,
            ...(privateKey().trim() ? { private_key: privateKey().trim() } : {}),
            ...(publicKey().trim() ? { public_key: publicKey().trim() } : {}),
          },
        });
        if (res.data) props.onSaved(res.data);
      } else {
        if (!privateKey().trim() || !publicKey().trim()) {
          setErr('Both private and public key are required');
          return;
        }
        const res = await sshKeyControllerCreate({
          body: {
            name: name().trim(),
            private_key: privateKey().trim(),
            public_key: publicKey().trim(),
            description: desc().trim() || undefined,
          } as CreateSshKeyDto,
        });
        if (res.data) props.onSaved(res.data);
      }
    } catch (e: any) {
      setErr(e?.message ?? 'Failed to save SSH key');
    } finally {
      setSaving(false);
    }
  };

  return (
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm">
      <form
        onSubmit={submit}
        class="bg-base-100 border border-base-300 rounded-xl shadow-2xl w-full max-w-lg mx-4 p-6 flex flex-col gap-4 max-h-[90vh] overflow-y-auto"
      >
        <div class="flex items-center justify-between">
          <div>
            <h2 class="text-base font-semibold">{editing ? 'Edit SSH Key' : 'Add SSH Key'}</h2>
            <p class="text-xs text-base-content/40 mt-0.5">
              {editing ? 'Update key details' : 'Add a new SSH key for server authentication'}
            </p>
          </div>
          <button type="button" class="btn btn-ghost btn-xs" onClick={props.onClose}>✕</button>
        </div>

        <div class="grid grid-cols-2 gap-3">
          <fieldset class="fieldset col-span-2">
            <legend class="fieldset-legend text-base-content/70">Name <span class="text-error">*</span></legend>
            <input
              class="input input-bordered w-full"
              placeholder="my-deploy-key"
              value={name()}
              onInput={e => setName(e.currentTarget.value)}
              required
            />
          </fieldset>

          <fieldset class="fieldset col-span-2">
            <legend class="fieldset-legend text-base-content/70">Description</legend>
            <input
              class="input input-bordered w-full"
              placeholder="Optional description"
              value={desc()}
              onInput={e => setDesc(e.currentTarget.value)}
            />
          </fieldset>
        </div>

        {/* Private key */}
        <div>
          <div class="flex items-center justify-between mb-1.5">
            <label class="text-sm text-base-content/70">
              Private Key {!editing && <span class="text-error">*</span>}
            </label>
            <button
              type="button"
              class="btn btn-ghost btn-xs gap-1 text-base-content/40"
              onClick={() => setShowPrivate(p => !p)}
            >
              {showPrivate() ? <EyeOff class="w-3.5 h-3.5" /> : <Eye class="w-3.5 h-3.5" />}
              {showPrivate() ? 'Hide' : 'Show'}
            </button>
          </div>
          <textarea
            class="textarea textarea-bordered w-full font-mono text-xs resize-none"
            rows={showPrivate() ? 6 : 3}
            placeholder={editing ? 'Leave blank to keep existing key' : '-----BEGIN OPENSSH PRIVATE KEY-----\n…\n-----END OPENSSH PRIVATE KEY-----'}
            value={privateKey()}
            onInput={e => setPrivateKey(e.currentTarget.value)}
            style={{ filter: showPrivate() ? 'none' : 'blur(3px)', transition: 'filter 0.2s' }}
          />
        </div>

        {/* Public key */}
        <div>
          <label class="block text-sm text-base-content/70 mb-1.5">
            Public Key {!editing && <span class="text-error">*</span>}
          </label>
          <textarea
            class="textarea textarea-bordered w-full font-mono text-xs resize-none"
            rows={3}
            placeholder="ssh-ed25519 AAAA… user@host"
            value={publicKey()}
            onInput={e => setPublicKey(e.currentTarget.value)}
          />
        </div>

        <Show when={err()}>
          <p class="text-xs text-error">{err()}</p>
        </Show>

        <div class="flex justify-end gap-2 pt-1">
          <button type="button" class="btn btn-ghost btn-sm" onClick={props.onClose}>Cancel</button>
          <button type="submit" class="btn btn-neutral btn-sm gap-1.5" disabled={saving()}>
            {saving() && <span class="loading loading-spinner loading-xs" />}
            {saving() ? 'Saving…' : editing ? 'Save Changes' : 'Add Key'}
          </button>
        </div>
      </form>
    </div>
  );
}

// ── Main page ───────────────────────────────────────────────────────────────

export default function SshKeysPage() {
  const navigate = useNavigate();
  if (!authSession()) navigate('/auth', { replace: true });

  const [showCreate, setShowCreate] = createSignal(false);
  const [editKey, setEditKey] = createSignal<SshKeyResponseDto | null>(null);
  const [deletingId, setDeletingId] = createSignal<number | null>(null);
  const [copied, setCopied] = createSignal<number | null>(null);

  const [keys, { mutate, refetch }] = createResource(async () => {
    const res = await sshKeyControllerList();
    return (res.data as SshKeyResponseDto[]) ?? [];
  });

  const copyPublicKey = async (key: SshKeyResponseDto) => {
    await navigator.clipboard.writeText(key.public_key);
    setCopied(key.id);
    setTimeout(() => setCopied(null), 2000);
  };

  const deleteKey = async (id: number) => {
    setDeletingId(id);
    try {
      await sshKeyControllerDelete({ path: { id } });
      mutate(prev => prev?.filter(k => k.id !== id) ?? []);
    } finally {
      setDeletingId(null);
    }
  };

  const handleSaved = (k: SshKeyResponseDto) => {
    mutate(prev => {
      const exists = prev?.find(x => x.id === k.id);
      if (exists) return prev?.map(x => x.id === k.id ? k : x) ?? [];
      return [...(prev ?? []), k];
    });
    setShowCreate(false);
    setEditKey(null);
  };

  return (
    <>
      <div class="min-h-screen flex bg-base-100 text-base-content">
        <Sidebar />

        <div class="flex-1 flex flex-col min-w-0">
          {/* Header */}
          <header class="flex items-center gap-2 px-6 py-3 border-b border-base-300 text-sm">
            <Rocket class="w-4 h-4 text-base-content/40" />
            <button onClick={() => navigate('/dashboard')} class="text-base-content/50 hover:text-base-content transition-colors">
              Dashboard
            </button>
            <span class="text-base-content/20">/</span>
            <span class="font-medium flex items-center gap-1.5">
              <FileKey class="w-4 h-4" /> SSH Keys
            </span>
          </header>

          <main class="flex-1 px-8 py-8">
            {/* Title row */}
            <div class="flex items-center justify-between mb-6">
              <div>
                <h1 class="text-2xl font-semibold">SSH Keys</h1>
                <p class="text-sm text-base-content/40 mt-1">
                  Manage SSH keys used for server and repository authentication
                </p>
              </div>
              <div class="flex items-center gap-2">
                <button
                  type="button"
                  class="btn btn-ghost btn-sm gap-1.5"
                  onClick={() => refetch()}
                >
                  <RefreshCw class="w-3.5 h-3.5" /> Refresh
                </button>
                <button
                  type="button"
                  class="btn btn-neutral btn-sm gap-1.5"
                  onClick={() => setShowCreate(true)}
                >
                  <Plus class="w-4 h-4" /> Add SSH Key
                </button>
              </div>
            </div>

            {/* Loading */}
            <Show when={keys.loading}>
              <div class="flex justify-center py-20">
                <span class="loading loading-spinner loading-md text-base-content/40" />
              </div>
            </Show>

            {/* Empty */}
            <Show when={!keys.loading && (keys() ?? []).length === 0}>
              <div class="flex flex-col items-center justify-center py-24 text-base-content/30">
                <KeyRound class="w-14 h-14 mb-4" />
                <p class="text-sm font-medium text-base-content/40">No SSH keys yet</p>
                <p class="text-xs mt-1">Add a key to authenticate with your remote servers</p>
                <button
                  type="button"
                  class="btn btn-outline btn-sm mt-5 gap-1.5"
                  onClick={() => setShowCreate(true)}
                >
                  <Plus class="w-4 h-4" /> Add your first key
                </button>
              </div>
            </Show>

            {/* Keys list */}
            <Show when={!keys.loading && (keys() ?? []).length > 0}>
              <div class="flex flex-col gap-3">
                <For each={keys()}>
                  {(key) => (
                    <div class="bg-base-200 border border-base-300 rounded-xl p-5 flex items-start gap-4 hover:border-base-content/20 transition-colors">
                      {/* Key icon */}
                      <div class="w-10 h-10 rounded-lg bg-base-300 flex items-center justify-center shrink-0 mt-0.5">
                        <FileKey class="w-5 h-5 text-base-content/50" />
                      </div>

                      {/* Info */}
                      <div class="flex-1 min-w-0">
                        <div class="flex items-center gap-2 flex-wrap">
                          <p class="text-sm font-semibold">{key.name}</p>
                          <Show when={key.has_private_key}>
                            <span class="inline-flex items-center gap-1 text-xs bg-success/15 text-success px-2 py-0.5 rounded">
                              <CheckCircle class="w-3 h-3" /> Private key set
                            </span>
                          </Show>
                          <Show when={!key.has_private_key}>
                            <span class="text-xs bg-warning/15 text-warning px-2 py-0.5 rounded">
                              No private key
                            </span>
                          </Show>
                        </div>

                        <Show when={key.description}>
                          <p class="text-xs text-base-content/40 mt-0.5">{key.description}</p>
                        </Show>

                        {/* Public key preview */}
                        <div class="mt-2 flex items-center gap-2 bg-base-300/60 rounded-md px-3 py-1.5">
                          <p class="font-mono text-xs text-base-content/50 truncate flex-1">
                            {truncateKey(key.public_key)}
                          </p>
                          <button
                            type="button"
                            class="btn btn-ghost btn-xs gap-1 shrink-0"
                            onClick={() => copyPublicKey(key)}
                            title="Copy public key"
                          >
                            {copied() === key.id
                              ? <><CheckCircle class="w-3.5 h-3.5 text-success" /> Copied</>
                              : <><Copy class="w-3.5 h-3.5" /> Copy</>}
                          </button>
                        </div>

                        {/* Timestamps */}
                        <div class="flex items-center gap-4 mt-2 text-[11px] text-base-content/30">
                          <span class="flex items-center gap-1">
                            <Clock class="w-3 h-3" /> Added {formatDate(key.created_at)}
                          </span>
                          <Show when={key.last_used_at}>
                            <span class="flex items-center gap-1">
                              Last used {formatDate(key.last_used_at!)}
                            </span>
                          </Show>
                          <Show when={!key.last_used_at}>
                            <span>Never used</span>
                          </Show>
                        </div>
                      </div>

                      {/* Actions */}
                      <div class="flex items-center gap-1 shrink-0">
                        <button
                          type="button"
                          class="btn btn-ghost btn-sm text-base-content/40 hover:text-base-content"
                          onClick={() => setEditKey(key)}
                          title="Edit"
                        >
                          <Pencil class="w-4 h-4" />
                        </button>
                        <button
                          type="button"
                          class="btn btn-ghost btn-sm text-base-content/30 hover:text-error"
                          onClick={() => deleteKey(key.id)}
                          disabled={deletingId() === key.id}
                          title="Delete"
                        >
                          {deletingId() === key.id
                            ? <span class="loading loading-spinner loading-xs" />
                            : <Trash2 class="w-4 h-4" />}
                        </button>
                      </div>
                    </div>
                  )}
                </For>
              </div>
            </Show>
          </main>
        </div>
      </div>

      {/* Create modal */}
      <Show when={showCreate()}>
        <KeyModal onClose={() => setShowCreate(false)} onSaved={handleSaved} />
      </Show>

      {/* Edit modal */}
      <Show when={editKey()}>
        <KeyModal
          initial={editKey()!}
          onClose={() => setEditKey(null)}
          onSaved={handleSaved}
        />
      </Show>
    </>
  );
}
