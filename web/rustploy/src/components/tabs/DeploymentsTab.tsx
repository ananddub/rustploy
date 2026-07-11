import { createSignal, For, Show } from 'solid-js';
import { Lightning, ArrowClockwise, Clock } from 'phosphor-solid';
import { formatDate, formatDuration, deployStatusBadge } from '../../lib/utils';

type Props = {
  serviceLabel?: string;
  onDeploy: () => Promise<void>;
  onRedeploy: () => Promise<void>;
};

type DeploymentRecord = {
  id: number;
  title: string;
  description?: string;
  status: 'RUNNING' | 'SUCCESS' | 'FAILED' | 'CANCELLED';
  log_path: string;
  started_at?: number;
  finished_at?: number;
  created_at: number;
};

export default function DeploymentsTab(props: Props) {
  const [deploying, setDeploying] = createSignal(false);
  const [redeploying, setRedeploying] = createSignal(false);
  const label = () => props.serviceLabel ?? 'service';
  const deployments: DeploymentRecord[] = [];

  const handleDeploy = async () => {
    setDeploying(true);
    try { await props.onDeploy(); } finally { setDeploying(false); }
  };

  const handleRedeploy = async () => {
    setRedeploying(true);
    try { await props.onRedeploy(); } finally { setRedeploying(false); }
  };

  return (
    <div class="flex flex-col gap-6">
      <section class="bg-base-200 border border-base-300 rounded-lg p-6">
        <div class="flex items-center justify-between">
          <div>
            <h2 class="text-base font-semibold">Deployments</h2>
            <p class="text-sm text-base-content/40 mt-1">History of all deployments for this {label()}.</p>
          </div>
          <div class="flex items-center gap-2">
            <button class="btn btn-ghost btn-sm gap-1.5 hover:bg-base-300 text-base-content/50 hover:text-base-content" onClick={handleRedeploy} disabled={redeploying()}>
              {redeploying() ? <span class="loading loading-spinner loading-xs" /> : <ArrowClockwise size={14} />}
              Redeploy
            </button>
            <button class="btn btn-neutral btn-sm gap-1.5" onClick={handleDeploy} disabled={deploying()}>
              {deploying() ? <span class="loading loading-spinner loading-xs" /> : <Lightning size={14} weight="fill" />}
              Deploy
            </button>
          </div>
        </div>
      </section>

      <section class="bg-base-200 border border-base-300 rounded-lg overflow-hidden">
        <Show when={deployments.length === 0}>
          <div class="flex flex-col items-center justify-center py-16 text-base-content/25">
            <Lightning size={40} class="mb-3 opacity-40" />
            <p class="text-sm">No deployments yet</p>
            <p class="text-xs mt-1 opacity-70">Deploy your {label()} to see history here.</p>
          </div>
        </Show>
        <Show when={deployments.length > 0}>
          <div class="grid grid-cols-[1fr_120px_100px_110px_80px] gap-4 px-5 py-2.5 border-b border-base-300 text-xs text-base-content/35 font-medium uppercase tracking-wide">
            <span>Deployment</span><span>Status</span><span>Duration</span><span>Started</span><span></span>
          </div>
          <For each={deployments}>
            {(d) => (
              <div class="grid grid-cols-[1fr_120px_100px_110px_80px] gap-4 items-center px-5 py-3 border-b border-base-300 last:border-0 hover:bg-base-300/30 transition-colors">
                <div class="min-w-0">
                  <p class="text-sm font-medium truncate">{d.title}</p>
                  <Show when={d.description}>
                    <p class="text-xs text-base-content/40 truncate mt-0.5">{d.description}</p>
                  </Show>
                </div>
                <div>{deployStatusBadge(d.status)}</div>
                <div class="text-xs text-base-content/50 flex items-center gap-1">
                  <Clock size={12} />{formatDuration(d.started_at, d.finished_at)}
                </div>
                <div class="text-xs text-base-content/40">{formatDate(d.created_at)}</div>
                <div class="flex justify-end">
                  <button class="btn btn-ghost btn-xs text-base-content/40 hover:text-base-content hover:bg-base-300">Logs</button>
                </div>
              </div>
            )}
          </For>
        </Show>
      </section>
    </div>
  );
}
