import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import {
	Zap,
	Search,
	Rocket,
	Boxes,
	Loader2,
	CircleCheck,
	CircleX,
	GitBranch
} from 'lucide-react';
import { PageLayout } from '@/components/PageLayout';
import { StatusBadge } from '@/components/shared/StatusBadge';
import { getAuthSession } from '$lib/auth';
import { USE_MOCK_DATA, getDeploymentsMock, type DeploymentMock } from '$lib/mocks';
import { Card, CardContent } from '@/components/ui/card';
import { Table, TableHeader, TableBody, TableRow, TableHead, TableCell } from '@/components/ui/table';
import { Input } from '@/components/ui/input';

export default function DeploymentsPage() {
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const [useMock, setUseMock] = useState(USE_MOCK_DATA);
	const [mockDeployments] = useState<DeploymentMock[]>(getDeploymentsMock());
	const [searchQuery, setSearchQuery] = useState('');
	const [filterState, setFilterState] = useState('all');

	const displayDeployments = mockDeployments.map((d) => ({
		id: d.id,
		title: `${d.projectName} · ${d.serviceName}`,
		kind: d.kind,
		state: d.state,
		branch: d.branch,
		commitHash: d.commitHash,
		commitMsg: d.commitMessage,
		duration: `${d.durationSeconds}s`,
		createdAt: d.createdAt
	}));

	const filtered = displayDeployments.filter((d) => {
		const matchSearch =
			!searchQuery ||
			d.id.toLowerCase().includes(searchQuery.toLowerCase()) ||
			d.title.toLowerCase().includes(searchQuery.toLowerCase());
		const matchState = filterState === 'all' || d.state === filterState;
		return matchSearch && matchState;
	});

	const stats = {
		total: displayDeployments.length,
		running: displayDeployments.filter((d) => d.state === 'running' || d.state === 'building').length,
		success: displayDeployments.filter((d) => d.state === 'done').length,
		failed: displayDeployments.filter((d) => d.state === 'error').length
	};

	const stateFilters = ['all', 'running', 'done', 'error'];

	function getServiceIcon(kind: string | undefined) {
		if (kind === 'application') return Rocket;
		if (kind === 'compose') return Boxes;
		return Rocket;
	}

	return (
		<PageLayout>
			<main className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up flex flex-col min-h-0 bg-[#171717] border border-[#262626] rounded-2xl shadow-md space-y-6">
				{/* Stats Cards */}
				<div className="grid grid-cols-2 md:grid-cols-4 gap-3.5">
					<Card className="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs">
						<CardContent className="p-4">
							<div className="flex items-center justify-between">
								<div>
									<p className="text-[10px] font-semibold text-[#a1a1aa] uppercase tracking-wider">Total</p>
									<p className="text-2xl font-bold mt-1 text-[#FAFAFA]">{stats.total}</p>
								</div>
								<div className="w-8 h-8 rounded-lg bg-[#262626] flex items-center justify-center border border-white/10">
									<Zap className="w-4 h-4 text-[#a1a1aa]" />
								</div>
							</div>
						</CardContent>
					</Card>

					<Card className="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs">
						<CardContent className="p-4">
							<div className="flex items-center justify-between">
								<div>
									<p className="text-[10px] font-semibold text-[#a1a1aa] uppercase tracking-wider">Running</p>
									<p className="text-2xl font-bold mt-1 text-blue-400">{stats.running}</p>
								</div>
								<div className="w-8 h-8 rounded-lg bg-blue-500/10 flex items-center justify-center border border-blue-500/20">
									<Loader2 className="w-4 h-4 text-blue-400 animate-spin" />
								</div>
							</div>
						</CardContent>
					</Card>

					<Card className="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs">
						<CardContent className="p-4">
							<div className="flex items-center justify-between">
								<div>
									<p className="text-[10px] font-semibold text-[#a1a1aa] uppercase tracking-wider">Succeeded</p>
									<p className="text-2xl font-bold mt-1 text-green-400">{stats.success}</p>
								</div>
								<div className="w-8 h-8 rounded-lg bg-green-500/10 flex items-center justify-center border border-green-500/20">
									<CircleCheck className="w-4 h-4 text-green-400" />
								</div>
							</div>
						</CardContent>
					</Card>

					<Card className="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs">
						<CardContent className="p-4">
							<div className="flex items-center justify-between">
								<div>
									<p className="text-[10px] font-semibold text-[#a1a1aa] uppercase tracking-wider">Failed</p>
									<p className="text-2xl font-bold mt-1 text-red-400">{stats.failed}</p>
								</div>
								<div className="w-8 h-8 rounded-lg bg-red-500/10 flex items-center justify-center border border-red-500/20">
									<CircleX className="w-4 h-4 text-red-400" />
								</div>
							</div>
						</CardContent>
					</Card>
				</div>

				<div className="space-y-4">
					<div className="flex items-center gap-3 flex-wrap justify-between">
						<div className="relative flex-1 max-w-sm">
							<Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-[#737373]" />
							<Input
								value={searchQuery}
								onChange={(e) => setSearchQuery(e.target.value)}
								placeholder="Search by deployment ID or project..."
								className="pl-9 text-xs h-9 bg-[#141414] border-[#262626] text-[#FAFAFA] placeholder:text-[#737373]"
							/>
						</div>
						<div className="flex items-center gap-1 bg-[#141414] p-1 rounded-lg border border-[#262626]">
							{stateFilters.map((sf) => (
								<button
									key={sf}
									className={`px-2.5 py-1 text-xs font-medium rounded-md transition-colors capitalize cursor-pointer ${
										filterState === sf
											? 'bg-[#262626] text-[#FAFAFA]'
											: 'text-[#737373] hover:text-[#FAFAFA]'
									}`}
									onClick={() => setFilterState(sf)}
								>
									{sf}
								</button>
							))}
						</div>
					</div>

					<Card className="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs overflow-hidden">
						<CardContent className="p-0">
							<Table className="w-full text-left text-xs">
								<TableHeader className="bg-[#141414] border-b border-[#262626] text-[#737373]">
									<TableRow className="hover:bg-transparent border-[#262626]">
										<TableHead className="text-xs text-[#737373]">SERVICE / DEPLOYMENT</TableHead>
										<TableHead className="text-xs text-[#737373]">BRANCH & COMMIT</TableHead>
										<TableHead className="text-xs text-[#737373]">STATUS</TableHead>
										<TableHead className="text-xs text-[#737373]">DURATION</TableHead>
										<TableHead className="text-xs text-[#737373] text-right">TIME</TableHead>
									</TableRow>
								</TableHeader>
								<TableBody className="divide-y divide-[#262626]">
									{filtered.map((d) => {
										const ServiceIcon = getServiceIcon(d.kind);
										return (
											<TableRow key={d.id} className="border-[#262626] hover:bg-[#262626]/30 transition-colors">
												<TableCell>
													<div className="flex items-center gap-3">
														<div className="w-8 h-8 rounded-lg bg-[#262626] border border-white/10 flex items-center justify-center shrink-0">
															<ServiceIcon className="w-4 h-4 text-[#FAFAFA]" />
														</div>
														<div className="flex flex-col">
															<span className="text-xs font-semibold text-[#FAFAFA]">{d.title}</span>
															<span className="text-[10px] font-mono text-[#737373]">ID: #{d.id}</span>
														</div>
													</div>
												</TableCell>
												<TableCell className="font-mono text-xs text-[#a1a1aa]">
													<div className="flex items-center gap-1.5">
														<GitBranch className="w-3.5 h-3.5 text-[#737373]" />
														<span>{d.branch}</span>
														<span className="text-[#737373]">({d.commitHash})</span>
													</div>
													<p className="text-[10px] text-[#737373] mt-0.5 truncate max-w-xs">{d.commitMsg}</p>
												</TableCell>
												<TableCell>
													<StatusBadge status={d.state ?? 'idle'} pulse={d.state === 'running' || d.state === 'building'} />
												</TableCell>
												<TableCell className="text-xs text-[#a1a1aa] font-mono">
													{d.duration}
												</TableCell>
												<TableCell className="text-right text-xs text-[#737373] font-mono">
													{d.createdAt}
												</TableCell>
											</TableRow>
										);
									})}
								</TableBody>
							</Table>
						</CardContent>
					</Card>
				</div>
			</main>
		</PageLayout>
	);
}
