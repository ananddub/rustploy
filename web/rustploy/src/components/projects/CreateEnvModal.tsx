import { createSignal, Show } from 'solid-js';
import { environmentControllerCreate } from '../../client/sdk.gen';
import type { EnvironmentResponseDto } from '../../client/types.gen';
import { Modal } from '..';

type Props = {
  projectId: number;
  onClose: () => void;
  onCreated: (env: EnvironmentResponseDto) => void;
};

export default function CreateEnvModal(props: Props) {
  const [name, setName] = createSignal('');
  const [envVar, setEnvVar] = createSignal('');
  const [desc, setDesc] = createSignal('');
  const [loading, setLoading] = createSignal(false);
  const [error, setError] = createSignal('');

  const submit = async (e: Event) => {
    e.preventDefault();
    setError('');
    setLoading(true);
    try {
      const res = await environmentControllerCreate({
        body: {
          name: name(),
          env_var: envVar(),
          description: desc() || undefined,
          is_default: false,
          project_id: props.projectId,
        },
      });
      if (res.error || !res.data) throw new Error('Failed to create environment');
      props.onCreated(res.data);
      props.onClose();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Something went wrong');
    } finally {
      setLoading(false);
    }
  };

  return (
    <Modal title="New Environment" onClose={props.onClose}>
      <form onSubmit={submit} class="flex flex-col gap-4">
        <fieldset class="fieldset">
          <legend class="fieldset-legend text-base-content/70">Name <span class="text-error">*</span></legend>
          <input class="input input-bordered w-full" placeholder="production" value={name()} onInput={e => setName(e.currentTarget.value)} required />
        </fieldset>

        <fieldset class="fieldset">
          <legend class="fieldset-legend text-base-content/70">Env Var Prefix <span class="text-error">*</span></legend>
          <input class="input input-bordered w-full" placeholder="PRODUCTION" value={envVar()} onInput={e => setEnvVar(e.currentTarget.value)} required />
        </fieldset>

        <fieldset class="fieldset">
          <legend class="fieldset-legend text-base-content/70">Description</legend>
          <input class="input input-bordered w-full" placeholder="Optional" value={desc()} onInput={e => setDesc(e.currentTarget.value)} />
        </fieldset>

        <Show when={error()}>
          <div class="alert alert-error text-sm py-2"><span>{error()}</span></div>
        </Show>

        <div class="flex justify-end gap-2 pt-1">
          <button type="button" class="btn btn-ghost btn-sm" onClick={props.onClose} disabled={loading()}>Cancel</button>
          <button type="submit" class="btn btn-neutral btn-sm" disabled={loading()}>
            {loading() && <span class="loading loading-spinner loading-xs" />}
            {loading() ? 'Creating…' : 'Create'}
          </button>
        </div>
      </form>
    </Modal>
  );
}
