import { createSignal, Show } from 'solid-js';
import { applicationControllerCreate } from '../../client/sdk.gen';
import type { ApplicationResponseDto } from '../../client/types.gen';
import { Modal } from '..';

type Props = {
  environmentId: number;
  onClose: () => void;
  onCreated: (app: ApplicationResponseDto) => void;
};

export default function CreateApplicationModal(props: Props) {
  const [name, setName] = createSignal('');
  const [desc, setDesc] = createSignal('');
  const [loading, setLoading] = createSignal(false);
  const [error, setError] = createSignal('');

  const submit = async (e: Event) => {
    e.preventDefault();
    setError('');
    setLoading(true);
    try {
      const res = await applicationControllerCreate({
        body: {
          name: name(),
          description: desc() || undefined,
          build_type: 'NIXPACKS',
          source_type: 'GITHUB',
          environment_id: props.environmentId,
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
    <Modal title="Create Application" onClose={props.onClose}>
      <form onSubmit={submit} class="flex flex-col gap-4">
        <fieldset class="fieldset">
          <legend class="fieldset-legend text-base-content/70">
            Name <span class="text-error">*</span>
          </legend>
          <input
            class="input input-bordered w-full"
            placeholder="my-app"
            value={name()}
            onInput={e => setName(e.currentTarget.value)}
            required
            autofocus
          />
        </fieldset>

        <fieldset class="fieldset">
          <legend class="fieldset-legend text-base-content/70">Description</legend>
          <input
            class="input input-bordered w-full"
            placeholder="Optional"
            value={desc()}
            onInput={e => setDesc(e.currentTarget.value)}
          />
        </fieldset>

        <Show when={error()}>
          <div class="alert alert-error text-sm py-2">
            <span>{error()}</span>
          </div>
        </Show>

        <div class="flex justify-end gap-2 pt-1">
          <button type="button" class="btn btn-ghost btn-sm" onClick={props.onClose} disabled={loading()}>
            Cancel
          </button>
          <button type="submit" class="btn btn-neutral btn-sm" disabled={loading()}>
            {loading() && <span class="loading loading-spinner loading-xs" />}
            {loading() ? 'Creating…' : 'Create Application'}
          </button>
        </div>
      </form>
    </Modal>
  );
}
