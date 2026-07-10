import { createSignal, Show } from 'solid-js';
import { Info, Settings, RefreshCw } from 'lucide-solid';
import {
  applicationControllerPatch,
  applicationControllerDeploy,
  applicationControllerReload,
  applicationControllerRebuild,
  applicationControllerStart,
} from '../../../client/sdk.gen';
import type { ApplicationResponseDto } from '../../../client/types.gen';
import { DeployActions, BranchSelect } from '../../../components';

type Props = {
  app: ApplicationResponseDto;
  onUpdated: (app: ApplicationResponseDto) => void;
};

const PROVIDERS = [
  { id: 'GITHUB', label: 'Github' },
  { id: 'GITLAB', label: 'Gitlab' },
  { id: 'BITBUCKET', label: 'Bitbucket' },
  { id: 'GITEA', label: 'Gitea' },
  { id: 'DOCKER', label: 'Docker' },
  { id: 'GIT', label: 'Git' },
  { id: 'DROP', label: 'Drop' },
];

const BUILD_TYPES = [
  { id: 'DOCKERFILE', label: 'Dockerfile' },
  { id: 'RAILPACK', label: 'Railpack', isNew: true },
  { id: 'NIXPACKS', label: 'Nixpacks' },
  { id: 'HEROKU_BUILDPACKS', label: 'Heroku Buildpacks' },
  { id: 'PAKETO_BUILDPACKS', label: 'Paketo Buildpacks' },
  { id: 'STATIC', label: 'Static' },
];

export default function GeneralTab(props: Props) {
  const [provider, setProvider] = createSignal(props.app.source_type ?? 'GITHUB');
  const [buildType, setBuildType] = createSignal(props.app.build_type ?? 'NIXPACKS');
  const [triggerType, setTriggerType] = createSignal(props.app.trigger_type ?? 'PUSH');
  const [autoDeploy, setAutoDeploy] = createSignal(true);
  const [cleanCache, setCleanCache] = createSignal(false);
  const [branch, setBranch] = createSignal(props.app.branch ?? '');
  const [buildPath, setBuildPath] = createSignal('/');
  const [publishDir, setPublishDir] = createSignal('');
  const [saving, setSaving] = createSignal(false);

  const saveProvider = async () => {
    setSaving(true);
    try {
      const res = await applicationControllerPatch({
        path: { id: props.app.id },
        body: {},
      });
      if (res.data) props.onUpdated(res.data);
    } finally {
      setSaving(false);
    }
  };

  const saveBuildType = async () => {
    setSaving(true);
    try {
      const res = await applicationControllerPatch({
        path: { id: props.app.id },
        body: { build_type: buildType() },
      });
      if (res.data) props.onUpdated(res.data);
    } finally {
      setSaving(false);
    }
  };

  return (
    <div class="flex flex-col gap-6">
      {/* Deploy Settings */}
      <section class="bg-base-200 border border-base-300 rounded-lg p-6">
        <h2 class="text-base font-semibold mb-4">Deploy Settings</h2>
        <DeployActions
          autoDeploy={autoDeploy()}
          onAutoDeploy={setAutoDeploy}
          cleanCache={cleanCache()}
          onCleanCache={setCleanCache}
          onDeploy={async () => { await applicationControllerDeploy({ path: { id: props.app.id } }); }}
          onReload={async () => { await applicationControllerReload({ path: { id: props.app.id } }); }}
          onRebuild={async () => { await applicationControllerRebuild({ path: { id: props.app.id } }); }}
          onStart={async () => { await applicationControllerStart({ path: { id: props.app.id } }); }}
        />
      </section>

      {/* Provider */}
      <section class="bg-base-200 border border-base-300 rounded-lg p-6">
        <div class="flex items-center justify-between mb-1">
          <h2 class="text-base font-semibold">Provider</h2>
          <RefreshCw class="w-4 h-4 text-base-content/40" />
        </div>
        <p class="text-sm text-base-content/40 mb-4">Select the source of your code</p>

        {/* Provider tabs */}
        <div class="flex flex-wrap gap-1 border-b border-base-300 mb-5">
          {PROVIDERS.map(p => (
            <button
              class={`px-4 py-2 text-sm transition-colors border-b-2 -mb-px ${
                provider() === p.id
                  ? 'border-base-content text-base-content font-medium'
                  : 'border-transparent text-base-content/50 hover:text-base-content'
              }`}
              onClick={() => setProvider(p.id)}
            >
              {p.label}
            </button>
          ))}
        </div>

        {/* Provider fields */}

        {/* GITHUB / GITLAB / GITEA / BITBUCKET — account + repo + branch */}
        <Show when={['GITHUB', 'GITLAB', 'GITEA', 'BITBUCKET'].includes(provider())}>
          <div class="flex flex-col gap-4">
            <div>
              <label class="block text-sm mb-1.5">
                {provider() === 'GITHUB' ? 'Github Account'
                  : provider() === 'GITLAB' ? 'Gitlab Account'
                  : provider() === 'GITEA' ? 'Gitea Account'
                  : 'Bitbucket Account'}
              </label>
              <select class="select select-bordered w-full">
                <option>Select a {provider()} Account</option>
              </select>
            </div>
            <div>
              <label class="block text-sm mb-1.5">Repository</label>
              <select class="select select-bordered w-full">
                <option>Select repository</option>
              </select>
            </div>
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label class="block text-sm mb-1.5">Branch</label>
                <BranchSelect
                  value={branch()}
                  onChange={setBranch}
                  owner={props.app.owner ?? ''}
                  repo={props.app.repository ?? ''}
                  provider={provider()}
                />
              </div>
              <div>
                <label class="block text-sm mb-1.5">Build Path</label>
                <input class="input input-bordered w-full" value={buildPath()} onInput={e => setBuildPath(e.currentTarget.value)} />
              </div>
            </div>
            <div>
              <label class="block text-sm mb-1.5">Trigger Type</label>
              <select class="select select-bordered w-full" value={triggerType()} onChange={e => setTriggerType(e.currentTarget.value)}>
                <option value="PUSH">On Push</option>
                <option value="TAG">On Tag</option>
              </select>
            </div>
            <div>
              <label class="block text-sm mb-1.5 flex items-center gap-1">Watch Paths</label>
              <div class="flex gap-2">
                <input class="input input-bordered flex-1" placeholder="src/**, dist/*.js" />
                <button class="btn btn-ghost btn-sm">+</button>
              </div>
            </div>
            <div class="flex items-center gap-2">
              <input type="checkbox" class="toggle toggle-sm" id="submodules" />
              <label for="submodules" class="text-sm cursor-pointer">Enable Submodules</label>
            </div>
            <div class="flex justify-end">
              <button class="btn btn-neutral btn-sm" onClick={saveProvider} disabled={saving()}>{saving() && <span class="loading loading-spinner loading-xs" />}{saving() ? 'Saving…' : 'Save'}</button>
            </div>
          </div>
        </Show>

        {/* GIT — URL, SSH Key, Branch, Build Path, Watch Paths */}
        <Show when={provider() === 'GIT'}>
          <div class="flex flex-col gap-4">
            <div>
              <label class="block text-sm mb-1.5">Repository URL</label>
              <input
                class="input input-bordered w-full font-mono"
                placeholder="git@github.com:user/repo.git"
                value={props.app.custom_git_url ?? ''}
                // onInput={e => setCustomGitUrl(e.currentTarget.value)}
              />
            </div>
            <div>
              <label class="block text-sm mb-1.5">SSH Key</label>
              <select class="select select-bordered w-full">
                <option value="">None</option>
              </select>
            </div>
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label class="block text-sm mb-1.5">Branch</label>
                <BranchSelect
                  value={props.app.custom_git_branch ?? ''}
                  onChange={setBranch}
                  repoUrl={props.app.custom_git_url ?? ''}
                />
              </div>
              <div>
                <label class="block text-sm mb-1.5">Build Path</label>
                <input class="input input-bordered w-full" value="/" />
              </div>
            </div>
            <div>
              <label class="block text-sm mb-1.5">Watch Paths</label>
              <div class="flex gap-2">
                <input class="input input-bordered flex-1" placeholder="src/**, dist/*.js" />
                <button class="btn btn-ghost btn-sm">+</button>
              </div>
            </div>
            <div class="flex items-center gap-2">
              <input type="checkbox" class="toggle toggle-sm" id="submodules-git" />
              <label for="submodules-git" class="text-sm cursor-pointer">Enable Submodules</label>
            </div>
            <div class="flex justify-end">
              <button class="btn btn-neutral btn-sm" onClick={saveProvider} disabled={saving()}>{saving() && <span class="loading loading-spinner loading-xs" />}{saving() ? 'Saving…' : 'Save'}</button>
            </div>
          </div>
        </Show>

        {/* DOCKER — image, username, password, registry URL */}
        <Show when={provider() === 'DOCKER'}>
          <div class="flex flex-col gap-4">
            <div>
              <label class="block text-sm mb-1.5">Docker Image</label>
              <input class="input input-bordered w-full font-mono" placeholder="nginx:latest" value={props.app.docker_image ?? ''} />
            </div>
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label class="block text-sm mb-1.5">Username</label>
                <input class="input input-bordered w-full" placeholder="Optional" />
              </div>
              <div>
                <label class="block text-sm mb-1.5">Password</label>
                <input class="input input-bordered w-full" type="password" placeholder="Optional" />
              </div>
            </div>
            <div>
              <label class="block text-sm mb-1.5">Registry URL</label>
              <input class="input input-bordered w-full font-mono" placeholder="registry.example.com" value={props.app.registry_url ?? ''} />
            </div>
            <div class="flex justify-end">
              <button class="btn btn-neutral btn-sm" onClick={saveProvider} disabled={saving()}>{saving() && <span class="loading loading-spinner loading-xs" />}{saving() ? 'Saving…' : 'Save'}</button>
            </div>
          </div>
        </Show>

        {/* DROP — drag & drop */}
        <Show when={provider() === 'DROP'}>
          <div class="flex flex-col gap-4">
            <div class="border-2 border-dashed border-base-300 rounded-lg p-8 flex flex-col items-center gap-2 text-base-content/40 hover:border-base-content/30 transition-colors cursor-pointer">
              <p class="text-sm">Drop your zip file here</p>
              <p class="text-xs">or click to browse</p>
            </div>
            <div class="flex justify-end">
              <button class="btn btn-neutral btn-sm">Upload</button>
            </div>
          </div>
        </Show>
      </section>

      {/* Build Type */}
      <section class="bg-base-200 border border-base-300 rounded-lg p-6">
        <div class="flex items-center justify-between mb-1">
          <h2 class="text-base font-semibold">Build Type</h2>
          <Settings class="w-4 h-4 text-base-content/40" />
        </div>
        <p class="text-sm text-base-content/40 mb-4">Select the way of building your code</p>

        {/* Info banner */}
        <div class="flex items-start gap-2 bg-primary/10 border border-primary/20 rounded-md px-3 py-2.5 mb-5">
          <Info class="w-4 h-4 text-primary shrink-0 mt-0.5" />
          <p class="text-xs text-primary/90 leading-relaxed">
            Builders can consume significant memory and CPU resources (recommended: 4+ GB RAM and 2+ CPU cores).
            For production environments, please review our <a class="underline" href="#">Production Guide</a> for best practices.
          </p>
        </div>

        <div class="flex flex-col gap-2 mb-4">
          <p class="text-sm font-medium">Build Type</p>
          {BUILD_TYPES.map(bt => (
            <label class="flex items-center gap-2 cursor-pointer">
              <input
                type="radio"
                class="radio radio-sm"
                name="build_type"
                value={bt.id}
                checked={buildType() === bt.id}
                onChange={() => setBuildType(bt.id)}
              />
              <span class="text-sm">{bt.label}</span>
              <Show when={bt.isNew}>
                <span class="badge badge-xs badge-neutral">New</span>
              </Show>
            </label>
          ))}
        </div>

        <Show when={buildType() === 'STATIC'}>
          <div class="mb-4">
            <label class="block text-sm font-medium mb-1">Publish Directory</label>
            <p class="text-xs text-base-content/40 mb-2">Allows you to serve a single directory via NGINX after the build phase.</p>
            <input class="input input-bordered w-full" placeholder="Publish Directory" value={publishDir()} onInput={e => setPublishDir(e.currentTarget.value)} />
          </div>
        </Show>

        <div class="flex justify-end">
          <button class="btn btn-neutral btn-sm" onClick={saveBuildType} disabled={saving()}>
            {saving() && <span class="loading loading-spinner loading-xs" />}
            {saving() ? 'Saving…' : 'Save'}
          </button>
        </div>
      </section>
    </div>
  );
}
