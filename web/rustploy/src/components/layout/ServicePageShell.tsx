import { type JSX, Show } from 'solid-js';
import { useNavigate } from '@solidjs/router';
import { PencilSimple, Trash, Cpu, FolderOpen, RocketLaunch, CaretRight } from 'phosphor-solid';
import PageLayout from './PageLayout';

type Tab = string;

type Props = {
  projectId: string;
  name: string;
  appName: string;
  tabs: readonly Tab[];
  activeTab: Tab;
  onTabChange: (tab: Tab) => void;
  loading: boolean;
  onEdit?: () => void;
  onDelete?: () => void;
  children: JSX.Element;
};

export default function ServicePageShell(props: Props) {
  const navigate = useNavigate();

  return (
    <PageLayout>
      <div class="flex flex-col min-w-0 flex-1">
        <header class="px-6 pt-4 border-b border-base-300">

          {/* Breadcrumb */}
          <div class="flex items-center gap-1.5 text-xs mb-3 text-base-content/40">
            <RocketLaunch size={13} />
            <button
              onClick={() => navigate('/dashboard')}
              class="hover:text-base-content transition-colors"
            >
              Dashboard
            </button>
            <CaretRight size={11} class="opacity-40" />
            <button
              onClick={() => navigate('/projects')}
              class="hover:text-base-content transition-colors flex items-center gap-1"
            >
              <FolderOpen size={13} /> Projects
            </button>
            <CaretRight size={11} class="opacity-40" />
            <button
              onClick={() => navigate(`/projects/${props.projectId}`)}
              class="hover:text-base-content transition-colors"
            >
              Project
            </button>
            <CaretRight size={11} class="opacity-40" />
            <span class="text-base-content/70 font-medium">{props.name || '…'}</span>
          </div>

          {/* Title row */}
          <div class="flex items-center justify-between mb-3">
            <div>
              <h1 class="text-lg font-semibold leading-tight">{props.name || '…'}</h1>
              <p class="text-xs text-base-content/40 mt-0.5 font-mono">{props.appName}</p>
            </div>
            <div class="flex items-center gap-1.5">
              <button class="btn btn-outline btn-sm gap-1.5 text-base-content/50 hover:text-base-content border-base-300 hover:border-base-content/30 hover:bg-base-200">
                <Cpu size={13} /> Rustploy Server
              </button>
              <button
                class="p-1.5 rounded-md text-base-content/40 hover:text-base-content hover:bg-base-200 transition-all"
                onClick={props.onEdit}
                title="Edit"
              >
                <PencilSimple size={14} />
              </button>
              <button
                class="p-1.5 rounded-md text-base-content/40 hover:text-error hover:bg-error/10 transition-all"
                onClick={props.onDelete}
                title="Delete"
              >
                <Trash size={14} />
              </button>
            </div>
          </div>

          {/* Tab bar */}
          <div class="flex overflow-x-auto -mb-px scrollbar-none">
            {props.tabs.map(tab => (
              <button
                class={`px-4 py-2 text-sm whitespace-nowrap border-b-2 transition-all duration-150 ${
                  props.activeTab === tab
                    ? 'border-base-content text-base-content font-medium'
                    : 'border-transparent text-base-content/45 hover:text-base-content/80 hover:border-base-content/20'
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
              <span class="loading loading-spinner loading-md text-base-content/30" />
            </div>
          </Show>
          <Show when={!props.loading}>
            <div class="animate-fade-up">
              {props.children}
            </div>
          </Show>
        </main>
      </div>
    </PageLayout>
  );
}
