import { createSignal, createResource, Show, For } from 'solid-js';
import { publicListBranches } from '../../client/sdk.gen';
import { RefreshCw } from 'lucide-solid';

type Props = {
  value: string;
  onChange: (v: string) => void;
  repoUrl?: string;  // initial URL hint
  owner?: string;
  repo?: string;
  provider?: string;
};

function buildInitialUrl(props: Props): string {
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
  const [repoInput, setRepoInput] = createSignal(buildInitialUrl(props));
  const [trigger, setTrigger] = createSignal(0);
  const [fetchUrl, setFetchUrl] = createSignal('');

  const [branches] = createResource(fetchUrl, async (url) => {
    if (!url) return [];
    console.log('[BranchSelect] fetching branches for:', url);
    const res = await publicListBranches({ query: { query: url } });
    console.log('[BranchSelect] response:', res.data, res.error);
    const raw = (res.data as string[]) ?? [];
    return raw
      .filter(b => b !== 'HEAD' && !b.startsWith('refs/tags/'))
      .map(b => b.replace('refs/heads/', ''));
  });

  const load = () => {
    const url = repoInput().trim();
    if (!url) return;
    setFetchUrl(url);
  };

  return (
    <div class="flex flex-col gap-2">
      {/* Repo URL input + fetch button */}
      <div class="flex gap-2">
        <input
          class="input input-bordered flex-1 text-sm font-mono"
          placeholder="https://github.com/owner/repo.git"
          value={repoInput()}
          onInput={e => setRepoInput(e.currentTarget.value)}
        />
        <button
          type="button"
          class="btn btn-ghost btn-sm"
          onClick={load}
          disabled={branches.loading || !repoInput()}
          title="Load branches"
        >
          <Show when={branches.loading} fallback={<RefreshCw class="w-4 h-4" />}>
            <span class="loading loading-spinner loading-xs" />
          </Show>
        </button>
      </div>

      {/* Branch dropdown */}
      <Show when={branches() && branches()!.length > 0}>
        <select
          class="select select-bordered w-full"
          value={props.value}
          onChange={e => props.onChange(e.currentTarget.value)}
        >
          <option value="">Select branch</option>
          <For each={branches()}>
            {(b) => <option value={b}>{b}</option>}
          </For>
        </select>
      </Show>

      <Show when={branches.error}>
        <p class="text-xs text-error">Failed to load branches. Check URL.</p>
      </Show>

      <Show when={!branches() && !branches.loading && fetchUrl()}>
        <p class="text-xs text-base-content/40">No branches found.</p>
      </Show>
    </div>
  );
}
