import { createSignal, For, Show } from 'solid-js';
import { Layers, ChevronDown, Check, Plus } from 'lucide-solid';
import type { EnvironmentResponseDto } from '../../client/types.gen';

type Props = {
  envs: EnvironmentResponseDto[];
  selectedId: number | null;
  onSelect: (id: number) => void;
  onCreateNew: () => void;
};

export default function EnvDropdown(props: Props) {
  const [open, setOpen] = createSignal(false);
  const selected = () => props.envs.find(e => e.id === props.selectedId);

  return (
    <div class="relative">
      <button
        class="flex items-center gap-1.5 font-medium text-base-content hover:text-primary transition-colors"
        onClick={() => setOpen(v => !v)}
      >
        <Layers class="w-3.5 h-3.5" />
        {selected()?.name ?? '...'}
        <ChevronDown class="w-3.5 h-3.5 text-base-content/40" />
      </button>

      <Show when={open()}>
        <div class="fixed inset-0 z-10" onClick={() => setOpen(false)} />
        <div class="absolute top-full left-0 mt-1 z-20 bg-base-200 border border-base-300 rounded-lg shadow-xl min-w-[180px] overflow-hidden">
          <div class="px-3 py-2 text-[10px] uppercase tracking-widest text-base-content/30 border-b border-base-300">
            Environments
          </div>

          <For each={props.envs}>
            {(env) => (
              <button
                class="w-full flex items-center gap-2 px-3 py-2 text-sm hover:bg-base-300 transition-colors text-left"
                onClick={() => { props.onSelect(env.id); setOpen(false); }}
              >
                <Show when={props.selectedId === env.id} fallback={<span class="w-3.5" />}>
                  <Check class="w-3.5 h-3.5 text-primary shrink-0" />
                </Show>
                <span class="truncate flex-1">{env.name}</span>
                <Show when={env.is_default}>
                  <span class="badge badge-xs badge-neutral">default</span>
                </Show>
              </button>
            )}
          </For>

          <div class="border-t border-base-300">
            <button
              class="w-full flex items-center gap-2 px-3 py-2 text-sm text-base-content/60 hover:bg-base-300 hover:text-base-content transition-colors"
              onClick={() => { setOpen(false); props.onCreateNew(); }}
            >
              <Plus class="w-3.5 h-3.5" />
              New environment
            </button>
          </div>
        </div>
      </Show>
    </div>
  );
}
