import React, { useState, useEffect, useRef } from 'react';
import { useNavigate } from 'react-router-dom';
import { motion, AnimatePresence } from 'framer-motion';
import {
	Activity,
	Play,
	Pause,
	RefreshCw,
	Server,
	Cpu,
	HardDrive,
	ArrowUpRight,
	ArrowDownRight,
	Zap,
	ShieldAlert,
	Sliders
} from 'lucide-react';
import { PageLayout } from '$lib/../components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { Card, CardContent, CardHeader, CardTitle } from '$lib/../components/ui/card';
import { Button } from '$lib/../components/ui/button';
import { Badge } from '$lib/../components/ui/badge';
import { toastSuccess, toastInfo } from '$lib/toast';

interface TelemetryPoint {
	timestamp: string;
	v1: number;
	v2: number;
	v3?: number;
	v4?: number;
}

export default function MonitoringPage() {
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const [isStreaming, setIsStreaming] = useState(true);
	const [selectedNode, setSelectedNode] = useState('production-01');
	const [timeRange, setTimeRange] = useState<'1m' | '5m' | '15m' | '1h'>('1m');

	// Initial data points generator for real-time streaming
	const generateInitialSeries = (base1: number, base2: number, count = 30): TelemetryPoint[] => {
		const now = Date.now();
		return Array.from({ length: count }, (_, i) => {
			const time = new Date(now - (count - i) * 1000).toLocaleTimeString([], {
				hour: '2-digit',
				minute: '2-digit',
				second: '2-digit'
			});
			return {
				timestamp: time,
				v1: Math.min(100, Math.max(5, base1 + Math.sin(i * 0.4) * 15 + (Math.random() * 8 - 4))),
				v2: Math.min(100, Math.max(5, base2 + Math.cos(i * 0.4) * 12 + (Math.random() * 6 - 3))),
				v3: Math.min(100, Math.max(5, (base1 + base2) / 2 + Math.sin(i * 0.2) * 10 + (Math.random() * 4 - 2))),
				v4: Math.min(100, Math.max(5, base1 * 0.7 + Math.cos(i * 0.3) * 14 + (Math.random() * 10 - 5)))
			};
		});
	};

	const [cpuData, setCpuData] = useState<TelemetryPoint[]>(() => generateInitialSeries(18, 35));
	const [ramData, setRamData] = useState<TelemetryPoint[]>(() => generateInitialSeries(32, 48));
	const [netData, setNetData] = useState<TelemetryPoint[]>(() => generateInitialSeries(42, 24));
	const [iopsData, setIopsData] = useState<TelemetryPoint[]>(() => generateInitialSeries(55, 30));
	const [latencyData, setLatencyData] = useState<TelemetryPoint[]>(() => generateInitialSeries(14, 28));

	// Real-time live interval tick effect (runs every 1000ms when streaming)
	useEffect(() => {
		if (!isStreaming) return;

		const interval = setInterval(() => {
			const timeStr = new Date().toLocaleTimeString([], {
				hour: '2-digit',
				minute: '2-digit',
				second: '2-digit'
			});

			const updateSeries = (prev: TelemetryPoint[], base1: number, base2: number) => {
				const last = prev[prev.length - 1] || { v1: base1, v2: base2, v3: 20, v4: 25 };
				const nextV1 = Math.min(100, Math.max(5, last.v1 + (Math.random() * 10 - 5)));
				const nextV2 = Math.min(100, Math.max(5, last.v2 + (Math.random() * 8 - 4)));
				const nextV3 = Math.min(100, Math.max(5, (last.v3 || 20) + (Math.random() * 6 - 3)));
				const nextV4 = Math.min(100, Math.max(5, (last.v4 || 25) + (Math.random() * 12 - 6)));

				const nextPoint: TelemetryPoint = {
					timestamp: timeStr,
					v1: Number(nextV1.toFixed(1)),
					v2: Number(nextV2.toFixed(1)),
					v3: Number(nextV3.toFixed(1)),
					v4: Number(nextV4.toFixed(1))
				};
				return [...prev.slice(1), nextPoint];
			};

			setCpuData((prev) => updateSeries(prev, 20, 38));
			setRamData((prev) => updateSeries(prev, 34, 52));
			setNetData((prev) => updateSeries(prev, 45, 26));
			setIopsData((prev) => updateSeries(prev, 50, 32));
			setLatencyData((prev) => updateSeries(prev, 16, 30));
		}, 1000);

		return () => clearInterval(interval);
	}, [isStreaming]);

	// Convert numeric series to smooth SVG cubic path
	const createSvgPath = (data: number[], width: number, height: number, maxVal = 100) => {
		if (!data || data.length < 2) return '';
		const step = width / (data.length - 1);
		const points = data.map((val, i) => {
			const x = i * step;
			const y = height - (val / maxVal) * (height - 10) - 5;
			return { x, y };
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

	const latestCpu = cpuData[cpuData.length - 1]?.v1 || 18.4;
	const latestRam = ramData[ramData.length - 1]?.v1 || 32.1;
	const latestNetIn = netData[netData.length - 1]?.v1 || 42.8;
	const latestNetOut = netData[netData.length - 1]?.v2 || 18.4;
	const latestLatency = latencyData[latencyData.length - 1]?.v1 || 14;

	return (
		<PageLayout>
			<motion.div
				initial={{ opacity: 0, y: 15 }}
				animate={{ opacity: 1, y: 0 }}
				transition={{ duration: 0.35 }}
				className="flex-1 m-3.5 overflow-y-auto no-scrollbar p-7 flex flex-col min-h-0 bg-[#171717] border border-[#272727] rounded-2xl shadow-xl space-y-6"
			>
				{/* Top Controls Header */}
				<div className="flex flex-col lg:flex-row lg:items-center justify-between gap-4">
					<div>
						<h1 className="text-3xl font-extrabold tracking-tight text-[#FAFAFA] flex items-center gap-3">
							<Activity className="w-8 h-8 text-green-400" />
							Real-Time Telemetry & Monitoring
						</h1>
						<p className="text-xs text-[#a1a1aa] mt-1 flex items-center gap-2">
							High-frequency live metrics stream · Updating every 1,000ms
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
									Live (1s)
								</>
							) : (
								<>
									<Play className="w-3.5 h-3.5" /> Paused
								</>
							)}
						</Button>

						<button
							onClick={() => {
								setCpuData(generateInitialSeries(20, 35));
								setRamData(generateInitialSeries(30, 50));
								toastSuccess('Telemetry history reset');
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
						<p className="text-3xl font-extrabold text-[#FAFAFA] font-mono mt-1">{latestCpu}%</p>
						<div className="flex items-center gap-1 mt-1 text-[11px] text-green-400 font-mono">
							<ArrowUpRight className="w-3.5 h-3.5" /> +1.2% vs last 5m
						</div>
					</Card>

					<Card className="bg-[#141414] border border-[#272727] rounded-xl p-4 shadow-md">
						<p className="text-[11px] font-semibold text-[#a1a1aa] uppercase tracking-wider">RAM ALLOCATION</p>
						<p className="text-3xl font-extrabold text-[#FAFAFA] font-mono mt-1">{(latestRam * 0.16).toFixed(2)} GB</p>
						<p className="text-[11px] text-[#737373] font-mono mt-1">{latestRam}% of 16.0 GB total</p>
					</Card>

					<Card className="bg-[#141414] border border-[#272727] rounded-xl p-4 shadow-md">
						<p className="text-[11px] font-semibold text-[#a1a1aa] uppercase tracking-wider">BANDWIDTH (IN / OUT)</p>
						<p className="text-3xl font-extrabold text-[#FAFAFA] font-mono mt-1">{latestNetIn.toFixed(1)} MB/s</p>
						<p className="text-[11px] text-[#737373] font-mono mt-1">Egress: {latestNetOut.toFixed(1)} MB/s</p>
					</Card>

					<Card className="bg-[#141414] border border-[#272727] rounded-xl p-4 shadow-md">
						<p className="text-[11px] font-semibold text-[#a1a1aa] uppercase tracking-wider">LATENCY (P95)</p>
						<p className="text-3xl font-extrabold text-green-400 font-mono mt-1">{latestLatency.toFixed(0)} ms</p>
						<p className="text-[11px] text-[#737373] font-mono mt-1">HTTP Traefik Ingress</p>
					</Card>
				</div>

				{/* 1. CPU Core Multi-Line Graph (8 Cores animated) */}
				<Card className="bg-[#141414] border border-[#272727] rounded-xl p-6 shadow-md overflow-hidden">
					<div className="flex items-center justify-between mb-4">
						<div className="flex items-center gap-2.5">
							<Cpu className="w-5 h-5 text-blue-400" />
							<div>
								<h2 className="text-base font-bold text-[#FAFAFA]">CPU Core Utilization Streams (8 Cores)</h2>
								<p className="text-xs text-[#a1a1aa]">Real-time per-core thread utilization lines with noise frequency variance</p>
							</div>
						</div>
						<div className="flex items-center gap-3 text-xs font-mono">
							<span className="flex items-center gap-1.5"><span className="w-2.5 h-2.5 rounded-full bg-blue-400" /> Core 0-1</span>
							<span className="flex items-center gap-1.5"><span className="w-2.5 h-2.5 rounded-full bg-emerald-400" /> Core 2-3</span>
							<span className="flex items-center gap-1.5"><span className="w-2.5 h-2.5 rounded-full bg-purple-400" /> Core 4-5</span>
							<span className="flex items-center gap-1.5"><span className="w-2.5 h-2.5 rounded-full bg-amber-400" /> Core 6-7</span>
						</div>
					</div>

					<div className="h-56 w-full relative">
						<svg className="w-full h-full overflow-visible" viewBox="0 0 800 200" preserveAspectRatio="none">
							<defs>
								<linearGradient id="cpuGrad1" x1="0" y1="0" x2="0" y2="1">
									<stop offset="0%" stopColor="#60a5fa" stopOpacity="0.25" />
									<stop offset="100%" stopColor="#60a5fa" stopOpacity="0" />
								</linearGradient>
								<linearGradient id="cpuGrad2" x1="0" y1="0" x2="0" y2="1">
									<stop offset="0%" stopColor="#34d399" stopOpacity="0.2" />
									<stop offset="100%" stopColor="#34d399" stopOpacity="0" />
								</linearGradient>
							</defs>

							{/* Horizontal Grid lines */}
							{[0, 50, 100, 150, 200].map((y) => (
								<line key={y} x1="0" y1={y} x2="800" y2={y} stroke="#272727" strokeDasharray="4 4" strokeWidth="1" />
							))}

							{/* Gradient Area under line 1 */}
							<path
								d={createAreaPath(createSvgPath(cpuData.map((d) => d.v1), 800, 200), 800, 200)}
								fill="url(#cpuGrad1)"
							/>

							{/* Multi-Line 1 (Core 0-1 Blue) */}
							<path
								d={createSvgPath(cpuData.map((d) => d.v1), 800, 200)}
								fill="none"
								stroke="#60a5fa"
								strokeWidth="2.5"
								strokeLinecap="round"
							/>

							{/* Multi-Line 2 (Core 2-3 Emerald) */}
							<path
								d={createSvgPath(cpuData.map((d) => d.v2), 800, 200)}
								fill="none"
								stroke="#34d399"
								strokeWidth="2.5"
								strokeLinecap="round"
							/>

							{/* Multi-Line 3 (Core 4-5 Purple) */}
							<path
								d={createSvgPath(cpuData.map((d) => d.v3 || 20), 800, 200)}
								fill="none"
								stroke="#c084fc"
								strokeWidth="2"
								strokeDasharray="5 3"
								strokeLinecap="round"
							/>

							{/* Multi-Line 4 (Core 6-7 Amber) */}
							<path
								d={createSvgPath(cpuData.map((d) => d.v4 || 25), 800, 200)}
								fill="none"
								stroke="#fbbf24"
								strokeWidth="2"
								strokeLinecap="round"
							/>
						</svg>
					</div>

					<div className="flex items-center justify-between text-[11px] font-mono text-[#737373] mt-3 border-t border-[#272727] pt-2">
						<span>{cpuData[0]?.timestamp || '12:00:00'}</span>
						<span>{cpuData[Math.floor(cpuData.length / 2)]?.timestamp || '12:00:15'}</span>
						<span>{cpuData[cpuData.length - 1]?.timestamp || '12:00:30'}</span>
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
								<span className="flex items-center gap-1.5"><span className="w-2.5 h-2.5 rounded-full bg-cyan-400" /> In: {latestNetIn.toFixed(1)} MB/s</span>
								<span className="flex items-center gap-1.5"><span className="w-2.5 h-2.5 rounded-full bg-pink-400" /> Out: {latestNetOut.toFixed(1)} MB/s</span>
							</div>
						</div>

						<div className="h-44 w-full relative">
							<svg className="w-full h-full overflow-visible" viewBox="0 0 400 160" preserveAspectRatio="none">
								{[0, 40, 80, 120, 160].map((y) => (
									<line key={y} x1="0" y1={y} x2="400" y2={y} stroke="#272727" strokeDasharray="3 3" strokeWidth="1" />
								))}

								<path
									d={createSvgPath(netData.map((d) => d.v1), 400, 160)}
									fill="none"
									stroke="#22d3ee"
									strokeWidth="2.5"
									strokeLinecap="round"
								/>

								<path
									d={createSvgPath(netData.map((d) => d.v2), 400, 160)}
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
								<span className="flex items-center gap-1.5"><span className="w-2.5 h-2.5 rounded-full bg-indigo-400" /> Read IOPS</span>
								<span className="flex items-center gap-1.5"><span className="w-2.5 h-2.5 rounded-full bg-orange-400" /> Write IOPS</span>
							</div>
						</div>

						<div className="h-44 w-full relative">
							<svg className="w-full h-full overflow-visible" viewBox="0 0 400 160" preserveAspectRatio="none">
								{[0, 40, 80, 120, 160].map((y) => (
									<line key={y} x1="0" y1={y} x2="400" y2={y} stroke="#272727" strokeDasharray="3 3" strokeWidth="1" />
								))}

								<path
									d={createSvgPath(iopsData.map((d) => d.v1), 400, 160)}
									fill="none"
									stroke="#818cf8"
									strokeWidth="2.5"
									strokeLinecap="round"
								/>

								<path
									d={createSvgPath(iopsData.map((d) => d.v2), 400, 160)}
									fill="none"
									stroke="#fb923c"
									strokeWidth="2.5"
									strokeLinecap="round"
								/>
							</svg>
						</div>
					</Card>
				</div>
			</motion.div>
		</PageLayout>
	);
}
