import { createResource, createSignal, Show } from 'solid-js';
import { useNavigate, useParams } from '@solidjs/router';
import { authSession } from '../../lib/auth';
import { composeControllerGet } from '../../client/sdk.gen';
import type { ComposeResponseDto } from '../../client/types.gen';
import ServicePageShell from '../../components/layout/ServicePageShell';
import GeneralTab from './tabs/General';
import EnvironmentTab from './tabs/Environment';
import DomainsTab from './tabs/Domains';
import DeploymentsTab from './tabs/Deployments';
import ContainersTab from './tabs/Containers';
import BackupsTab from './tabs/Backups';
import SchedulesTab from './tabs/Schedules';
import VolumeBackupsTab from './tabs/VolumeBackups';
import LogsTab from './tabs/Logs';
import MonitoringTab from './tabs/Monitoring';
import AdvancedTab from './tabs/Advanced';

const TABS = [
  'General', 'Environment', 'Domains', 'Deployments',
  'Containers', 'Backups', 'Schedules', 'Volume Backups',
  'Logs', 'Monitoring', 'Advanced',
] as const;

type Tab = (typeof TABS)[number];

export default function ComposePage() {
  const navigate = useNavigate();
  const params = useParams();
  const composeId = () => parseInt(params.composeId);
  const projectId = () => params.projectId;

  if (!authSession()) navigate('/auth', { replace: true });

  const [activeTab, setActiveTab] = createSignal<Tab>('General');

  const [compose, { mutate }] = createResource(async () => {
    const res = await composeControllerGet({ path: { id: composeId() } });
    return res.data ?? null;
  });

  const handleUpdated = (updated: ComposeResponseDto) => mutate(updated);

  return (
    <ServicePageShell
      projectId={projectId()}
      name={compose()?.name ?? ''}
      appName={compose()?.app_name ?? ''}
      tabs={TABS}
      activeTab={activeTab()}
      onTabChange={t => setActiveTab(t as Tab)}
      loading={compose.loading}
    >
      <Show when={compose()}>
        {/* General + Environment always mounted — keeps Monaco editor state alive */}
        <div style={{ display: activeTab() === 'General' ? 'block' : 'none' }}>
          <GeneralTab compose={compose()!} onUpdated={handleUpdated} />
        </div>
        <div style={{ display: activeTab() === 'Environment' ? 'block' : 'none' }}>
          <EnvironmentTab compose={compose()!} onUpdated={handleUpdated} active={activeTab() === 'Environment'} />
        </div>

        <Show when={activeTab() === 'Domains'}><DomainsTab compose={compose()!} /></Show>
        <Show when={activeTab() === 'Deployments'}><DeploymentsTab compose={compose()!} /></Show>
        <Show when={activeTab() === 'Containers'}><ContainersTab compose={compose()!} /></Show>
        <Show when={activeTab() === 'Backups'}><BackupsTab compose={compose()!} /></Show>
        <Show when={activeTab() === 'Schedules'}><SchedulesTab compose={compose()!} /></Show>
        <Show when={activeTab() === 'Volume Backups'}><VolumeBackupsTab compose={compose()!} /></Show>
        <Show when={activeTab() === 'Logs'}><LogsTab compose={compose()!} /></Show>
        <Show when={activeTab() === 'Monitoring'}><MonitoringTab compose={compose()!} /></Show>
        <Show when={activeTab() === 'Advanced'}><AdvancedTab compose={compose()!} /></Show>
      </Show>
    </ServicePageShell>
  );
}
