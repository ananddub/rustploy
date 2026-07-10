import { createSignal, createEffect, Show, onMount, onCleanup } from 'solid-js';
import { Info, X } from 'lucide-solid';
import { environmentControllerPatch } from '../../client/sdk.gen';
import type { EnvironmentResponseDto } from '../../client/types.gen';
import loader from '@monaco-editor/loader';

type Props = {
  env: EnvironmentResponseDto;
  onClose: () => void;
  onUpdated: (env: EnvironmentResponseDto) => void;
};

export default function ProjectEnvironmentModal(props: Props) {
  const [envVars, setEnvVars] = createSignal(props.env.env_var ?? '');
  const [loading, setLoading] = createSignal(false);
  const [error, setError] = createSignal('');
  let editorContainer: HTMLDivElement | undefined;
  let editorInstance: any = null;

  onMount(async () => {
    if (!editorContainer) return;
    const monaco = await loader.init();

    editorInstance = monaco.editor.create(editorContainer, {
      value: envVars(),
      language: 'ini',
      theme: 'vs-dark',
      fontSize: 13,
      fontFamily: 'monospace',
      minimap: { enabled: false },
      scrollBeyondLastLine: false,
      lineNumbers: 'on',
      renderLineHighlight: 'line',
      wordWrap: 'on',
      automaticLayout: true,
      tabSize: 2,
      padding: { top: 8, bottom: 8 },
    });

    editorInstance.onDidChangeModelContent(() => {
      setEnvVars(editorInstance.getValue());
    });
    setTimeout(() => editorInstance?.layout(), 50);
  });

  onCleanup(() => {
    editorInstance?.dispose();
  });

  const submit = async (e: Event) => {
    e.preventDefault();
    setError('');
    setLoading(true);
    try {
      const res = await environmentControllerPatch({
        path: { id: props.env.id },
        body: { env_var: envVars() },
      });
      if (res.error || !res.data) throw new Error('Failed to update environment');
      props.onUpdated(res.data);
      props.onClose();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Something went wrong');
    } finally {
      setLoading(false);
    }
  };

  return (
    <>
      <div class="fixed inset-0 bg-black/60 z-40" onClick={props.onClose} />

      <div class="fixed inset-0 z-50 flex items-center justify-center p-4">
        <div class="bg-base-200 border border-base-300 rounded-lg w-full max-w-2xl shadow-2xl">

          {/* Header */}
          <div class="flex items-start justify-between px-6 pt-5 pb-3">
            <div>
              <h2 class="font-semibold text-base-content text-base">Project Environment</h2>
              <p class="text-sm text-base-content/50 mt-0.5">
                Update the env variables accessible to all services of this project.
              </p>
            </div>
            <button
              class="text-base-content/40 hover:text-base-content transition-colors ml-4 mt-0.5"
              onClick={props.onClose}
            >
              <X class="w-4 h-4" />
            </button>
          </div>

          {/* Info banner */}
          <div class="mx-6 mb-4">
            <div class="flex items-start gap-2 bg-primary/10 border border-primary/20 rounded-md px-3 py-2.5">
              <Info class="w-4 h-4 text-primary shrink-0 mt-0.5" />
              <p class="text-xs text-primary/90 leading-relaxed">
                Use this syntax to reference project-level variables:{' '}
                <code class="font-mono font-semibold">DATABASE_URL=$&#123;&#123;project.DATABASE_URL&#125;&#125;</code>
              </p>
            </div>
          </div>

          <form onSubmit={submit} class="px-6 pb-5 flex flex-col gap-3">
            <p class="text-sm font-medium text-base-content/70">Environment variables</p>

            {/* Monaco Editor container */}
            <div
              ref={editorContainer}
              class="rounded-md overflow-hidden border border-base-300"
              style={{ height: '380px' }}
            />

            <Show when={error()}>
              <div class="alert alert-error text-sm py-2">
                <span>{error()}</span>
              </div>
            </Show>

            <div class="flex justify-end">
              <button type="submit" class="btn btn-neutral btn-sm" disabled={loading()}>
                {loading() && <span class="loading loading-spinner loading-xs" />}
                {loading() ? 'Updating…' : 'Update'}
              </button>
            </div>
          </form>
        </div>
      </div>
    </>
  );
}
