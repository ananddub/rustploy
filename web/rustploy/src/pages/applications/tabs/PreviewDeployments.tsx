import { createSignal, Show } from 'solid-js';
import { Eye, GitBranch, Info, ToggleLeft } from 'lucide-solid';
import type { ApplicationResponseDto } from '../../../client/types.gen';
import {
  applicationControllerPatchGithubSource,
  applicationControllerPatchGitlabSource,
  applicationControllerPatchGiteaSource,
  applicationControllerPatchBitbucketSource,
} from '../../../client/sdk.gen';

type Props = {
  app: ApplicationResponseDto;
  onUpdated: (app: ApplicationResponseDto) => void;
};

export default function PreviewDeploymentsTab(props: Props) {
  const [enabled, setEnabled] = createSignal(false);
  const [wildcard, setWildcard] = createSignal('');
  const [port, setPort] = createSignal('3000');
  const [https, setHttps] = createSignal(false);
  const [path, setPath] = createSignal('/');
  const [limit, setLimit] = createSignal('3');
  const [saving, setSaving] = createSignal(false);

  // Source type is the same as the main app; branch-based preview is only for git providers
  const isGitProvider = () =>
    ['GITHUB', 'GITLAB', 'GITEA', 'BITBUCKET', 'GIT'].includes(
      props.app.source_type,
    );

  return (
    <div class="flex flex-col gap-6">
      {/* Enable toggle */}
      <section class="bg-base-200 border border-base-300 rounded-lg p-6">
        <div class="flex items-start justify-between">
          <div>
            <h2 class="text-base font-semibold">Preview Deployments</h2>
            <p class="text-sm text-base-content/40 mt-1 max-w-lg">
              Automatically deploy pull-request / merge-request branches as
              isolated preview environments. Requires a Git provider source.
            </p>
          </div>
          <input
            type="checkbox"
            class="toggle"
            checked={enabled()}
            onChange={e => setEnabled(e.currentTarget.checked)}
          />
        </div>

        <Show when={!isGitProvider()}>
          <div class="flex items-start gap-2 mt-4 bg-warning/10 border border-warning/20 rounded-md px-3 py-2.5">
            <Info class="w-4 h-4 text-warning shrink-0 mt-0.5" />
            <p class="text-xs text-warning/90 leading-relaxed">
              Preview deployments require a GitHub, GitLab, Gitea, Bitbucket, or
              custom Git source. Switch the provider in the General tab first.
            </p>
          </div>
        </Show>
      </section>

      {/* Config — only visible when enabled + git provider */}
      <Show when={enabled() && isGitProvider()}>
        <section class="bg-base-200 border border-base-300 rounded-lg p-6">
          <h2 class="text-base font-semibold mb-4">Preview Configuration</h2>

          <div class="flex flex-col gap-4">
            <div class="grid grid-cols-2 gap-4">
              <fieldset class="fieldset">
                <legend class="fieldset-legend text-base-content/70">
                  Wildcard Domain
                </legend>
                <input
                  class="input input-bordered w-full"
                  placeholder="*.preview.example.com"
                  value={wildcard()}
                  onInput={e => setWildcard(e.currentTarget.value)}
                />
                <p class="text-xs text-base-content/40 mt-1">
                  Subdomains will be generated per branch.
                </p>
              </fieldset>

              <fieldset class="fieldset">
                <legend class="fieldset-legend text-base-content/70">
                  Exposed Port
                </legend>
                <input
                  class="input input-bordered w-full"
                  type="number"
                  placeholder="3000"
                  value={port()}
                  onInput={e => setPort(e.currentTarget.value)}
                />
              </fieldset>
            </div>

            <div class="grid grid-cols-2 gap-4">
              <fieldset class="fieldset">
                <legend class="fieldset-legend text-base-content/70">
                  Path Prefix
                </legend>
                <input
                  class="input input-bordered w-full"
                  placeholder="/"
                  value={path()}
                  onInput={e => setPath(e.currentTarget.value)}
                />
              </fieldset>

              <fieldset class="fieldset">
                <legend class="fieldset-legend text-base-content/70">
                  Max Active Previews
                </legend>
                <input
                  class="input input-bordered w-full"
                  type="number"
                  min="1"
                  max="20"
                  value={limit()}
                  onInput={e => setLimit(e.currentTarget.value)}
                />
              </fieldset>
            </div>

            <div class="flex items-center gap-3">
              <input
                id="preview-https"
                type="checkbox"
                class="toggle toggle-sm"
                checked={https()}
                onChange={e => setHttps(e.currentTarget.checked)}
              />
              <label for="preview-https" class="text-sm cursor-pointer">
                Enable HTTPS / SSL
              </label>
            </div>

            <div class="flex justify-end">
              <button
                class="btn btn-neutral btn-sm"
                disabled={saving()}
                onClick={() => {
                  /* PATCH endpoint for preview settings not yet in API */
                }}
              >
                {saving() && <span class="loading loading-spinner loading-xs" />}
                {saving() ? 'Saving…' : 'Save'}
              </button>
            </div>
          </div>
        </section>

        {/* Active previews */}
        <section class="bg-base-200 border border-base-300 rounded-lg p-6">
          <h2 class="text-base font-semibold mb-1">Active Previews</h2>
          <p class="text-sm text-base-content/40 mb-6">
            Running preview environments for open pull requests.
          </p>
          <div class="flex flex-col items-center justify-center py-10 text-base-content/30">
            <Eye class="w-10 h-10 mb-3" />
            <p class="text-sm">No active preview environments</p>
            <p class="text-xs mt-1">
              Open a pull request on your repository to trigger a preview.
            </p>
          </div>
        </section>
      </Show>
    </div>
  );
}
