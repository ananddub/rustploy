import { Box } from 'lucide-solid';
import type { ComposeResponseDto } from '../../../client/types.gen';
type Props = { compose: ComposeResponseDto };
export default function ContainersTab(props: Props) {
  return (
    <div class="bg-base-200 border border-base-300 rounded-lg p-6">
      <h2 class="text-base font-semibold mb-1">Containers</h2>
      <p class="text-sm text-base-content/40 mb-6">Running containers for this compose stack.</p>
      <div class="flex flex-col items-center justify-center py-14 text-base-content/30">
        <Box class="w-10 h-10 mb-3" />
        <p class="text-sm">No containers running</p>
        <p class="text-xs mt-1">Deploy your compose to see containers here.</p>
      </div>
    </div>
  );
}
