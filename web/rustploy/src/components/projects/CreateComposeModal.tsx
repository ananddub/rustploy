import { createSignal, Show } from 'solid-js';
import { composeControllerCreate } from '../../client/sdk.gen';
import type { ComposeResponseDto } from '../../client/types.gen';
import { Modal } from '..';

type Props = {
  environmentId: number;
  onClose: () => void;
  onCreated: (compose: ComposeResponseDto) => void;
};

export default function CreateComposeModal(props: Props) {
  const [name, setName] = createSignal('');
  const [desc, setDesc] = createSignal('');
  const [loading, setLoading] = createSignal(false);
  const [error, setError] = createSignal('');

  const submit = async (e: Event) => {
    e.preventDefault();
    setError('');
    setLoading(true);
    try {
      const res = await composeControllerCreate({
        body: {
          name: name(),
          description: desc() || undefined,
          compose_file: '',
          compose_type: 'DOCKER-COMPOSE',
          source_type: 'RAW',
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
    <Modal title="Create Compose" onClose={props.onClose}>
      <form onSubmit={submit} class="flex flex-col gap-4">
        <fieldset class="fieldset">
          <legend class="fieldset-legend text-base-content/70">
            Name <span class="text-error">*</span>
          </legend>
          <input
            class="input input-bordered w-full"
            placeholder="my-compose"
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
            {loading() ? 'Creating…' : 'Create Compose'}
          </button>
        </div>
      </form>
    </Modal>
  );
}
