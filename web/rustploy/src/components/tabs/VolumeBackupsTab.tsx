import { createSignal, For, Show } from 'solid-js';
import { Plus, HardDrive, Trash2, Download, Clock, RefreshCw } from 'lucide-solid';
import { formatDate, backupStatusBadge } from '../../lib/utils';

type Props = {
  serviceLabel?: string;
  serviceId: number;
};

type VolumeBackup = {
  id: number;
  name: string;
  size?: string;
  status: 'RUNNING' | 'SUCCESS' | 'FAILED';
  created_at: number;
  finished_at?: number;
};

/**
 * VolumeBackupsTab — shared between Application and Compose pages.
 */
export default function VolumeBackupsTab(props: Props) {
  const [backupName, setBackupName] = createSignal('');
  const [creating, setCreating] = createSignal(false);
  const label = () => props.serviceLabel ?? 'service';

  // TODO: replace with createResource once GET /volume-backups?service_id=X is available
  const backups: VolumeBackup[] = [];

  const createBackup = async () => {
    if (!backupName().trim()) return;
    setCreating(true);
    try {
      await new Promise(r => setTimeout(r, 400));
      setBackupName('');
    } finally { setCreating(false); }
  };

  return (
    <div class="flex flex-col gap-6">

      <section class="bg-base-200 border border-base-300 rounded-lg p-6">
        <h2 class="text-base font-semibold mb-1">Volume Backups</h2>
        <p class="text-sm text-base-content/40 mb-5">
          Create and manage backups of volumes attached to this {label()}.
        </p>
        <div class="flex gap-3 items-end">
          <fieldset class="fieldset flex-1">
            <legend class="fieldset-legend text-base-content/70">Backup Name</legend>
            <input
              class="input input-bordered w-full"
              placeholder="backup-2026-07-10"
              value={backupName()}
              onInput={e => setBackupName(e.currentTarget.value)}
            />
          </fieldset>
          <button
            class="btn btn-neutral btn-sm gap-1.5 mb-0.5"
            onClick={createBackup}
            disabled={creating() || !backupName().trim()}
          >
            {creating() ? <span class="loading loading-spinner loading-xs" /> : <Plus class="w-4 h-4" />}
            Create Backup
          </button>
        </div>
      </section>

      <section class="bg-base-200 border border-base-300 rounded-lg overflow-hidden">
        <div class="flex items-center justify-between px-5 py-3.5 border-b border-base-300">
          <h2 class="text-sm font-semibold">Backup History</h2>
          <button class="btn btn-ghost btn-xs gap-1 text-base-content/40">
            <RefreshCw class="w-3 h-3" /> Refresh
          </button>
        </div>

        <Show when={backups.length === 0}>
          <div class="flex flex-col items-center justify-center py-14 text-base-content/30">
            <HardDrive class="w-10 h-10 mb-3" />
            <p class="text-sm">No backups yet</p>
            <p class="text-xs mt-1">Create a backup above to protect your volume data.</p>
          </div>
        </Show>

        <Show when={backups.length > 0}>
          <div class="grid grid-cols-[1fr_100px_100px_140px_80px] gap-4 px-5 py-2 border-b border-base-300 text-xs text-base-content/40 font-medium uppercase tracking-wide">
            <span>Name</span><span>Size</span><span>Status</span><span>Created</span><span></span>
          </div>
          <For each={backups}>
            {(b) => (
              <div class="grid grid-cols-[1fr_100px_100px_140px_80px] gap-4 items-center px-5 py-3 border-b border-base-300 last:border-0 hover:bg-base-300/40 transition-colors">
                <div class="flex items-center gap-2 min-w-0">
                  <HardDrive class="w-3.5 h-3.5 text-base-content/40 shrink-0" />
                  <span class="text-sm font-medium truncate">{b.name}</span>
                </div>
                <span class="text-xs text-base-content/60">{b.size ?? '—'}</span>
                <div>{backupStatusBadge(b.status)}</div>
                <div class="text-xs text-base-content/40 flex items-center gap-1">
                  <Clock class="w-3 h-3" />{formatDate(b.created_at)}
                </div>
                <div class="flex justify-end gap-1">
                  <button class="btn btn-ghost btn-xs text-base-content/40 hover:text-base-content" title="Download">
                    <Download class="w-3.5 h-3.5" />
                  </button>
                  <button class="btn btn-ghost btn-xs text-base-content/40 hover:text-error" title="Delete">
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
