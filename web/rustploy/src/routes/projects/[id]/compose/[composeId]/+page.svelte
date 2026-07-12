<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { getAuthSession } from '$lib/auth';
	import { composeControllerGet, composeControllerDeploy, composeControllerRedeploy } from '$lib/client/sdk.gen';
	import type { ComposeResponseDto } from '$lib/client/types.gen';
	import ServicePageShell from '$lib/components/ServicePageShell.svelte';
	import DeploymentsTab from '$lib/components/tabs/DeploymentsTab.svelte';
	import LogsTab from '$lib/components/tabs/LogsTab.svelte';
	import MonitoringTab from '$lib/components/tabs/MonitoringTab.svelte';
	import SchedulesTab from '$lib/components/tabs/SchedulesTab.svelte';
	import VolumeBackupsTab from '$lib/components/tabs/VolumeBackupsTab.svelte';
	import ComposeGeneralTab from './ComposeGeneralTab.svelte';
	import ComposeEnvironmentTab from './ComposeEnvironmentTab.svelte';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	const composeId = $derived(parseInt(page.params.composeId ?? '0'));
	const projectId = $derived(page.params.id ?? '');

	const TABS = ['General', 'Environment', 'Domains', 'Deployments', 'Containers', 'Backups', 'Schedules', 'Volume Backups', 'Logs', 'Monitoring', 'Advanced'] as const;

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
		{#if activeTab === 'Deployments'}
			<DeploymentsTab
				serviceLabel="compose"
				onDeploy={async () => { await composeControllerDeploy({ path: { id: composeId } }); }}
				onRedeploy={async () => { await composeControllerRedeploy({ path: { id: composeId } }); }}
			/>
		{/if}
		{#if activeTab === 'Logs'}<LogsTab serviceLabel="compose" />{/if}
		{#if activeTab === 'Monitoring'}<MonitoringTab serviceLabel="compose" />{/if}
		{#if activeTab === 'Schedules'}<SchedulesTab serviceLabel="compose" serviceId={composeId} />{/if}
		{#if activeTab === 'Volume Backups'}<VolumeBackupsTab serviceLabel="compose" serviceId={composeId} />{/if}
		{#if activeTab === 'Domains'}
			<div class="bg-card border border-border rounded-lg p-6">
				<h2 class="text-base font-semibold mb-1">Domains</h2>
				<p class="text-sm text-muted-foreground">Manage custom domains for this compose service.</p>
				<div class="flex flex-col items-center justify-center py-12 text-muted-foreground/30">
					<p class="text-sm">No domains configured yet.</p>
				</div>
			</div>
		{/if}
		{#if activeTab === 'Containers'}
			<div class="bg-card border border-border rounded-lg p-6">
				<h2 class="text-base font-semibold mb-1">Containers</h2>
				<p class="text-sm text-muted-foreground">Running containers for this compose service.</p>
				<div class="flex flex-col items-center justify-center py-12 text-muted-foreground/30">
					<p class="text-sm">No containers running.</p>
				</div>
			</div>
		{/if}
		{#if activeTab === 'Backups'}
			<div class="bg-card border border-border rounded-lg p-6">
				<h2 class="text-base font-semibold mb-1">Backups</h2>
				<p class="text-sm text-muted-foreground">Backup configuration for this compose service.</p>
				<div class="flex flex-col items-center justify-center py-12 text-muted-foreground/30">
					<p class="text-sm">No backups configured yet.</p>
				</div>
			</div>
		{/if}
		{#if activeTab === 'Advanced'}
			<div class="bg-card border border-border rounded-lg p-6 flex flex-col gap-6">
				<div>
					<h2 class="text-base font-semibold mb-1">Advanced Settings</h2>
					<p class="text-sm text-muted-foreground">Advanced configuration for this compose service.</p>
				</div>
				<div class="bg-card border border-destructive/30 rounded-lg p-6">
					<h2 class="text-base font-semibold text-destructive mb-1">Danger Zone</h2>
					<p class="text-sm text-muted-foreground mb-4">These actions are irreversible.</p>
					<button class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-destructive text-destructive-foreground text-sm font-medium hover:bg-destructive/90">
						Delete Compose
					</button>
				</div>
			</div>
		{/if}
	{/if}
</ServicePageShell>
