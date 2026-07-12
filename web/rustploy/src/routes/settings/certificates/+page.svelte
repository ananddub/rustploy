<script lang="ts">
	import { goto } from '$app/navigation';
	import { Shield, Plus, Trash2, RefreshCw, ChevronDown, AlertTriangle, Loader2 } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import * as Alert from '$lib/components/ui/alert';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	// Static certificate data
	const certificates = [
		{ id: 'cert1', name: 'Wildcard Production', domain: '*.example.com', type: 'Wildcard', status: 'valid', expires: '2027-03-15', autoRenew: true, issuer: "Let's Encrypt", isChain: true, chainLength: 3 },
		{ id: 'cert2', name: 'API Certificate', domain: 'api.example.com', type: 'Single', status: 'valid', expires: '2027-01-20', autoRenew: true, issuer: "Let's Encrypt", isChain: false, chainLength: 1 },
		{ id: 'cert3', name: 'Staging Cert', domain: 'staging.example.com', type: 'Single', status: 'expiring', expires: '2026-08-01', autoRenew: false, issuer: "Let's Encrypt", isChain: false, chainLength: 1 }
	];

	function daysUntil(dateStr: string): number {
		const diff = new Date(dateStr).getTime() - Date.now();
		return Math.ceil(diff / (1000 * 60 * 60 * 24));
	}

	function statusColor(status: string): string {
		if (status === 'valid') return 'text-green-400';
		if (status === 'expiring') return 'text-yellow-400';
		return 'text-red-400';
	}
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<Shield class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Certificates</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto animate-fade-up">
		<div class="w-full">
			<Card.Root>
				<Card.Header>
					<div class="flex items-center justify-between">
						<div>
							<Card.Title class="text-lg flex items-center gap-2">
								<Shield class="w-5 h-5 text-muted-foreground" />
								Certificates
							</Card.Title>
							<Card.Description>Create certificates in the Traefik directory</Card.Description>
						</div>
						<Button size="sm" class="gap-1.5 text-xs">
							<Plus class="w-3.5 h-3.5" />
							Add Certificate
						</Button>
					</div>
					<!-- Warning Alert -->
					<Alert.Root variant="destructive" class="mt-3">
						<AlertTriangle class="h-4 w-4" />
						<Alert.Title>Warning</Alert.Title>
						<Alert.Description class="text-xs">
							Certificates are created in the Traefik directory. Using invalid certificates can break your Traefik instance, preventing access to your applications.
						</Alert.Description>
					</Alert.Root>
				</Card.Header>
				<Card.Content class="pt-4 border-t">
					{#if certificates.length === 0}
						<div class="flex flex-col items-center gap-3 min-h-[25vh] justify-center">
							<Shield class="w-8 h-8 text-muted-foreground/40" />
							<p class="text-sm text-muted-foreground">No certificates created yet</p>
							<Button size="sm" variant="outline" class="gap-1.5 text-xs">
								<Plus class="w-3.5 h-3.5" />
								Add Certificate
							</Button>
						</div>
					{:else}
						<div class="flex flex-col gap-2">
							{#each certificates as cert, i (cert.id)}
								<div class="flex items-center justify-between p-3.5 rounded-lg border animate-fade-up" style="animation-delay: {i * 40}ms">
									<div class="flex items-center gap-3">
										<Shield class="w-4 h-4 {statusColor(cert.status)} shrink-0" />
										<div class="flex flex-col gap-0.5">
											<div class="flex items-center gap-2">
												<span class="text-sm font-medium">{cert.name}</span>
												<Badge variant="outline" class="text-[10px]">{cert.type}</Badge>
												{#if cert.isChain}
													<Badge variant="secondary" class="text-[10px]">Chain ({cert.chainLength})</Badge>
												{/if}
											</div>
											<div class="flex items-center gap-2 text-[11px] text-muted-foreground">
												<span class="font-mono">{cert.domain}</span>
												<span>·</span>
												<span>{cert.issuer}</span>
											</div>
										</div>
									</div>
									<div class="flex items-center gap-3">
										<div class="text-right">
											<p class="text-xs {statusColor(cert.status)} font-medium">
												{#if cert.status === 'valid'}Valid{:else}Expiring Soon{/if}
											</p>
											<p class="text-[10px] text-muted-foreground">{daysUntil(cert.expires)} days · {cert.expires}</p>
										</div>
										{#if cert.autoRenew}
											<Badge variant="outline" class="text-[9px] text-green-400 border-green-400/30">Auto-Renew</Badge>
										{/if}
										<Button variant="ghost" size="sm" class="h-7 w-7 p-0">
											<RefreshCw class="w-3.5 h-3.5" />
										</Button>
										<Button variant="ghost" size="sm" class="h-7 w-7 p-0">
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
