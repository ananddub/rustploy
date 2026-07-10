import { createResource, createSignal, For, Show } from 'solid-js';
import { Zap, RefreshCw, CheckCircle, XCircle, Clock, Loader, GitBranch } from 'lucide-solid';
import type { ApplicationResponseDto } from '../../../client/types.gen';
import {
  applicationControllerDeploy,
  applicationControllerRedeploy,
} from '../../../client/sdk.gen';

type Props = { app: ApplicationResponseDto };

// Local type matching the Deployment db model fields the backend would return
type DeploymentRecord = {
  id: number;
  title: string;
  description?: string;
  status: 'RUNNING' | 'SUCCESS' | 'FAILED' | 'CANCELLED';
  log_path: string;
  error_message?: string;
  is_preview_deployment: number;
  started_at?: number;
  finished_at?: number;
  application_id?: number;
  created_at: number;
};

function statusBadge(status: string) {
  switch (status) {
    case 'SUCCESS':
      return (
        <span class="inline-flex items-center gap-1 text-xs font-medium text-success">
          <CheckCircle class="w-3.5 h-3.5" /> Success
        </span>
      );
    case 'FAILED':
      return (
        <span class="inline-flex items-center gap-1 text-xs font-medium text-error">
          <XCircle class="w-3.5 h-3.5" /> Failed
        </span>
      );
    case 'RUNNING':
      return (
        <span class="inline-flex items-center gap-1 text-xs font-medium text-warning">
          <Loader class="w-3.5 h-3.5 animate-spin" /> Running
        </span>
      );
    case 'CANCELLED':
      return (
        <span class="inline-flex items-center gap-1 text-xs font-medium text-base-content/40">
          <XCircle class="w-3.5 h-3.5" /> Cancelled
        </span>
      );
    default:
      return <span class="text-xs text-base-content/40">{status}</span>;
  }
}

function formatDuration(started?: number, finished?: number) {
  if (!started) return '—';
  const end = finished ?? Math.floor(Date.now() / 1000);
  const secs = end - started;
  if (secs < 60) return `${secs}s`;
  const m = Math.floor(secs / 60);
  const s = secs % 60;
  return `${m}m ${s}s`;
}

function formatDate(ts: number) {
  return new Date(ts * 1000).toLocaleString(undefined, {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  });
}

export default function DeploymentsTab(props: Props) {
  const [deploying, setDeploying] = createSignal(false);
  const [redeploying, setRedeploying] = createSignal(false);

  // The backend has no list-deployments endpoint exposed yet.
  // Once it's added, replace this stub with a real createResource call.
  const deployments: DeploymentRecord[] = [];

  const triggerDeploy = async () => {
    setDeploying(true);
    try {
      await applicationControllerDeploy({ path: { id: props.app.id } });
    } finally {
      setDeploying(false);
    }
  };

  const triggerRedeploy = async () => {
    setRedeploying(true);
    try {
      await applicationControllerRedeploy({ path: { id: props.app.id } });
    } finally {
      setRedeploying(false);
    }
  };

  return (
    <div class="flex flex-col gap-6">
      {/* Header actions */}
      <section class="bg-base-200 border border-base-300 rounded-lg p-6">
        <div class="flex items-center justify-between">
          <div>
            <h2 class="text-base font-semibold">Deployments</h2>
            <p class="text-sm text-base-content/40 mt-1">
              History of all deployments for this application.
            </p>
          </div>
          <div class="flex items-center gap-2">
            <button
              class="btn btn-ghost btn-sm gap-1.5"
              onClick={triggerRedeploy}
              disabled={redeploying()}
            >
              {redeploying()
                ? <span class="loading loading-spinner loading-xs" />
                : <RefreshCw class="w-3.5 h-3.5" />}
              Redeploy
            </button>
            <button
              class="btn btn-neutral btn-sm gap-1.5"
              onClick={triggerDeploy}
              disabled={deploying()}
            >
              {deploying()
                ? <span class="loading loading-spinner loading-xs" />
                : <Zap class="w-3.5 h-3.5" />}
              Deploy
            </button>
          </div>
        </div>
      </section>

      {/* Deployments list */}
      <section class="bg-base-200 border border-base-300 rounded-lg overflow-hidden">
        <Show when={deployments.length === 0}>
          <div class="flex flex-col items-center justify-center py-16 text-base-content/30">
            <Zap class="w-10 h-10 mb-3" />
            <p class="text-sm">No deployments yet</p>
            <p class="text-xs mt-1">Deploy your application to see history here.</p>
          </div>
        </Show>

        <Show when={deployments.length > 0}>
          {/* table header */}
          <div class="grid grid-cols-[1fr_120px_100px_110px_80px] gap-4 px-5 py-2.5 border-b border-base-300 text-xs text-base-content/40 font-medium uppercase tracking-wide">
            <span>Deployment</span>
            <span>Status</span>
            <span>Duration</span>
            <span>Started</span>
            <span></span>
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
                <div>{statusBadge(d.status)}</div>
                <div class="text-xs text-base-content/60 flex items-center gap-1">
                  <Clock class="w-3 h-3" />
                  {formatDuration(d.started_at, d.finished_at)}
                </div>
                <div class="text-xs text-base-content/40">{formatDate(d.created_at)}</div>
                <div class="flex justify-end">
                  <button class="btn btn-ghost btn-xs text-base-content/40 hover:text-base-content">
                    Logs
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
