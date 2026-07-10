import { createResource, createSignal, Show } from 'solid-js';
import { useNavigate, useParams } from '@solidjs/router';
import { Pencil, Trash2, Server, FolderOpen, Rocket } from 'lucide-solid';
import { authSession } from '../../lib/auth';
import { applicationControllerGet } from '../../client/sdk.gen';
import type { ApplicationResponseDto } from '../../client/types.gen';
import { Sidebar } from '../../components';
import GeneralTab from './tabs/General';
import EnvironmentTab from './tabs/Environment';
import DomainsTab from './tabs/Domains';
import DeploymentsTab from './tabs/Deployments';
import LogsTab from './tabs/Logs';
import MonitoringTab from './tabs/Monitoring';
import AdvancedTab from './tabs/Advanced';
import PatchesTab from './tabs/Patches';

const TABS = [
  'General', 'Environment', 'Domains', 'Deployments',
  'Preview Deployments', 'Schedules', 'Volume Backups',
  'Logs', 'Patches', 'Monitoring', 'Advanced',
];

export default function ApplicationPage() {
  const navigate = useNavigate();
  const params = useParams();
  const appId = () => parseInt(params.appId);
  const projectId = () => params.projectId;

  if (!authSession()) navigate('/auth', { replace: true });

  const [activeTab, setActiveTab] = createSignal('General');

  const [app, { mutate }] = createResource(async () => {
    const res = await applicationControllerGet({ path: { id: appId() } });
    return res.data ?? null;
  });

  const handleUpdated = (updated: ApplicationResponseDto) => mutate(updated);

  return (
    <div class="min-h-screen flex bg-base-100 text-base-content">
      <Sidebar />

      <div class="flex-1 flex flex-col min-w-0">
        <header class="px-6 pt-4 border-b border-base-300">
          <div class="flex items-center gap-2 text-sm mb-3">
            <Rocket class="w-4 h-4 text-base-content/40" />
            <button onClick={() => navigate('/dashboard')} class="text-base-content/50 hover:text-base-content transition-colors">Dashboard</button>
            <span class="text-base-content/20">/</span>
            <button onClick={() => navigate('/projects')} class="text-base-content/50 hover:text-base-content transition-colors flex items-center gap-1">
              <FolderOpen class="w-3.5 h-3.5" /> Projects
            </button>
            <span class="text-base-content/20">/</span>
            <button onClick={() => navigate(`/projects/${projectId()}`)} class="text-base-content/50 hover:text-base-content transition-colors">Project</button>
            <span class="text-base-content/20">/</span>
            <span class="text-base-content font-medium">{app()?.name ?? '...'}</span>
          </div>

          <div class="flex items-center justify-between mb-3">
            <div>
              <h1 class="text-lg font-semibold">{app()?.name ?? '...'}</h1>
              <p class="text-xs text-base-content/40 mt-0.5 font-mono">{app()?.app_name ?? ''}</p>
            </div>
            <div class="flex items-center gap-2">
              <button class="btn btn-outline btn-sm gap-1.5 text-base-content/60">
                <Server class="w-3.5 h-3.5" /> Rustploy Server
              </button>
              <button class="btn btn-ghost btn-xs text-base-content/40 hover:text-base-content"><Pencil class="w-3.5 h-3.5" /></button>
              <button class="btn btn-ghost btn-xs text-base-content/40 hover:text-error"><Trash2 class="w-3.5 h-3.5" /></button>
            </div>
          </div>

          <div class="flex overflow-x-auto -mb-px">
            {TABS.map(tab => (
              <button
                class={`px-4 py-2 text-sm whitespace-nowrap border-b-2 transition-colors ${
                  activeTab() === tab ? 'border-base-content text-base-content font-medium' : 'border-transparent text-base-content/50 hover:text-base-content'
                }`}
                onClick={() => setActiveTab(tab)}
              >
                {tab}
              </button>
            ))}
          </div>
        </header>

        <main class="flex-1 px-6 py-6 overflow-y-auto">
          <Show when={app.loading}>
            <div class="flex justify-center py-20">
              <span class="loading loading-spinner loading-md text-base-content/40" />
            </div>
          </Show>

          <Show when={!app.loading && app()}>
            <div style={{ display: activeTab() === 'General' ? 'block' : 'none' }}>
              <GeneralTab app={app()!} onUpdated={handleUpdated} />
            </div>
            <div style={{ display: activeTab() === 'Environment' ? 'block' : 'none' }}>
              <EnvironmentTab app={app()!} onUpdated={handleUpdated} active={activeTab() === 'Environment'} />
            </div>
            <Show when={activeTab() === 'Domains'}>
              <DomainsTab app={app()!} />
            </Show>
            <Show when={activeTab() === 'Deployments'}>
              <DeploymentsTab app={app()!} />
            </Show>
            <Show when={activeTab() === 'Logs'}>
              <LogsTab app={app()!} />
            </Show>
            <Show when={activeTab() === 'Patches'}>
              <PatchesTab app={app()!} />
            </Show>
            <Show when={activeTab() === 'Monitoring'}>
              <MonitoringTab app={app()!} />
            </Show>
            <Show when={activeTab() === 'Advanced'}>
              <AdvancedTab app={app()!} onUpdated={handleUpdated} />
            </Show>
            <Show when={['Preview Deployments', 'Schedules', 'Volume Backups'].includes(activeTab())}>
              <div class="flex flex-col items-center justify-center py-20 text-base-content/30">
                <p class="text-sm">{activeTab()} — coming soon</p>
              </div>
            </Show>
          </Show>
        </main>
      </div>
    </div>
  );
}
