<script lang="ts">
	import { goto } from '$app/navigation';
	import { Layers, Plus, Trash2, CheckCircle } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	const destinations = [
		{ id: 1, name: 'AWS S3 Backups', provider: 'aws', bucket: 'rustploy-backups', region: 'us-east-1', status: 'connected' },
		{ id: 2, name: 'DigitalOcean Spaces', provider: 'do', bucket: 'do-backup-space', region: 'nyc3', status: 'connected' }
	];
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<Layers class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">S3 Destinations</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto animate-fade-up">
		<div class="w-full">
			<Card.Root>
				<Card.Header>
					<div class="flex items-center justify-between">
						<div>
							<Card.Title class="text-base flex items-center gap-2">
								<Layers class="w-4 h-4 text-muted-foreground" />
								S3 Destinations
							</Card.Title>
							<Card.Description>Configure backup destinations for your databases and volumes</Card.Description>
						</div>
						<Button size="sm" class="gap-1.5 text-xs"><Plus class="w-3.5 h-3.5" />Add Destination</Button>
					</div>
				</Card.Header>
				<Card.Content>
					{#if destinations.length === 0}
						<div class="flex flex-col items-center gap-3 min-h-[25vh] justify-center">
							<Layers class="w-8 h-8 text-muted-foreground/40" />
							<p class="text-sm text-muted-foreground">No destinations configured</p>
						</div>
					{:else}
						<div class="flex flex-col gap-2">
							{#each destinations as dest (dest.id)}
								<div class="flex items-center justify-between p-3.5 rounded-lg border">
									<div class="flex items-center gap-3">
										<div class="w-8 h-8 rounded-md bg-primary/10 flex items-center justify-center">
											<Layers class="w-4 h-4 text-primary" />
										</div>
										<div>
											<div class="flex items-center gap-2">
												<span class="text-sm font-medium">{dest.name}</span>
												<Badge variant="default" class="text-[9px] gap-1"><CheckCircle class="w-2.5 h-2.5" />Connected</Badge>
											</div>
											<p class="text-[11px] text-muted-foreground font-mono">{dest.bucket} · {dest.region}</p>
										</div>
									</div>
									<Button variant="ghost" size="sm" class="h-7 w-7 p-0"><Trash2 class="w-3.5 h-3.5 text-destructive" /></Button>
								</div>
							{/each}
						</div>
					{/if}
				</Card.Content>
			</Card.Root>
		</div>
	</main>
</PageLayout>
