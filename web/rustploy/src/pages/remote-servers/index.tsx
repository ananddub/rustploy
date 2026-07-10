import { createResource, createSignal, For, Show } from 'solid-js';
import { useNavigate } from '@solidjs/router';
import {
  Plus, Server, Trash2, Rocket, Globe, CheckCircle,
  XCircle, RefreshCw, Power, PowerOff, TestTube2, FileKey,
} from 'lucide-solid';
import { authSession } from '../../lib/auth';
import {
  remoteServerControllerList,
  remoteServerControllerCreate,
  remoteServerControllerDelete,
  remoteServerControllerActivate,
  remoteServerControllerDeactivate,
  remoteServerControllerTestConnection,
  sshKeyControllerList,
} from '../../client/sdk.gen';
import type {
  RemoteServerResponseDto,
  CreateRemoteServerDto,
  SshKeyResponseDto,
} from '../../client/types.gen';
import { Sidebar } from '../../components';

// ── helpers ────────────────────────────────────────────────────────────────

function statusDot(status: string) {
  switch (status) {
    case 'ACTIVE':
      return <span class="w-2 h-2 rounded-full bg-success shrink-0" title="Active" />;
    case 'INACTIVE':
      return <span class="w-2 h-2 rounded-full bg-base-content/25 shrink-0" title="Inactive" />;
    case 'CONNECTING':
      return <span class="w-2 h-2 rounded-full bg-warning shrink-0 animate-pulse" title="Connecting" />;
    default:
      return <span class="w-2 h-2 rounded-full bg-error shrink-0" title="Error" />;
  }
}

function statusBadge(status: string) {
  const map: Record<string, string> = {
    ACTIVE: 'bg-success/15 text-success',
    INACTIVE: 'bg-base-content/10 text-base-content/50',
    CONNECTING: 'bg-warning/15 text-warning',
  };
  return (
    <span class={`inline-flex items-center px-2 py-0.5 rounded text-xs font-medium ${map[status] ?? 'bg-error/15 text-error'}`}>
      {status.charAt(0) + status.slice(1).toLowerCase()}
    </span>
  );
}

const SERVER_TYPES = ['REMOTE', 'SWARM'];

// ── Create modal ────────────────────────────────────────────────────────────

function CreateModal(props: {
  onClose: () => void;
  onCreated: (s: RemoteServerResponseDto) => void;
}) {
  const [name, setName] = createSignal('');
  const [ip, setIp] = createSignal('');
  const [port, setPort] = createSignal('22');
  const [user, setUser] = createSignal('root');
  const [serverType, setServerType] = createSignal('REMOTE');
  const [sshKeyId, setSshKeyId] = createSignal<number | null>(null);
  const [desc, setDesc] = createSignal('');
  const [saving, setSaving] = createSignal(false);
  const [err, setErr] = createSignal('');

  // Fetch available SSH keys — API: GET /ssh-keys
  const [sshKeys] = createResource<SshKeyResponseDto[]>(async () => {
    const res = await sshKeyControllerList();
    return (res.data as SshKeyResponseDto[]) ?? [];
  });

  const submit = async (e: SubmitEvent) => {
    e.preventDefault();
    if (!name().trim() || !ip().trim()) return;
    setSaving(true);
    setErr('');
    try {
      const body: CreateRemoteServerDto = {
        name: name().trim(),
        ip_address: ip().trim(),
        port: parseInt(port()) || 22,
        username: user().trim() || 'root',
        server_type: serverType(),
        description: desc().trim() || undefined,
        ssh_key_id: sshKeyId() ?? undefined,
      };
      const res = await remoteServerControllerCreate({ body });
      if (res.data) props.onCreated(res.data);
    } catch (e: any) {
      setErr(e?.message ?? 'Failed to create server');
    } finally {
      setSaving(false);
    }
  };

  return (
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm">
      <form
        onSubmit={submit}
        class="bg-base-100 border border-base-300 rounded-xl shadow-2xl w-full max-w-md mx-4 p-6 flex flex-col gap-4"
      >
        <div class="flex items-center justify-between">
          <div>
            <h2 class="text-base font-semibold">Add Remote Server</h2>
            <p class="text-xs text-base-content/40 mt-0.5">Connect a new server via SSH</p>
          </div>
          <button type="button" class="btn btn-ghost btn-xs" onClick={props.onClose}>✕</button>
        </div>

        <fieldset class="fieldset">
          <legend class="fieldset-legend text-base-content/70">Name <span class="text-error">*</span></legend>
          <input class="input input-bordered w-full" placeholder="production-1" value={name()} onInput={e => setName(e.currentTarget.value)} required />
        </fieldset>

        <div class="grid grid-cols-2 gap-3">
          <fieldset class="fieldset">
            <legend class="fieldset-legend text-base-content/70">IP Address <span class="text-error">*</span></legend>
            <input class="input input-bordered w-full font-mono" placeholder="192.168.1.100" value={ip()} onInput={e => setIp(e.currentTarget.value)} required />
          </fieldset>
          <fieldset class="fieldset">
            <legend class="fieldset-legend text-base-content/70">Port</legend>
            <input class="input input-bordered w-full font-mono" type="number" placeholder="22" value={port()} onInput={e => setPort(e.currentTarget.value)} />
          </fieldset>
        </div>

        <div class="grid grid-cols-2 gap-3">
          <fieldset class="fieldset">
            <legend class="fieldset-legend text-base-content/70">Username</legend>
            <input class="input input-bordered w-full font-mono" placeholder="root" value={user()} onInput={e => setUser(e.currentTarget.value)} />
          </fieldset>
          <fieldset class="fieldset">
            <legend class="fieldset-legend text-base-content/70">Server Type</legend>
            <select class="select select-bordered w-full" value={serverType()} onChange={e => setServerType(e.currentTarget.value)}>
              {SERVER_TYPES.map(t => <option value={t}>{t}</option>)}
            </select>
          </fieldset>
        </div>

        {/* SSH Key select — loaded from API */}
        <fieldset class="fieldset">
          <legend class="fieldset-legend text-base-content/70 flex items-center gap-1.5">
            <FileKey class="w-3.5 h-3.5" /> SSH Key
          </legend>
          <Show when={sshKeys.loading}>
            <div class="select select-bordered w-full flex items-center gap-2 text-base-content/40">
              <span class="loading loading-spinner loading-xs" />
              <span class="text-sm">Loading keys…</span>
            </div>
          </Show>
          <Show when={!sshKeys.loading}>
            <select
              class="select select-bordered w-full"
              value={sshKeyId() ?? ''}
              onChange={e => setSshKeyId(e.currentTarget.value ? Number(e.currentTarget.value) : null)}
            >
              <option value="">None (password auth)</option>
              <For each={sshKeys() ?? []}>
                {key => (
                  <option value={key.id}>
                    {key.name}{key.description ? ` — ${key.description}` : ''}
                  </option>
                )}
              </For>
            </select>
            <Show when={(sshKeys() ?? []).length === 0}>
              <p class="text-xs text-base-content/40 mt-1">
                No SSH keys found.{' '}
                <a href="/ssh-keys" class="underline hover:text-base-content">Add one first</a>.
              </p>
            </Show>
          </Show>
        </fieldset>

        <fieldset class="fieldset">
          <legend class="fieldset-legend text-base-content/70">Description</legend>
          <input class="input input-bordered w-full" placeholder="Optional description" value={desc()} onInput={e => setDesc(e.currentTarget.value)} />
        </fieldset>

        <Show when={err()}>
          <p class="text-xs text-error">{err()}</p>
        </Show>

        <div class="flex justify-end gap-2 pt-1">
          <button type="button" class="btn btn-ghost btn-sm" onClick={props.onClose}>Cancel</button>
          <button type="submit" class="btn btn-neutral btn-sm gap-1.5" disabled={saving()}>
            {saving() && <span class="loading loading-spinner loading-xs" />}
            {saving() ? 'Adding…' : 'Add Server'}
          </button>
        </div>
      </form>
    </div>
  );
}

// ── Main page ───────────────────────────────────────────────────────────────

export default function RemoteServersPage() {
  const navigate = useNavigate();
  if (!authSession()) navigate('/auth', { replace: true });

  const [showCreate, setShowCreate] = createSignal(false);
  const [testingId, setTestingId] = createSignal<number | null>(null);
  const [togglingId, setTogglingId] = createSignal<number | null>(null);
  const [deletingId, setDeletingId] = createSignal<number | null>(null);
  const [testResults, setTestResults] = createSignal<Record<number, 'ok' | 'fail'>>({});

  const [servers, { mutate, refetch }] = createResource(async () => {
    const res = await remoteServerControllerList();
    return (res.data as RemoteServerResponseDto[]) ?? [];
  });

  // Fetch SSH keys so we can show the key name on each card
  const [sshKeys] = createResource<SshKeyResponseDto[]>(async () => {
    const res = await sshKeyControllerList();
    return (res.data as SshKeyResponseDto[]) ?? [];
  });

  const keyName = (id?: number) => {
    if (!id) return null;
    return sshKeys()?.find(k => k.id === id)?.name ?? `Key #${id}`;
  };

  const testConnection = async (id: number) => {
    setTestingId(id);
    try {
      const res = await remoteServerControllerTestConnection({ path: { id } });
      setTestResults(prev => ({ ...prev, [id]: res.data ? 'ok' : 'fail' }));
    } catch {
      setTestResults(prev => ({ ...prev, [id]: 'fail' }));
    } finally {
      setTestingId(null);
    }
  };

  const toggleActive = async (server: RemoteServerResponseDto) => {
    setTogglingId(server.id);
    try {
      const res = server.server_status === 'ACTIVE'
        ? await remoteServerControllerDeactivate({ path: { id: server.id } })
        : await remoteServerControllerActivate({ path: { id: server.id } });
      if (res.data?.server) {
        mutate(prev => prev?.map(s => s.id === server.id ? res.data!.server : s) ?? []);
      }
    } finally {
      setTogglingId(null);
    }
  };

  const deleteServer = async (id: number) => {
    setDeletingId(id);
    try {
      await remoteServerControllerDelete({ path: { id } });
      mutate(prev => prev?.filter(s => s.id !== id) ?? []);
    } finally {
      setDeletingId(null);
    }
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
              <Server class="w-4 h-4" /> Remote Servers
            </span>
          </header>

          <main class="flex-1 px-8 py-8">
            {/* Title row */}
            <div class="flex items-center justify-between mb-6">
              <div>
                <h1 class="text-2xl font-semibold">Remote Servers</h1>
                <p class="text-sm text-base-content/40 mt-1">
                  Manage SSH connections to your deployment servers
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
                  <Plus class="w-4 h-4" /> Add Server
                </button>
              </div>
            </div>

            {/* Loading */}
            <Show when={servers.loading}>
              <div class="flex justify-center py-20">
                <span class="loading loading-spinner loading-md text-base-content/40" />
              </div>
            </Show>

            {/* Empty */}
            <Show when={!servers.loading && (servers() ?? []).length === 0}>
              <div class="flex flex-col items-center justify-center py-24 text-base-content/30">
                <Server class="w-14 h-14 mb-4" />
                <p class="text-sm font-medium text-base-content/40">No remote servers yet</p>
                <p class="text-xs mt-1">Add a server to start deploying your applications</p>
                <button
                  type="button"
                  class="btn btn-outline btn-sm mt-5 gap-1.5"
                  onClick={() => setShowCreate(true)}
                >
                  <Plus class="w-4 h-4" /> Add your first server
                </button>
              </div>
            </Show>

            {/* Server list */}
            <Show when={!servers.loading && (servers() ?? []).length > 0}>
              <div class="flex flex-col gap-3">
                <For each={servers()}>
                  {(server) => (
                    <div class="bg-base-200 border border-base-300 rounded-xl p-5 flex items-center gap-4 hover:border-base-content/20 transition-colors">
                      {/* Status dot + icon */}
                      <div class="relative shrink-0">
                        <div class="w-10 h-10 rounded-lg bg-base-300 flex items-center justify-center">
                          <Server class="w-5 h-5 text-base-content/50" />
                        </div>
                        <div class="absolute -bottom-0.5 -right-0.5">
                          {statusDot(server.server_status)}
                        </div>
                      </div>

                      {/* Info */}
                      <div class="flex-1 min-w-0">
                        <div class="flex items-center gap-2">
                          <p class="text-sm font-semibold truncate">{server.name}</p>
                          {statusBadge(server.server_status)}
                          <span class="text-xs bg-base-300 text-base-content/50 px-1.5 py-0.5 rounded font-mono">
                            {server.server_type}
                          </span>
                        </div>
                        <div class="flex items-center gap-3 mt-1 text-xs text-base-content/40">
                          <span class="font-mono flex items-center gap-1">
                            <Globe class="w-3 h-3" />
                            {server.ip_address}:{server.port}
                          </span>
                          <span class="flex items-center gap-1">
                            <span class="font-mono">{server.username}</span>
                          </span>
                          <Show when={keyName(server.ssh_key_id)}>
                            <span class="flex items-center gap-1">
                              <FileKey class="w-3 h-3" />
                              {keyName(server.ssh_key_id)}
                            </span>
                          </Show>
                          <Show when={server.description}>
                            <span class="truncate max-w-xs">{server.description}</span>
                          </Show>
                        </div>
                        {/* Test result feedback */}
                        <Show when={testResults()[server.id]}>
                          <p class={`text-xs mt-1 flex items-center gap-1 ${testResults()[server.id] === 'ok' ? 'text-success' : 'text-error'}`}>
                            {testResults()[server.id] === 'ok'
                              ? <><CheckCircle class="w-3 h-3" /> Connection successful</>
                              : <><XCircle class="w-3 h-3" /> Connection failed</>}
                          </p>
                        </Show>
                      </div>

                      {/* Actions */}
                      <div class="flex items-center gap-1 shrink-0">
                        {/* Test connection */}
                        <button
                          type="button"
                          class="btn btn-ghost btn-sm gap-1.5 text-base-content/50 hover:text-base-content"
                          onClick={() => testConnection(server.id)}
                          disabled={testingId() === server.id}
                          title="Test connection"
                        >
                          {testingId() === server.id
                            ? <span class="loading loading-spinner loading-xs" />
                            : <TestTube2 class="w-4 h-4" />}
                          <span class="hidden sm:inline">Test</span>
                        </button>

                        {/* Activate / Deactivate */}
                        <button
                          type="button"
                          class={`btn btn-ghost btn-sm gap-1.5 ${server.server_status === 'ACTIVE' ? 'text-warning hover:text-warning' : 'text-success hover:text-success'}`}
                          onClick={() => toggleActive(server)}
                          disabled={togglingId() === server.id}
                          title={server.server_status === 'ACTIVE' ? 'Deactivate' : 'Activate'}
                        >
                          {togglingId() === server.id
                            ? <span class="loading loading-spinner loading-xs" />
                            : server.server_status === 'ACTIVE'
                              ? <PowerOff class="w-4 h-4" />
                              : <Power class="w-4 h-4" />}
                          <span class="hidden sm:inline">
                            {server.server_status === 'ACTIVE' ? 'Deactivate' : 'Activate'}
                          </span>
                        </button>

                        {/* Delete */}
                        <button
                          type="button"
                          class="btn btn-ghost btn-sm text-base-content/30 hover:text-error"
                          onClick={() => deleteServer(server.id)}
                          disabled={deletingId() === server.id}
                          title="Delete server"
                        >
                          {deletingId() === server.id
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

      <Show when={showCreate()}>
        <CreateModal
          onClose={() => setShowCreate(false)}
          onCreated={(s) => {
            mutate(prev => [...(prev ?? []), s]);
            setShowCreate(false);
          }}
        />
      </Show>
    </>
  );
}
