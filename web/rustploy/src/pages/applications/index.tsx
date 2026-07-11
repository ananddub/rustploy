import { createResource, createSignal, Show } from 'solid-js';
import { useNavigate, useParams } from '@solidjs/router';
import { authSession } from '../../lib/auth';
import { applicationControllerGet } from '../../client/sdk.gen';
import type { ApplicationResponseDto } from '../../client/types.gen';
import ServicePageShell from '../../components/layout/ServicePageShell';
import GeneralTab from './tabs/General';
import EnvironmentTab from './tabs/Environment';
import DomainsTab from './tabs/Domains';
import DeploymentsTab from './tabs/Deployments';
import PreviewDeploymentsTab from './tabs/PreviewDeployments';
import SchedulesTab from './tabs/Schedules';
import VolumeBackupsTab from './tabs/VolumeBackups';
import LogsTab from './tabs/Logs';
import PatchesTab from './tabs/Patches';
import MonitoringTab from './tabs/Monitoring';
import AdvancedTab from './tabs/Advanced';

const TABS = [
  'General', 'Environment', 'Domains', 'Deployments',
  'Preview Deployments', 'Schedules', 'Volume Backups',
  'Logs', 'Patches', 'Monitoring', 'Advanced',
] as const;

type Tab = (typeof TABS)[number];

export default function ApplicationPage() {
  const navigate = useNavigate();
  const params = useParams();
  const appId = () => parseInt(params.appId);
  const projectId = () => params.projectId;

  if (!authSession()) navigate('/auth', { replace: true });

  const [activeTab, setActiveTab] = createSignal<Tab>('General');

  const [app, { mutate }] = createResource(async () => {
    const res = await applicationControllerGet({ path: { id: appId() } });
    return res.data ?? null;
  });

  const handleUpdated = (updated: ApplicationResponseDto) => mutate(updated);

  return (
    <ServicePageShell
      projectId={projectId()}
      name={app()?.name ?? ''}
      appName={app()?.app_name ?? ''}
      tabs={TABS}
      activeTab={activeTab()}
      onTabChange={t => setActiveTab(t as Tab)}
      loading={app.loading}
    >
      <Show when={app()}>
        {/* General + Environment always mounted — keeps Monaco editor state alive */}
        <div style={{ display: activeTab() === 'General' ? 'block' : 'none' }}>
          <GeneralTab app={app()!} onUpdated={handleUpdated} />
        </div>
        <div style={{ display: activeTab() === 'Environment' ? 'block' : 'none' }}>
          <EnvironmentTab app={app()!} onUpdated={handleUpdated} active={activeTab() === 'Environment'} />
        </div>

        <Show when={activeTab() === 'Domains'}><DomainsTab app={app()!} /></Show>
        <Show when={activeTab() === 'Deployments'}><DeploymentsTab app={app()!} /></Show>
        <Show when={activeTab() === 'Preview Deployments'}><PreviewDeploymentsTab app={app()!} onUpdated={handleUpdated} /></Show>
        <Show when={activeTab() === 'Schedules'}><SchedulesTab app={app()!} /></Show>
        <Show when={activeTab() === 'Volume Backups'}><VolumeBackupsTab app={app()!} /></Show>
        <Show when={activeTab() === 'Logs'}><LogsTab app={app()!} /></Show>
        <Show when={activeTab() === 'Patches'}><PatchesTab app={app()!} /></Show>
        <Show when={activeTab() === 'Monitoring'}><MonitoringTab app={app()!} /></Show>
        <Show when={activeTab() === 'Advanced'}><AdvancedTab app={app()!} onUpdated={handleUpdated} /></Show>
      </Show>
    </ServicePageShell>
  );
}
