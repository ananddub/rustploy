import { createSignal, Show } from 'solid-js';
import { Plus, Globe, Trash2 } from 'lucide-solid';
import type { ApplicationResponseDto } from '../../../client/types.gen';

type Props = { app: ApplicationResponseDto };

export default function DomainsTab(props: Props) {
  const [host, setHost] = createSignal('');
  const [port, setPort] = createSignal('3000');
  const [https, setHttps] = createSignal(false);
  const [path, setPath] = createSignal('/');
  const [saving, setSaving] = createSignal(false);

  return (
    <div class="flex flex-col gap-6">
      {/* Add Domain */}
      <section class="bg-base-200 border border-base-300 rounded-lg p-6">
        <h2 class="text-base font-semibold mb-1">Add Domain</h2>
        <p class="text-sm text-base-content/40 mb-5">Configure a custom domain for this application.</p>

        <div class="flex flex-col gap-4">
          <div class="grid grid-cols-2 gap-4">
            <fieldset class="fieldset">
              <legend class="fieldset-legend text-base-content/70">Host <span class="text-error">*</span></legend>
              <input class="input input-bordered w-full" placeholder="example.com" value={host()} onInput={e => setHost(e.currentTarget.value)} />
            </fieldset>
            <fieldset class="fieldset">
              <legend class="fieldset-legend text-base-content/70">Port</legend>
              <input class="input input-bordered w-full" placeholder="3000" value={port()} onInput={e => setPort(e.currentTarget.value)} />
            </fieldset>
          </div>

          <fieldset class="fieldset">
            <legend class="fieldset-legend text-base-content/70">Path</legend>
            <input class="input input-bordered w-full" placeholder="/" value={path()} onInput={e => setPath(e.currentTarget.value)} />
          </fieldset>

          <div class="flex items-center gap-3">
            <input id="https" type="checkbox" class="toggle toggle-sm" checked={https()} onChange={e => setHttps(e.currentTarget.checked)} />
            <label for="https" class="text-sm cursor-pointer">HTTPS / SSL</label>
          </div>

          <div class="flex justify-end">
            <button class="btn btn-neutral btn-sm gap-1.5" disabled={saving()}>
              {saving() && <span class="loading loading-spinner loading-xs" />}
              <Plus class="w-4 h-4" /> Add Domain
            </button>
          </div>
        </div>
      </section>

      {/* Domains list */}
      <section class="bg-base-200 border border-base-300 rounded-lg p-6">
        <h2 class="text-base font-semibold mb-4">Domains</h2>
        <div class="flex flex-col items-center justify-center py-10 text-base-content/30">
          <Globe class="w-10 h-10 mb-3" />
          <p class="text-sm">No domains configured</p>
        </div>
      </section>
    </div>
  );
}
