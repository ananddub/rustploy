<script lang="ts">
	import { goto } from '$app/navigation';
	import { Bell, Plus, Trash2, TestTube, Mail, MessageSquare, Webhook, Send, Loader2 } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Switch } from '$lib/components/ui/switch';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import * as Select from '$lib/components/ui/select';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	let channels = $state([
		{ id: 1, name: 'Deploy Alerts', type: 'slack', target: '#deployments', enabled: true, events: ['deploy.success', 'deploy.failed'] },
		{ id: 2, name: 'Email Admin', type: 'email', target: 'admin@example.com', enabled: true, events: ['deploy.failed', 'server.down'] },
		{ id: 3, name: 'Webhook CI', type: 'webhook', target: 'https://ci.example.com/hook', enabled: false, events: ['deploy.success'] },
		{ id: 4, name: 'Discord Team', type: 'discord', target: '#ops-alerts', enabled: true, events: ['deploy.failed', 'server.down', 'cert.expiring'] },
		{ id: 5, name: 'Telegram Ops', type: 'telegram', target: '@ops_bot', enabled: true, events: ['deploy.success', 'deploy.failed'] }
	]);

	let showCreate = $state(false);
	let testing = $state<number | null>(null);

	const typeIcons: Record<string, typeof Bell> = {
		slack: MessageSquare,
		email: Mail,
		webhook: Webhook,
		discord: MessageSquare,
		telegram: Send
	};

	const typeLabels: Record<string, string> = {
		slack: 'Slack',
		email: 'Email',
		webhook: 'Webhook',
		discord: 'Discord',
		telegram: 'Telegram'
	};

	function toggleChannel(id: number) {
		channels = channels.map((c) => (c.id === id ? { ...c, enabled: !c.enabled } : c));
	}

	function removeChannel(id: number) {
		channels = channels.filter((c) => c.id !== id);
	}

	async function testChannel(id: number) {
		testing = id;
		await new Promise((r) => setTimeout(r, 1000));
		testing = null;
	}
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<Bell class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Notifications</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto animate-fade-up">
		<div class="w-full">
			<Card.Root>
				<Card.Header>
					<div class="flex items-center justify-between">
						<div>
							<Card.Title class="text-lg flex items-center gap-2">
								<Bell class="w-5 h-5 text-muted-foreground" />
								Notification Channels
							</Card.Title>
							<Card.Description>Configure where and when you receive deployment notifications</Card.Description>
						</div>
						<Button size="sm" class="gap-1.5 text-xs" onclick={() => (showCreate = true)}>
							<Plus class="w-3.5 h-3.5" />
							Add Channel
						</Button>
					</div>
				</Card.Header>
				<Card.Content class="pt-4 border-t">
					{#if channels.length === 0}
						<div class="flex flex-col items-center gap-3 min-h-[25vh] justify-center">
							<Bell class="w-8 h-8 text-muted-foreground/40" />
							<p class="text-sm text-muted-foreground">No notification channels configured</p>
							<Button size="sm" variant="outline" class="gap-1.5 text-xs" onclick={() => (showCreate = true)}>
								<Plus class="w-3.5 h-3.5" />
								Add Channel
							</Button>
						</div>
					{:else}
						<div class="flex flex-col gap-2">
							{#each channels as channel, i (channel.id)}
								{@const Icon = typeIcons[channel.type] ?? Bell}
								<div class="flex items-center justify-between p-3.5 rounded-lg border animate-fade-up" style="animation-delay: {i * 40}ms">
									<div class="flex items-center gap-3 flex-1 min-w-0">
										<div class="w-8 h-8 rounded-md bg-primary/10 flex items-center justify-center shrink-0">
											<Icon class="w-4 h-4 text-primary" />
										</div>
										<div class="flex-1 min-w-0">
											<div class="flex items-center gap-2">
												<span class="text-sm font-medium">{channel.name}</span>
												<Badge variant="outline" class="text-[9px]">{typeLabels[channel.type]}</Badge>
											</div>
											<span class="text-[11px] text-muted-foreground font-mono truncate block">{channel.target}</span>
										</div>
										<div class="flex flex-wrap gap-1 max-w-[200px]">
											{#each channel.events as event}
												<Badge variant="secondary" class="text-[9px] px-1.5">{event}</Badge>
											{/each}
										</div>
									</div>
									<div class="flex items-center gap-2 ml-3">
										<Switch checked={channel.enabled} onCheckedChange={() => toggleChannel(channel.id)} />
										<Button
											variant="ghost"
											size="sm"
											class="h-7 text-xs gap-1"
											disabled={testing === channel.id}
											onclick={() => testChannel(channel.id)}
										>
											{#if testing === channel.id}
												<Loader2 class="w-3 h-3 animate-spin" />
											{:else}
												<TestTube class="w-3 h-3" />
											{/if}
											Test
										</Button>
										<Button variant="ghost" size="sm" class="h-7 w-7 p-0" onclick={() => removeChannel(channel.id)}>
											<Trash2 class="w-3.5 h-3.5 text-destructive" />
										</Button>
									</div>
								</div>
							{/each}
						</div>
					{/if}
				</Card.Content>
			</Card.Root>
		</div>
	</main>
</PageLayout>

<!-- Create Dialog -->
<Dialog.Root bind:open={showCreate}>
	<Dialog.Content class="sm:max-w-lg">
		<Dialog.Header>
			<Dialog.Title>Add Notification Channel</Dialog.Title>
			<Dialog.Description>Configure a new notification destination</Dialog.Description>
		</Dialog.Header>
		<div class="space-y-4 py-4">
			<div class="space-y-1.5">
				<Label class="text-xs font-medium">Channel Name</Label>
				<Input placeholder="e.g. Deploy Alerts" />
			</div>
			<div class="space-y-1.5">
				<Label class="text-xs font-medium">Type</Label>
				<Select.Root type="single">
					<Select.Trigger class="w-full">
						<span class="text-muted-foreground text-sm">Select type...</span>
					</Select.Trigger>
					<Select.Content>
						<Select.Item value="slack">Slack</Select.Item>
						<Select.Item value="discord">Discord</Select.Item>
						<Select.Item value="email">Email</Select.Item>
						<Select.Item value="telegram">Telegram</Select.Item>
						<Select.Item value="webhook">Webhook</Select.Item>
					</Select.Content>
				</Select.Root>
			</div>
			<div class="space-y-1.5">
				<Label class="text-xs font-medium">Webhook URL / Target</Label>
				<Input placeholder="https://hooks.slack.com/..." />
			</div>
		</div>
		<Dialog.Footer>
			<Button variant="outline" size="sm" onclick={() => (showCreate = false)}>Cancel</Button>
			<Button size="sm" class="gap-1.5">
				<Plus class="w-3.5 h-3.5" />
				Create Channel
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
