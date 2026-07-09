import { type JSX, createSignal } from 'solid-js';
import Sidebar from './Sidebar';

type Props = {
  children: JSX.Element;
};

/**
 * PageLayout — wraps every authenticated page with the shared Sidebar.
 * Usage:
 *   <PageLayout>
 *     <main>...</main>
 *   </PageLayout>
 */
export default function PageLayout(props: Props) {
  const [dragging, setDragging] = createSignal(false);

  return (
    <div class={`min-h-screen flex bg-base-100 text-base-content ${dragging() ? 'cursor-col-resize select-none' : ''}`}>
      <Sidebar onWidthChange={(w) => setDragging(w > 0)} />
      <div class="flex-1 flex flex-col min-w-0">
        {props.children}
      </div>
    </div>
  );
}
