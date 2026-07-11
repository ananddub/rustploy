import { createSignal, Show } from 'solid-js';
import { applicationControllerPatchResources } from '../../../client/sdk.gen';
import type { ApplicationResponseDto } from '../../../client/types.gen';
import { Trash2 } from 'lucide-solid';

type Props = { app: ApplicationResponseDto; onUpdated: (a: ApplicationResponseDto) => void };

export default function AdvancedTab(props: Props) {
  // ApplicationResponseDto does not carry resource fields —
  // they live in a separate resources endpoint. Start empty; user fills them in.
  const [memRes, setMemRes] = createSignal('');
  const [memLimit, setMemLimit] = createSignal('');
  const [cpuRes, setCpuRes] = createSignal('');
  const [cpuLimit, setCpuLimit] = createSignal('');
  const [replicas, setReplicas] = createSignal('1');
  const [saving, setSaving] = createSignal(false);

  const save = async () => {
    setSaving(true);
    try {
      const res = await applicationControllerPatchResources({
        path: { id: props.app.id },
        body: {
          ...(memRes()    ? { memory_reservation: memRes() }    : {}),
          ...(memLimit()  ? { memory_limit:       memLimit() }  : {}),
          ...(cpuRes()    ? { cpu_reservation:    cpuRes() }    : {}),
          ...(cpuLimit()  ? { cpu_limit:          cpuLimit() }  : {}),
          replicas: parseInt(replicas()) || 1,
        },
      });
      if (res.data) props.onUpdated(res.data);
    } finally {
      setSaving(false);
    }
  };

  return (
    <div class="flex flex-col gap-6">

      {/* Resources */}
      <section class="bg-base-200 border border-base-300 rounded-lg p-6">
        <h2 class="text-base font-semibold mb-1">Resource Limits</h2>
        <p class="text-sm text-base-content/40 mb-5">
          Configure CPU and Memory limits for this application.
        </p>

        <div class="grid grid-cols-2 gap-4">
          <fieldset class="fieldset">
            <legend class="fieldset-legend text-base-content/70">Memory Reservation</legend>
            <input class="input input-bordered w-full" placeholder="512m"
              value={memRes()} onInput={e => setMemRes(e.currentTarget.value)} />
          </fieldset>
          <fieldset class="fieldset">
            <legend class="fieldset-legend text-base-content/70">Memory Limit</legend>
            <input class="input input-bordered w-full" placeholder="1g"
              value={memLimit()} onInput={e => setMemLimit(e.currentTarget.value)} />
          </fieldset>
          <fieldset class="fieldset">
            <legend class="fieldset-legend text-base-content/70">CPU Reservation</legend>
            <input class="input input-bordered w-full" placeholder="0.25"
              value={cpuRes()} onInput={e => setCpuRes(e.currentTarget.value)} />
          </fieldset>
          <fieldset class="fieldset">
            <legend class="fieldset-legend text-base-content/70">CPU Limit</legend>
            <input class="input input-bordered w-full" placeholder="0.5"
              value={cpuLimit()} onInput={e => setCpuLimit(e.currentTarget.value)} />
          </fieldset>
        </div>

        <fieldset class="fieldset mt-4">
          <legend class="fieldset-legend text-base-content/70">Replicas</legend>
          <input class="input input-bordered w-32" type="number" min="1"
            value={replicas()} onInput={e => setReplicas(e.currentTarget.value)} />
        </fieldset>

        <div class="flex justify-end mt-4">
          <button class="btn btn-neutral btn-sm gap-1.5" onClick={save} disabled={saving()}>
            {saving() && <span class="loading loading-spinner loading-xs" />}
            {saving() ? 'Saving…' : 'Save'}
          </button>
        </div>
      </section>

      {/* Danger Zone */}
      <section class="bg-base-200 border border-error/30 rounded-lg p-6">
        <h2 class="text-base font-semibold text-error mb-1">Danger Zone</h2>
        <p class="text-sm text-base-content/40 mb-4">These actions are irreversible.</p>
        <button class="btn btn-error btn-sm gap-1.5">
          <Trash2 class="w-3.5 h-3.5" /> Delete Application
        </button>
      </section>
    </div>
  );
}
