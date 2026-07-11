import { type JSX, Show } from 'solid-js';
import { useNavigate } from '@solidjs/router';
import { Pencil, Trash2, Server, FolderOpen, Rocket } from 'lucide-solid';
import PageLayout from './PageLayout';

type Tab = string;

type Props = {
  // Breadcrumb / header
  projectId: string;
  name: string;
  appName: string;

  // Tab bar
  tabs: readonly Tab[];
  activeTab: Tab;
  onTabChange: (tab: Tab) => void;

  // Keep-alive tabs (always mounted, shown/hidden via display)
  keepAliveTabs?: readonly Tab[];

  // Loading state
  loading: boolean;

  // Actions (edit / delete buttons wired by parent)
  onEdit?: () => void;
  onDelete?: () => void;

  children: JSX.Element;
};

/**
 * ServicePageShell — shared chrome used by both ApplicationPage and ComposePage.
 * Renders the sidebar layout, breadcrumb, title row, tab bar, and loading state.
 * The parent provides the tab content as children.
 */
export default function ServicePageShell(props: Props) {
  const navigate = useNavigate();

  return (
    <PageLayout>
      <div class="flex flex-col min-w-0 flex-1">
        <header class="px-6 pt-4 border-b border-base-300">

          {/* Breadcrumb */}
          <div class="flex items-center gap-2 text-sm mb-3">
            <Rocket class="w-4 h-4 text-base-content/40" />
            <button
              onClick={() => navigate('/dashboard')}
              class="text-base-content/50 hover:text-base-content transition-colors"
            >
              Dashboard
            </button>
            <span class="text-base-content/20">/</span>
            <button
              onClick={() => navigate('/projects')}
              class="text-base-content/50 hover:text-base-content transition-colors flex items-center gap-1"
            >
              <FolderOpen class="w-3.5 h-3.5" /> Projects
            </button>
            <span class="text-base-content/20">/</span>
            <button
              onClick={() => navigate(`/projects/${props.projectId}`)}
              class="text-base-content/50 hover:text-base-content transition-colors"
            >
              Project
            </button>
            <span class="text-base-content/20">/</span>
            <span class="text-base-content font-medium">{props.name || '…'}</span>
          </div>

          {/* Title row */}
          <div class="flex items-center justify-between mb-3">
            <div>
              <h1 class="text-lg font-semibold">{props.name || '…'}</h1>
              <p class="text-xs text-base-content/40 mt-0.5 font-mono">{props.appName}</p>
            </div>
            <div class="flex items-center gap-2">
              <button class="btn btn-outline btn-sm gap-1.5 text-base-content/60">
                <Server class="w-3.5 h-3.5" /> Rustploy Server
              </button>
              <button
                class="btn btn-ghost btn-xs text-base-content/40 hover:text-base-content"
                onClick={props.onEdit}
              >
                <Pencil class="w-3.5 h-3.5" />
              </button>
              <button
                class="btn btn-ghost btn-xs text-base-content/40 hover:text-error"
                onClick={props.onDelete}
              >
                <Trash2 class="w-3.5 h-3.5" />
              </button>
            </div>
          </div>

          {/* Tab bar */}
          <div class="flex overflow-x-auto -mb-px scrollbar-none">
            {props.tabs.map(tab => (
              <button
                class={`px-4 py-2 text-sm whitespace-nowrap border-b-2 transition-colors ${
                  props.activeTab === tab
                    ? 'border-base-content text-base-content font-medium'
                    : 'border-transparent text-base-content/50 hover:text-base-content'
                }`}
                onClick={() => props.onTabChange(tab)}
              >
                {tab}
              </button>
            ))}
          </div>
        </header>

        <main class="flex-1 px-6 py-6 overflow-y-auto">
          <Show when={props.loading}>
            <div class="flex justify-center py-20">
              <span class="loading loading-spinner loading-md text-base-content/40" />
            </div>
          </Show>
          <Show when={!props.loading}>
            {props.children}
          </Show>
        </main>
      </div>
    </PageLayout>
  );
}
