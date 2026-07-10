import { createSignal, For, Show } from 'solid-js';
import { Plus, Calendar, Trash2, Clock, Play } from 'lucide-solid';
import type { ApplicationResponseDto } from '../../../client/types.gen';

type Props = { app: ApplicationResponseDto };

// Cron schedule entries — backed by the `schedules` table.
// The API endpoint for schedules is not yet exposed in the generated client,
// so this tab renders the full UI ready to wire up when the endpoint lands.
type Schedule = {
  id: number;
  name: string;
  cron: string;
  enabled: boolean;
  last_run?: number;
  next_run?: number;
};

const CRON_PRESETS = [
  { label: 'Every hour', value: '0 * * * *' },
  { label: 'Every day at midnight', value: '0 0 * * *' },
  { label: 'Every Monday at 8 am', value: '0 8 * * 1' },
  { label: 'Every 30 minutes', value: '*/30 * * * *' },
];

function formatTs(ts?: number) {
  if (!ts) return '—';
  return new Date(ts * 1000).toLocaleString(undefined, {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  });
}

export default function SchedulesTab(props: Props) {
  const [name, setName] = createSignal('');
  const [cron, setCron] = createSignal('0 0 * * *');
  const [saving, setSaving] = createSignal(false);

  // Stub — replace with createResource calling the schedules API when available
  const schedules: Schedule[] = [];

  const addSchedule = async () => {
    if (!name().trim() || !cron().trim()) return;
    setSaving(true);
    try {
      // TODO: POST /schedules { name, cron, application_id: props.app.id }
      await new Promise(r => setTimeout(r, 300));
      setName('');
      setCron('0 0 * * *');
    } finally {
      setSaving(false);
    }
  };

  return (
    <div class="flex flex-col gap-6">
      {/* Add schedule */}
      <section class="bg-base-200 border border-base-300 rounded-lg p-6">
        <h2 class="text-base font-semibold mb-1">Add Schedule</h2>
        <p class="text-sm text-base-content/40 mb-5">
          Trigger a deployment automatically on a cron schedule.
        </p>

        <div class="flex flex-col gap-4">
          <fieldset class="fieldset">
            <legend class="fieldset-legend text-base-content/70">
              Schedule Name <span class="text-error">*</span>
            </legend>
            <input
              class="input input-bordered w-full"
              placeholder="Nightly deploy"
              value={name()}
              onInput={e => setName(e.currentTarget.value)}
            />
          </fieldset>

          <div>
            <label class="block text-sm mb-1.5 text-base-content/70">
              Cron Expression <span class="text-error">*</span>
            </label>
            <div class="flex gap-2">
              <input
                class="input input-bordered flex-1 font-mono"
                placeholder="0 0 * * *"
                value={cron()}
                onInput={e => setCron(e.currentTarget.value)}
              />
              <select
                class="select select-bordered"
                onChange={e => setCron(e.currentTarget.value)}
              >
                <option value="">Presets…</option>
                {CRON_PRESETS.map(p => (
                  <option value={p.value}>{p.label}</option>
                ))}
              </select>
            </div>
            <p class="text-xs text-base-content/40 mt-1 font-mono">
              {cron() || '—'}
            </p>
          </div>

          <div class="flex justify-end">
            <button
              class="btn btn-neutral btn-sm gap-1.5"
              onClick={addSchedule}
              disabled={saving() || !name().trim() || !cron().trim()}
            >
              {saving()
                ? <span class="loading loading-spinner loading-xs" />
                : <Plus class="w-4 h-4" />}
              Add Schedule
            </button>
          </div>
        </div>
      </section>

      {/* Schedules list */}
      <section class="bg-base-200 border border-base-300 rounded-lg overflow-hidden">
        <div class="px-5 py-3.5 border-b border-base-300">
          <h2 class="text-sm font-semibold">Active Schedules</h2>
        </div>

        <Show when={schedules.length === 0}>
          <div class="flex flex-col items-center justify-center py-14 text-base-content/30">
            <Calendar class="w-10 h-10 mb-3" />
            <p class="text-sm">No schedules configured</p>
            <p class="text-xs mt-1">Add a cron schedule above to automate deploys.</p>
          </div>
        </Show>

        <Show when={schedules.length > 0}>
          <div class="grid grid-cols-[1fr_160px_140px_140px_80px] gap-4 px-5 py-2 border-b border-base-300 text-xs text-base-content/40 font-medium uppercase tracking-wide">
            <span>Name</span>
            <span>Cron</span>
            <span>Last Run</span>
            <span>Next Run</span>
            <span></span>
          </div>

          <For each={schedules}>
            {(s) => (
              <div class="grid grid-cols-[1fr_160px_140px_140px_80px] gap-4 items-center px-5 py-3 border-b border-base-300 last:border-0 hover:bg-base-300/40 transition-colors">
                <div class="flex items-center gap-2 min-w-0">
                  <input
                    type="checkbox"
                    class="toggle toggle-xs"
                    checked={s.enabled}
                  />
                  <span class="text-sm font-medium truncate">{s.name}</span>
                </div>
                <span class="text-xs font-mono text-base-content/60">{s.cron}</span>
                <div class="text-xs text-base-content/40 flex items-center gap-1">
                  <Clock class="w-3 h-3" />
                  {formatTs(s.last_run)}
                </div>
                <div class="text-xs text-base-content/40 flex items-center gap-1">
                  <Play class="w-3 h-3" />
                  {formatTs(s.next_run)}
                </div>
                <div class="flex justify-end">
                  <button class="btn btn-ghost btn-xs text-base-content/40 hover:text-error">
                    <Trash2 class="w-3.5 h-3.5" />
                  </button>
                </div>
              </div>
            )}
          </For>
        </Show>
      </section>
    </div>
  );
}
