<script lang="ts">
	import { goto } from '$app/navigation';
	import { Package, RefreshCw, Play, Square, Terminal, Cpu, HardDrive } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import { USE_MOCK_DATA, getDockerContainersMock, type DockerContainerMock } from '$lib/mocks';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	let useMock = $state(USE_MOCK_DATA);
	let containers = $state<DockerContainerMock[]>(getDockerContainersMock());
</script>

<PageLayout>
	<!-- Top Breadcrumb Bar -->
	<header class="flex items-center justify-between px-6 py-3 border-b border-[#262626] text-xs bg-[#0A0A0A] shrink-0">
		<div class="flex items-center gap-2">
			<Package class="w-3.5 h-3.5 text-[#a1a1aa]" />
			<span class="font-medium text-[#FAFAFA]">Docker Containers</span>
		</div>

		<!-- Mock Data Dev Toggle Switch -->
		<div class="flex items-center gap-2 px-3 py-1 rounded-full bg-[#141414] border border-[#262626]">
			<span class="text-[11px] text-[#a1a1aa]">Data Source:</span>
			<button
				onclick={() => (useMock = !useMock)}
				class="text-[11px] font-semibold px-2 py-0.5 rounded transition-colors {useMock
					? 'bg-[#262626] text-[#FAFAFA] border border-white/10'
					: 'text-[#737373] hover:text-[#FAFAFA]'}"
			>
				{useMock ? 'Mock Demo Data' : 'Live Rust Backend API'}
			</button>
		</div>
	</header>

	<main class="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up flex flex-col min-h-0 bg-[#171717] border border-[#262626] rounded-2xl shadow-md space-y-6">
		<div class="flex items-center justify-between">
			<div>
				<h1 class="text-3xl font-bold tracking-tight text-[#FAFAFA]">Docker Containers</h1>
				<p class="text-sm text-[#a1a1aa] mt-1">Live container engine stats, ports, and memory usage</p>
			</div>
		</div>

		<div class="border border-[#262626] rounded-xl overflow-hidden bg-[#171717]">
			<table class="w-full text-left text-xs">
				<thead class="bg-[#141414] border-b border-[#262626] text-[#737373] uppercase tracking-wider font-semibold">
					<tr>
						<th class="px-5 py-3">CONTAINER</th>
						<th class="px-5 py-3">IMAGE</th>
						<th class="px-5 py-3">STATUS</th>
						<th class="px-5 py-3">PORTS</th>
						<th class="px-5 py-3">CPU / MEM</th>
						<th class="px-5 py-3 text-right">ACTION</th>
					</tr>
				</thead>
				<tbody class="divide-y divide-[#262626]">
					{#each containers as c (c.id)}
						<tr class="hover:bg-[#262626]/30 transition-colors">
							<td class="px-5 py-3.5 font-semibold text-[#FAFAFA]">
								<div class="flex items-center gap-2.5">
									<Package class="w-4 h-4 text-[#a1a1aa]" />
									<div>
										<p>{c.name}</p>
										<p class="text-[10px] font-mono text-[#737373]">{c.id}</p>
									</div>
								</div>
							</td>
							<td class="px-5 py-3.5 font-mono text-[#a1a1aa]">{c.image}</td>
							<td class="px-5 py-3.5">
								<Badge variant="outline" class="text-[10px] px-2 py-0.5 border-green-500/30 text-green-400 bg-green-500/10">
									{c.status}
								</Badge>
							</td>
							<td class="px-5 py-3.5 font-mono text-[#a1a1aa]">{c.ports}</td>
							<td class="px-5 py-3.5 font-mono text-[#a1a1aa]">
								{c.cpuPercent}% · {c.memUsageMb} MB
							</td>
							<td class="px-5 py-3.5 text-right">
								<Button variant="ghost" size="sm" class="h-7 text-xs text-[#FAFAFA]">Logs</Button>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	</main>
</PageLayout>
