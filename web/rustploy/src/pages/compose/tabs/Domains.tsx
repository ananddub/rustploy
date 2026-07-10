import { createSignal } from 'solid-js';
import { Globe, Plus } from 'lucide-solid';
import type { ComposeResponseDto } from '../../../client/types.gen';
type Props = { compose: ComposeResponseDto };
export default function ComposeDomainsTab(props: Props) {
  const [host, setHost] = createSignal('');
  const [port, setPort] = createSignal('80');
  const [https, setHttps] = createSignal(false);
  return (
    <div class="flex flex-col gap-6">
      <section class="bg-base-200 border border-base-300 rounded-lg p-6">
        <h2 class="text-base font-semibold mb-1">Add Domain</h2>
        <p class="text-sm text-base-content/40 mb-5">Configure a custom domain for this compose service.</p>
        <div class="flex flex-col gap-4">
          <div class="grid grid-cols-2 gap-4">
            <fieldset class="fieldset">
              <legend class="fieldset-legend text-base-content/70">Host <span class="text-error">*</span></legend>
              <input class="input input-bordered w-full" placeholder="example.com" value={host()} onInput={e => setHost(e.currentTarget.value)} />
            </fieldset>
            <fieldset class="fieldset">
              <legend class="fieldset-legend text-base-content/70">Port</legend>
              <input class="input input-bordered w-full" placeholder="80" value={port()} onInput={e => setPort(e.currentTarget.value)} />
            </fieldset>
          </div>
          <div class="flex items-center gap-3">
            <input id="https-c" type="checkbox" class="toggle toggle-sm" checked={https()} onChange={e => setHttps(e.currentTarget.checked)} />
            <label for="https-c" class="text-sm cursor-pointer">HTTPS / SSL</label>
          </div>
          <div class="flex justify-end">
            <button class="btn btn-neutral btn-sm gap-1.5"><Plus class="w-4 h-4" /> Add Domain</button>
          </div>
        </div>
      </section>
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
