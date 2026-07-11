import { createSignal, For, Show } from 'solid-js';
import { Zap, RefreshCw, Clock } from 'lucide-solid';
import { formatDate, formatDuration, deployStatusBadge } from '../../lib/utils';

type Props = {
  serviceLabel?: string;
  onDeploy: () => Promise<void>;
  onRedeploy: () => Promise<void>;
};

// Matches the deployment table schema.
// Wire createResource once GET /deployments?application_id=X (or compose_id) is available.
type DeploymentRecord = {
  id: number;
  title: string;
  description?: string;
  status: 'RUNNING' | 'SUCCESS' | 'FAILED' | 'CANCELLED';
  log_path: string;
  error_message?: string;
  started_at?: number;
  finished_at?: number;
  created_at: number;
};

/**
 * DeploymentsTab — shared between Application and Compose pages.
 * Parent provides onDeploy / onRedeploy so the API call stays in the page layer.
 */
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
            <p class="text-sm text-base-content/40 mt-1">
              History of all deployments for this {label()}.
            </p>
          </div>
          <div class="flex items-center gap-2">
            <button class="btn btn-ghost btn-sm gap-1.5" onClick={handleRedeploy} disabled={redeploying()}>
              {redeploying() ? <span class="loading loading-spinner loading-xs" /> : <RefreshCw class="w-3.5 h-3.5" />}
              Redeploy
            </button>
            <button class="btn btn-neutral btn-sm gap-1.5" onClick={handleDeploy} disabled={deploying()}>
              {deploying() ? <span class="loading loading-spinner loading-xs" /> : <Zap class="w-3.5 h-3.5" />}
              Deploy
            </button>
          </div>
        </div>
      </section>

      <section class="bg-base-200 border border-base-300 rounded-lg overflow-hidden">
        <Show when={deployments.length === 0}>
          <div class="flex flex-col items-center justify-center py-16 text-base-content/30">
            <Zap class="w-10 h-10 mb-3" />
            <p class="text-sm">No deployments yet</p>
            <p class="text-xs mt-1">Deploy your {label()} to see history here.</p>
          </div>
        </Show>

        <Show when={deployments.length > 0}>
          <div class="grid grid-cols-[1fr_120px_100px_110px_80px] gap-4 px-5 py-2.5 border-b border-base-300 text-xs text-base-content/40 font-medium uppercase tracking-wide">
            <span>Deployment</span><span>Status</span><span>Duration</span><span>Started</span><span></span>
          </div>
          <For each={deployments}>
            {(d) => (
              <div class="grid grid-cols-[1fr_120px_100px_110px_80px] gap-4 items-center px-5 py-3 border-b border-base-300 last:border-0 hover:bg-base-300/40 transition-colors">
                <div class="min-w-0">
                  <p class="text-sm font-medium truncate">{d.title}</p>
                  <Show when={d.description}>
                    <p class="text-xs text-base-content/40 truncate mt-0.5">{d.description}</p>
                  </Show>
                </div>
                <div>{deployStatusBadge(d.status)}</div>
                <div class="text-xs text-base-content/60 flex items-center gap-1">
                  <Clock class="w-3 h-3" />{formatDuration(d.started_at, d.finished_at)}
                </div>
                <div class="text-xs text-base-content/40">{formatDate(d.created_at)}</div>
                <div class="flex justify-end">
                  <button class="btn btn-ghost btn-xs text-base-content/40 hover:text-base-content">Logs</button>
                </div>
              </div>
            )}
          </For>
        </Show>
      </section>
    </div>
  );
}
