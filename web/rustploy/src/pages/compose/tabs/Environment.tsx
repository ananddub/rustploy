import { createSignal, onMount, onCleanup, Show, createEffect } from 'solid-js';
import { composeControllerPatch } from '../../../client/sdk.gen';
import type { ComposeResponseDto } from '../../../client/types.gen';
import loader from '@monaco-editor/loader';

type Props = {
  compose: ComposeResponseDto;
  onUpdated: (c: ComposeResponseDto) => void;
  active: boolean;
};

export default function ComposeEnvironmentTab(props: Props) {
  const [envVars, setEnvVars] = createSignal(props.compose.env_var ?? '');
  const [saving, setSaving] = createSignal(false);
  const [saved, setSaved] = createSignal(false);
  let editorContainer: HTMLDivElement | undefined;
  let editorInstance: any = null;

  onMount(async () => {
    if (!editorContainer) return;
    const monaco = await loader.init();
    editorInstance = monaco.editor.create(editorContainer, {
      value: props.compose.env_var ?? '',
      language: 'ini',
      theme: 'vs-dark',
      fontSize: 13,
      fontFamily: 'monospace',
      minimap: { enabled: false },
      scrollBeyondLastLine: false,
      lineNumbers: 'on',
      wordWrap: 'on',
      automaticLayout: true,
      padding: { top: 8, bottom: 8 },
    });
    editorInstance.onDidChangeModelContent(() => setEnvVars(editorInstance.getValue()));
    setTimeout(() => editorInstance?.layout(), 50);
  });

  createEffect(() => {
    if (props.active && editorInstance) setTimeout(() => editorInstance.layout(), 10);
  });

  onCleanup(() => editorInstance?.dispose());

  const save = async () => {
    setSaving(true);
    try {
      const res = await composeControllerPatch({
        path: { id: props.compose.id },
        body: { env_var: envVars() },
      });
      if (res.data) {
        props.onUpdated(res.data);
        setSaved(true);
        setTimeout(() => setSaved(false), 2000);
      }
    } finally {
      setSaving(false);
    }
  };

  return (
    <div class="bg-base-200 border border-base-300 rounded-lg p-6 flex flex-col gap-4">
      <div>
        <h2 class="text-base font-semibold">Environment Variables</h2>
        <p class="text-sm text-base-content/40 mt-1">Set environment variables in <code class="font-mono">KEY=VALUE</code> format.</p>
      </div>
      <div class="rounded-md overflow-hidden border border-base-300" style={{ height: '420px' }}>
        <div ref={editorContainer} style={{ height: '100%', width: '100%' }} />
      </div>
      <div class="flex justify-end items-center gap-3">
        <Show when={saved()}><span class="text-sm text-success">Saved!</span></Show>
        <button class="btn btn-neutral btn-sm" onClick={save} disabled={saving()}>
          {saving() && <span class="loading loading-spinner loading-xs" />}
          {saving() ? 'Saving…' : 'Save'}
        </button>
      </div>
    </div>
  );
}
