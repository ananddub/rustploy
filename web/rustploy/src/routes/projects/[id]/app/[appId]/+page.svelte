<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { getAuthSession } from '$lib/auth';
	import { applicationControllerGet, applicationControllerDeploy, applicationControllerReload, applicationControllerRebuild, applicationControllerStart } from '$lib/client/sdk.gen';
	import type { ApplicationResponseDto } from '$lib/client/types.gen';
	import ServicePageShell from '$lib/components/ServicePageShell.svelte';
	import DeploymentsTab from '$lib/components/tabs/DeploymentsTab.svelte';
	import LogsTab from '$lib/components/tabs/LogsTab.svelte';
	import MonitoringTab from '$lib/components/tabs/MonitoringTab.svelte';
	import SchedulesTab from '$lib/components/tabs/SchedulesTab.svelte';
	import VolumeBackupsTab from '$lib/components/tabs/VolumeBackupsTab.svelte';
	import GeneralTab from './GeneralTab.svelte';
	import EnvironmentTab from './EnvironmentTab.svelte';
	import DomainsTab from './DomainsTab.svelte';
	import AdvancedTab from './AdvancedTab.svelte';
	import PreviewDeploymentsTab from './PreviewDeploymentsTab.svelte';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	const appId = $derived(parseInt(page.params.appId ?? '0'));
	const projectId = $derived(page.params.id ?? '');

	const TABS = ['General', 'Environment', 'Domains', 'Deployments', 'Preview Deployments', 'Schedules', 'Volume Backups', 'Logs', 'Patches', 'Monitoring', 'Advanced'] as const;

	let activeTab = $state('General');
	let app = $state<ApplicationResponseDto | null>(null);
	let loading = $state(true);

	$effect(() => {
		applicationControllerGet({ path: { id: appId } }).then((res: { data?: ApplicationResponseDto | null }) => {
			app = res.data ?? null;
			loading = false;
		});
	});

	function handleUpdated(updated: ApplicationResponseDto) { app = updated; }
</script>

<ServicePageShell
	{projectId}
	name={app?.name ?? ''}
	appName={app?.app_name ?? ''}
	tabs={TABS}
	{activeTab}
	onTabChange={(t) => (activeTab = t)}
	{loading}
>
	{#if app}
		<div style="display: {activeTab === 'General' ? 'block' : 'none'}">
			<GeneralTab {app} onUpdated={handleUpdated} />
		</div>
		<div style="display: {activeTab === 'Environment' ? 'block' : 'none'}">
			<EnvironmentTab {app} onUpdated={handleUpdated} active={activeTab === 'Environment'} />
		</div>
		{#if activeTab === 'Domains'}<DomainsTab {app} />{/if}
		{#if activeTab === 'Deployments'}
			<DeploymentsTab
				serviceLabel="application"
				onDeploy={async () => { await applicationControllerDeploy({ path: { id: appId } }); }}
				onRedeploy={async () => { await applicationControllerRebuild({ path: { id: appId } }); }}
			/>
		{/if}
		{#if activeTab === 'Preview Deployments'}<PreviewDeploymentsTab {app} onUpdated={handleUpdated} />{/if}
		{#if activeTab === 'Logs'}<LogsTab serviceLabel="application" />{/if}
		{#if activeTab === 'Monitoring'}<MonitoringTab serviceLabel="application" />{/if}
		{#if activeTab === 'Schedules'}<SchedulesTab serviceLabel="application" serviceId={appId} />{/if}
		{#if activeTab === 'Volume Backups'}<VolumeBackupsTab serviceLabel="application" serviceId={appId} />{/if}
		{#if activeTab === 'Advanced'}<AdvancedTab {app} onUpdated={handleUpdated} />{/if}
		{#if activeTab === 'Patches'}
			<div class="bg-card border border-border rounded-lg p-6">
				<h2 class="text-base font-semibold mb-1">Patches</h2>
				<p class="text-sm text-muted-foreground">Apply patches to your application configuration.</p>
				<div class="flex flex-col items-center justify-center py-12 text-muted-foreground/30">
					<p class="text-sm">No patches configured yet.</p>
				</div>
			</div>
		{/if}
	{/if}
</ServicePageShell>
