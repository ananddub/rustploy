import { type JSX } from 'solid-js';
import { X } from 'lucide-solid';

type Props = {
  title: string;
  subtitle?: string;
  onClose: () => void;
  children: JSX.Element;
  width?: string; // e.g. 'max-w-md', 'max-w-lg'
};

/**
 * Modal — reusable modal with backdrop, scrollable body, header and close button.
 */
export default function Modal(props: Props) {
  return (
    <>
      {/* Backdrop */}
      <div class="fixed inset-0 bg-black/60 z-40" onClick={props.onClose} />

      {/* Dialog */}
      <div class="fixed inset-0 z-50 flex items-center justify-center p-4">
        <div class={`bg-base-200 border border-base-300 rounded-lg w-full shadow-2xl flex flex-col max-h-[90vh] ${props.width ?? 'max-w-md'}`}>

          {/* Header — fixed, never scrolls */}
          <div class="flex items-start justify-between px-5 py-4 border-b border-base-300 shrink-0">
            <div>
              <h2 class="font-semibold text-base-content">{props.title}</h2>
              {props.subtitle && (
                <p class="text-sm text-base-content/50 mt-0.5">{props.subtitle}</p>
              )}
            </div>
            <button
              class="text-base-content/40 hover:text-base-content transition-colors mt-0.5 ml-4 shrink-0"
              onClick={props.onClose}
            >
              <X class="w-4 h-4" />
            </button>
          </div>

          {/* Scrollable body */}
          <div class="overflow-y-auto flex-1 px-5 py-4">
            {props.children}
          </div>

        </div>
      </div>
    </>
  );
}
