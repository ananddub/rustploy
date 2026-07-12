<script lang="ts">
	import { goto } from '$app/navigation';
	import { Globe, FileCode, Plus, Trash2, Pencil } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	const files = [
		{ name: 'http.yml', path: '/etc/traefik/dynamic/http.yml', size: '1.2 KB', modified: '2026-07-12 15:30' },
		{ name: 'tls.yml', path: '/etc/traefik/dynamic/tls.yml', size: '0.8 KB', modified: '2026-07-10 10:00' },
		{ name: 'middlewares.yml', path: '/etc/traefik/dynamic/middlewares.yml', size: '2.1 KB', modified: '2026-07-08 14:20' }
	];
</script>

<PageLayout>
	<header class="flex items-center justify-between px-6 py-3 border-b border-border text-sm">
		<div class="flex items-center gap-2">
			<Globe class="w-4 h-4 text-muted-foreground" />
			<span class="font-medium">Traefik File System</span>
		</div>
		<Button size="sm" class="gap-1.5 text-xs"><Plus class="w-3.5 h-3.5" />Create File</Button>
	</header>

	<main class="flex-1 p-6 overflow-y-auto animate-fade-up">
		<div class="w-full">
			<Card.Root>
				<Card.Header>
					<Card.Title class="text-base flex items-center gap-2">
						<FileCode class="w-4 h-4 text-muted-foreground" />
						Dynamic Configuration Files
					</Card.Title>
					<Card.Description>Manage Traefik dynamic configuration files</Card.Description>
				</Card.Header>
				<Card.Content>
					<div class="flex flex-col gap-2">
						{#each files as file}
							<div class="flex items-center justify-between p-3.5 rounded-lg border">
								<div class="flex items-center gap-3">
									<FileCode class="w-4 h-4 text-muted-foreground" />
									<div>
										<p class="text-sm font-medium font-mono">{file.name}</p>
										<p class="text-[11px] text-muted-foreground">{file.path} · {file.size}</p>
									</div>
								</div>
								<div class="flex items-center gap-2">
									<span class="text-[10px] text-muted-foreground">{file.modified}</span>
									<Button variant="ghost" size="sm" class="h-7 w-7 p-0"><Pencil class="w-3.5 h-3.5" /></Button>
									<Button variant="ghost" size="sm" class="h-7 w-7 p-0"><Trash2 class="w-3.5 h-3.5 text-destructive" /></Button>
								</div>
							</div>
						{/each}
					</div>
				</Card.Content>
			</Card.Root>
		</div>
	</main>
</PageLayout>
