import { createSignal } from 'solid-js';
import { FileText, RefreshCw } from 'lucide-solid';

type Props = {
  /** Human-readable noun used in placeholder text, e.g. "application" or "compose" */
  serviceLabel?: string;
};

/**
 * LogsTab — shared between Application and Compose pages.
 */
export default function LogsTab(props: Props) {
  const [lines, setLines] = createSignal('100');
  const label = () => props.serviceLabel ?? 'service';

  return (
    <div class="bg-base-200 border border-base-300 rounded-lg p-6 flex flex-col gap-4">
      <div class="flex items-center justify-between">
        <div>
          <h2 class="text-base font-semibold">Logs</h2>
          <p class="text-sm text-base-content/40 mt-1">Live logs for this {label()}.</p>
        </div>
        <div class="flex items-center gap-2">
          <select
            class="select select-bordered select-sm"
            value={lines()}
            onChange={e => setLines(e.currentTarget.value)}
          >
            <option value="50">50 lines</option>
            <option value="100">100 lines</option>
            <option value="200">200 lines</option>
            <option value="500">500 lines</option>
          </select>
          <button class="btn btn-ghost btn-sm gap-1.5">
            <RefreshCw class="w-3.5 h-3.5" /> Refresh
          </button>
        </div>
      </div>

      <div class="rounded-md bg-[#0d0d0d] border border-base-300 p-4 font-mono text-xs text-base-content/60 min-h-64 flex items-center justify-center">
        <div class="flex flex-col items-center gap-2 text-base-content/30">
          <FileText class="w-8 h-8" />
          <p>No logs available. Deploy your {label()} first.</p>
        </div>
      </div>
    </div>
  );
}
