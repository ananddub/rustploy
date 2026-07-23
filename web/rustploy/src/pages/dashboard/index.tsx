import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import {
	Rocket,
	ArrowRight,
	Cpu
} from 'lucide-react';
import { PageLayout } from '$lib/../components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { getDeploymentsMock, type DeploymentMock } from '$lib/mocks';
import { Card, CardContent, CardHeader, CardTitle } from '$lib/../components/ui/card';
import { Button } from '$lib/../components/ui/button';
import { Badge } from '$lib/../components/ui/badge';
import { Progress } from '$lib/../components/ui/progress';

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

	return (
		<PageLayout>
			<div className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up flex flex-col min-h-0 bg-[#171717] border border-[#262626] rounded-2xl shadow-md space-y-7">
				{/* Greeting Header */}
				<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
					<div>
						<h1 className="text-3xl font-bold tracking-tight text-[#FAFAFA]">
							Welcome back, {fullName}
						</h1>
						<p className="text-xs text-[#a1a1aa] mt-1">
							Rustploy Dashboard · System Version v0.29.12
						</p>
					</div>
					<Button
						variant="secondary"
						size="default"
						className="gap-2 text-sm font-medium bg-[#262626] hover:bg-[#333333] text-[#FAFAFA] border border-[#3f3f46]/60 px-4 py-2 rounded-lg shadow-2xs self-start sm:self-auto cursor-pointer"
						onClick={() => navigate('/projects')}
					>
						Go to projects
						<ArrowRight className="h-4 w-4" />
					</Button>
				</div>

				{/* 4 Grid Stats */}
				<div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
					<Card className="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs hover:border-[#3f3f46] transition-colors">
						<CardHeader className="pb-1 pt-4 px-5">
							<p className="text-xs uppercase tracking-wider text-[#a1a1aa] font-semibold">PROJECTS</p>
						</CardHeader>
						<CardContent className="pb-4 px-5">
							<p className="text-4xl font-bold text-[#FAFAFA] tracking-tight">5</p>
							<div className="flex items-center gap-2 mt-2">
								<span className="px-2 py-0.5 rounded border border-[#262626] bg-[#262626]/40 text-[11px] text-[#a1a1aa]">3 Prod</span>
								<span className="px-2 py-0.5 rounded border border-[#262626] bg-[#262626]/40 text-[11px] text-[#a1a1aa]">2 Staging</span>
							</div>
						</CardContent>
					</Card>

					<Card className="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs hover:border-[#3f3f46] transition-colors">
						<CardHeader className="pb-1 pt-4 px-5">
							<p className="text-xs uppercase tracking-wider text-[#a1a1aa] font-semibold">SERVICES</p>
						</CardHeader>
						<CardContent className="pb-4 px-5">
							<p className="text-4xl font-bold text-[#FAFAFA] tracking-tight">12</p>
							<div className="flex items-center gap-1.5 mt-2">
								<span className="px-2 py-0.5 rounded border border-[#262626] bg-[#262626]/40 text-[11px] text-[#FAFAFA]">6 Apps</span>
								<span className="px-2 py-0.5 rounded border border-[#262626] bg-[#262626]/40 text-[11px] text-[#FAFAFA]">4 Compose</span>
								<span className="px-2 py-0.5 rounded border border-[#262626] bg-[#262626]/40 text-[11px] text-[#FAFAFA]">2 DB</span>
							</div>
						</CardContent>
					</Card>

					<Card className="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs hover:border-[#3f3f46] transition-colors">
						<CardHeader className="pb-1 pt-4 px-5">
							<p className="text-xs uppercase tracking-wider text-[#a1a1aa] font-semibold">DEPLOYS / 7D</p>
						</CardHeader>
						<CardContent className="pb-4 px-5">
							<p className="text-4xl font-bold text-[#FAFAFA] tracking-tight">42</p>
							<div className="flex items-end gap-1.5 h-5 mt-2">
								{sparklineBars.map((barHeight, idx) => (
									<div
										key={idx}
										style={{ height: `${Math.max(4, barHeight)}px` }}
										className="flex-1 bg-[#3f3f46]/60 rounded-xs hover:bg-[#FAFAFA] transition-colors"
										title="Day deploy activity"
									/>
								))}
							</div>
						</CardContent>
					</Card>

					<Card className="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs hover:border-[#3f3f46] transition-colors">
						<CardHeader className="pb-1 pt-4 px-5">
							<div className="flex items-center justify-between">
								<p className="text-xs uppercase tracking-wider text-[#a1a1aa] font-semibold">STATUS</p>
								<span className="text-[11px] font-mono text-[#a1a1aa]">100% Ready</span>
							</div>
						</CardHeader>
						<CardContent className="pb-4 px-5">
							<div className="flex flex-col gap-1.5 text-sm mt-1">
								<div className="flex items-center gap-2">
									<span className="w-2 h-2 rounded-full bg-[#22c55e]" />
									<span className="font-bold text-[#FAFAFA] text-sm">3</span>
									<span className="text-[#a1a1aa] text-sm">running</span>
								</div>
								<div className="flex items-center gap-2">
									<span className="w-2 h-2 rounded-full bg-[#ef4444]" />
									<span className="font-bold text-[#FAFAFA] text-sm">1</span>
									<span className="text-[#a1a1aa] text-sm">errored</span>
								</div>
							</div>
						</CardContent>
					</Card>
				</div>

				{/* Server Resources Monitoring Widget */}
				<Card className="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs p-5">
					<div className="flex items-center justify-between mb-4">
						<div className="flex items-center gap-2.5">
							<Cpu className="h-4 w-4 text-[#a1a1aa]" />
							<h2 className="text-sm font-semibold text-[#FAFAFA]">Server Resources & Performance</h2>
						</div>
						<span className="text-xs font-mono text-[#737373]">Host: production-01 · Ubuntu 24.04</span>
					</div>
					<div className="grid grid-cols-1 md:grid-cols-3 gap-5">
						<div className="space-y-2">
							<div className="flex items-center justify-between text-xs">
								<span className="text-[#a1a1aa] font-medium">CPU Load</span>
								<span className="text-[#FAFAFA] font-mono font-semibold">14%</span>
							</div>
							<Progress value={14} className="h-1.5 bg-[#262626]" />
						</div>
						<div className="space-y-2">
							<div className="flex items-center justify-between text-xs">
								<span className="text-[#a1a1aa] font-medium">RAM Usage</span>
								<span className="text-[#FAFAFA] font-mono font-semibold">5.1 GB / 16.0 GB (32%)</span>
							</div>
							<Progress value={32} className="h-1.5 bg-[#262626]" />
						</div>
						<div className="space-y-2">
							<div className="flex items-center justify-between text-xs">
								<span className="text-[#a1a1aa] font-medium">Disk Storage</span>
								<span className="text-[#FAFAFA] font-mono font-semibold">38.4 GB / 160 GB (24%)</span>
							</div>
							<Progress value={24} className="h-1.5 bg-[#262626]" />
						</div>
					</div>
				</Card>

				{/* Recent Deployments Table */}
				<Card className="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs overflow-hidden flex-1 flex flex-col">
					<CardHeader className="border-b border-[#262626] py-3.5 px-5 bg-[#171717]">
						<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
							<div className="flex items-center gap-2.5">
								<Rocket className="h-4 w-4 text-[#a1a1aa]" />
								<CardTitle className="text-sm font-semibold text-[#FAFAFA]">Recent deployments & activity stream</CardTitle>
							</div>
							<div className="flex items-center gap-1 bg-[#141414] p-0.5 rounded-lg border border-[#262626]">
								{(['all', 'running', 'errored'] as const).map((f) => (
									<button
										key={f}
										className={`px-2.5 py-1 text-xs font-medium rounded-md transition-colors capitalize cursor-pointer ${
											activeFilter === f
												? 'bg-[#262626] text-[#FAFAFA]'
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
					<CardContent className="p-0 flex-1 flex flex-col justify-center bg-[#171717]">
						<div className="divide-y divide-[#262626]">
							{filteredDeployments.map((d) => (
								<div key={d.id} className="flex items-center justify-between px-6 py-3.5 hover:bg-[#262626]/30 transition-colors">
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
								</div>
							))}
						</div>
					</CardContent>
				</Card>
			</div>
		</PageLayout>
	);
}
