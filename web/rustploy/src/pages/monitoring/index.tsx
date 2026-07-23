import React, { useState, useEffect, useRef } from 'react';
import { useNavigate } from 'react-router-dom';
import {
	Activity,
	Play,
	Pause,
	RefreshCw,
	Server,
	Cpu,
	ArrowUpRight,
	ChevronsUpDown,
	Check
} from 'lucide-react';
import { PageLayout } from '@/components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { getMonitoringMock, generateLoopingTick, type TelemetrySeriesPoint, type TelemetryTimeRange } from '$lib/mocks';
import { Card } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { toastSuccess, toastInfo } from '$lib/toast';

const nodesList = [
	{ id: 'production-01', label: 'production-01 (Ubuntu 24.04)' },
	{ id: 'staging-worker-02', label: 'staging-worker-02 (Debian 12)' },
	{ id: 'backup-replica-03', label: 'backup-replica-03 (Ubuntu 22.04)' }
];

const timeScaleRanges: { id: TelemetryTimeRange; label: string }[] = [
	{ id: '1m', label: '1m' },
	{ id: '5m', label: '5m' },
	{ id: '15m', label: '15m' },
	{ id: '1d', label: '1d' },
	{ id: '7d', label: '7d' },
	{ id: '1mth', label: '1mth' },
	{ id: '1yr', label: '1yr' },
	{ id: 'Max', label: 'Max' }
];

export default function MonitoringPage() {
	const navigate = useNavigate();

	const [isStreaming, setIsStreaming] = useState(true);
	const [selectedNode, setSelectedNode] = useState('production-01');
	const [nodeMenuOpen, setNodeMenuOpen] = useState(false);
	const [timeRange, setTimeRange] = useState<TelemetryTimeRange>('1m');

	// Advanced Viz Controls
	const [chartMode, setChartMode] = useState<'spline' | 'stepped' | 'area'>('spline');
	const [activeCoreFilter, setActiveCoreFilter] = useState<'all' | 'c1' | 'c3' | 'c5' | 'c7'>('all');

	// Hover interactive tooltips state for each graph
	const [cpuHoverIdx, setCpuHoverIdx] = useState<number | null>(null);
	const [netHoverIdx, setNetHoverIdx] = useState<number | null>(null);
	const [iopsHoverIdx, setIopsHoverIdx] = useState<number | null>(null);

	// Initial dataset loaded from mock provider
	const [telemetryBuffer, setTelemetryBuffer] = useState<TelemetrySeriesPoint[]>(() =>
		getMonitoringMock(selectedNode, 40, timeRange)
	);

	const tickIndexRef = useRef(40);

	// When node or time range changes, reset telemetry stream with rescaled mock
	useEffect(() => {
		const mockData = getMonitoringMock(selectedNode, 40, timeRange);
		setTelemetryBuffer(mockData);
		tickIndexRef.current = 40;
	}, [selectedNode, timeRange]);

	// Real-time live looping tick interval (runs continuously every 1000ms for short ranges)
	useEffect(() => {
		if (!isStreaming) return;

		const interval = setInterval(() => {
			tickIndexRef.current += 1;
			setTelemetryBuffer((prev) => {
				if (!prev || prev.length === 0) {
					return getMonitoringMock(selectedNode, 40, timeRange);
				}
				const lastPoint = prev[prev.length - 1];
				const nextPoint = generateLoopingTick(lastPoint, tickIndexRef.current);
				return [...prev.slice(1), nextPoint];
			});
		}, 1000);

		return () => clearInterval(interval);
	}, [isStreaming, selectedNode, timeRange]);

	// Convert numeric series to smooth SVG path (supporting Spline cubic curves or Stepped lines)
	const createSvgPath = (
		data: (number | undefined)[],
		width: number,
		height: number,
		maxVal = 100,
		mode: 'spline' | 'stepped' | 'area' = 'spline'
	) => {
		if (!data || data.length < 2) return '';
		const safeMax = maxVal <= 0 ? 100 : maxVal;
		const step = width / (data.length - 1);
		const paddingTop = 12;
		const paddingBottom = 12;
		const availableHeight = height - paddingTop - paddingBottom;

		const points = data.map((val, i) => {
			const numVal = typeof val === 'number' && !isNaN(val) ? val : 0;
			const clampedVal = Math.min(safeMax, Math.max(0, numVal));
			const x = i * step;
			const y = height - paddingBottom - (clampedVal / safeMax) * availableHeight;
			return { x: isNaN(x) ? 0 : x, y: isNaN(y) ? height / 2 : y };
		});

		if (mode === 'stepped') {
			let d = `M ${points[0].x},${points[0].y}`;
			for (let i = 0; i < points.length - 1; i++) {
				const curr = points[i];
				const next = points[i + 1];
				d += ` H ${next.x} V ${next.y}`;
			}
			return d;
		}

		let d = `M ${points[0].x},${points[0].y}`;
		for (let i = 0; i < points.length - 1; i++) {
			const curr = points[i];
			const next = points[i + 1];
			const cp1x = curr.x + step / 2;
			const cp1y = curr.y;
			const cp2x = next.x - step / 2;
			const cp2y = next.y;
			d += ` C ${cp1x},${cp1y} ${cp2x},${cp2y} ${next.x},${next.y}`;
		}
		return d;
	};

	const createAreaPath = (linePath: string, width: number, height: number) => {
		if (!linePath) return '';
		return `${linePath} L ${width},${height} L 0,${height} Z`;
	};

	// Helper to calculate X and Y coordinates for interactive hover dots
	const getPointCoords = (val: number, idx: number, total: number, width: number, height: number, maxVal = 100) => {
		const safeMax = maxVal <= 0 ? 100 : maxVal;
		const step = width / Math.max(1, total - 1);
		const paddingTop = 12;
		const paddingBottom = 12;
		const availableHeight = height - paddingTop - paddingBottom;
		const clampedVal = Math.min(safeMax, Math.max(0, val));
		const x = idx * step;
		const y = height - paddingBottom - (clampedVal / safeMax) * availableHeight;
		return { x: isNaN(x) ? 0 : x, y: isNaN(y) ? height / 2 : y };
	};

	// Mouse hover index calculation
	const handleMouseMove = (e: React.MouseEvent<SVGSVGElement>, total: number, setHoverIdx: (idx: number | null) => void) => {
		const rect = e.currentTarget.getBoundingClientRect();
		const mouseX = e.clientX - rect.left;
		const ratio = Math.max(0, Math.min(1, mouseX / rect.width));
		const idx = Math.round(ratio * (total - 1));
		setHoverIdx(idx);
	};

	const latest: TelemetrySeriesPoint = telemetryBuffer && telemetryBuffer.length > 0
		? telemetryBuffer[telemetryBuffer.length - 1]
		: {
				timestamp: '12:00:00',
				cpuCore1: 52, cpuCore2: 48, cpuCore3: 56, cpuCore4: 45,
				cpuCore5: 40, cpuCore6: 46, cpuCore7: 58, cpuCore8: 38,
				cpuAvg: 48.0, ramUsedGb: 6.72, ramTotalGb: 16.0, ramPercent: 42,
				netRxMbps: 48.0, netTxMbps: 26.0, diskReadIops: 1100, diskWriteIops: 580,
				httpRps: 850, httpLatencyP95Ms: 14
		  };

	// Data Science Metrics Calculations
	const cpuValues = telemetryBuffer.map((d) => d.cpuAvg);
	const cpuMin = Math.min(...cpuValues).toFixed(1);
	const cpuMax = Math.max(...cpuValues).toFixed(1);
	const cpuAvgStats = (cpuValues.reduce((a, b) => a + b, 0) / Math.max(1, cpuValues.length)).toFixed(1);

	const netRxValues = telemetryBuffer.map((d) => d.netRxMbps);
	const netRxMax = Math.max(...netRxValues).toFixed(1);
	const netRxAvg = (netRxValues.reduce((a, b) => a + b, 0) / Math.max(1, netRxValues.length)).toFixed(1);

	const iopsValues = telemetryBuffer.map((d) => d.diskReadIops);
	const iopsMax = Math.max(...iopsValues).toFixed(0);

	// CPU 8-Core Live Heatmap Array
	const coresData = [
		{ name: 'C1', val: latest.cpuCore1, col: 'bg-blue-500' },
		{ name: 'C2', val: latest.cpuCore2, col: 'bg-blue-400' },
		{ name: 'C3', val: latest.cpuCore3, col: 'bg-emerald-500' },
		{ name: 'C4', val: latest.cpuCore4, col: 'bg-emerald-400' },
		{ name: 'C5', val: latest.cpuCore5, col: 'bg-purple-500' },
		{ name: 'C6', val: latest.cpuCore6, col: 'bg-purple-400' },
		{ name: 'C7', val: latest.cpuCore7, col: 'bg-amber-500' },
		{ name: 'C8', val: latest.cpuCore8, col: 'bg-amber-400' }
	];

	return (
		<PageLayout>
			<div className="m-3.5 flex-1 flex flex-col min-h-0 bg-[#171717] border border-[#272727] rounded-2xl shadow-xl p-5 space-y-4 overflow-hidden select-none animate-fade-up">
				{/* Top Controls & Stock Market Time Scale Header */}
				<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 shrink-0">
					<div>
						<h1 className="text-2xl font-extrabold tracking-tight text-[#FAFAFA] flex items-center gap-2.5">
							<Activity className="w-6 h-6 text-green-400" />
							Real-Time Telemetry & Data Science Analytics
						</h1>
						<p className="text-xs text-[#a1a1aa] mt-0.5">
							Stock-market style time zoom (1m to Max) · Multi-core heatmap · Crosshair analytics
						</p>
					</div>

					<div className="flex flex-wrap items-center gap-2.5">
						{/* Stock-Market Style Time Scale Range Selector */}
						<div className="flex items-center gap-0.5 bg-[#141414] p-1 rounded-xl border border-[#272727]">
							{timeScaleRanges.map((r) => (
								<button
									key={r.id}
									onClick={() => {
										setTimeRange(r.id);
										toastInfo(`Rescaled time zoom to ${r.label}`);
									}}
									className={`px-2 py-1 text-xs font-semibold rounded-lg transition-all cursor-pointer ${
										timeRange === r.id
											? 'bg-[#272727] text-[#FAFAFA] font-bold shadow-xs'
											: 'text-[#737373] hover:text-[#FAFAFA]'
									}`}
								>
									{r.label}
								</button>
							))}
						</div>

						{/* Chart Render Style Switcher */}
						<div className="flex items-center gap-1 bg-[#141414] p-1 rounded-xl border border-[#272727]">
							{(['spline', 'stepped', 'area'] as const).map((mode) => (
								<button
									key={mode}
									onClick={() => setChartMode(mode)}
									className={`px-2.5 py-1 text-xs font-semibold rounded-lg capitalize transition-all cursor-pointer ${
										chartMode === mode ? 'bg-[#272727] text-[#FAFAFA] shadow-xs' : 'text-[#737373] hover:text-[#FAFAFA]'
									}`}
								>
									{mode}
								</button>
							))}
						</div>

						{/* Node Selector Menu */}
						<div className="relative">
							<button
								onClick={() => setNodeMenuOpen(!nodeMenuOpen)}
								className="flex items-center gap-2 px-3 py-1.5 rounded-xl bg-[#141414] hover:bg-[#272727] border border-[#272727] text-xs transition-colors cursor-pointer text-[#FAFAFA] font-medium"
							>
								<Server className="w-3.5 h-3.5 text-[#a1a1aa]" />
								<span>{nodesList.find((n) => n.id === selectedNode)?.label}</span>
								<ChevronsUpDown className="w-3.5 h-3.5 text-[#a1a1aa] ml-1" />
							</button>

							{nodeMenuOpen && (
								<div className="absolute top-full right-0 mt-1.5 z-50 w-64 bg-[#18181b] border border-[#272727] rounded-xl shadow-2xl p-1 space-y-0.5 animate-fade-up">
									{nodesList.map((node) => (
										<button
											key={node.id}
											onClick={() => {
												setSelectedNode(node.id);
												setNodeMenuOpen(false);
											}}
											className={`w-full flex items-center justify-between px-3 py-2 rounded-lg text-xs transition-colors cursor-pointer text-left ${
												selectedNode === node.id
													? 'bg-[#272727] text-[#FAFAFA] font-bold'
													: 'text-[#a1a1aa] hover:bg-[#272727]/60 hover:text-[#FAFAFA]'
											}`}
										>
											<span>{node.label}</span>
											{selectedNode === node.id && <Check className="w-3.5 h-3.5 text-[#FAFAFA]" />}
										</button>
									))}
								</div>
							)}
						</div>

						{/* Stream Toggle Button */}
						<Button
							variant="secondary"
							size="sm"
							onClick={() => {
								setIsStreaming(!isStreaming);
								toastInfo(isStreaming ? 'Live telemetry stream paused' : 'Resumed live telemetry stream');
							}}
							className={`gap-2 text-xs font-semibold rounded-xl border transition-all cursor-pointer ${
								isStreaming
									? 'bg-green-500/10 text-green-400 border-green-500/30 hover:bg-green-500/20'
									: 'bg-amber-500/10 text-amber-400 border-amber-500/30 hover:bg-amber-500/20'
							}`}
						>
							{isStreaming ? (
								<>
									<Pause className="w-3.5 h-3.5" />
									<span className="relative flex h-2 w-2">
										<span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75" />
										<span className="relative inline-flex rounded-full h-2 w-2 bg-green-500" />
									</span>
									Live Loop (1s)
								</>
							) : (
								<>
									<Play className="w-3.5 h-3.5" /> Paused
								</>
							)}
						</Button>

						<button
							onClick={() => {
								setTelemetryBuffer(getMonitoringMock(selectedNode, 40, timeRange));
								tickIndexRef.current = 40;
								toastSuccess('Telemetry stream buffer reset');
							}}
							className="p-2 rounded-xl border border-[#272727] bg-[#141414] text-[#a1a1aa] hover:text-[#FAFAFA] hover:bg-[#272727] transition-colors cursor-pointer"
							title="Reset Telemetry Buffer"
						>
							<RefreshCw className="w-4 h-4" />
						</button>
					</div>
				</div>

				{/* Quick Stats Summary Grid & 8-Core Heatmap Matrix Strip */}
				<div className="grid grid-cols-2 lg:grid-cols-5 gap-3 shrink-0">
					<Card className="bg-[#141414] border border-[#272727] rounded-xl p-3.5 shadow-md">
						<p className="text-[10px] font-semibold text-[#a1a1aa] uppercase tracking-wider">CPU AVG LOAD</p>
						<p className="text-2xl font-extrabold text-[#FAFAFA] font-mono mt-0.5">{latest.cpuAvg}%</p>
						<div className="flex items-center gap-1 mt-0.5 text-[10px] text-green-400 font-mono">
							<ArrowUpRight className="w-3 h-3" /> Min: {cpuMin}% · Max: {cpuMax}%
						</div>
					</Card>

					<Card className="bg-[#141414] border border-[#272727] rounded-xl p-3.5 shadow-md">
						<p className="text-[10px] font-semibold text-[#a1a1aa] uppercase tracking-wider">RAM ALLOCATION</p>
						<p className="text-2xl font-extrabold text-[#FAFAFA] font-mono mt-0.5">{latest.ramUsedGb} GB</p>
						<p className="text-[10px] text-[#737373] font-mono mt-0.5">{latest.ramPercent}% of 16.0 GB total</p>
					</Card>

					<Card className="bg-[#141414] border border-[#272727] rounded-xl p-3.5 shadow-md">
						<p className="text-[10px] font-semibold text-[#a1a1aa] uppercase tracking-wider">BANDWIDTH (IN / OUT)</p>
						<p className="text-2xl font-extrabold text-[#FAFAFA] font-mono mt-0.5">{latest.netRxMbps} MB/s</p>
						<p className="text-[10px] text-[#737373] font-mono mt-0.5">Avg: {netRxAvg} MB/s · Peak: {netRxMax} MB/s</p>
					</Card>

					<Card className="bg-[#141414] border border-[#272727] rounded-xl p-3.5 shadow-md">
						<p className="text-[10px] font-semibold text-[#a1a1aa] uppercase tracking-wider">LATENCY (P95)</p>
						<p className="text-2xl font-extrabold text-green-400 font-mono mt-0.5">{latest.httpLatencyP95Ms} ms</p>
						<p className="text-[10px] text-[#737373] font-mono mt-0.5">{latest.httpRps} RPS Traefik Ingress</p>
					</Card>

					{/* Live 8-Core Workload Heatmap Tile */}
					<Card className="col-span-2 lg:col-span-1 bg-[#141414] border border-[#272727] rounded-xl p-3 shadow-md flex flex-col justify-between">
						<div className="flex items-center justify-between">
							<p className="text-[10px] font-semibold text-[#a1a1aa] uppercase tracking-wider">8-CORE HEATMAP</p>
							<span className="text-[10px] font-mono text-green-400">8 Threads</span>
						</div>
						<div className="grid grid-cols-4 gap-1.5 mt-1">
							{coresData.map((c, i) => (
								<div
									key={i}
									className="flex flex-col items-center bg-[#1c1c1f] p-1 rounded-md border border-white/5"
									title={`${c.name}: ${c.val}%`}
								>
									<span className="text-[9px] font-mono text-[#a1a1aa]">{c.name}</span>
									<span className="text-[10px] font-mono font-bold text-[#FAFAFA]">{Math.round(c.val)}%</span>
									<div className="w-full bg-[#272727] h-1 rounded-full mt-0.5 overflow-hidden">
										<div className={`h-full ${c.col}`} style={{ width: `${c.val}%` }} />
									</div>
								</div>
							))}
						</div>
					</Card>
				</div>

				{/* 1. CPU Core Multi-Line Graph (With Glow Effects & Per-Core Interactive Filters) */}
				<Card className="bg-[#141414] border border-[#272727] rounded-xl p-4 shadow-md flex-1 flex flex-col min-h-0 overflow-hidden relative">
					<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-2 mb-2 shrink-0">
						<div className="flex items-center gap-2">
							<Cpu className="w-4 h-4 text-blue-400" />
							<div className="flex items-center gap-2">
								<h2 className="text-sm font-bold text-[#FAFAFA]">CPU Core Utilization Streams (8 Cores)</h2>
								<span className="text-[10px] font-mono px-2 py-0.5 rounded bg-[#272727] text-[#a1a1aa] border border-white/10">
									Range: {timeRange} · Avg: {cpuAvgStats}% · Peak: {cpuMax}%
								</span>
							</div>
						</div>

						{/* Interactive Per-Core Filter Buttons */}
						<div className="flex items-center gap-1.5 text-[11px] font-mono">
							<button
								onClick={() => setActiveCoreFilter('all')}
								className={`px-2 py-0.5 rounded-lg border transition-colors cursor-pointer ${
									activeCoreFilter === 'all'
										? 'bg-[#272727] text-[#FAFAFA] border-white/20 font-bold'
										: 'text-[#737373] border-transparent hover:text-[#FAFAFA]'
								}`}
							>
								All Cores
							</button>

							<button
								onClick={() => setActiveCoreFilter(activeCoreFilter === 'c1' ? 'all' : 'c1')}
								className={`flex items-center gap-1 px-2 py-0.5 rounded-lg border transition-colors cursor-pointer ${
									activeCoreFilter === 'c1'
										? 'bg-blue-500/20 text-blue-400 border-blue-500/40 font-bold'
										: 'text-blue-400/70 border-transparent hover:text-blue-400'
								}`}
							>
								<span className="w-2 h-2 rounded-full bg-blue-400" /> Core 1-2
							</button>

							<button
								onClick={() => setActiveCoreFilter(activeCoreFilter === 'c3' ? 'all' : 'c3')}
								className={`flex items-center gap-1 px-2 py-0.5 rounded-lg border transition-colors cursor-pointer ${
									activeCoreFilter === 'c3'
										? 'bg-emerald-500/20 text-emerald-400 border-emerald-500/40 font-bold'
										: 'text-emerald-400/70 border-transparent hover:text-emerald-400'
								}`}
							>
								<span className="w-2 h-2 rounded-full bg-emerald-400" /> Core 3-4
							</button>

							<button
								onClick={() => setActiveCoreFilter(activeCoreFilter === 'c5' ? 'all' : 'c5')}
								className={`flex items-center gap-1 px-2 py-0.5 rounded-lg border transition-colors cursor-pointer ${
									activeCoreFilter === 'c5'
										? 'bg-purple-500/20 text-purple-400 border-purple-500/40 font-bold'
										: 'text-purple-400/70 border-transparent hover:text-purple-400'
								}`}
							>
								<span className="w-2 h-2 rounded-full bg-purple-400" /> Core 5-6
							</button>

							<button
								onClick={() => setActiveCoreFilter(activeCoreFilter === 'c7' ? 'all' : 'c7')}
								className={`flex items-center gap-1 px-2 py-0.5 rounded-lg border transition-colors cursor-pointer ${
									activeCoreFilter === 'c7'
										? 'bg-amber-500/20 text-amber-400 border-amber-500/40 font-bold'
										: 'text-amber-400/70 border-transparent hover:text-amber-400'
								}`}
							>
								<span className="w-2 h-2 rounded-full bg-amber-400" /> Core 7-8
							</button>
						</div>
					</div>

					{/* Graph Layout Grid with Left Y-Axis Scale */}
					<div className="flex-1 flex items-stretch gap-2 min-h-0 relative">
						{/* Y-Axis Percentage Labels */}
						<div className="flex flex-col justify-between text-[10px] font-mono text-[#737373] text-right py-1 select-none w-8 shrink-0">
							<span>100%</span>
							<span>75%</span>
							<span>50%</span>
							<span>25%</span>
							<span>0%</span>
						</div>

						{/* Interactive SVG Graph Container */}
						<div className="flex-1 flex flex-col min-w-0 min-h-0 relative">
							<div className="flex-1 relative overflow-hidden rounded-lg bg-[#0f0f10] border border-[#272727]/50">
								<svg
									className="w-full h-full cursor-crosshair"
									viewBox="0 0 800 200"
									preserveAspectRatio="none"
									onMouseMove={(e) => handleMouseMove(e, telemetryBuffer.length, setCpuHoverIdx)}
									onMouseLeave={() => setCpuHoverIdx(null)}
								>
									<defs>
										{/* Glow Shadow Filters */}
										<filter id="glow-blue" x="-20%" y="-20%" width="140%" height="140%">
											<feGaussianBlur stdDeviation="3" result="blur" />
											<feComposite in="SourceGraphic" in2="blur" operator="over" />
										</filter>
										<filter id="glow-emerald" x="-20%" y="-20%" width="140%" height="140%">
											<feGaussianBlur stdDeviation="3" result="blur" />
											<feComposite in="SourceGraphic" in2="blur" operator="over" />
										</filter>

										<linearGradient id="cpuGrad1" x1="0" y1="0" x2="0" y2="1">
											<stop offset="0%" stopColor="#60a5fa" stopOpacity="0.3" />
											<stop offset="100%" stopColor="#60a5fa" stopOpacity="0" />
										</linearGradient>
									</defs>

									{/* Y-Axis Horizontal Grid Lines */}
									{[12, 56, 100, 144, 188].map((y) => (
										<line key={y} x1="0" y1={y} x2="800" y2={y} stroke="#272727" strokeDasharray="4 4" strokeWidth="1" />
									))}

									{/* X-Axis Vertical Reference Grid Lines */}
									{[0, 200, 400, 600, 800].map((x) => (
										<line key={x} x1={x} y1="0" x2={x} y2="200" stroke="#272727" strokeDasharray="4 4" strokeWidth="1" />
									))}

									{/* Gradient Area under Core 1 */}
									{(chartMode === 'area' || activeCoreFilter === 'c1') && (
										<path
											d={createAreaPath(createSvgPath(telemetryBuffer.map((d) => d.cpuCore1), 800, 200, 100, chartMode), 800, 200)}
											fill="url(#cpuGrad1)"
										/>
									)}

									{/* Multi-Line 1 (Core 1-2 Blue) */}
									{(activeCoreFilter === 'all' || activeCoreFilter === 'c1') && (
										<path
											d={createSvgPath(telemetryBuffer.map((d) => d.cpuCore1), 800, 200, 100, chartMode)}
											fill="none"
											stroke="#60a5fa"
											strokeWidth={activeCoreFilter === 'c1' ? '3.5' : '2.5'}
											strokeLinecap="round"
											filter="url(#glow-blue)"
											opacity={activeCoreFilter === 'all' || activeCoreFilter === 'c1' ? 1 : 0.15}
										/>
									)}

									{/* Multi-Line 2 (Core 3-4 Emerald) */}
									{(activeCoreFilter === 'all' || activeCoreFilter === 'c3') && (
										<path
											d={createSvgPath(telemetryBuffer.map((d) => d.cpuCore3), 800, 200, 100, chartMode)}
											fill="none"
											stroke="#34d399"
											strokeWidth={activeCoreFilter === 'c3' ? '3.5' : '2.5'}
											strokeLinecap="round"
											filter="url(#glow-emerald)"
											opacity={activeCoreFilter === 'all' || activeCoreFilter === 'c3' ? 1 : 0.15}
										/>
									)}

									{/* Multi-Line 3 (Core 5-6 Purple) */}
									{(activeCoreFilter === 'all' || activeCoreFilter === 'c5') && (
										<path
											d={createSvgPath(telemetryBuffer.map((d) => d.cpuCore5), 800, 200, 100, chartMode)}
											fill="none"
											stroke="#c084fc"
											strokeWidth={activeCoreFilter === 'c5' ? '3.5' : '2'}
											strokeDasharray={chartMode === 'stepped' ? undefined : '5 3'}
											strokeLinecap="round"
											opacity={activeCoreFilter === 'all' || activeCoreFilter === 'c5' ? 1 : 0.15}
										/>
									)}

									{/* Multi-Line 4 (Core 7-8 Amber) */}
									{(activeCoreFilter === 'all' || activeCoreFilter === 'c7') && (
										<path
											d={createSvgPath(telemetryBuffer.map((d) => d.cpuCore7), 800, 200, 100, chartMode)}
											fill="none"
											stroke="#fbbf24"
											strokeWidth={activeCoreFilter === 'c7' ? '3.5' : '2'}
											strokeLinecap="round"
											opacity={activeCoreFilter === 'all' || activeCoreFilter === 'c7' ? 1 : 0.15}
										/>
									)}

									{/* Interactive Hover Crosshair & Data Dots */}
									{cpuHoverIdx !== null && telemetryBuffer[cpuHoverIdx] && (() => {
										const pt = telemetryBuffer[cpuHoverIdx];
										const c1 = getPointCoords(pt.cpuCore1, cpuHoverIdx, telemetryBuffer.length, 800, 200);
										const c3 = getPointCoords(pt.cpuCore3, cpuHoverIdx, telemetryBuffer.length, 800, 200);
										const c5 = getPointCoords(pt.cpuCore5, cpuHoverIdx, telemetryBuffer.length, 800, 200);
										const c7 = getPointCoords(pt.cpuCore7, cpuHoverIdx, telemetryBuffer.length, 800, 200);

										return (
											<g>
												<line x1={c1.x} y1="0" x2={c1.x} y2="200" stroke="#737373" strokeDasharray="3 3" strokeWidth="1.5" />
												<circle cx={c1.x} cy={c1.y} r="5" fill="#60a5fa" stroke="#FFFFFF" strokeWidth="2" />
												<circle cx={c3.x} cy={c3.y} r="5" fill="#34d399" stroke="#FFFFFF" strokeWidth="2" />
												<circle cx={c5.x} cy={c5.y} r="5" fill="#c084fc" stroke="#FFFFFF" strokeWidth="2" />
												<circle cx={c7.x} cy={c7.y} r="5" fill="#fbbf24" stroke="#FFFFFF" strokeWidth="2" />
											</g>
										);
									})()}
								</svg>

								{/* Floating Hover Tooltip Card */}
								{cpuHoverIdx !== null && telemetryBuffer[cpuHoverIdx] && (() => {
									const pt = telemetryBuffer[cpuHoverIdx];
									const ratio = cpuHoverIdx / (telemetryBuffer.length - 1);
									return (
										<div
											style={{ left: `${Math.min(82, Math.max(5, ratio * 100))}%` }}
											className="absolute top-2 pointer-events-none -translate-x-1/2 z-30 bg-[#18181b] border border-[#272727] rounded-xl shadow-2xl p-2.5 font-mono text-[11px] text-[#FAFAFA] space-y-0.5 w-48 animate-fade-up"
										>
											<div className="flex items-center justify-between text-[#a1a1aa] border-b border-[#272727] pb-1 mb-1 font-bold">
												<span>Time: {pt.timestamp}</span>
												<span className="text-green-400">Avg: {pt.cpuAvg}%</span>
											</div>
											<div className="flex items-center justify-between text-blue-400">
												<span>Core 1-2:</span> <span className="font-bold">{pt.cpuCore1}%</span>
											</div>
											<div className="flex items-center justify-between text-emerald-400">
												<span>Core 3-4:</span> <span className="font-bold">{pt.cpuCore3}%</span>
											</div>
											<div className="flex items-center justify-between text-purple-400">
												<span>Core 5-6:</span> <span className="font-bold">{pt.cpuCore5}%</span>
											</div>
											<div className="flex items-center justify-between text-amber-400">
												<span>Core 7-8:</span> <span className="font-bold">{pt.cpuCore7}%</span>
											</div>
										</div>
									);
								})()}
							</div>

							{/* Bottom X-Axis Timestamp Axis */}
							<div className="flex items-center justify-between text-[10px] font-mono text-[#737373] pt-1.5 shrink-0 select-none">
								<span>{telemetryBuffer[0]?.timestamp || '-'}</span>
								<span>{telemetryBuffer[10]?.timestamp || '-'}</span>
								<span>{telemetryBuffer[20]?.timestamp || '-'}</span>
								<span>{telemetryBuffer[30]?.timestamp || '-'}</span>
								<span className="text-green-400 font-bold">{telemetryBuffer[telemetryBuffer.length - 1]?.timestamp || 'Live'}</span>
							</div>
						</div>
					</div>
				</Card>

				{/* 2. Dual Graph Row: Network Bandwidth & Disk IOPS */}
				<div className="grid grid-cols-1 lg:grid-cols-2 gap-4 h-48 shrink-0">
					{/* Network Bandwidth (Ingress vs Egress with Y-Axis & Hover Tooltip) */}
					<Card className="bg-[#141414] border border-[#272727] rounded-xl p-4 shadow-md flex flex-col justify-between min-h-0 overflow-hidden relative">
						<div className="flex items-center justify-between mb-1 shrink-0">
							<div className="flex items-center gap-2">
								<h3 className="text-xs font-bold text-[#FAFAFA]">Network I/O Bandwidth</h3>
								<span className="text-[10px] font-mono text-[#a1a1aa]">Peak: {netRxMax} MB/s</span>
							</div>
							<div className="flex items-center gap-2.5 text-[11px] font-mono">
								<span className="flex items-center gap-1"><span className="w-2 h-2 rounded-full bg-cyan-400" /> In: {latest.netRxMbps} MB/s</span>
								<span className="flex items-center gap-1"><span className="w-2 h-2 rounded-full bg-pink-400" /> Out: {latest.netTxMbps} MB/s</span>
							</div>
						</div>

						<div className="flex-1 flex items-stretch gap-2 min-h-0">
							<div className="flex flex-col justify-between text-[9px] font-mono text-[#737373] text-right py-0.5 select-none w-9 shrink-0">
								<span>100M</span>
								<span>50M</span>
								<span>0M</span>
							</div>

							<div className="flex-1 flex flex-col min-w-0 min-h-0 relative">
								<div className="flex-1 relative overflow-hidden rounded-lg bg-[#0f0f10] border border-[#272727]/50">
									<svg
										className="w-full h-full cursor-crosshair"
										viewBox="0 0 400 120"
										preserveAspectRatio="none"
										onMouseMove={(e) => handleMouseMove(e, telemetryBuffer.length, setNetHoverIdx)}
										onMouseLeave={() => setNetHoverIdx(null)}
									>
										<defs>
											<linearGradient id="netGrad" x1="0" y1="0" x2="0" y2="1">
												<stop offset="0%" stopColor="#22d3ee" stopOpacity="0.25" />
												<stop offset="100%" stopColor="#22d3ee" stopOpacity="0" />
											</linearGradient>
										</defs>

										{[10, 60, 110].map((y) => (
											<line key={y} x1="0" y1={y} x2="400" y2={y} stroke="#272727" strokeDasharray="3 3" strokeWidth="1" />
										))}

										{chartMode === 'area' && (
											<path
												d={createAreaPath(createSvgPath(telemetryBuffer.map((d) => d.netRxMbps), 400, 120, 100, chartMode), 400, 120)}
												fill="url(#netGrad)"
											/>
										)}

										<path
											d={createSvgPath(telemetryBuffer.map((d) => d.netRxMbps), 400, 120, 100, chartMode)}
											fill="none"
											stroke="#22d3ee"
											strokeWidth="2.5"
											strokeLinecap="round"
										/>

										<path
											d={createSvgPath(telemetryBuffer.map((d) => d.netTxMbps), 400, 120, 100, chartMode)}
											fill="none"
											stroke="#f472b6"
											strokeWidth="2.5"
											strokeLinecap="round"
										/>

										{netHoverIdx !== null && telemetryBuffer[netHoverIdx] && (() => {
											const pt = telemetryBuffer[netHoverIdx];
											const rx = getPointCoords(pt.netRxMbps, netHoverIdx, telemetryBuffer.length, 400, 120);
											const tx = getPointCoords(pt.netTxMbps, netHoverIdx, telemetryBuffer.length, 400, 120);
											return (
												<g>
													<line x1={rx.x} y1="0" x2={rx.x} y2="120" stroke="#737373" strokeDasharray="3 3" strokeWidth="1.5" />
													<circle cx={rx.x} cy={rx.y} r="4.5" fill="#22d3ee" stroke="#FFFFFF" strokeWidth="1.5" />
													<circle cx={tx.x} cy={tx.y} r="4.5" fill="#f472b6" stroke="#FFFFFF" strokeWidth="1.5" />
												</g>
											);
										})()}
									</svg>

									{netHoverIdx !== null && telemetryBuffer[netHoverIdx] && (() => {
										const pt = telemetryBuffer[netHoverIdx];
										const ratio = netHoverIdx / (telemetryBuffer.length - 1);
										return (
											<div
												style={{ left: `${Math.min(75, Math.max(10, ratio * 100))}%` }}
												className="absolute top-1 pointer-events-none -translate-x-1/2 z-30 bg-[#18181b] border border-[#272727] rounded-xl shadow-2xl p-2 font-mono text-[11px] text-[#FAFAFA] space-y-0.5 w-40 animate-fade-up"
											>
												<p className="text-[10px] text-[#a1a1aa] font-bold border-b border-[#272727] pb-0.5">{pt.timestamp}</p>
												<p className="text-cyan-400 font-bold">Ingress: {pt.netRxMbps} MB/s</p>
												<p className="text-pink-400 font-bold">Egress: {pt.netTxMbps} MB/s</p>
											</div>
										);
									})()}
								</div>

								<div className="flex items-center justify-between text-[9px] font-mono text-[#737373] pt-1 shrink-0 select-none">
									<span>{telemetryBuffer[0]?.timestamp || '-'}</span>
									<span>{telemetryBuffer[20]?.timestamp || '-'}</span>
									<span className="text-cyan-400 font-bold">{telemetryBuffer[telemetryBuffer.length - 1]?.timestamp || 'Live'}</span>
								</div>
							</div>
						</div>
					</Card>

					{/* Disk Read/Write IOPS Graph with Y-Axis & Hover Tooltip */}
					<Card className="bg-[#141414] border border-[#272727] rounded-xl p-4 shadow-md flex flex-col justify-between min-h-0 overflow-hidden relative">
						<div className="flex items-center justify-between mb-1 shrink-0">
							<div className="flex items-center gap-2">
								<h3 className="text-xs font-bold text-[#FAFAFA]">Disk I/O Operations (IOPS)</h3>
								<span className="text-[10px] font-mono text-[#a1a1aa]">Peak: {iopsMax} IOPS</span>
							</div>
							<div className="flex items-center gap-2.5 text-[11px] font-mono">
								<span className="flex items-center gap-1"><span className="w-2 h-2 rounded-full bg-indigo-400" /> Read: {latest.diskReadIops}</span>
								<span className="flex items-center gap-1"><span className="w-2 h-2 rounded-full bg-orange-400" /> Write: {latest.diskWriteIops}</span>
							</div>
						</div>

						<div className="flex-1 flex items-stretch gap-2 min-h-0">
							<div className="flex flex-col justify-between text-[9px] font-mono text-[#737373] text-right py-0.5 select-none w-9 shrink-0">
								<span>2.0K</span>
								<span>1.0K</span>
								<span>0.0K</span>
							</div>

							<div className="flex-1 flex flex-col min-w-0 min-h-0 relative">
								<div className="flex-1 relative overflow-hidden rounded-lg bg-[#0f0f10] border border-[#272727]/50">
									<svg
										className="w-full h-full cursor-crosshair"
										viewBox="0 0 400 120"
										preserveAspectRatio="none"
										onMouseMove={(e) => handleMouseMove(e, telemetryBuffer.length, setIopsHoverIdx)}
										onMouseLeave={() => setIopsHoverIdx(null)}
									>
										{[10, 60, 110].map((y) => (
											<line key={y} x1="0" y1={y} x2="400" y2={y} stroke="#272727" strokeDasharray="3 3" strokeWidth="1" />
										))}

										<path
											d={createSvgPath(telemetryBuffer.map((d) => d.diskReadIops), 400, 120, 2000, chartMode)}
											fill="none"
											stroke="#818cf8"
											strokeWidth="2.5"
											strokeLinecap="round"
										/>

										<path
											d={createSvgPath(telemetryBuffer.map((d) => d.diskWriteIops), 400, 120, 2000, chartMode)}
											fill="none"
											stroke="#fb923c"
											strokeWidth="2.5"
											strokeLinecap="round"
										/>

										{iopsHoverIdx !== null && telemetryBuffer[iopsHoverIdx] && (() => {
											const pt = telemetryBuffer[iopsHoverIdx];
											const r = getPointCoords(pt.diskReadIops, iopsHoverIdx, telemetryBuffer.length, 400, 120, 2000);
											const w = getPointCoords(pt.diskWriteIops, iopsHoverIdx, telemetryBuffer.length, 400, 120, 2000);
											return (
												<g>
													<line x1={r.x} y1="0" x2={r.x} y2="120" stroke="#737373" strokeDasharray="3 3" strokeWidth="1.5" />
													<circle cx={r.x} cy={r.y} r="4.5" fill="#818cf8" stroke="#FFFFFF" strokeWidth="1.5" />
													<circle cx={w.x} cy={w.y} r="4.5" fill="#fb923c" stroke="#FFFFFF" strokeWidth="1.5" />
												</g>
											);
										})()}
									</svg>

									{iopsHoverIdx !== null && telemetryBuffer[iopsHoverIdx] && (() => {
										const pt = telemetryBuffer[iopsHoverIdx];
										const ratio = iopsHoverIdx / (telemetryBuffer.length - 1);
										return (
											<div
												style={{ left: `${Math.min(75, Math.max(10, ratio * 100))}%` }}
												className="absolute top-1 pointer-events-none -translate-x-1/2 z-30 bg-[#18181b] border border-[#272727] rounded-xl shadow-2xl p-2 font-mono text-[11px] text-[#FAFAFA] space-y-0.5 w-40 animate-fade-up"
											>
												<p className="text-[10px] text-[#a1a1aa] font-bold border-b border-[#272727] pb-0.5">{pt.timestamp}</p>
												<p className="text-indigo-400 font-bold">Read: {pt.diskReadIops} IOPS</p>
												<p className="text-orange-400 font-bold">Write: {pt.diskWriteIops} IOPS</p>
											</div>
										);
									})()}
								</div>

								<div className="flex items-center justify-between text-[9px] font-mono text-[#737373] pt-1 shrink-0 select-none">
									<span>{telemetryBuffer[0]?.timestamp || '-'}</span>
									<span>{telemetryBuffer[20]?.timestamp || '-'}</span>
									<span className="text-indigo-400 font-bold">{telemetryBuffer[telemetryBuffer.length - 1]?.timestamp || 'Live'}</span>
								</div>
							</div>
						</div>
					</Card>
				</div>
			</div>
		</PageLayout>
	);
}
