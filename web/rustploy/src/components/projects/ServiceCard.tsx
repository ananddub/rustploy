import { Box } from 'lucide-solid';
import type { ApplicationResponseDto } from '../../client/types.gen';

type Props = {
  app: ApplicationResponseDto;
};

const statusColor = (status: string) => {
  switch (status?.toLowerCase()) {
    case 'running': return 'bg-success';
    case 'stopped':
    case 'exited':  return 'bg-error';
    case 'idle':    return 'bg-base-content/30';
    default:        return 'bg-warning';
  }
};

const formatDate = (ts: number) =>
  new Date(ts * 1000).toLocaleDateString('en-IN', {
    day: '2-digit', month: 'short', year: 'numeric',
  });

export default function ServiceCard(props: Props) {
  return (
    <div class="w-56 bg-base-200 border border-base-300 rounded-lg p-4 flex flex-col gap-3 hover:border-base-content/20 transition-colors cursor-pointer group shrink-0">
      <div class="flex items-center gap-3">
        <div class="w-9 h-9 rounded-lg bg-primary/10 flex items-center justify-center shrink-0 relative">
          <Box class="w-4 h-4 text-primary" />
          <span
            class={`absolute -top-0.5 -right-0.5 w-2.5 h-2.5 rounded-full border-2 border-base-200 ${statusColor(props.app.app_status)}`}
          />
        </div>
        <div class="min-w-0">
          <p class="font-medium text-sm truncate group-hover:text-primary transition-colors">
            {props.app.app_name}
          </p>
          <p class="text-xs text-base-content/40">{props.app.build_type}</p>
        </div>
      </div>

      <div class="flex items-center justify-between border-t border-base-300 pt-2">
        <span class="flex items-center gap-1.5 text-xs text-base-content/50">
          <span class={`w-1.5 h-1.5 rounded-full ${statusColor(props.app.app_status)}`} />
          {props.app.app_status ?? 'unknown'}
        </span>
        <span class="text-xs text-base-content/30">{formatDate(props.app.created_at)}</span>
      </div>
    </div>
  );
}
