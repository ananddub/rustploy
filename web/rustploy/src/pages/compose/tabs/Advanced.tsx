import { Trash2 } from 'lucide-solid';
import type { ComposeResponseDto } from '../../../client/types.gen';
type Props = { compose: ComposeResponseDto };
export default function ComposeAdvancedTab(props: Props) {
  return (
    <div class="flex flex-col gap-6">
      <section class="bg-base-200 border border-base-300 rounded-lg p-6">
        <h2 class="text-base font-semibold mb-1">Advanced Settings</h2>
        <p class="text-sm text-base-content/40 mb-5">Advanced configuration for this compose.</p>
        <fieldset class="fieldset">
          <legend class="fieldset-legend text-base-content/70">Command Override</legend>
          <input class="input input-bordered w-full font-mono" placeholder="docker compose up -d" value={props.compose.command ?? ''} />
        </fieldset>
        <fieldset class="fieldset mt-4">
          <legend class="fieldset-legend text-base-content/70">Compose Path</legend>
          <input class="input input-bordered w-full font-mono" placeholder="./docker-compose.yml" value={props.compose.compose_path ?? ''} />
        </fieldset>
        <div class="flex justify-end mt-4">
          <button class="btn btn-neutral btn-sm">Save</button>
        </div>
      </section>
      <section class="bg-base-200 border border-error/30 rounded-lg p-6">
        <h2 class="text-base font-semibold text-error mb-1">Danger Zone</h2>
        <p class="text-sm text-base-content/40 mb-4">These actions are irreversible.</p>
        <button class="btn btn-error btn-sm gap-1.5"><Trash2 class="w-3.5 h-3.5" /> Delete Compose</button>
      </section>
    </div>
  );
}
