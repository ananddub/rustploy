import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { motion, AnimatePresence } from 'framer-motion';
import {
	Rocket,
	ArrowRight,
	Cpu,
	Activity,
	Layers,
	CheckCircle2,
	AlertCircle,
	Clock
} from 'lucide-react';
import { PageLayout } from '@/components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { getDeploymentsMock, type DeploymentMock } from '$lib/mocks';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Progress } from '@/components/ui/progress';

export default function DashboardPage() {
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const userName = session?.user.first_name || session?.user.email?.split('@')[0] || 'Aditya Sahu';
	const fullName = `${userName} (addy)`;

	const [mockDeployments] = useState<DeploymentMock[]>(getDeploymentsMock());
	const [activeFilter, setActiveFilter] = useState<'all' | 'running' | 'errored'>('all');

	const filteredDeployments = mockDeployments.filter((d) => {
		if (activeFilter === 'running') return d.state === 'running' || d.state === 'building';
		if (activeFilter === 'errored') return d.state === 'error';
		return true;
	});

	const sparklineBars = [15, 25, 10, 40, 30, 20, 12];

	const containerVariants = {
		hidden: { opacity: 0 },
		visible: {
			opacity: 1,
			transition: {
				staggerChildren: 0.08
			}
		}
	};

	const itemVariants = {
		hidden: { opacity: 0, y: 15 },
		visible: {
			opacity: 1,
			y: 0,
			transition: { duration: 0.35 }
		}
	};

	return (
		<PageLayout>
			<motion.div
				initial="hidden"
				animate="visible"
				variants={containerVariants}
				className="flex-1 m-3.5 overflow-y-auto no-scrollbar p-7 flex flex-col min-h-0 bg-[#171717] border border-[#272727] rounded-2xl shadow-xl space-y-7"
			>
				{/* Greeting Header */}
				<motion.div variants={itemVariants} className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
					<div>
						<h1 className="text-3xl font-extrabold tracking-tight text-[#FAFAFA] flex items-center gap-2">
							Welcome back, {fullName}
						</h1>
						<p className="text-xs text-[#a1a1aa] mt-1 flex items-center gap-2">
							<span className="inline-block w-2 h-2 rounded-full bg-green-500 animate-pulse" />
							Rustploy High-Availability Engine · Version v0.29.12
						</p>
					</div>
					<motion.div whileHover={{ scale: 1.03 }} whileTap={{ scale: 0.97 }}>
						<Button
							variant="secondary"
							size="default"
							className="gap-2 text-sm font-semibold bg-[#272727] hover:bg-[#3f3f46] text-[#FAFAFA] border border-[#3f3f46]/80 px-4 py-2 rounded-xl shadow-lg cursor-pointer transition-all"
							onClick={() => navigate('/projects')}
						>
							Go to projects
							<ArrowRight className="h-4 w-4" />
						</Button>
					</motion.div>
				</motion.div>

				{/* 4 Staggered Animated Grid Metric Cards */}
				<motion.div variants={itemVariants} className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
					<motion.div whileHover={{ y: -3, scale: 1.01 }} transition={{ duration: 0.2 }}>
						<Card className="bg-[#141414] border border-[#272727] rounded-xl shadow-md hover:border-[#3f3f46] transition-colors">
							<CardHeader className="pb-1 pt-4 px-5">
								<p className="text-xs uppercase tracking-wider text-[#a1a1aa] font-semibold flex items-center justify-between">
									<span>PROJECTS</span>
									<Layers className="w-4 h-4 text-[#737373]" />
								</p>
							</CardHeader>
							<CardContent className="pb-4 px-5">
								<p className="text-4xl font-extrabold text-[#FAFAFA] tracking-tight">5</p>
								<div className="flex items-center gap-2 mt-2">
									<span className="px-2 py-0.5 rounded-md border border-[#272727] bg-[#272727]/60 text-[11px] text-[#a1a1aa]">3 Prod</span>
									<span className="px-2 py-0.5 rounded-md border border-[#272727] bg-[#272727]/60 text-[11px] text-[#a1a1aa]">2 Staging</span>
								</div>
							</CardContent>
						</Card>
					</motion.div>

					<motion.div whileHover={{ y: -3, scale: 1.01 }} transition={{ duration: 0.2 }}>
						<Card className="bg-[#141414] border border-[#272727] rounded-xl shadow-md hover:border-[#3f3f46] transition-colors">
							<CardHeader className="pb-1 pt-4 px-5">
								<p className="text-xs uppercase tracking-wider text-[#a1a1aa] font-semibold flex items-center justify-between">
									<span>SERVICES</span>
									<Activity className="w-4 h-4 text-[#737373]" />
								</p>
							</CardHeader>
							<CardContent className="pb-4 px-5">
								<p className="text-4xl font-extrabold text-[#FAFAFA] tracking-tight">12</p>
								<div className="flex items-center gap-1.5 mt-2">
									<span className="px-2 py-0.5 rounded-md border border-[#272727] bg-[#272727]/60 text-[11px] text-[#FAFAFA]">6 Apps</span>
									<span className="px-2 py-0.5 rounded-md border border-[#272727] bg-[#272727]/60 text-[11px] text-[#FAFAFA]">4 Compose</span>
									<span className="px-2 py-0.5 rounded-md border border-[#272727] bg-[#272727]/60 text-[11px] text-[#FAFAFA]">2 DB</span>
								</div>
							</CardContent>
						</Card>
					</motion.div>

					<motion.div whileHover={{ y: -3, scale: 1.01 }} transition={{ duration: 0.2 }}>
						<Card className="bg-[#141414] border border-[#272727] rounded-xl shadow-md hover:border-[#3f3f46] transition-colors">
							<CardHeader className="pb-1 pt-4 px-5">
								<p className="text-xs uppercase tracking-wider text-[#a1a1aa] font-semibold flex items-center justify-between">
									<span>DEPLOYS / 7D</span>
									<Rocket className="w-4 h-4 text-[#737373]" />
								</p>
							</CardHeader>
							<CardContent className="pb-4 px-5">
								<p className="text-4xl font-extrabold text-[#FAFAFA] tracking-tight">42</p>
								<div className="flex items-end gap-1.5 h-5 mt-2">
									{sparklineBars.map((barHeight, idx) => (
										<motion.div
											key={idx}
											initial={{ scaleY: 0 }}
											animate={{ scaleY: 1 }}
											transition={{ duration: 0.4, delay: idx * 0.05 }}
											style={{ height: `${Math.max(4, barHeight)}px`, transformOrigin: 'bottom' }}
											className="flex-1 bg-[#3f3f46]/80 rounded-xs hover:bg-[#FAFAFA] transition-colors cursor-pointer"
											title="Daily build volume"
										/>
									))}
								</div>
							</CardContent>
						</Card>
					</motion.div>

					<motion.div whileHover={{ y: -3, scale: 1.01 }} transition={{ duration: 0.2 }}>
						<Card className="bg-[#141414] border border-[#272727] rounded-xl shadow-md hover:border-[#3f3f46] transition-colors">
							<CardHeader className="pb-1 pt-4 px-5">
								<div className="flex items-center justify-between">
									<p className="text-xs uppercase tracking-wider text-[#a1a1aa] font-semibold">STATUS</p>
									<span className="text-[11px] font-mono text-green-400 font-semibold">100% Operational</span>
								</div>
							</CardHeader>
							<CardContent className="pb-4 px-5">
								<div className="flex flex-col gap-1.5 text-sm mt-1">
									<div className="flex items-center gap-2">
										<CheckCircle2 className="w-4 h-4 text-green-400" />
										<span className="font-bold text-[#FAFAFA] text-sm">3</span>
										<span className="text-[#a1a1aa] text-sm">running</span>
									</div>
									<div className="flex items-center gap-2">
										<AlertCircle className="w-4 h-4 text-red-400" />
										<span className="font-bold text-[#FAFAFA] text-sm">1</span>
										<span className="text-[#a1a1aa] text-sm">errored</span>
									</div>
								</div>
							</CardContent>
						</Card>
					</motion.div>
				</motion.div>

				{/* Animated Server Resources Monitoring Widget */}
				<motion.div variants={itemVariants}>
					<Card className="bg-[#141414] border border-[#272727] rounded-xl shadow-md p-5">
						<div className="flex items-center justify-between mb-4">
							<div className="flex items-center gap-2.5">
								<Cpu className="h-4 w-4 text-[#a1a1aa]" />
								<h2 className="text-sm font-semibold text-[#FAFAFA]">Server Resources & Real-time Telemetry</h2>
							</div>
							<span className="text-xs font-mono text-[#737373]">Host: production-01 · Ubuntu 24.04</span>
						</div>
						<div className="grid grid-cols-1 md:grid-cols-3 gap-5">
							<div className="space-y-2">
								<div className="flex items-center justify-between text-xs">
									<span className="text-[#a1a1aa] font-medium">CPU Load</span>
									<span className="text-[#FAFAFA] font-mono font-semibold">14%</span>
								</div>
								<Progress value={14} className="h-1.5 bg-[#272727]" />
							</div>
							<div className="space-y-2">
								<div className="flex items-center justify-between text-xs">
									<span className="text-[#a1a1aa] font-medium">RAM Usage</span>
									<span className="text-[#FAFAFA] font-mono font-semibold">5.1 GB / 16.0 GB (32%)</span>
								</div>
								<Progress value={32} className="h-1.5 bg-[#272727]" />
							</div>
							<div className="space-y-2">
								<div className="flex items-center justify-between text-xs">
									<span className="text-[#a1a1aa] font-medium">Disk Storage</span>
									<span className="text-[#FAFAFA] font-mono font-semibold">38.4 GB / 160 GB (24%)</span>
								</div>
								<Progress value={24} className="h-1.5 bg-[#272727]" />
							</div>
						</div>
					</Card>
				</motion.div>

				{/* Animated Deployments Stream */}
				<motion.div variants={itemVariants} className="flex-1 flex flex-col min-h-0">
					<Card className="bg-[#141414] border border-[#272727] rounded-xl shadow-md overflow-hidden flex-1 flex flex-col">
						<CardHeader className="border-b border-[#272727] py-3.5 px-5 bg-[#141414]">
							<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
								<div className="flex items-center gap-2.5">
									<Clock className="h-4 w-4 text-[#a1a1aa]" />
									<CardTitle className="text-sm font-semibold text-[#FAFAFA]">Recent deployments & activity stream</CardTitle>
								</div>
								<div className="flex items-center gap-1 bg-[#1c1c1c] p-0.5 rounded-lg border border-[#272727]">
									{(['all', 'running', 'errored'] as const).map((f) => (
										<button
											key={f}
											className={`px-2.5 py-1 text-xs font-medium rounded-md transition-all capitalize cursor-pointer ${
												activeFilter === f
													? 'bg-[#272727] text-[#FAFAFA] shadow-xs'
													: 'text-[#737373] hover:text-[#FAFAFA]'
											}`}
											onClick={() => setActiveFilter(f)}
										>
											{f}
										</button>
									))}
								</div>
							</div>
						</CardHeader>
						<CardContent className="p-0 flex-1 flex flex-col justify-center bg-[#141414]">
							<div className="divide-y divide-[#272727]">
								<AnimatePresence mode="popLayout">
									{filteredDeployments.map((d) => (
										<motion.div
											key={d.id}
											initial={{ opacity: 0, x: -10 }}
											animate={{ opacity: 1, x: 0 }}
											exit={{ opacity: 0, x: 10 }}
											transition={{ duration: 0.2 }}
											className="flex items-center justify-between px-6 py-3.5 hover:bg-[#272727]/40 transition-colors"
										>
											<div className="flex items-center gap-3.5">
												<span
													className={`h-2.5 w-2.5 rounded-full shrink-0 ${
														d.state === 'done'
															? 'bg-[#22c55e]'
															: d.state === 'building'
																? 'bg-blue-500 animate-pulse'
																: 'bg-[#ef4444]'
													}`}
												/>
												<div>
													<p className="text-sm font-semibold text-[#FAFAFA]">
														{d.projectName} · {d.serviceName}
														<span className="text-[#737373] font-normal ml-2 text-xs">#{d.id}</span>
													</p>
													<p className="text-xs text-[#a1a1aa] mt-0.5 font-mono">
														{d.branch} ({d.commitHash}) — {d.commitMessage}
													</p>
												</div>
											</div>
											<div className="flex items-center gap-3.5">
												<Badge
													variant="outline"
													className={`text-xs capitalize ${
														d.state === 'done'
															? 'border-green-500/30 text-green-400 bg-green-500/10'
															: d.state === 'building'
																? 'border-blue-500/30 text-blue-400 bg-blue-500/10'
																: 'border-red-500/30 text-red-400 bg-red-500/10'
													}`}
												>
													{d.state}
												</Badge>
												<span className="text-xs text-[#737373] font-mono">{d.createdAt}</span>
											</div>
										</motion.div>
									))}
								</AnimatePresence>
							</div>
						</CardContent>
					</Card>
				</motion.div>
			</motion.div>
		</PageLayout>
	);
}
