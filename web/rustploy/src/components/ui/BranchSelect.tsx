import { createSignal, createResource, createEffect, For, Show } from 'solid-js';
import { publicListBranches } from '../../client/sdk.gen';
import { RefreshCw } from 'lucide-solid';

type Props = {
  value: string;
  onChange: (v: string) => void;
  // For GIT provider — caller owns the URL input and passes it here reactively
  repoUrl?: string;
  // For hosted providers — we build the URL from owner + repo + provider
  owner?: string;
  repo?: string;
  provider?: string;
};

function buildUrl(props: Props): string {
  if (props.repoUrl) return props.repoUrl;
  if (props.owner && props.repo) {
    const p = (props.provider ?? 'GITHUB').toUpperCase();
    if (p === 'GITHUB')    return `https://github.com/${props.owner}/${props.repo}.git`;
    if (p === 'GITLAB')    return `https://gitlab.com/${props.owner}/${props.repo}.git`;
    if (p === 'GITEA')     return `https://gitea.com/${props.owner}/${props.repo}.git`;
    if (p === 'BITBUCKET') return `https://bitbucket.org/${props.owner}/${props.repo}.git`;
  }
  return '';
}

export default function BranchSelect(props: Props) {
  // fetchKey drives the resource — bump it to force a re-fetch
  const [fetchKey, setFetchKey] = createSignal<{ url: string; tick: number } | null>(null);

  // Debounce auto-fetch when the derived URL changes (e.g. user types in GIT URL field)
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;

  createEffect(() => {
    const url = buildUrl(props).trim();
    if (!url) return;

    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      setFetchKey(prev => ({ url, tick: (prev?.tick ?? 0) + 1 }));
    }, 600);
  });

  const [branches, { refetch }] = createResource(fetchKey, async (key) => {
    if (!key?.url) return [];
    const res = await publicListBranches({ query: { query: { query: key.url } } });
    const raw = (res.data as string[] | undefined) ?? [];
    return raw
      .filter(b => b !== 'HEAD' && !b.startsWith('refs/tags/'))
      .map(b => b.replace('refs/heads/', ''));
  });

  const manualRefresh = (e: MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    const url = buildUrl(props).trim();
    if (!url) return;
    // bump tick so resource re-runs even if url is the same
    setFetchKey(prev => ({ url, tick: (prev?.tick ?? 0) + 1 }));
  };

  return (
    <div class="flex gap-2 items-center">
      <div class="relative flex-1">
        <select
          class="select select-bordered w-full"
          value={props.value}
          onChange={e => props.onChange(e.currentTarget.value)}
          disabled={branches.loading || !branches()?.length}
        >
          <Show when={branches.loading}>
            <option value="">Loading branches…</option>
          </Show>
          <Show when={!branches.loading && (!branches() || branches()!.length === 0)}>
            <option value="">
              {buildUrl(props) ? 'No branches found' : 'Enter repo URL first'}
            </option>
          </Show>
          <Show when={!branches.loading && (branches()?.length ?? 0) > 0}>
            <option value="">Select branch</option>
            <For each={branches()}>
              {b => <option value={b}>{b}</option>}
            </For>
          </Show>
        </select>

        {/* Loading spinner overlay inside the select row */}
        <Show when={branches.loading}>
          <span class="absolute right-8 top-1/2 -translate-y-1/2 loading loading-spinner loading-xs text-base-content/40 pointer-events-none" />
        </Show>
      </div>

      {/* Manual refresh button — type="button" prevents form submit / page reload */}
      <button
        type="button"
        class="btn btn-ghost btn-sm btn-square"
        onClick={manualRefresh}
        disabled={branches.loading || !buildUrl(props)}
        title="Refresh branches"
      >
        <RefreshCw class={`w-4 h-4 ${branches.loading ? 'animate-spin' : ''}`} />
      </button>
    </div>
  );
}
