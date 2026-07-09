import { type JSX } from 'solid-js';
import { X } from 'lucide-solid';

type Props = {
  title: string;
  onClose: () => void;
  children: JSX.Element;
  width?: string; // e.g. 'max-w-md', 'max-w-lg'
};

/**
 * Modal — reusable modal with backdrop, header and close button.
 * Usage:
 *   <Modal title="Create Project" onClose={() => setOpen(false)}>
 *     <form>...</form>
 *   </Modal>
 */
export default function Modal(props: Props) {
  return (
    <>
      {/* Backdrop */}
      <div class="fixed inset-0 bg-black/60 z-40" onClick={props.onClose} />

      {/* Dialog */}
      <div class="fixed inset-0 z-50 flex items-center justify-center p-4">
        <div class={`bg-base-200 border border-base-300 rounded-lg w-full shadow-2xl ${props.width ?? 'max-w-md'}`}>
          {/* Header */}
          <div class="flex items-center justify-between px-5 py-4 border-b border-base-300">
            <h2 class="font-semibold text-base-content">{props.title}</h2>
            <button
              class="text-base-content/40 hover:text-base-content transition-colors"
              onClick={props.onClose}
            >
              <X class="w-4 h-4" />
            </button>
          </div>

          {/* Content */}
          <div class="px-5 py-4">
            {props.children}
          </div>
        </div>
      </div>
    </>
  );
}
