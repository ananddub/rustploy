import { Activity } from 'lucide-solid';
import type { ApplicationResponseDto } from '../../../client/types.gen';

type Props = { app: ApplicationResponseDto };

export default function MonitoringTab(props: Props) {
  return (
    <div class="bg-base-200 border border-base-300 rounded-lg p-6">
      <h2 class="text-base font-semibold mb-1">Monitoring</h2>
      <p class="text-sm text-base-content/40 mb-6">CPU, Memory and Network usage for this application.</p>

      <div class="grid grid-cols-3 gap-4 mb-6">
        {[{ label: 'CPU Usage', value: '—' }, { label: 'Memory', value: '—' }, { label: 'Network', value: '—' }].map(s => (
          <div class="bg-base-300 rounded-lg p-4 text-center">
            <p class="text-xs text-base-content/40 mb-1">{s.label}</p>
            <p class="text-2xl font-semibold">{s.value}</p>
          </div>
        ))}
      </div>

      <div class="flex flex-col items-center justify-center py-10 text-base-content/30">
        <Activity class="w-10 h-10 mb-3" />
        <p class="text-sm">No monitoring data available.</p>
        <p class="text-xs mt-1">Deploy your application to start collecting metrics.</p>
      </div>
    </div>
  );
}
