<script lang="ts">
	import { goto } from '$app/navigation';
	import { Shield, Plus, Trash2, RefreshCw, AlertTriangle } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import * as Alert from '$lib/components/ui/alert';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	// Mock certificates data
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
	<!-- Top Breadcrumb Header Bar -->
	<header class="flex items-center justify-between px-6 py-3 border-b border-[#262626] text-xs bg-[#0A0A0A] shrink-0">
		<div class="flex items-center gap-2">
			<Shield class="w-3.5 h-3.5 text-[#a1a1aa]" />
			<span class="font-medium text-[#FAFAFA]">Certificates</span>
		</div>
	</header>

	<main class="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up min-h-0">
		<!-- Horizontally Centered Max-Width Shell (max-w-5xl mx-auto) -->
		<div class="max-w-5xl mx-auto space-y-6">
			<Card.Root class="bg-[#171717] border border-[#262626] rounded-xl shadow-md overflow-hidden">
				<Card.Header class="p-6 pb-4">
					<div class="flex items-center justify-between">
						<div>
							<Card.Title class="text-lg font-semibold text-[#FAFAFA] flex items-center gap-2">
								<Shield class="w-5 h-5 text-[#a1a1aa]" />
								Certificates
							</Card.Title>
							<Card.Description class="text-xs text-[#a1a1aa] mt-1">Create certificates in the Traefik directory</Card.Description>
						</div>
						<Button size="sm" class="gap-1.5 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A]">
							<Plus class="w-3.5 h-3.5" />
							Add Certificate
						</Button>
					</div>

					<!-- Warning Alert Box -->
					<div class="mt-4 p-3.5 rounded-lg bg-orange-500/10 border border-orange-500/20 flex items-start gap-3 text-orange-400">
						<AlertTriangle class="h-4 w-4 shrink-0 mt-0.5" />
						<div>
							<p class="text-xs font-semibold">Warning</p>
							<p class="text-[11px] text-orange-400/90 mt-0.5 leading-relaxed">
								Certificates are created in the Traefik directory. Using invalid certificates can break your Traefik instance, preventing access to your applications.
							</p>
						</div>
					</div>
				</Card.Header>

				<Card.Content class="p-6 pt-2 border-t border-[#262626]">
					{#if certificates.length === 0}
						<div class="flex flex-col items-center gap-3 py-16 justify-center text-center">
							<div class="w-12 h-12 rounded-xl bg-[#141414] border border-[#262626] flex items-center justify-center">
								<Shield class="w-6 h-6 text-[#737373]" />
							</div>
							<p class="text-sm font-semibold text-[#FAFAFA]">You don't have any certificates created</p>
							<Button size="sm" class="gap-1.5 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] mt-1">
								<Plus class="w-3.5 h-3.5" />
								Add Certificate
							</Button>
						</div>
					{:else}
						<div class="flex flex-col gap-2.5 pt-2">
							{#each certificates as cert, i (cert.id)}
								<div class="flex items-center justify-between p-4 rounded-xl border border-[#262626] bg-[#141414] hover:border-[#3f3f46] transition-all">
									<div class="flex items-center gap-3.5">
										<Shield class="w-4 h-4 {statusColor(cert.status)} shrink-0" />
										<div class="flex flex-col gap-0.5">
											<div class="flex items-center gap-2">
												<span class="text-xs font-semibold text-[#FAFAFA]">{cert.name}</span>
												<Badge variant="outline" class="text-[10px] border-[#262626] text-[#a1a1aa] bg-[#262626]">{cert.type}</Badge>
												{#if cert.isChain}
													<Badge variant="outline" class="text-[10px] border-[#262626] text-[#a1a1aa]">Chain ({cert.chainLength})</Badge>
												{/if}
											</div>
											<div class="flex items-center gap-2 text-[11px] text-[#737373] font-mono">
												<span>{cert.domain}</span>
												<span>·</span>
												<span>{cert.issuer}</span>
											</div>
										</div>
									</div>
									<div class="flex items-center gap-3">
										<div class="text-right">
											<p class="text-xs {statusColor(cert.status)} font-semibold font-mono">
												{#if cert.status === 'valid'}Valid{:else}Expiring Soon{/if}
											</p>
											<p class="text-[10px] text-[#737373] font-mono">{daysUntil(cert.expires)} days · {cert.expires}</p>
										</div>
										{#if cert.autoRenew}
											<Badge variant="outline" class="text-[9px] text-green-400 border-green-500/30 bg-green-500/10 font-mono">Auto-Renew</Badge>
										{/if}
										<button class="p-1.5 rounded-lg border border-[#262626] text-[#a1a1aa] hover:text-[#FAFAFA] hover:bg-[#262626] transition-colors">
											<RefreshCw class="w-3.5 h-3.5" />
										</button>
										<button class="p-1.5 rounded-lg border border-[#262626] text-[#a1a1aa] hover:text-red-400 hover:bg-red-500/10 transition-colors">
											<Trash2 class="w-3.5 h-3.5" />
										</button>
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
