<script lang="ts">
	import { goto } from '$app/navigation';
	import { ChartLine, Cpu, MemoryStick, HardDrive, Network, ArrowDownUp, RefreshCw } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Progress } from '$lib/components/ui/progress';
	import { Badge } from '$lib/components/ui/badge';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	// Static monitoring data (20 points each)
	const cpuHistory = [32, 38, 45, 42, 51, 48, 55, 52, 58, 61, 56, 53, 49, 54, 57, 60, 55, 50, 52, 48];
	const memoryHistory = [62, 64, 66, 65, 68, 70, 69, 72, 71, 73, 74, 72, 71, 73, 75, 74, 72, 73, 71, 70];
	const diskHistory = [35, 36, 36, 37, 37, 38, 38, 39, 39, 40, 40, 41, 41, 42, 42, 43, 43, 44, 44, 45];
	const networkInHistory = [12, 18, 15, 22, 28, 25, 21, 17, 23, 30, 26, 24, 19, 21, 27, 32, 28, 24, 20, 19];
	const networkOutHistory = [8, 12, 10, 14, 18, 16, 13, 9, 15, 20, 17, 14, 11, 13, 16, 21, 18, 14, 12, 11];
	const blockReadHistory = [5, 8, 12, 6, 15, 9, 7, 18, 11, 6, 14, 8, 5, 10, 16, 7, 12, 9, 6, 8];
	const blockWriteHistory = [3, 5, 7, 4, 9, 6, 5, 11, 7, 4, 8, 5, 3, 6, 10, 5, 7, 5, 4, 5];
	const dockerDiskHistory = [18, 18, 19, 19, 20, 20, 21, 21, 22, 22, 23, 23, 24, 24, 25, 25, 26, 26, 27, 27];

	// Derived current values
	const currentCpu = cpuHistory[cpuHistory.length - 1];
	const currentMem = memoryHistory[memoryHistory.length - 1];
	const currentDisk = diskHistory[diskHistory.length - 1];
	const currentNetIn = networkInHistory[networkInHistory.length - 1];
	const currentNetOut = networkOutHistory[networkOutHistory.length - 1];
	const currentDockerDisk = dockerDiskHistory[dockerDiskHistory.length - 1];

	// System totals (static)
	const totalMemoryGB = 16;
	const usedMemoryGB = ((currentMem / 100) * totalMemoryGB).toFixed(1);
	const totalDiskGB = 512;
	const usedDiskGB = ((currentDisk / 100) * totalDiskGB).toFixed(0);
	const dockerDiskGB = ((currentDockerDisk / 100) * totalDiskGB).toFixed(0);

	let refreshing = $state(false);
	function refresh() {
		refreshing = true;
		setTimeout(() => { refreshing = false; }, 1000);
	}

	// SVG path helpers
	function sparklinePath(data: number[], width: number, height: number, padding = 4): string {
		const max = Math.max(...data);
		const min = Math.min(...data);
		const range = max - min || 1;
		const step = width / (data.length - 1);
		const usableHeight = height - padding * 2;

		return data
			.map((v, i) => {
				const x = i * step;
				const y = padding + usableHeight - ((v - min) / range) * usableHeight;
				return `${i === 0 ? 'M' : 'L'} ${x.toFixed(1)} ${y.toFixed(1)}`;
			})
			.join(' ');
	}

	function smoothPath(data: number[], width: number, height: number, padding = 4): string {
		const max = Math.max(...data);
		const min = Math.min(...data);
		const range = max - min || 1;
		const step = width / (data.length - 1);
		const usableHeight = height - padding * 2;

		const points = data.map((v, i) => ({
			x: i * step,
			y: padding + usableHeight - ((v - min) / range) * usableHeight
		}));

		if (points.length < 2) return '';

		let path = `M ${points[0].x.toFixed(1)} ${points[0].y.toFixed(1)}`;
		for (let i = 1; i < points.length; i++) {
			const prev = points[i - 1];
			const curr = points[i];
			const cpx = (prev.x + curr.x) / 2;
			path += ` C ${cpx.toFixed(1)} ${prev.y.toFixed(1)}, ${cpx.toFixed(1)} ${curr.y.toFixed(1)}, ${curr.x.toFixed(1)} ${curr.y.toFixed(1)}`;
		}
		return path;
	}

	function areaPath(data: number[], width: number, height: number, padding = 4): string {
		const line = smoothPath(data, width, height, padding);
		return `${line} L ${width} ${height} L 0 ${height} Z`;
	}
</script>

<PageLayout>
	<header class="flex items-center justify-between px-6 py-3 border-b border-border">
		<div class="flex items-center gap-2 text-sm">
			<ChartLine class="w-4 h-4 text-primary" />
			<span class="font-medium">Monitoring</span>
		</div>
		<div class="flex items-center gap-2">
			<Badge variant="outline" class="text-[10px]">Live</Badge>
			<Button variant="outline" size="sm" class="h-7 text-xs gap-1.5" onclick={refresh}>
				<RefreshCw class="w-3 h-3 {refreshing ? 'animate-spin' : ''}" />
				Refresh
			</Button>
		</div>
	</header>

	<main class="flex-1 p-6 animate-fade-up">
		<div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
			<!-- 1. CPU Usage -->
			<Card.Root>
				<Card.Header class="pb-2">
					<Card.Title class="text-sm font-medium flex items-center gap-2">
						<Cpu class="w-4 h-4 text-[oklch(var(--chart-1))]" />
						CPU Usage
					</Card.Title>
				</Card.Header>
				<Card.Content class="space-y-3">
					<div class="flex items-baseline justify-between">
						<span class="text-2xl font-bold">{currentCpu}%</span>
						<span class="text-xs text-muted-foreground">of total capacity</span>
					</div>
					<Progress value={currentCpu} class="h-1.5" />
					<div class="h-[200px] relative">
						<svg viewBox="0 0 400 120" class="w-full h-full" preserveAspectRatio="none">
							<defs>
								<linearGradient id="cpuGradient" x1="0%" y1="0%" x2="0%" y2="100%">
									<stop offset="0%" stop-color="oklch(var(--chart-1))" stop-opacity="0.4" />
									<stop offset="100%" stop-color="oklch(var(--chart-1))" stop-opacity="0.02" />
								</linearGradient>
							</defs>
							<path d={areaPath(cpuHistory, 400, 120)} fill="url(#cpuGradient)" />
							<path d={smoothPath(cpuHistory, 400, 120)} fill="none" stroke="oklch(var(--chart-1))" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
						</svg>
						<div class="absolute top-1 left-2 text-[10px] text-muted-foreground">{Math.max(...cpuHistory)}%</div>
						<div class="absolute bottom-1 left-2 text-[10px] text-muted-foreground">{Math.min(...cpuHistory)}%</div>
					</div>
				</Card.Content>
			</Card.Root>

			<!-- 2. Memory Usage -->
			<Card.Root>
				<Card.Header class="pb-2">
					<Card.Title class="text-sm font-medium flex items-center gap-2">
						<MemoryStick class="w-4 h-4 text-[oklch(var(--chart-2))]" />
						Memory Usage
					</Card.Title>
				</Card.Header>
				<Card.Content class="space-y-3">
					<div class="flex items-baseline justify-between">
						<span class="text-2xl font-bold">{usedMemoryGB} GB</span>
						<span class="text-xs text-muted-foreground">/ {totalMemoryGB} GB ({currentMem}%)</span>
					</div>
					<Progress value={currentMem} class="h-1.5" />
					<div class="h-[200px] relative">
						<svg viewBox="0 0 400 120" class="w-full h-full" preserveAspectRatio="none">
							<defs>
								<linearGradient id="memGradient" x1="0%" y1="0%" x2="0%" y2="100%">
									<stop offset="0%" stop-color="oklch(var(--chart-2))" stop-opacity="0.4" />
									<stop offset="100%" stop-color="oklch(var(--chart-2))" stop-opacity="0.02" />
								</linearGradient>
							</defs>
							<path d={areaPath(memoryHistory, 400, 120)} fill="url(#memGradient)" />
							<path d={smoothPath(memoryHistory, 400, 120)} fill="none" stroke="oklch(var(--chart-2))" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
						</svg>
						<div class="absolute top-1 left-2 text-[10px] text-muted-foreground">{Math.max(...memoryHistory)}%</div>
						<div class="absolute bottom-1 left-2 text-[10px] text-muted-foreground">{Math.min(...memoryHistory)}%</div>
					</div>
				</Card.Content>
			</Card.Root>

			<!-- 3. Disk Space -->
			<Card.Root>
				<Card.Header class="pb-2">
					<Card.Title class="text-sm font-medium flex items-center gap-2">
						<HardDrive class="w-4 h-4 text-[oklch(var(--chart-3))]" />
						Disk Space
					</Card.Title>
				</Card.Header>
				<Card.Content class="space-y-3">
					<div class="flex items-baseline justify-between">
						<span class="text-2xl font-bold">{usedDiskGB} GB</span>
						<span class="text-xs text-muted-foreground">/ {totalDiskGB} GB ({currentDisk}%)</span>
					</div>
					<Progress value={currentDisk} class="h-1.5" />
					<div class="h-[200px] relative">
						<svg viewBox="0 0 400 120" class="w-full h-full" preserveAspectRatio="none">
							<defs>
								<linearGradient id="diskGradient" x1="0%" y1="0%" x2="0%" y2="100%">
									<stop offset="0%" stop-color="oklch(var(--chart-3))" stop-opacity="0.4" />
									<stop offset="100%" stop-color="oklch(var(--chart-3))" stop-opacity="0.02" />
								</linearGradient>
							</defs>
							<path d={areaPath(diskHistory, 400, 120)} fill="url(#diskGradient)" />
							<path d={smoothPath(diskHistory, 400, 120)} fill="none" stroke="oklch(var(--chart-3))" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
						</svg>
						<div class="absolute top-1 left-2 text-[10px] text-muted-foreground">{Math.max(...diskHistory)}%</div>
						<div class="absolute bottom-1 left-2 text-[10px] text-muted-foreground">{Math.min(...diskHistory)}%</div>
					</div>
				</Card.Content>
			</Card.Root>

			<!-- 4. Network I/O -->
			<Card.Root>
				<Card.Header class="pb-2">
					<Card.Title class="text-sm font-medium flex items-center gap-2">
						<Network class="w-4 h-4 text-[oklch(var(--chart-4))]" />
						Network I/O
					</Card.Title>
				</Card.Header>
				<Card.Content class="space-y-3">
					<div class="flex items-baseline justify-between">
						<span class="text-2xl font-bold">↓ {currentNetIn} MB/s</span>
						<span class="text-xs text-muted-foreground">↑ {currentNetOut} MB/s</span>
					</div>
					<div class="h-[200px] relative">
						<svg viewBox="0 0 400 120" class="w-full h-full" preserveAspectRatio="none">
							<defs>
								<linearGradient id="netInGradient" x1="0%" y1="0%" x2="0%" y2="100%">
									<stop offset="0%" stop-color="oklch(var(--chart-1))" stop-opacity="0.3" />
									<stop offset="100%" stop-color="oklch(var(--chart-1))" stop-opacity="0.02" />
								</linearGradient>
								<linearGradient id="netOutGradient" x1="0%" y1="0%" x2="0%" y2="100%">
									<stop offset="0%" stop-color="oklch(var(--chart-4))" stop-opacity="0.3" />
									<stop offset="100%" stop-color="oklch(var(--chart-4))" stop-opacity="0.02" />
								</linearGradient>
							</defs>
							<path d={areaPath(networkInHistory, 400, 120)} fill="url(#netInGradient)" />
							<path d={smoothPath(networkInHistory, 400, 120)} fill="none" stroke="oklch(var(--chart-1))" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
							<path d={areaPath(networkOutHistory, 400, 120)} fill="url(#netOutGradient)" />
							<path d={smoothPath(networkOutHistory, 400, 120)} fill="none" stroke="oklch(var(--chart-4))" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" stroke-dasharray="4 2" />
						</svg>
						<div class="absolute top-1 left-2 text-[10px] text-muted-foreground">{Math.max(...networkInHistory)} MB/s</div>
						<div class="absolute bottom-1 left-2 text-[10px] text-muted-foreground">{Math.min(...networkOutHistory)} MB/s</div>
					</div>
					<div class="flex items-center gap-4 text-xs">
						<div class="flex items-center gap-1.5">
							<span class="w-3 h-0.5 rounded bg-[oklch(var(--chart-1))]"></span>
							<span class="text-muted-foreground">Inbound</span>
						</div>
						<div class="flex items-center gap-1.5">
							<span class="w-3 h-0.5 rounded bg-[oklch(var(--chart-4))] opacity-70"></span>
							<span class="text-muted-foreground">Outbound</span>
						</div>
					</div>
				</Card.Content>
			</Card.Root>

			<!-- 5. Block I/O -->
			<Card.Root>
				<Card.Header class="pb-2">
					<Card.Title class="text-sm font-medium flex items-center gap-2">
						<ArrowDownUp class="w-4 h-4 text-[oklch(var(--chart-5))]" />
						Block I/O
					</Card.Title>
				</Card.Header>
				<Card.Content class="space-y-3">
					<div class="flex items-baseline justify-between">
						<span class="text-2xl font-bold">{blockReadHistory[blockReadHistory.length - 1]} MB/s</span>
						<span class="text-xs text-muted-foreground">Read / {blockWriteHistory[blockWriteHistory.length - 1]} MB/s Write</span>
					</div>
					<div class="h-[200px] relative">
						<svg viewBox="0 0 400 120" class="w-full h-full" preserveAspectRatio="none">
							<defs>
								<linearGradient id="blockReadGradient" x1="0%" y1="0%" x2="0%" y2="100%">
									<stop offset="0%" stop-color="oklch(var(--chart-5))" stop-opacity="0.3" />
									<stop offset="100%" stop-color="oklch(var(--chart-5))" stop-opacity="0.02" />
								</linearGradient>
								<linearGradient id="blockWriteGradient" x1="0%" y1="0%" x2="0%" y2="100%">
									<stop offset="0%" stop-color="oklch(var(--chart-3))" stop-opacity="0.3" />
									<stop offset="100%" stop-color="oklch(var(--chart-3))" stop-opacity="0.02" />
								</linearGradient>
							</defs>
							<path d={areaPath(blockReadHistory, 400, 120)} fill="url(#blockReadGradient)" />
							<path d={smoothPath(blockReadHistory, 400, 120)} fill="none" stroke="oklch(var(--chart-5))" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
							<path d={areaPath(blockWriteHistory, 400, 120)} fill="url(#blockWriteGradient)" />
							<path d={smoothPath(blockWriteHistory, 400, 120)} fill="none" stroke="oklch(var(--chart-3))" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" stroke-dasharray="4 2" />
						</svg>
						<div class="absolute top-1 left-2 text-[10px] text-muted-foreground">{Math.max(...blockReadHistory)} MB/s</div>
						<div class="absolute bottom-1 left-2 text-[10px] text-muted-foreground">{Math.min(...blockWriteHistory)} MB/s</div>
					</div>
					<div class="flex items-center gap-4 text-xs">
						<div class="flex items-center gap-1.5">
							<span class="w-3 h-0.5 rounded bg-[oklch(var(--chart-5))]"></span>
							<span class="text-muted-foreground">Read</span>
						</div>
						<div class="flex items-center gap-1.5">
							<span class="w-3 h-0.5 rounded bg-[oklch(var(--chart-3))] opacity-70"></span>
							<span class="text-muted-foreground">Write</span>
						</div>
					</div>
				</Card.Content>
			</Card.Root>

			<!-- 6. Docker Disk Usage -->
			<Card.Root>
				<Card.Header class="pb-2">
					<Card.Title class="text-sm font-medium flex items-center gap-2">
						<HardDrive class="w-4 h-4 text-[oklch(var(--chart-5))]" />
						Docker Disk Usage
					</Card.Title>
				</Card.Header>
				<Card.Content class="space-y-3">
					<div class="flex items-baseline justify-between">
						<span class="text-2xl font-bold">{dockerDiskGB} GB</span>
						<span class="text-xs text-muted-foreground">/ {totalDiskGB} GB ({currentDockerDisk}%)</span>
					</div>
					<Progress value={currentDockerDisk} class="h-1.5" />
					<div class="h-[200px] relative">
						<svg viewBox="0 0 400 120" class="w-full h-full" preserveAspectRatio="none">
							<defs>
								<linearGradient id="dockerGradient" x1="0%" y1="0%" x2="0%" y2="100%">
									<stop offset="0%" stop-color="oklch(var(--chart-5))" stop-opacity="0.4" />
									<stop offset="100%" stop-color="oklch(var(--chart-5))" stop-opacity="0.02" />
								</linearGradient>
							</defs>
							<path d={areaPath(dockerDiskHistory, 400, 120)} fill="url(#dockerGradient)" />
							<path d={smoothPath(dockerDiskHistory, 400, 120)} fill="none" stroke="oklch(var(--chart-5))" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
						</svg>
						<div class="absolute top-1 left-2 text-[10px] text-muted-foreground">{Math.max(...dockerDiskHistory)}%</div>
						<div class="absolute bottom-1 left-2 text-[10px] text-muted-foreground">{Math.min(...dockerDiskHistory)}%</div>
					</div>
				</Card.Content>
			</Card.Root>
		</div>
	</main>
</PageLayout>
