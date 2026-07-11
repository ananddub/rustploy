import { createResource, createSignal, For, Show } from 'solid-js';
import { Plus, Globe, Trash, PencilSimple, ArrowSquareOut, ArrowClockwise, Lock, LockOpen } from 'phosphor-solid';
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
    <div class="flex flex-col gap-6 animate-fade-up">

      {/* Header */}
      <section class="bg-base-200 border border-base-300 rounded-lg p-6">
        <div class="flex items-center justify-between">
          <div>
            <h2 class="text-base font-semibold">Domains</h2>
            <p class="text-sm text-base-content/40 mt-1">Configure custom domains and routing for this application.</p>
          </div>
          <div class="flex items-center gap-2">
            <button
              class="btn btn-ghost btn-sm gap-1.5 text-base-content/50 hover:text-base-content hover:bg-base-300"
              onClick={() => refetch()}
              disabled={domains.loading}
            >
              <ArrowClockwise size={14} class={domains.loading ? 'animate-spin' : ''} />
              Refresh
            </button>
            <button
              class="btn btn-neutral btn-sm gap-1.5"
              onClick={() => setShowAdd(true)}
            >
              <Plus size={14} weight="bold" /> Add Domain
            </button>
          </div>
        </div>
      </section>

      {/* List */}
      <section class="bg-base-200 border border-base-300 rounded-lg overflow-hidden">
        <Show when={domains.loading}>
          <div class="flex justify-center py-14">
            <span class="loading loading-spinner loading-md text-base-content/30" />
          </div>
        </Show>

        <Show when={!domains.loading && (domains() ?? []).length === 0}>
          <div class="flex flex-col items-center justify-center py-16 text-base-content/25">
            <Globe size={40} class="mb-3 opacity-40" />
            <p class="text-sm">No domains configured</p>
            <p class="text-xs mt-1 opacity-70">Add a domain above to expose this application.</p>
          </div>
        </Show>

        <Show when={!domains.loading && (domains() ?? []).length > 0}>
          <div class="grid grid-cols-[1fr_80px_80px_130px_140px_72px] gap-4 px-5 py-2.5 border-b border-base-300 text-xs text-base-content/35 font-medium uppercase tracking-wide">
            <span>Host</span><span>Port</span><span>Type</span><span>TLS</span><span>Added</span><span></span>
          </div>

          <For each={domains() ?? []}>
            {(domain) => (
              <div class="grid grid-cols-[1fr_80px_80px_130px_140px_72px] gap-4 items-center px-5 py-3 border-b border-base-300 last:border-0 hover:bg-base-300/30 transition-colors duration-100">

                <div class="min-w-0">
                  <div class="flex items-center gap-1.5">
                    <Globe size={13} class="text-base-content/40 shrink-0" />
                    <span class="text-sm font-medium truncate">{domain.host}</span>
                    <Show when={domain.path && domain.path !== '/'}>
                      <span class="text-xs text-base-content/35 font-mono">{domain.path}</span>
                    </Show>
                  </div>
                  <a
                    href={domainUrl(domain)}
                    target="_blank"
                    rel="noopener noreferrer"
                    class="text-xs text-base-content/35 hover:text-primary transition-colors flex items-center gap-1 mt-0.5 w-fit"
                  >
                    <ArrowSquareOut size={11} />{domainUrl(domain)}
                  </a>
                </div>

                <span class="text-xs text-base-content/50 font-mono">{domain.port ?? '—'}</span>
                <span class="text-xs text-base-content/50 uppercase font-mono">{domain.domain_type}</span>

                <div>
                  <Show when={domain.https}>
                    <span class="inline-flex items-center gap-1 text-xs font-medium text-success">
                      <Lock size={13} weight="fill" />
                      {domain.certificate_type === 'letsencrypt' ? "Let's Encrypt"
                        : domain.certificate_type === 'custom' ? 'Custom' : 'HTTPS'}
                    </span>
                  </Show>
                  <Show when={!domain.https}>
                    <span class="inline-flex items-center gap-1 text-xs text-base-content/35">
                      <LockOpen size={13} /> None
                    </span>
                  </Show>
                </div>

                <span class="text-xs text-base-content/35">{formatDate(domain.created_at)}</span>

                <div class="flex items-center justify-end gap-0.5">
                  <button
                    class="p-1.5 rounded-md text-base-content/35 hover:text-base-content hover:bg-base-300 transition-all"
                    title="Edit"
                    onClick={() => setEditing(domain)}
                  >
                    <PencilSimple size={13} />
                  </button>
                  <button
                    class="p-1.5 rounded-md text-base-content/35 hover:text-error hover:bg-error/10 transition-all"
                    title="Delete"
                    disabled={deletingId() === domain.id}
                    onClick={() => handleDelete(domain.id)}
                  >
                    <Show when={deletingId() === domain.id} fallback={<Trash size={13} />}>
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
