<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { getAuthSession } from '$lib/auth';
	import {
		composeControllerGet,
		composeControllerDeploy,
		composeControllerRedeploy,
		composeControllerCancel
	} from '$lib/client/sdk.gen';
	import type { ComposeResponseDto } from '$lib/client/types.gen';
	import ServicePageShell from '$lib/components/ServicePageShell.svelte';
	import DeploymentsTab from '$lib/components/tabs/DeploymentsTab.svelte';
	import LogsTab from '$lib/components/tabs/LogsTab.svelte';
	import MonitoringTab from '$lib/components/tabs/MonitoringTab.svelte';
	import SchedulesTab from '$lib/components/tabs/SchedulesTab.svelte';
	import VolumeBackupsTab from '$lib/components/tabs/VolumeBackupsTab.svelte';
	import ComposeGeneralTab from './ComposeGeneralTab.svelte';
	import ComposeEnvironmentTab from './ComposeEnvironmentTab.svelte';
	import ComposeDomainsTab from './ComposeDomainsTab.svelte';
	import ComposeAdvancedTab from './ComposeAdvancedTab.svelte';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	const composeId = $derived(parseInt(page.params.composeId ?? '0'));
	const projectId = $derived(page.params.id ?? '');

	const TABS = [
		'General', 'Environment', 'Domains', 'Deployments',
		'Schedules', 'Volume Backups', 'Logs', 'Monitoring', 'Advanced'
	] as const;

	let activeTab = $state('General');
	let compose = $state<ComposeResponseDto | null>(null);
	let loading = $state(true);

	$effect(() => {
		composeControllerGet({ path: { id: composeId } }).then((res: { data?: ComposeResponseDto | null }) => {
			compose = res.data ?? null;
			loading = false;
		});
	});

	function handleUpdated(updated: ComposeResponseDto) { compose = updated; }
</script>

<ServicePageShell
	{projectId}
	name={compose?.name ?? ''}
	appName={compose?.app_name ?? ''}
	tabs={TABS}
	{activeTab}
	onTabChange={(t) => (activeTab = t)}
	{loading}
>
	{#if compose}
		<div style="display: {activeTab === 'General' ? 'block' : 'none'}">
			<ComposeGeneralTab {compose} onUpdated={handleUpdated} />
		</div>
		<div style="display: {activeTab === 'Environment' ? 'block' : 'none'}">
			<ComposeEnvironmentTab {compose} onUpdated={handleUpdated} active={activeTab === 'Environment'} />
		</div>
		{#if activeTab === 'Domains'}<ComposeDomainsTab {compose} />{/if}
		{#if activeTab === 'Deployments'}
			<DeploymentsTab
				serviceLabel="compose"
				serviceId={composeId}
				serviceType="compose"
				onDeploy={async () => { await composeControllerDeploy({ path: { id: composeId } }); }}
				onRedeploy={async () => { await composeControllerRedeploy({ path: { id: composeId } }); }}
			/>
		{/if}
		{#if activeTab === 'Logs'}
			<LogsTab serviceLabel="compose" serviceId={composeId} appName={compose.app_name} serviceType="compose" />
		{/if}
		{#if activeTab === 'Monitoring'}
			<MonitoringTab serviceLabel="compose" serviceId={composeId} appName={compose.app_name} serviceType="compose" />
		{/if}
		{#if activeTab === 'Schedules'}<SchedulesTab serviceLabel="compose" serviceId={composeId} />{/if}
		{#if activeTab === 'Volume Backups'}<VolumeBackupsTab serviceLabel="compose" serviceId={composeId} />{/if}
		{#if activeTab === 'Advanced'}<ComposeAdvancedTab {compose} onUpdated={handleUpdated} />{/if}
	{/if}
</ServicePageShell>
