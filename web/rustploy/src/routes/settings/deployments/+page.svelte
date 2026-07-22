<script lang="ts">
	import { goto } from '$app/navigation';
	import { Zap, Save, Loader2 } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Switch } from '$lib/components/ui/switch';
	import { Separator } from '$lib/components/ui/separator';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	let concurrency = $state('3');
	let autoRollback = $state(true);
	let keepDeployments = $state('50');
	let logRetention = $state('30');
	let saving = $state(false);
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<Zap class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Deployment Settings</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto animate-fade-up">
		<div class="w-full space-y-6">
			<Card.Root>
				<Card.Header>
					<Card.Title class="text-base">Build Configuration</Card.Title>
					<Card.Description>Configure how deployments are built and executed</Card.Description>
				</Card.Header>
				<Card.Content class="space-y-4">
					<div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
						<div class="space-y-1.5">
							<Label class="text-sm">Build Concurrency</Label>
							<Input bind:value={concurrency} type="number" placeholder="3" />
							<p class="text-[10px] text-muted-foreground">Max concurrent builds</p>
						</div>
						<div class="space-y-1.5">
							<Label class="text-sm">Keep Deployments</Label>
							<Input bind:value={keepDeployments} type="number" placeholder="50" />
							<p class="text-[10px] text-muted-foreground">Number of deployments to retain</p>
						</div>
					</div>
					<div class="space-y-1.5">
						<Label class="text-sm">Log Retention (days)</Label>
						<Input bind:value={logRetention} type="number" class="w-40" />
					</div>
					<Separator />
					<div class="flex items-center justify-between">
						<div>
							<p class="text-sm font-medium">Auto Rollback</p>
							<p class="text-[11px] text-muted-foreground">Automatically rollback on failed deployments</p>
						</div>
						<Switch bind:checked={autoRollback} />
					</div>
				</Card.Content>
				<Card.Footer class="border-t pt-4 flex justify-end">
					<Button size="sm" class="gap-1.5 text-xs" disabled={saving}>
						<Save class="w-3.5 h-3.5" />Save
					</Button>
				</Card.Footer>
			</Card.Root>
		</div>
	</main>
</PageLayout>
