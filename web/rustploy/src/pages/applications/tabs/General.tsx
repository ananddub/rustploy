import { createSignal, Show } from 'solid-js';
import { Info, Settings } from 'lucide-solid';
import {
  applicationControllerPatch,
  applicationControllerPatchGithubSource,
  applicationControllerPatchGitlabSource,
  applicationControllerPatchGiteaSource,
  applicationControllerPatchBitbucketSource,
  applicationControllerPatchCustomGitSource,
  applicationControllerPatchDockerSource,
  applicationControllerPatchDropSource,
  applicationControllerPatchBuild,
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
  // { id: 'GITHUB',    label: 'Github' },
  // { id: 'GITLAB',    label: 'Gitlab' },
  // { id: 'BITBUCKET', label: 'Bitbucket' },
  // { id: 'GITEA',     label: 'Gitea' },
  { id: 'DOCKER',    label: 'Docker' },
  { id: 'GIT',       label: 'Git' },
  { id: 'DROP',      label: 'Drop' },
];

const BUILD_TYPES = [
  { id: 'DOCKERFILE',        label: 'Dockerfile' },
  { id: 'RAILPACK',          label: 'Railpack', isNew: true },
  { id: 'NIXPACKS',          label: 'Nixpacks' },
  { id: 'HEROKU_BUILDPACKS', label: 'Heroku Buildpacks' },
  { id: 'PAKETO_BUILDPACKS', label: 'Paketo Buildpacks' },
  { id: 'STATIC',            label: 'Static' },
];

export default function GeneralTab(props: Props) {
  const [provider, setProvider] = createSignal(props.app.source_type ?? 'GITHUB');
  const [buildType, setBuildType] = createSignal(props.app.build_type ?? 'NIXPACKS');
  const [triggerType, setTriggerType] = createSignal(props.app.trigger_type ?? 'PUSH');
  const [autoDeploy, setAutoDeploy] = createSignal(true);
  const [cleanCache, setCleanCache] = createSignal(false);
  const [saving, setSaving] = createSignal(false);
  const [publishDir, setPublishDir] = createSignal(props.app.build_type === 'STATIC' ? '' : '');

  // --- Hosted git providers (GitHub / GitLab / Gitea / Bitbucket) ---
  const [branch, setBranch] = createSignal(props.app.branch ?? '');
  const [buildPath, setBuildPath] = createSignal('/');
  const [watchPaths, setWatchPaths] = createSignal('');
  const [enableSubmodules, setEnableSubmodules] = createSignal(false);

  // --- Custom GIT ---
  const [gitUrl, setGitUrl] = createSignal(props.app.custom_git_url ?? '');
  const [gitBranch, setGitBranch] = createSignal(props.app.custom_git_branch ?? '');
  const [gitBuildPath, setGitBuildPath] = createSignal('/');
  const [gitWatchPaths, setGitWatchPaths] = createSignal('');
  const [gitSubmodules, setGitSubmodules] = createSignal(false);

  // --- Docker ---
  const [dockerImage, setDockerImage] = createSignal(props.app.docker_image ?? '');
  const [dockerUser, setDockerUser] = createSignal('');
  const [dockerPass, setDockerPass] = createSignal('');
  const [registryUrl, setRegistryUrl] = createSignal(props.app.registry_url ?? '');

  // --------------- save handlers ---------------

  const saveGitHosted = async () => {
    setSaving(true);
    try {
      let res;
      const base = { branch: branch(), build_path: buildPath() };
      if (provider() === 'GITHUB') {
        res = await applicationControllerPatchGithubSource({
          path: { id: props.app.id },
          body: { owner: props.app.owner ?? '', repository: props.app.repository ?? '', ...base },
        });
      } else if (provider() === 'GITLAB') {
        res = await applicationControllerPatchGitlabSource({
          path: { id: props.app.id },
          body: {
            gitlab_owner: props.app.gitlab_owner ?? '',
            gitlab_repository: props.app.gitlab_repository ?? '',
            gitlab_branch: branch(),
            gitlab_build_path: buildPath(),
          },
        });
      } else if (provider() === 'GITEA') {
        res = await applicationControllerPatchGiteaSource({
          path: { id: props.app.id },
          body: {
            gitea_owner: props.app.gitea_owner ?? '',
            gitea_repository: props.app.gitea_repository ?? '',
            gitea_branch: branch(),
            gitea_build_path: buildPath(),
          },
        });
      } else if (provider() === 'BITBUCKET') {
        res = await applicationControllerPatchBitbucketSource({
          path: { id: props.app.id },
          body: {
            bitbucket_owner: props.app.bitbucket_owner ?? '',
            bitbucket_repository: props.app.bitbucket_repository ?? '',
            bitbucket_branch: branch(),
            bitbucket_build_path: buildPath(),
          },
        });
      }
      if (res?.data) props.onUpdated(res.data);
    } finally {
      setSaving(false);
    }
  };

  const saveGit = async () => {
    setSaving(true);
    try {
      const res = await applicationControllerPatchCustomGitSource({
        path: { id: props.app.id },
        body: {
          custom_git_url: gitUrl(),
          custom_git_branch: gitBranch(),
          custom_git_build_path: gitBuildPath(),
        },
      });
      if (res.data) props.onUpdated(res.data);
    } finally {
      setSaving(false);
    }
  };

  const saveDocker = async () => {
    setSaving(true);
    try {
      const res = await applicationControllerPatchDockerSource({
        path: { id: props.app.id },
        body: {
          docker_image: dockerImage(),
          ...(dockerUser() ? { docker_username: dockerUser() } : {}),
          ...(dockerPass() ? { docker_password: dockerPass() } : {}),
          ...(registryUrl() ? { registry_url: registryUrl() } : {}),
        },
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

  // Derive owner/repo for the currently active hosted provider
  const hostedOwner = () => {
    if (provider() === 'GITLAB') return props.app.gitlab_owner ?? '';
    if (provider() === 'GITEA')  return props.app.gitea_owner ?? '';
    if (provider() === 'BITBUCKET') return props.app.bitbucket_owner ?? '';
    return props.app.owner ?? '';
  };
  const hostedRepo = () => {
    if (provider() === 'GITLAB') return props.app.gitlab_repository ?? '';
    if (provider() === 'GITEA')  return props.app.gitea_repository ?? '';
    if (provider() === 'BITBUCKET') return props.app.bitbucket_repository ?? '';
    return props.app.repository ?? '';
  };

  return (
    <div class="flex flex-col gap-6">

      {/* ── Deploy Settings ── */}
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

      {/* ── Provider ── */}
      <section class="bg-base-200 border border-base-300 rounded-lg p-6">
        <h2 class="text-base font-semibold mb-1">Provider</h2>
        <p class="text-sm text-base-content/40 mb-4">Select the source of your code</p>

        {/* Provider tab bar */}
        <div class="flex flex-wrap gap-0 border-b border-base-300 mb-5">
          {PROVIDERS.map(p => (
            <button
              type="button"
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

        {/* ── GITHUB / GITLAB / GITEA / BITBUCKET ── */}
        <Show when={['GITHUB', 'GITLAB', 'GITEA', 'BITBUCKET'].includes(provider())}>
          <div class="flex flex-col gap-4">
            <div>
              <label class="block text-sm mb-1.5">
                {provider() === 'GITHUB' ? 'Github Account'
                  : provider() === 'GITLAB' ? 'Gitlab Account'
                  : provider() === 'GITEA'  ? 'Gitea Account'
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
                {/* No URL input — BranchSelect builds the URL from owner+repo+provider */}
                <BranchSelect
                  value={branch()}
                  onChange={setBranch}
                  owner={hostedOwner()}
                  repo={hostedRepo()}
                  provider={provider()}
                />
              </div>
              <div>
                <label class="block text-sm mb-1.5">Build Path</label>
                <input
                  class="input input-bordered w-full"
                  value={buildPath()}
                  onInput={e => setBuildPath(e.currentTarget.value)}
                />
              </div>
            </div>

            <div>
              <label class="block text-sm mb-1.5">Trigger Type</label>
              <select
                class="select select-bordered w-full"
                value={triggerType()}
                onChange={e => setTriggerType(e.currentTarget.value)}
              >
                <option value="PUSH">On Push</option>
                <option value="TAG">On Tag</option>
              </select>
            </div>

            <div>
              <label class="block text-sm mb-1.5">Watch Paths</label>
              <input
                class="input input-bordered w-full"
                placeholder="src/**, dist/*.js"
                value={watchPaths()}
                onInput={e => setWatchPaths(e.currentTarget.value)}
              />
              <p class="text-xs text-base-content/40 mt-1">
                Comma-separated glob patterns. Leave empty to watch everything.
              </p>
            </div>

            <div class="flex items-center gap-2">
              <input
                type="checkbox"
                class="toggle toggle-sm"
                id="submodules"
                checked={enableSubmodules()}
                onChange={e => setEnableSubmodules(e.currentTarget.checked)}
              />
              <label for="submodules" class="text-sm cursor-pointer">Enable Submodules</label>
            </div>

            <div class="flex justify-end">
              <button
                type="button"
                class="btn btn-neutral btn-sm"
                onClick={saveGitHosted}
                disabled={saving()}
              >
                {saving() && <span class="loading loading-spinner loading-xs" />}
                {saving() ? 'Saving…' : 'Save'}
              </button>
            </div>
          </div>
        </Show>

        {/* ── GIT (custom) ── */}
        <Show when={provider() === 'GIT'}>
          <div class="flex flex-col gap-4">
            <div>
              <label class="block text-sm mb-1.5">Repository URL</label>
              {/* Single source of truth for the URL — BranchSelect reads it reactively */}
              <input
                type="text"
                class="input input-bordered w-full font-mono"
                placeholder="https://github.com/user/repo.git or git@github.com:user/repo.git"
                value={gitUrl()}
                onInput={e => setGitUrl(e.currentTarget.value)}
              />
              <p class="text-xs text-base-content/40 mt-1">
                Branches are fetched automatically as you type.
              </p>
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
                {/* repoUrl is reactive: when gitUrl() changes the effect re-fetches */}
                <BranchSelect
                  value={gitBranch()}
                  onChange={setGitBranch}
                  repoUrl={gitUrl()}
                />
              </div>
              <div>
                <label class="block text-sm mb-1.5">Build Path</label>
                <input
                  class="input input-bordered w-full"
                  value={gitBuildPath()}
                  onInput={e => setGitBuildPath(e.currentTarget.value)}
                />
              </div>
            </div>

            <div>
              <label class="block text-sm mb-1.5">Watch Paths</label>
              <input
                class="input input-bordered w-full"
                placeholder="src/**, dist/*.js"
                value={gitWatchPaths()}
                onInput={e => setGitWatchPaths(e.currentTarget.value)}
              />
              <p class="text-xs text-base-content/40 mt-1">
                Comma-separated glob patterns. Leave empty to watch everything.
              </p>
            </div>

            <div class="flex items-center gap-2">
              <input
                type="checkbox"
                class="toggle toggle-sm"
                id="submodules-git"
                checked={gitSubmodules()}
                onChange={e => setGitSubmodules(e.currentTarget.checked)}
              />
              <label for="submodules-git" class="text-sm cursor-pointer">Enable Submodules</label>
            </div>

            <div class="flex justify-end">
              <button
                type="button"
                class="btn btn-neutral btn-sm"
                onClick={saveGit}
                disabled={saving()}
              >
                {saving() && <span class="loading loading-spinner loading-xs" />}
                {saving() ? 'Saving…' : 'Save'}
              </button>
            </div>
          </div>
        </Show>

        {/* ── DOCKER ── */}
        <Show when={provider() === 'DOCKER'}>
          <div class="flex flex-col gap-4">
            <div>
              <label class="block text-sm mb-1.5">Docker Image</label>
              <input
                class="input input-bordered w-full font-mono"
                placeholder="nginx:latest"
                value={dockerImage()}
                onInput={e => setDockerImage(e.currentTarget.value)}
              />
            </div>
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label class="block text-sm mb-1.5">Username</label>
                <input
                  class="input input-bordered w-full"
                  placeholder="Optional"
                  value={dockerUser()}
                  onInput={e => setDockerUser(e.currentTarget.value)}
                />
              </div>
              <div>
                <label class="block text-sm mb-1.5">Password</label>
                <input
                  class="input input-bordered w-full"
                  type="password"
                  placeholder="Optional"
                  value={dockerPass()}
                  onInput={e => setDockerPass(e.currentTarget.value)}
                />
              </div>
            </div>
            <div>
              <label class="block text-sm mb-1.5">Registry URL</label>
              <input
                class="input input-bordered w-full font-mono"
                placeholder="registry.example.com"
                value={registryUrl()}
                onInput={e => setRegistryUrl(e.currentTarget.value)}
              />
            </div>
            <div class="flex justify-end">
              <button
                type="button"
                class="btn btn-neutral btn-sm"
                onClick={saveDocker}
                disabled={saving()}
              >
                {saving() && <span class="loading loading-spinner loading-xs" />}
                {saving() ? 'Saving…' : 'Save'}
              </button>
            </div>
          </div>
        </Show>

        {/* ── DROP ── */}
        <Show when={provider() === 'DROP'}>
          <div class="flex flex-col gap-4">
            <div class="border-2 border-dashed border-base-300 rounded-lg p-8 flex flex-col items-center gap-2 text-base-content/40 hover:border-base-content/30 transition-colors cursor-pointer">
              <p class="text-sm">Drop your zip file here</p>
              <p class="text-xs">or click to browse</p>
            </div>
            <div class="flex justify-end">
              <button type="button" class="btn btn-neutral btn-sm">Upload</button>
            </div>
          </div>
        </Show>
      </section>

      {/* ── Build Type ── */}
      <section class="bg-base-200 border border-base-300 rounded-lg p-6">
        <div class="flex items-center justify-between mb-1">
          <h2 class="text-base font-semibold">Build Type</h2>
          <Settings class="w-4 h-4 text-base-content/40" />
        </div>
        <p class="text-sm text-base-content/40 mb-4">Select the way of building your code</p>

        <div class="flex items-start gap-2 bg-primary/10 border border-primary/20 rounded-md px-3 py-2.5 mb-5">
          <Info class="w-4 h-4 text-primary shrink-0 mt-0.5" />
          <p class="text-xs text-primary/90 leading-relaxed">
            Builders can consume significant memory and CPU resources (recommended: 4+ GB RAM and
            2+ CPU cores). For production environments, please review our{' '}
            <a class="underline" href="#">Production Guide</a> for best practices.
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
            <p class="text-xs text-base-content/40 mb-2">
              Serve a single directory via NGINX after the build phase.
            </p>
            <input
              class="input input-bordered w-full"
              placeholder="dist"
              value={publishDir()}
              onInput={e => setPublishDir(e.currentTarget.value)}
            />
          </div>
        </Show>

        <div class="flex justify-end">
          <button
            type="button"
            class="btn btn-neutral btn-sm"
            onClick={saveBuildType}
            disabled={saving()}
          >
            {saving() && <span class="loading loading-spinner loading-xs" />}
            {saving() ? 'Saving…' : 'Save'}
          </button>
        </div>
      </section>
    </div>
  );
}
