<script lang="ts">
	import { goto } from '$app/navigation';
	import { ChartLine, RefreshCw } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import * as Chart from '$lib/components/ui/chart/index.js';
	import { Button } from '$lib/components/ui/button';
	import { LineChart, AreaChart, Area, ChartClipPath, ArcChart, Text } from 'layerchart';
	import { scaleUtc } from 'd3-scale';
	import { curveNatural } from 'd3-shape';
	import { cubicInOut } from 'svelte/easing';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	const now = Date.now();
	const tick = (i: number) => new Date(now - (19 - i) * 5000);

	const cpuSeries = [
		32, 38, 45, 42, 51, 48, 55, 52, 58, 61, 56, 53, 49, 54, 57, 60, 55, 50, 52, 48
	].map((v, i) => ({ date: tick(i), value: v }));
	const memorySeries = [
		62, 64, 66, 65, 68, 70, 69, 72, 71, 73, 74, 72, 71, 73, 75, 74, 72, 73, 71, 70
	].map((v, i) => ({ date: tick(i), usedGB: +(v * 0.16).toFixed(2) }));
	const diskSeries = [
		35, 36, 36, 37, 37, 38, 38, 39, 39, 40, 40, 41, 41, 42, 42, 43, 43, 44, 44, 45
	].map((v, i) => ({ date: tick(i), usedGB: +(v * 5.12).toFixed(0) }));
	const networkSeries = [
		12, 18, 15, 22, 28, 25, 21, 17, 23, 30, 26, 24, 19, 21, 27, 32, 28, 24, 20, 19
	].map((v, i) => ({
		date: tick(i),
		inMB: v,
		outMB: [8, 12, 10, 14, 18, 16, 13, 9, 15, 20, 17, 14, 11, 13, 16, 21, 18, 14, 12, 11][i]
	}));
	const blockSeries = [5, 8, 12, 6, 15, 9, 7, 18, 11, 6, 14, 8, 5, 10, 16, 7, 12, 9, 6, 8].map(
		(v, i) => ({
			date: tick(i),
			readMb: v,
			writeMb: [3, 5, 7, 4, 9, 6, 5, 11, 7, 4, 8, 5, 3, 6, 10, 5, 7, 5, 4, 5][i]
		})
	);

	const memTotalGB = 16;
	const diskTotalGB = 512;

	const dockerDiskItems = [
		{ key: 'images', label: 'Images', bytes: 4_200_000_000 },
		{ key: 'containers', label: 'Containers', bytes: 870_000_000 },
		{ key: 'volumes', label: 'Volumes', bytes: 2_100_000_000 },
		{ key: 'buildCache', label: 'Build Cache', bytes: 1_500_000_000 }
	];
	const dockerTotal = dockerDiskItems.reduce((s, d) => s + d.bytes, 0);

	function fmtBytes(b: number) {
		if (b >= 1e9) return `${(b / 1e9).toFixed(1)} GB`;
		if (b >= 1e6) return `${(b / 1e6).toFixed(0)} MB`;
		return `${(b / 1e3).toFixed(0)} KB`;
	}

	const currentCpu = cpuSeries.at(-1)!.value;
	const currentMem = memorySeries.at(-1)!.usedGB;
	const currentDisk = diskSeries.at(-1)!.usedGB;
	const currentNet = networkSeries.at(-1)!;
	const currentBlk = blockSeries.at(-1)!;

	const cpuConfig: Chart.ChartConfig = { value: { label: 'CPU %', color: 'var(--color-chart-1)' } };
	const memConfig: Chart.ChartConfig = {
		usedGB: { label: 'Memory GB', color: 'var(--color-chart-2)' }
	};
	const diskConfig: Chart.ChartConfig = {
		usedGB: { label: 'Disk GB', color: 'var(--color-chart-3)' }
	};
	const netConfig: Chart.ChartConfig = {
		inMB: { label: 'In MB', color: 'var(--color-chart-1)' },
		outMB: { label: 'Out MB', color: 'var(--color-chart-2)' }
	};
	const blkConfig: Chart.ChartConfig = {
		readMb: { label: 'Read MB', color: 'var(--color-chart-4)' },
		writeMb: { label: 'Write MB', color: 'var(--color-chart-5)' }
	};
	const dockerConfig: Chart.ChartConfig = {
		images: { label: 'Images', color: 'var(--color-chart-1)' },
		containers: { label: 'Containers', color: 'var(--color-chart-2)' },
		volumes: { label: 'Volumes', color: 'var(--color-chart-3)' },
		buildCache: { label: 'Build Cache', color: 'var(--color-chart-4)' }
	};

	const C = [
		'var(--chart-1)',
		'var(--chart-2)',
		'var(--chart-3)',
		'var(--chart-4)',
		'var(--chart-5)'
	];

	const xFmt = (v: Date) =>
		v.toLocaleTimeString('en-US', {
			hour: '2-digit',
			minute: '2-digit',
			second: '2-digit',
			hour12: false
		});

	let refreshing = $state(false);
	function refresh() {
		refreshing = true;
		setTimeout(() => (refreshing = false), 1000);
	}
</script>

<PageLayout>
	<header class="flex items-center justify-between px-6 py-3 border-b border-border">
		<div class="flex items-center gap-2 text-sm">
			<ChartLine class="w-4 h-4 text-primary" />
			<span class="font-medium">Monitoring</span>
		</div>
		<Button variant="outline" size="sm" class="h-7 text-xs gap-1.5" onclick={refresh}>
			<RefreshCw class="w-3 h-3 {refreshing ? 'animate-spin' : ''}" />
			Refresh
		</Button>
	</header>

	<main class="flex-1 p-6 overflow-y-auto">
		<div class="space-y-1 mb-6">
			<h1 class="text-2xl font-semibold tracking-tight">Monitoring</h1>
			<p class="text-sm text-muted-foreground">Watch the usage of your server in real time</p>
		</div>

		<div class="grid gap-6 lg:grid-cols-2">
			<!-- 1. CPU -->
			<Card.Root class="bg-background">
				<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
					<Card.Title class="text-sm font-medium">CPU Usage</Card.Title>
					<span class="text-2xl font-bold" style="color:{C[0]}">{currentCpu}%</span>
				</Card.Header>
				<Card.Content>
					<div class="flex flex-col gap-2 w-full">
						<span class="text-sm text-muted-foreground">Used: {currentCpu}% of total capacity</span>
						<div class="w-full h-2 rounded-full bg-muted overflow-hidden">
							<div
								class="h-full rounded-full transition-all duration-500"
								style="width:{currentCpu}%; background:{C[0]}"
							></div>
						</div>
						<Chart.Container
							config={cpuConfig}
							class="mt-2 w-full ![aspect-ratio:unset]"
							style="height:160px"
						>
							<LineChart
								data={cpuSeries}
								x="date"
								xScale={scaleUtc()}
								axis="x"
								yDomain={[0, 100]}
								series={[{ key: 'value', label: 'CPU %', color: cpuConfig.value.color }]}
								props={{
									spline: { curve: curveNatural, motion: 'tween', strokeWidth: 2 },
									xAxis: { format: xFmt },
									highlight: { points: { r: 4 } }
								}}
							>
								{#snippet tooltip()}<Chart.Tooltip hideLabel />{/snippet}
							</LineChart>
						</Chart.Container>
					</div>
				</Card.Content>
			</Card.Root>

			<!-- 2. Memory -->
			<Card.Root class="bg-background">
				<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
					<Card.Title class="text-sm font-medium">Memory Usage</Card.Title>
					<span class="text-2xl font-bold" style="color:{C[1]}">{currentMem} GB</span>
				</Card.Header>
				<Card.Content>
					<div class="flex flex-col gap-2 w-full">
						<span class="text-sm text-muted-foreground"
							>Used: {currentMem} GB / Limit: {memTotalGB} GB</span
						>
						<div class="w-full h-2 rounded-full bg-muted overflow-hidden">
							<div
								class="h-full rounded-full transition-all duration-500"
								style="width:{(currentMem / memTotalGB) * 100}%; background:{C[1]}"
							></div>
						</div>
						<Chart.Container
							config={memConfig}
							class="mt-2 w-full"
							style="height:160px;aspect-ratio:unset"
						>
							<LineChart
								data={memorySeries}
								x="date"
								xScale={scaleUtc()}
								axis="x"
								yDomain={[0, memTotalGB]}
								series={[{ key: 'usedGB', label: 'Memory GB', color: memConfig.usedGB.color }]}
								props={{
									spline: { curve: curveNatural, motion: 'tween', strokeWidth: 2 },
									xAxis: { format: xFmt },
									highlight: { points: { r: 4 } }
								}}
							>
								{#snippet tooltip()}<Chart.Tooltip hideLabel />{/snippet}
							</LineChart>
						</Chart.Container>
					</div>
				</Card.Content>
			</Card.Root>

			<!-- 3. Disk -->
			<Card.Root class="bg-background">
				<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
					<Card.Title class="text-sm font-medium">Disk Space</Card.Title>
					<span class="text-2xl font-bold" style="color:{C[2]}">{currentDisk} GB</span>
				</Card.Header>
				<Card.Content>
					<div class="flex flex-col gap-2 w-full">
						<span class="text-sm text-muted-foreground"
							>Used: {currentDisk} GB / Limit: {diskTotalGB} GB</span
						>
						<div class="w-full h-2 rounded-full bg-muted overflow-hidden">
							<div
								class="h-full rounded-full transition-all duration-500"
								style="width:{(currentDisk / diskTotalGB) * 100}%; background:{C[2]}"
							></div>
						</div>
						<Chart.Container
							config={diskConfig}
							class="mt-2 w-full"
							style="height:160px;aspect-ratio:unset"
						>
							<LineChart
								data={diskSeries}
								x="date"
								xScale={scaleUtc()}
								axis="x"
								yDomain={[0, diskTotalGB]}
								series={[{ key: 'usedGB', label: 'Disk GB', color: diskConfig.usedGB.color }]}
								props={{
									spline: { curve: curveNatural, motion: 'tween', strokeWidth: 2 },
									xAxis: { format: xFmt },
									highlight: { points: { r: 4 } }
								}}
							>
								{#snippet tooltip()}<Chart.Tooltip hideLabel />{/snippet}
							</LineChart>
						</Chart.Container>
					</div>
				</Card.Content>
			</Card.Root>

			<!-- 4. Docker Disk Usage -->
			<Card.Root class="bg-background">
				<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
					<Card.Title class="text-sm font-medium">Docker Disk Usage</Card.Title>
					<span class="text-sm text-muted-foreground">Total: {fmtBytes(dockerTotal)}</span>
				</Card.Header>
				<Card.Content class="flex flex-col items-center gap-3 pt-0">
					<div class="w-full" style="height:230px">
						<Chart.Container config={dockerConfig} class="w-full h-full" style="aspect-ratio:unset">
							<ArcChart
								data={dockerDiskItems}
								key="key"
								label="label"
								value="bytes"
								outerRadius={-12}
								innerRadius={-18}
								padding={20}
								range={[90, -270]}
								maxValue={dockerTotal}
								cornerRadius={8}
								padAngle={0.08}
								series={dockerDiskItems.map((d) => ({
									key: d.key,
									label: d.label,
									color: `var(--color-${d.key})`,
									data: [d]
								}))}
								props={{ arc: { track: { fill: 'var(--color-muted)' }, motion: 'tween' } }}
								tooltipContext={false}
							>
								{#snippet belowMarks()}
									<circle cx="0" cy="0" r="65" fill="var(--color-background)" />
								{/snippet}
								{#snippet aboveMarks()}
									<Text
										value={fmtBytes(dockerTotal)}
										textAnchor="middle"
										verticalAnchor="middle"
										class="fill-foreground font-bold"
										style="font-size:15px"
										dy={0}
									/>
								{/snippet}
							</ArcChart>
						</Chart.Container>
					</div>
					<div class="flex flex-wrap justify-center gap-x-5 gap-y-1.5">
						{#each dockerDiskItems as item, i (item.key)}
							<div class="flex items-center gap-1.5 text-xs text-muted-foreground">
								<span
									class="inline-block w-2.5 h-2.5 rounded-[2px] shrink-0"
									style="background:{C[i]}"
								></span>
								{item.label}
								<span class="font-mono font-medium text-foreground">{fmtBytes(item.bytes)}</span>
							</div>
						{/each}
					</div>
				</Card.Content>
			</Card.Root>

			<!-- 5. Block I/O — fixed card height, flex layout so chart fills remaining space -->
			<Card.Root class="bg-background flex flex-col" style="height:340px">
				<Card.Header class="shrink-0 pb-2">
					<Card.Title class="text-sm font-medium">Block I/O</Card.Title>
					<div class="flex items-center gap-5 mt-1.5 text-xs text-muted-foreground">
						<div class="flex items-center gap-1.5">
							<span
								class="inline-block w-2.5 h-2.5 rounded-[2px] shrink-0"
								style="background:{C[3]}"
							></span>
							Read: <strong class="text-foreground ml-0.5">{currentBlk.readMb} MB/s</strong>
						</div>
						<div class="flex items-center gap-1.5">
							<span
								class="inline-block w-2.5 h-2.5 rounded-[2px] shrink-0"
								style="background:{C[4]}"
							></span>
							Write: <strong class="text-foreground ml-0.5">{currentBlk.writeMb} MB/s</strong>
						</div>
					</div>
				</Card.Header>
				<Card.Content class="flex-1 min-h-0 px-4 pb-4 pt-0">
					<Chart.Container config={blkConfig} class="![aspect-ratio:unset] w-full h-full">
						<AreaChart
							data={blockSeries}
							x="date"
							xScale={scaleUtc()}
							legend={false}
							series={[
								{ key: 'readMb', label: 'Read MB', color: blkConfig.readMb.color },
								{ key: 'writeMb', label: 'Write MB', color: blkConfig.writeMb.color }
							]}
							props={{ xAxis: { format: xFmt }, yAxis: { format: () => '' } }}
						>
							{#snippet marks({ context })}
								<defs>
									<linearGradient id="fillRead" x1="0" y1="0" x2="0" y2="1">
										<stop offset="5%" stop-color="var(--color-readMb)" stop-opacity={1.0} />
										<stop offset="95%" stop-color="var(--color-readMb)" stop-opacity={0.1} />
									</linearGradient>
									<linearGradient id="fillWrite" x1="0" y1="0" x2="0" y2="1">
										<stop offset="5%" stop-color="var(--color-writeMb)" stop-opacity={0.8} />
										<stop offset="95%" stop-color="var(--color-writeMb)" stop-opacity={0.1} />
									</linearGradient>
								</defs>
								<ChartClipPath
									initialWidth={0}
									motion={{ width: { type: 'tween', duration: 1000, easing: cubicInOut } }}
								>
									{#each context.series.visibleSeries as s (s.key)}
										<Area
											seriesKey={s.key}
											curve={curveNatural}
											fillOpacity={0.4}
											line={{ class: 'stroke-2' }}
											motion="tween"
											{...s.props}
											fill={s.key === 'readMb' ? 'url(#fillRead)' : 'url(#fillWrite)'}
										/>
									{/each}
								</ChartClipPath>
							{/snippet}
							{#snippet tooltip()}
								<Chart.Tooltip
									labelFormatter={(v: Date) => v.toLocaleTimeString()}
									indicator="line"
								/>
							{/snippet}
						</AreaChart>
					</Chart.Container>
				</Card.Content>
			</Card.Root>

			<!-- 6. Network I/O — same fixed-height flex layout -->
			<Card.Root class="bg-background flex flex-col" style="height:340px">
				<Card.Header class="shrink-0 pb-2">
					<Card.Title class="text-sm font-medium">Network I/O</Card.Title>
					<div class="flex items-center gap-5 mt-1.5 text-xs text-muted-foreground">
						<div class="flex items-center gap-1.5">
							<span
								class="inline-block w-2.5 h-2.5 rounded-[2px] shrink-0"
								style="background:{C[0]}"
							></span>
							In: <strong class="text-foreground ml-0.5">{currentNet.inMB} MB/s</strong>
						</div>
						<div class="flex items-center gap-1.5">
							<span
								class="inline-block w-2.5 h-2.5 rounded-[2px] shrink-0"
								style="background:{C[1]}"
							></span>
							Out: <strong class="text-foreground ml-0.5">{currentNet.outMB} MB/s</strong>
						</div>
					</div>
				</Card.Header>
				<Card.Content class="flex-1 min-h-0 px-4 pb-4 pt-0">
					<Chart.Container config={netConfig} class="![aspect-ratio:unset] w-full h-full">
						<AreaChart
							data={networkSeries}
							x="date"
							xScale={scaleUtc()}
							legend={false}
							series={[
								{ key: 'inMB', label: 'In MB', color: netConfig.inMB.color },
								{ key: 'outMB', label: 'Out MB', color: netConfig.outMB.color }
							]}
							props={{ xAxis: { format: xFmt }, yAxis: { format: () => '' } }}
						>
							{#snippet marks({ context })}
								<defs>
									<linearGradient id="fillIn" x1="0" y1="0" x2="0" y2="1">
										<stop offset="5%" stop-color="var(--color-inMB)" stop-opacity={1.0} />
										<stop offset="95%" stop-color="var(--color-inMB)" stop-opacity={0.1} />
									</linearGradient>
									<linearGradient id="fillOut" x1="0" y1="0" x2="0" y2="1">
										<stop offset="5%" stop-color="var(--color-outMB)" stop-opacity={0.8} />
										<stop offset="95%" stop-color="var(--color-outMB)" stop-opacity={0.1} />
									</linearGradient>
								</defs>
								<ChartClipPath
									initialWidth={0}
									motion={{ width: { type: 'tween', duration: 1000, easing: cubicInOut } }}
								>
									{#each context.series.visibleSeries as s (s.key)}
										<Area
											seriesKey={s.key}
											curve={curveNatural}
											fillOpacity={0.4}
											line={{ class: 'stroke-2' }}
											motion="tween"
											{...s.props}
											fill={s.key === 'inMB' ? 'url(#fillIn)' : 'url(#fillOut)'}
										/>
									{/each}
								</ChartClipPath>
							{/snippet}
							{#snippet tooltip()}
								<Chart.Tooltip
									labelFormatter={(v: Date) => v.toLocaleTimeString()}
									indicator="line"
								/>
							{/snippet}
						</AreaChart>
					</Chart.Container>
				</Card.Content>
			</Card.Root>
		</div>
	</main>
</PageLayout>
