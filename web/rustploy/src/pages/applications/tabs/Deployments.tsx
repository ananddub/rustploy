import type { ApplicationResponseDto } from '../../../client/types.gen';
import { Zap } from 'lucide-solid';

type Props = { app: ApplicationResponseDto };

export default function DeploymentsTab(props: Props) {
  return (
    <div class="bg-base-200 border border-base-300 rounded-lg p-6">
      <div class="flex items-center justify-between mb-4">
        <div>
          <h2 class="text-base font-semibold">Deployments</h2>
          <p class="text-sm text-base-content/40 mt-1">History of all deployments for this application.</p>
        </div>
      </div>
      <div class="flex flex-col items-center justify-center py-16 text-base-content/30">
        <Zap class="w-10 h-10 mb-3" />
        <p class="text-sm">No deployments yet</p>
        <p class="text-xs mt-1">Deploy your application to see history here.</p>
      </div>
    </div>
  );
}
