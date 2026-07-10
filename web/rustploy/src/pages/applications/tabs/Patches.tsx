import { Wrench } from 'lucide-solid';
import type { ApplicationResponseDto } from '../../../client/types.gen';

type Props = { app: ApplicationResponseDto };

export default function PatchesTab(props: Props) {
  return (
    <div class="bg-base-200 border border-base-300 rounded-lg p-6">
      <h2 class="text-base font-semibold mb-1">Patches</h2>
      <p class="text-sm text-base-content/40 mb-6">File patches applied during build.</p>
      <div class="flex flex-col items-center justify-center py-12 text-base-content/30">
        <Wrench class="w-10 h-10 mb-3" />
        <p class="text-sm">No patches configured</p>
      </div>
    </div>
  );
}
