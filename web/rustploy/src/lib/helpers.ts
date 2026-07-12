// ─── Date / time ─────────────────────────────────────────────────────────────

export function formatDate(ts: number): string {
	return new Date(ts * 1000).toLocaleString(undefined, {
		month: 'short',
		day: 'numeric',
		hour: '2-digit',
		minute: '2-digit'
	});
}

export function formatDuration(started?: number, finished?: number): string {
	if (!started) return '—';
	const end = finished ?? Math.floor(Date.now() / 1000);
	const secs = end - started;
	if (secs < 60) return `${secs}s`;
	return `${Math.floor(secs / 60)}m ${secs % 60}s`;
}

// ─── Status helpers ───────────────────────────────────────────────────────────

export type DeployStatus = 'RUNNING' | 'SUCCESS' | 'FAILED' | 'CANCELLED' | string;

export function deployStatusColor(status: DeployStatus): string {
	const map: Record<string, string> = {
		SUCCESS: 'text-green-600 dark:text-green-400',
		FAILED: 'text-red-500',
		RUNNING: 'text-yellow-500',
		CANCELLED: 'text-muted-foreground'
	};
	return map[status] ?? 'text-muted-foreground';
}

export function deployStatusLabel(status: DeployStatus): string {
	const map: Record<string, string> = {
		SUCCESS: 'Success',
		FAILED: 'Failed',
		RUNNING: 'Running',
		CANCELLED: 'Cancelled'
	};
	return map[status] ?? status;
}

export function appStatusDot(status: string): string {
	switch (status?.toLowerCase()) {
		case 'running':
			return 'bg-green-500';
		case 'stopped':
		case 'exited':
		case 'error':
			return 'bg-red-500';
		case 'idle':
			return 'bg-muted-foreground/40';
		default:
			return 'bg-yellow-500';
	}
}
