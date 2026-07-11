import type { JSX } from 'solid-js';

// ─── Date / time ──────────────────────────────────────────────────────────────

/** Format a unix-seconds timestamp to a short locale string. */
export function formatDate(ts: number): string {
  return new Date(ts * 1000).toLocaleString(undefined, {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  });
}

/** Format elapsed seconds between two unix-seconds timestamps. */
export function formatDuration(started?: number, finished?: number): string {
  if (!started) return '—';
  const end = finished ?? Math.floor(Date.now() / 1000);
  const secs = end - started;
  if (secs < 60) return `${secs}s`;
  return `${Math.floor(secs / 60)}m ${secs % 60}s`;
}

// ─── Deployment status badge ──────────────────────────────────────────────────

type BadgeStatus = 'RUNNING' | 'SUCCESS' | 'FAILED' | 'CANCELLED' | string;

/** Inline coloured status label used across Deployments and VolumeBackups tabs. */
export function deployStatusBadge(status: BadgeStatus): JSX.Element {
  const map: Record<string, string> = {
    SUCCESS:   'text-success',
    FAILED:    'text-error',
    RUNNING:   'text-warning',
    CANCELLED: 'text-base-content/40',
  };
  const label: Record<string, string> = {
    SUCCESS: 'Success', FAILED: 'Failed', RUNNING: 'Running', CANCELLED: 'Cancelled',
  };
  const cls = map[status] ?? 'text-base-content/40';
  return (
    <span class={`inline-flex items-center gap-1 text-xs font-medium ${cls}`}>
      {label[status] ?? status}
    </span>
  );
}

/** Badge variant used in VolumeBackups (pill style). */
export function backupStatusBadge(status: BadgeStatus): JSX.Element {
  const map: Record<string, string> = {
    SUCCESS: 'bg-success/20 text-success border-success/30',
    FAILED:  'bg-error/20 text-error border-error/30',
    RUNNING: 'bg-warning/20 text-warning border-warning/30',
  };
  return (
    <span class={`badge badge-xs ${map[status] ?? ''}`}>
      {status.charAt(0) + status.slice(1).toLowerCase()}
    </span>
  );
}
