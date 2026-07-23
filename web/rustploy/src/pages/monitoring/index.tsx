import React, { useState, useEffect, useRef } from 'react';
import { useNavigate } from 'react-router-dom';
import { motion } from 'framer-motion';
import {
	Activity,
	Play,
	Pause,
	RefreshCw,
	Server,
	Cpu,
	ArrowUpRight
} from 'lucide-react';
import { PageLayout } from '$lib/../components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { getMonitoringMock, generateLoopingTick, type TelemetrySeriesPoint } from '$lib/mocks';
import { Card } from '$lib/../components/ui/card';
import { Button } from '$lib/../components/ui/button';
import { toastSuccess, toastInfo } from '$lib/toast';

export default function MonitoringPage() {
	const navigate = useNavigate();
	const session = getAuthSession();

	const [isStreaming, setIsStreaming] = useState(true);
	const [selectedNode, setSelectedNode] = useState('production-01');
	const [timeRange, setTimeRange] = useState<'1m' | '5m' | '15m' | '1h'>('1m');

	// Initial dataset loaded from mock provider
	const [telemetryBuffer, setTelemetryBuffer] = useState<TelemetrySeriesPoint[]>(() =>
		getMonitoringMock(selectedNode, 40)
	);

	const tickIndexRef = useRef(40);

	// When node changes, reset telemetry stream with host mock
	useEffect(() => {
		const mockData = getMonitoringMock(selectedNode, 40);
		setTelemetryBuffer(mockData);
		tickIndexRef.current = 40;
	}, [selectedNode]);

	// Real-time live looping tick interval (runs continuously every 1000ms)
	useEffect(() => {
		if (!isStreaming) return;

		const interval = setInterval(() => {
			tickIndexRef.current += 1;
			setTelemetryBuffer((prev) => {
				if (!prev || prev.length === 0) {
					return getMonitoringMock(selectedNode, 40);
				}
				const lastPoint = prev[prev.length - 1];
				const nextPoint = generateLoopingTick(lastPoint, tickIndexRef.current);
				return [...prev.slice(1), nextPoint];
			});
		}, 1000);

		return () => clearInterval(interval);
	}, [isStreaming, selectedNode]);

	// Convert numeric series to smooth SVG cubic path with bulletproof NaN protection
	const createSvgPath = (data: (number | undefined)[], width: number, height: number, maxVal = 100) => {
		if (!data || data.length < 2) return '';
		const safeMax = maxVal <= 0 ? 100 : maxVal;
		const step = width / (data.length - 1);
		
		const points = data.map((val, i) => {
			const numVal = typeof val === 'number' && !isNaN(val) ? val : 0;
			const x = i * step;
			const y = height - (Math.min(safeMax, Math.max(0, numVal)) / safeMax) * (height - 15) - 5;
			return { x: isNaN(x) ? 0 : x, y: isNaN(y) ? height / 2 : y };
		});

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

	const latest: TelemetrySeriesPoint = telemetryBuffer && telemetryBuffer.length > 0
		? telemetryBuffer[telemetryBuffer.length - 1]
		: {
				timestamp: '12:00:00',
				cpuCore1: 18, cpuCore2: 22, cpuCore3: 20, cpuCore4: 25,
				cpuCore5: 16, cpuCore6: 21, cpuCore7: 28, cpuCore8: 15,
				cpuAvg: 20.6, ramUsedGb: 5.12, ramTotalGb: 16.0, ramPercent: 32,
				netRxMbps: 42.8, netTxMbps: 18.4, diskReadIops: 1200, diskWriteIops: 650,
				httpRps: 840, httpLatencyP95Ms: 14
		  };

	return (
		<PageLayout>
			<div className="flex-1 m-3.5 overflow-y-auto no-scrollbar p-7 flex flex-col min-h-0 bg-[#171717] border border-[#272727] rounded-2xl shadow-xl space-y-6 animate-fade-up">
				{/* Top Controls Header */}
				<div className="flex flex-col lg:flex-row lg:items-center justify-between gap-4">
					<div>
						<h1 className="text-3xl font-extrabold tracking-tight text-[#FAFAFA] flex items-center gap-3">
							<Activity className="w-8 h-8 text-green-400" />
							Real-Time Telemetry & Monitoring
						</h1>
						<p className="text-xs text-[#a1a1aa] mt-1 flex items-center gap-2">
							High-frequency looping mock telemetry stream · Updating live every 1,000ms
						</p>
					</div>

					<div className="flex flex-wrap items-center gap-3">
						{/* Node Selector */}
						<div className="flex items-center gap-2 px-3 py-1.5 rounded-xl bg-[#141414] border border-[#272727] text-xs">
							<Server className="w-3.5 h-3.5 text-[#a1a1aa]" />
							<select
								value={selectedNode}
								onChange={(e) => setSelectedNode(e.target.value)}
								className="bg-transparent text-[#FAFAFA] font-medium focus:outline-none cursor-pointer"
							>
								<option value="production-01">production-01 (Ubuntu 24.04)</option>
								<option value="staging-worker-02">staging-worker-02 (Debian 12)</option>
								<option value="backup-replica-03">backup-replica-03 (Ubuntu 22.04)</option>
							</select>
						</div>

						{/* Time Range Selector */}
						<div className="flex items-center gap-1 bg-[#141414] p-1 rounded-xl border border-[#272727]">
							{(['1m', '5m', '15m', '1h'] as const).map((r) => (
								<button
									key={r}
									onClick={() => setTimeRange(r)}
									className={`px-2.5 py-1 text-xs font-semibold rounded-lg transition-all cursor-pointer ${
										timeRange === r ? 'bg-[#272727] text-[#FAFAFA] shadow-xs' : 'text-[#737373] hover:text-[#FAFAFA]'
									}`}
								>
									{r}
								</button>
							))}
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
								setTelemetryBuffer(getMonitoringMock(selectedNode, 40));
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

				{/* Quick Stats Summary Grid */}
				<div className="grid grid-cols-2 lg:grid-cols-4 gap-4">
					<Card className="bg-[#141414] border border-[#272727] rounded-xl p-4 shadow-md">
						<p className="text-[11px] font-semibold text-[#a1a1aa] uppercase tracking-wider">CPU AVG LOAD</p>
						<p className="text-3xl font-extrabold text-[#FAFAFA] font-mono mt-1">{latest.cpuAvg}%</p>
						<div className="flex items-center gap-1 mt-1 text-[11px] text-green-400 font-mono">
							<ArrowUpRight className="w-3.5 h-3.5" /> Live telemetry loop active
						</div>
					</Card>

					<Card className="bg-[#141414] border border-[#272727] rounded-xl p-4 shadow-md">
						<p className="text-[11px] font-semibold text-[#a1a1aa] uppercase tracking-wider">RAM ALLOCATION</p>
						<p className="text-3xl font-extrabold text-[#FAFAFA] font-mono mt-1">{latest.ramUsedGb} GB</p>
						<p className="text-[11px] text-[#737373] font-mono mt-1">{latest.ramPercent}% of 16.0 GB total</p>
					</Card>

					<Card className="bg-[#141414] border border-[#272727] rounded-xl p-4 shadow-md">
						<p className="text-[11px] font-semibold text-[#a1a1aa] uppercase tracking-wider">BANDWIDTH (IN / OUT)</p>
						<p className="text-3xl font-extrabold text-[#FAFAFA] font-mono mt-1">{latest.netRxMbps} MB/s</p>
						<p className="text-[11px] text-[#737373] font-mono mt-1">Egress: {latest.netTxMbps} MB/s</p>
					</Card>

					<Card className="bg-[#141414] border border-[#272727] rounded-xl p-4 shadow-md">
						<p className="text-[11px] font-semibold text-[#a1a1aa] uppercase tracking-wider">LATENCY (P95)</p>
						<p className="text-3xl font-extrabold text-green-400 font-mono mt-1">{latest.httpLatencyP95Ms} ms</p>
						<p className="text-[11px] text-[#737373] font-mono mt-1">{latest.httpRps} RPS Traefik Ingress</p>
					</Card>
				</div>

				{/* 1. CPU Core Multi-Line Graph (8 Cores animated) */}
				<Card className="bg-[#141414] border border-[#272727] rounded-xl p-6 shadow-md overflow-hidden">
					<div className="flex items-center justify-between mb-4">
						<div className="flex items-center gap-2.5">
							<Cpu className="w-5 h-5 text-blue-400" />
							<div>
								<h2 className="text-base font-bold text-[#FAFAFA]">CPU Core Utilization Streams (8 Cores)</h2>
								<p className="text-xs text-[#a1a1aa]">Real-time per-core thread utilization lines with continuous loop telemetry</p>
							</div>
						</div>
						<div className="flex items-center gap-3 text-xs font-mono">
							<span className="flex items-center gap-1.5"><span className="w-2.5 h-2.5 rounded-full bg-blue-400" /> Core 1-2</span>
							<span className="flex items-center gap-1.5"><span className="w-2.5 h-2.5 rounded-full bg-emerald-400" /> Core 3-4</span>
							<span className="flex items-center gap-1.5"><span className="w-2.5 h-2.5 rounded-full bg-purple-400" /> Core 5-6</span>
							<span className="flex items-center gap-1.5"><span className="w-2.5 h-2.5 rounded-full bg-amber-400" /> Core 7-8</span>
						</div>
					</div>

					<div className="h-56 w-full relative">
						<svg className="w-full h-full overflow-visible" viewBox="0 0 800 200" preserveAspectRatio="none">
							<defs>
								<linearGradient id="cpuGrad1" x1="0" y1="0" x2="0" y2="1">
									<stop offset="0%" stopColor="#60a5fa" stopOpacity="0.25" />
									<stop offset="100%" stopColor="#60a5fa" stopOpacity="0" />
								</linearGradient>
							</defs>

							{/* Horizontal Grid lines */}
							{[0, 50, 100, 150, 200].map((y) => (
								<line key={y} x1="0" y1={y} x2="800" y2={y} stroke="#272727" strokeDasharray="4 4" strokeWidth="1" />
							))}

							{/* Gradient Area under line 1 */}
							<path
								d={createAreaPath(createSvgPath(telemetryBuffer.map((d) => d.cpuCore1), 800, 200), 800, 200)}
								fill="url(#cpuGrad1)"
							/>

							{/* Multi-Line 1 (Core 1 Blue) */}
							<path
								d={createSvgPath(telemetryBuffer.map((d) => d.cpuCore1), 800, 200)}
								fill="none"
								stroke="#60a5fa"
								strokeWidth="2.5"
								strokeLinecap="round"
							/>

							{/* Multi-Line 2 (Core 3 Emerald) */}
							<path
								d={createSvgPath(telemetryBuffer.map((d) => d.cpuCore3), 800, 200)}
								fill="none"
								stroke="#34d399"
								strokeWidth="2.5"
								strokeLinecap="round"
							/>

							{/* Multi-Line 3 (Core 5 Purple) */}
							<path
								d={createSvgPath(telemetryBuffer.map((d) => d.cpuCore5), 800, 200)}
								fill="none"
								stroke="#c084fc"
								strokeWidth="2"
								strokeDasharray="5 3"
								strokeLinecap="round"
							/>

							{/* Multi-Line 4 (Core 7 Amber) */}
							<path
								d={createSvgPath(telemetryBuffer.map((d) => d.cpuCore7), 800, 200)}
								fill="none"
								stroke="#fbbf24"
								strokeWidth="2"
								strokeLinecap="round"
							/>
						</svg>
					</div>

					<div className="flex items-center justify-between text-[11px] font-mono text-[#737373] mt-3 border-t border-[#272727] pt-2">
						<span>{telemetryBuffer[0]?.timestamp || '12:00:00'}</span>
						<span>{telemetryBuffer[Math.floor(telemetryBuffer.length / 2)]?.timestamp || '12:00:20'}</span>
						<span>{telemetryBuffer[telemetryBuffer.length - 1]?.timestamp || '12:00:40'}</span>
					</div>
				</Card>

				{/* 2. Dual Graph Row: Network Bandwidth & Disk IOPS */}
				<div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
					{/* Network Bandwidth (Ingress vs Egress Real-time Lines) */}
					<Card className="bg-[#141414] border border-[#272727] rounded-xl p-5 shadow-md">
						<div className="flex items-center justify-between mb-3">
							<div>
								<h3 className="text-sm font-bold text-[#FAFAFA]">Network I/O Bandwidth</h3>
								<p className="text-xs text-[#a1a1aa]">Real-time ingress (rx) vs egress (tx) throughput</p>
							</div>
							<div className="flex items-center gap-3 text-xs font-mono">
								<span className="flex items-center gap-1.5"><span className="w-2.5 h-2.5 rounded-full bg-cyan-400" /> In: {latest.netRxMbps} MB/s</span>
								<span className="flex items-center gap-1.5"><span className="w-2.5 h-2.5 rounded-full bg-pink-400" /> Out: {latest.netTxMbps} MB/s</span>
							</div>
						</div>

						<div className="h-44 w-full relative">
							<svg className="w-full h-full overflow-visible" viewBox="0 0 400 160" preserveAspectRatio="none">
								{[0, 40, 80, 120, 160].map((y) => (
									<line key={y} x1="0" y1={y} x2="400" y2={y} stroke="#272727" strokeDasharray="3 3" strokeWidth="1" />
								))}

								<path
									d={createSvgPath(telemetryBuffer.map((d) => d.netRxMbps), 400, 160)}
									fill="none"
									stroke="#22d3ee"
									strokeWidth="2.5"
									strokeLinecap="round"
								/>

								<path
									d={createSvgPath(telemetryBuffer.map((d) => d.netTxMbps), 400, 160)}
									fill="none"
									stroke="#f472b6"
									strokeWidth="2.5"
									strokeLinecap="round"
								/>
							</svg>
						</div>
					</Card>

					{/* Disk Read/Write IOPS Graph */}
					<Card className="bg-[#141414] border border-[#272727] rounded-xl p-5 shadow-md">
						<div className="flex items-center justify-between mb-3">
							<div>
								<h3 className="text-sm font-bold text-[#FAFAFA]">Disk I/O Operations (IOPS)</h3>
								<p className="text-xs text-[#a1a1aa]">Storage controller read/write IOPS activity</p>
							</div>
							<div className="flex items-center gap-3 text-xs font-mono">
								<span className="flex items-center gap-1.5"><span className="w-2.5 h-2.5 rounded-full bg-indigo-400" /> Read: {latest.diskReadIops}</span>
								<span className="flex items-center gap-1.5"><span className="w-2.5 h-2.5 rounded-full bg-orange-400" /> Write: {latest.diskWriteIops}</span>
							</div>
						</div>

						<div className="h-44 w-full relative">
							<svg className="w-full h-full overflow-visible" viewBox="0 0 400 160" preserveAspectRatio="none">
								{[0, 40, 80, 120, 160].map((y) => (
									<line key={y} x1="0" y1={y} x2="400" y2={y} stroke="#272727" strokeDasharray="3 3" strokeWidth="1" />
								))}

								<path
									d={createSvgPath(telemetryBuffer.map((d) => d.diskReadIops), 400, 160, 2000)}
									fill="none"
									stroke="#818cf8"
									strokeWidth="2.5"
									strokeLinecap="round"
								/>

								<path
									d={createSvgPath(telemetryBuffer.map((d) => d.diskWriteIops), 400, 160, 2000)}
									fill="none"
									stroke="#fb923c"
									strokeWidth="2.5"
									strokeLinecap="round"
								/>
							</svg>
						</div>
					</Card>
				</div>
			</div>
		</PageLayout>
	);
}
