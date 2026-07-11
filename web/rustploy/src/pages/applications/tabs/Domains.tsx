import { createResource, createSignal, For, Show } from 'solid-js';
import { Plus, Globe, Trash2, Pencil, ExternalLink, RefreshCw, Lock, Unlock } from 'lucide-solid';
import type { ApplicationResponseDto, DomainResponseDto } from '../../../client/types.gen';
import { domainControllerListByApplication, domainControllerDelete } from '../../../client/sdk.gen';
import { formatDate } from '../../../lib/utils';
import AddDomainModal from './domains/AddDomainModal';
import EditDomainModal from './domains/EditDomainModal';

type Props = { app: ApplicationResponseDto };

function domainUrl(d: DomainResponseDto): string {
  const scheme = d.https ? 'https' : 'http';
  const portSuffix =
    d.port && ((d.https && d.port !== 443) || (!d.https && d.port !== 80))
      ? `:${d.port}` : '';
  const pathSuffix = d.path && d.path !== '/' ? d.path : '';
  return `${scheme}://${d.host}${portSuffix}${pathSuffix}`;
}

export default function DomainsTab(props: Props) {
  const [showAdd, setShowAdd] = createSignal(false);
  const [editing, setEditing] = createSignal<DomainResponseDto | null>(null);
  const [deletingId, setDeletingId] = createSignal<number | null>(null);

  const [domains, { mutate, refetch }] = createResource(
    () => props.app.id,
    async (appId) => {
      const res = await domainControllerListByApplication({ path: { application_id: appId } });
      return res.data ?? [];
    },
  );

  const handleCreated = (d: DomainResponseDto) => mutate(prev => [...(prev ?? []), d]);
  const handleUpdated = (d: DomainResponseDto) => mutate(prev => (prev ?? []).map(x => x.id === d.id ? d : x));

  const handleDelete = async (id: number) => {
    setDeletingId(id);
    try {
      await domainControllerDelete({ path: { id } });
      mutate(prev => (prev ?? []).filter(x => x.id !== id));
    } finally { setDeletingId(null); }
  };

  return (
    <div class="flex flex-col gap-6">

      {/* Header */}
      <section class="bg-base-200 border border-base-300 rounded-lg p-6">
        <div class="flex items-center justify-between">
          <div>
            <h2 class="text-base font-semibold">Domains</h2>
            <p class="text-sm text-base-content/40 mt-1">Configure custom domains and routing for this application.</p>
          </div>
          <div class="flex items-center gap-2">
            <button class="btn btn-ghost btn-sm gap-1.5" onClick={() => refetch()} disabled={domains.loading}>
              <RefreshCw class={`w-3.5 h-3.5 ${domains.loading ? 'animate-spin' : ''}`} /> Refresh
            </button>
            <button class="btn btn-neutral btn-sm gap-1.5" onClick={() => setShowAdd(true)}>
              <Plus class="w-4 h-4" /> Add Domain
            </button>
          </div>
        </div>
      </section>

      {/* List */}
      <section class="bg-base-200 border border-base-300 rounded-lg overflow-hidden">
        <Show when={domains.loading}>
          <div class="flex justify-center py-14">
            <span class="loading loading-spinner loading-md text-base-content/40" />
          </div>
        </Show>

        <Show when={!domains.loading && (domains() ?? []).length === 0}>
          <div class="flex flex-col items-center justify-center py-16 text-base-content/30">
            <Globe class="w-10 h-10 mb-3" />
            <p class="text-sm">No domains configured</p>
            <p class="text-xs mt-1">Add a domain above to expose this application.</p>
          </div>
        </Show>

        <Show when={!domains.loading && (domains() ?? []).length > 0}>
          <div class="grid grid-cols-[1fr_80px_80px_130px_140px_88px] gap-4 px-5 py-2.5 border-b border-base-300 text-xs text-base-content/40 font-medium uppercase tracking-wide">
            <span>Host</span><span>Port</span><span>Type</span><span>TLS</span><span>Added</span><span></span>
          </div>

          <For each={domains() ?? []}>
            {(domain) => (
              <div class="grid grid-cols-[1fr_80px_80px_130px_140px_88px] gap-4 items-center px-5 py-3 border-b border-base-300 last:border-0 hover:bg-base-300/40 transition-colors">

                <div class="min-w-0">
                  <div class="flex items-center gap-1.5">
                    <Globe class="w-3.5 h-3.5 text-base-content/40 shrink-0" />
                    <span class="text-sm font-medium truncate">{domain.host}</span>
                    <Show when={domain.path && domain.path !== '/'}>
                      <span class="text-xs text-base-content/40 font-mono">{domain.path}</span>
                    </Show>
                  </div>
                  <a href={domainUrl(domain)} target="_blank" rel="noopener noreferrer"
                    class="text-xs text-base-content/40 hover:text-base-content transition-colors flex items-center gap-1 mt-0.5 w-fit">
                    <ExternalLink class="w-3 h-3" />{domainUrl(domain)}
                  </a>
                </div>

                <span class="text-xs text-base-content/60 font-mono">{domain.port ?? '—'}</span>
                <span class="text-xs text-base-content/60 uppercase font-mono">{domain.domain_type}</span>

                <div>
                  <Show when={domain.https}>
                    <span class="inline-flex items-center gap-1 text-xs font-medium text-success">
                      <Lock class="w-3.5 h-3.5" />
                      {domain.certificate_type === 'letsencrypt' ? "Let's Encrypt"
                        : domain.certificate_type === 'custom' ? 'Custom' : 'HTTPS'}
                    </span>
                  </Show>
                  <Show when={!domain.https}>
                    <span class="inline-flex items-center gap-1 text-xs text-base-content/40">
                      <Unlock class="w-3.5 h-3.5" /> None
                    </span>
                  </Show>
                </div>

                <span class="text-xs text-base-content/40">{formatDate(domain.created_at)}</span>

                <div class="flex items-center justify-end gap-1">
                  <button class="btn btn-ghost btn-xs text-base-content/40 hover:text-base-content" title="Edit" onClick={() => setEditing(domain)}>
                    <Pencil class="w-3.5 h-3.5" />
                  </button>
                  <button class="btn btn-ghost btn-xs text-base-content/40 hover:text-error" title="Delete"
                    disabled={deletingId() === domain.id} onClick={() => handleDelete(domain.id)}>
                    <Show when={deletingId() === domain.id} fallback={<Trash2 class="w-3.5 h-3.5" />}>
                      <span class="loading loading-spinner loading-xs" />
                    </Show>
                  </button>
                </div>

              </div>
            )}
          </For>
        </Show>
      </section>

      <Show when={showAdd()}>
        <AddDomainModal appId={props.app.id} onClose={() => setShowAdd(false)} onCreated={handleCreated} />
      </Show>
      <Show when={editing() !== null}>
        <EditDomainModal domain={editing()!} onClose={() => setEditing(null)} onUpdated={handleUpdated} />
      </Show>

    </div>
  );
}
