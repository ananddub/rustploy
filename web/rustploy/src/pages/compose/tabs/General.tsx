import { createSignal, Show, onMount, onCleanup } from 'solid-js';
import { Info, Eye } from 'lucide-solid';
import {
  composeControllerPatch,
  composeControllerDeploy,
  composeControllerReload,
  composeControllerRedeploy,
  composeControllerStart,
} from '../../../client/sdk.gen';
import type { ComposeResponseDto } from '../../../client/types.gen';
import loader from '@monaco-editor/loader';
import { DeployActions, BranchSelect } from '../../../components';

type Props = {
  compose: ComposeResponseDto;
  onUpdated: (c: ComposeResponseDto) => void;
};

const PROVIDERS = [
  { id: 'GITHUB', label: 'GitHub' },
  { id: 'GITLAB', label: 'GitLab' },
  { id: 'BITBUCKET', label: 'Bitbucket' },
  { id: 'GITEA', label: 'Gitea' },
  { id: 'GIT', label: 'Git' },
  { id: 'RAW', label: 'Raw' },
];

const DEFAULT_COMPOSE = ``;

export default function ComposeGeneralTab(props: Props) {
  const [provider, setProvider] = createSignal(
    props.compose.source_type ?? "RAW",
  );
  const [composeFile, setComposeFile] = createSignal(
    props.compose.compose_file ?? "",
  );
  const [autoDeploy, setAutoDeploy] = createSignal(true);
  const [cleanCache, setCleanCache] = createSignal(false);
  const [saving, setSaving] = createSignal(false);
  const [branch, setBranch] = createSignal(props.compose.branch ?? "");

  let editorContainer: HTMLDivElement | undefined;
  let editorInstance: any = null;

  onMount(async () => {
    if (provider() !== "RAW") return;
    await initEditor();
  });

  const initEditor = async () => {
    if (editorInstance || !editorContainer) return;
    const monaco = await loader.init();
    editorInstance = monaco.editor.create(editorContainer, {
      value: composeFile(),
      language: "yaml",
      theme: "vs-dark",
      fontSize: 13,
      fontFamily: "monospace",
      minimap: { enabled: false },
      scrollBeyondLastLine: false,
      lineNumbers: "on",
      wordWrap: "off",
      automaticLayout: true,
      padding: { top: 8, bottom: 8 },
    });
    editorInstance.onDidChangeModelContent(() => {
      setComposeFile(editorInstance.getValue());
    });
    // Force layout after mount
    setTimeout(() => editorInstance?.layout(), 50);
  };

  onCleanup(() => editorInstance?.dispose());

  const save = async () => {
    setSaving(true);
    try {
      const res = await composeControllerPatch({
        path: { id: props.compose.id },
        body: { compose_file: composeFile() },
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
        <div class="flex items-center justify-between mb-4">
          <div>
            <h2 class="text-base font-semibold">Deploy Settings</h2>
            <p class="text-sm text-base-content/40 mt-0.5">
              Create a compose file to deploy your compose
            </p>
          </div>
          <button class="btn btn-ghost btn-sm">Compose</button>
        </div>

        <DeployActions
          autoDeploy={autoDeploy()}
          onAutoDeploy={setAutoDeploy}
          cleanCache={cleanCache()}
          onCleanCache={setCleanCache}
          onDeploy={async () => {
            await composeControllerDeploy({ path: { id: props.compose.id } });
          }}
          onReload={async () => {
            await composeControllerReload({ path: { id: props.compose.id } });
          }}
          onRebuild={async () => {
            await composeControllerRedeploy({ path: { id: props.compose.id } });
          }}
          onStart={async () => {
            await composeControllerStart({ path: { id: props.compose.id } });
          }}
        />
      </section>

      {/* Provider */}
      <section class="bg-base-200 border border-base-300 rounded-lg p-6">
        <div class="flex items-center justify-between mb-1">
          <div>
            <h2 class="text-base font-semibold">Provider</h2>
            <p class="text-sm text-base-content/40 mt-0.5">
              Select the source of your code
            </p>
          </div>
          <Show when={provider() === "RAW"}>
            <button class="btn btn-ghost btn-sm gap-1.5">
              <Eye class="w-3.5 h-3.5" /> Preview Compose
            </button>
          </Show>
        </div>

        {/* Provider tabs */}
        <div class="flex flex-wrap gap-0 border-b border-base-300 mb-5 mt-4">
          {PROVIDERS.map((p) => (
            <button
              class={`px-4 py-2 text-sm transition-colors border-b-2 -mb-px ${
                provider() === p.id
                  ? "border-base-content text-base-content font-medium"
                  : "border-transparent text-base-content/50 hover:text-base-content"
              }`}
              onClick={() => {
                setProvider(p.id);
                if (p.id === "RAW") {
                  setTimeout(() => {
                    if (editorInstance) editorInstance.layout();
                    else initEditor();
                  }, 50);
                }
              }}
            >
              {p.label}
            </button>
          ))}
        </div>

        {/* RAW — Monaco editor — always in DOM, hidden when not RAW */}
        <div style={{ display: provider() === "RAW" ? "block" : "none" }}>
          <p class="text-sm font-medium mb-1">Compose File</p>
          <p class="text-xs text-base-content/40 mb-3">
            Configure your Docker Compose file for this service.
          </p>
          <div
            class="rounded-md overflow-hidden border border-base-300"
            style={{ height: "420px" }}
          >
            <div
              ref={editorContainer}
              style={{ height: "100%", width: "100%" }}
            />
          </div>
          <div class="flex justify-end mt-4">
            <button
              class="btn btn-neutral btn-sm"
              onClick={save}
              disabled={saving()}
            >
              {saving() && <span class="loading loading-spinner loading-xs" />}
              {saving() ? "Saving…" : "Save"}
            </button>
          </div>
        </div>

        {/* Git providers */}
        <Show
          when={["GITHUB", "GITLAB", "GITEA", "BITBUCKET"].includes(provider())}
        >
          <div class="flex flex-col gap-4">
            <div>
              <label class="block text-sm mb-1.5">{provider()} Account</label>
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
                  owner={props.compose.owner ?? ''}
                  repo={props.compose.repository ?? ''}
                />
              </div>
              <div>
                <label class="block text-sm mb-1.5">Compose Path</label>
                <input
                  class="input input-bordered w-full"
                  value={props.compose.compose_path ?? "./docker-compose.yml"}
                />
              </div>
            </div>
            <div class="flex justify-end">
              <button
                class="btn btn-neutral btn-sm"
                onClick={save}
                disabled={saving()}
              >
                {saving() && (
                  <span class="loading loading-spinner loading-xs" />
                )}
                {saving() ? "Saving…" : "Save"}
              </button>
            </div>
          </div>
        </Show>

        {/* GIT */}
        <Show when={provider() === "GIT"}>
          <div class="flex flex-col gap-4">
            <div>
              <label class="block text-sm mb-1.5">Repository URL</label>
              <input
                class="input input-bordered w-full font-mono"
                placeholder="git@github.com:user/repo.git"
                // on:change={}
                value={props.compose.custom_git_url ?? ""}
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
                  value={props.compose.custom_git_branch ?? ''}
                  onChange={setBranch}
                  repoUrl={props.compose.custom_git_url ?? ''}
                />
              </div>
              <div>
                <label class="block text-sm mb-1.5">Compose Path</label>
                <input
                  class="input input-bordered w-full"
                  value={props.compose.compose_path ?? "./docker-compose.yml"}
                />
              </div>
            </div>
            <div class="flex justify-end">
              <button
                class="btn btn-neutral btn-sm"
                onClick={save}
                disabled={saving()}
              >
                {saving() && (
                  <span class="loading loading-spinner loading-xs" />
                )}
                {saving() ? "Saving…" : "Save"}
              </button>
            </div>
          </div>
        </Show>
      </section>
    </div>
  );
}
