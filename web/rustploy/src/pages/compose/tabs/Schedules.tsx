import { Calendar } from 'lucide-solid';
import type { ComposeResponseDto } from '../../../client/types.gen';
type Props = { compose: ComposeResponseDto };
export default function SchedulesTab(props: Props) {
  return (
    <div class="bg-base-200 border border-base-300 rounded-lg p-6">
      <h2 class="text-base font-semibold mb-1">Schedules</h2>
      <p class="text-sm text-base-content/40 mb-6">Configure cron schedules for this compose.</p>
      <div class="flex flex-col items-center justify-center py-14 text-base-content/30">
        <Calendar class="w-10 h-10 mb-3" />
        <p class="text-sm">No schedules configured</p>
      </div>
    </div>
  );
}
