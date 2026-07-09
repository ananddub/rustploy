import { createSignal } from 'solid-js';
import { projectControllerCreate } from '../../client/sdk.gen';
import { authSession } from '../../lib/auth';
import type { ProjectResponseDto } from '../../client/types.gen';
import { Modal } from '../../components';

type Props = {
  onClose: () => void;
  onCreated: (project: ProjectResponseDto) => void;
};

export default function CreateProjectModal(props: Props) {
  const [name, setName] = createSignal('');
  const [description, setDescription] = createSignal('');
  const [envVar, setEnvVar] = createSignal('');
  const [loading, setLoading] = createSignal(false);
  const [error, setError] = createSignal('');

  const submit = async (e: Event) => {
    e.preventDefault();
    setError('');
    setLoading(true);
    try {
      const session = authSession()!;
      const res = await projectControllerCreate({
        body: {
          name: name(),
          description: description() || undefined,
          env_var: envVar(),
          organization_id: session.user.group_id,
        },
      });
      if (res.error || !res.data) throw new Error((res.error as any)?.message ?? 'Failed to create');
      props.onCreated(res.data);
      props.onClose();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Something went wrong');
    } finally {
      setLoading(false);
    }
  };

  return (
    <Modal title="Create Project" onClose={props.onClose}>
      <form onSubmit={submit} class="flex flex-col gap-4">
        <fieldset class="fieldset">
          <legend class="fieldset-legend text-base-content/70">Name <span class="text-error">*</span></legend>
          <input
            class="input input-bordered w-full"
            placeholder="my-project"
            value={name()}
            onInput={(e) => setName(e.currentTarget.value)}
            required
          />
        </fieldset>

        <fieldset class="fieldset">
          <legend class="fieldset-legend text-base-content/70">Description</legend>
          <textarea
            class="textarea textarea-bordered w-full resize-none"
            placeholder="Optional description..."
            rows={3}
            value={description()}
            onInput={(e) => setDescription(e.currentTarget.value)}
          />
        </fieldset>

        <fieldset class="fieldset">
          <legend class="fieldset-legend text-base-content/70">Env Var Prefix <span class="text-error">*</span></legend>
          <input
            class="input input-bordered w-full"
            placeholder="MY_PROJECT"
            value={envVar()}
            onInput={(e) => setEnvVar(e.currentTarget.value)}
            required
          />
          <p class="text-xs text-base-content/40 mt-1">Used as prefix for environment variables</p>
        </fieldset>

        {error() && (
          <div class="alert alert-error text-sm py-2">
            <span>{error()}</span>
          </div>
        )}

        <div class="flex justify-end gap-2 pt-1">
          <button type="button" class="btn btn-ghost btn-sm" onClick={props.onClose} disabled={loading()}>
            Cancel
          </button>
          <button type="submit" class="btn btn-neutral btn-sm" disabled={loading()}>
            {loading() && <span class="loading loading-spinner loading-xs" />}
            {loading() ? 'Creating…' : 'Create Project'}
          </button>
        </div>
      </form>
    </Modal>
  );
}
