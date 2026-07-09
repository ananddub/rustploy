import { createSignal, Show } from 'solid-js';
import { Plus, Globe, Database, Layers, FileText, Bot, Download } from 'lucide-solid';

type ServiceType = 'application' | 'database' | 'compose' | 'template' | 'ai' | 'import';

type Props = {
  onSelect: (type: ServiceType) => void;
};

const options: { type: ServiceType; label: string; icon: any }[] = [
  { type: 'application', label: 'Application', icon: Globe },
  { type: 'database',    label: 'Database',    icon: Database },
  { type: 'compose',     label: 'Compose',     icon: Layers },
  { type: 'template',    label: 'Template',    icon: FileText },
  { type: 'ai',          label: 'AI Assistant', icon: Bot },
  { type: 'import',      label: 'Import',      icon: Download },
];

export default function CreateServiceDropdown(props: Props) {
  const [open, setOpen] = createSignal(false);

  return (
    <div class="relative">
      <button
        class="btn btn-neutral btn-sm gap-1.5"
        onClick={() => setOpen(v => !v)}
      >
        <Plus class="w-4 h-4" />
        Create Service
      </button>

      <Show when={open()}>
        <div class="fixed inset-0 z-10" onClick={() => setOpen(false)} />
        <div class="absolute top-full right-0 mt-1 z-20 bg-base-200 border border-base-300 rounded-lg shadow-xl w-48 overflow-hidden">
          <p class="px-3 py-2 text-[10px] uppercase tracking-widest text-base-content/30 border-b border-base-300">
            Actions
          </p>
          {options.map((opt) => (
            <button
              class="w-full flex items-center gap-2.5 px-3 py-2 text-sm text-base-content/70 hover:bg-base-300 hover:text-base-content transition-colors text-left"
              onClick={() => { setOpen(false); props.onSelect(opt.type); }}
            >
              <opt.icon class="w-4 h-4 shrink-0" />
              {opt.label}
            </button>
          ))}
        </div>
      </Show>
    </div>
  );
}
