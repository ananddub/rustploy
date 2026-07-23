import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import {
	Plus,
	FolderOpen,
	Search,
	Rocket,
	LayoutGrid,
	List,
	GitBranch
} from 'lucide-react';
import { PageLayout } from '$lib/../components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { USE_MOCK_DATA, getProjectsMock, type ProjectMock } from '$lib/mocks';
import { Button } from '$lib/../components/ui/button';
import { Input } from '$lib/../components/ui/input';
import { Badge } from '$lib/../components/ui/badge';
import { Card } from '$lib/../components/ui/card';

export default function ProjectsPage() {
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const [useMock, setUseMock] = useState(USE_MOCK_DATA);
	const [mockProjects] = useState<ProjectMock[]>(getProjectsMock());
	const [searchQuery, setSearchQuery] = useState('');
	const [selectedTag, setSelectedTag] = useState('All');
	const [viewMode, setViewMode] = useState<'grid' | 'table'>('grid');

	const availableTags = ['All', ...new Set(mockProjects.flatMap((p) => p.tags))];

	const filteredProjects = mockProjects.filter((p) => {
		const matchesSearch =
			!searchQuery ||
			p.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
			p.description.toLowerCase().includes(searchQuery.toLowerCase());
		const matchesTag = selectedTag === 'All' || p.tags.includes(selectedTag);
		return matchesSearch && matchesTag;
	});

	function getHealthBadge(status?: string) {
		switch (status) {
			case 'healthy':
				return { label: 'Healthy', color: 'text-green-400 border-green-500/30 bg-green-500/10', dot: 'bg-green-500' };
			case 'deploying':
				return { label: 'Deploying', color: 'text-blue-400 border-blue-500/30 bg-blue-500/10', dot: 'bg-blue-500 animate-pulse' };
			case 'error':
				return { label: 'Error', color: 'text-red-400 border-red-500/30 bg-red-500/10', dot: 'bg-red-500' };
			default:
				return { label: 'Idle', color: 'text-zinc-400 border-[#262626] bg-[#262626]/40', dot: 'bg-zinc-500' };
		}
	}

	return (
		<PageLayout>
			<main className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up flex flex-col min-h-0 bg-[#171717] border border-[#262626] rounded-2xl shadow-md space-y-6">
				<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
					<div>
						<h1 className="text-3xl font-bold tracking-tight text-[#FAFAFA]">Projects</h1>
						<p className="text-sm text-[#a1a1aa] mt-1">Manage your organization's services, deployment environments, and compose stacks</p>
					</div>
					<Button
						variant="default"
						size="default"
						className="gap-2 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] px-4 py-2 rounded-lg self-start sm:self-auto cursor-pointer"
					>
						<Plus className="w-4 h-4" /> Create Project
					</Button>
				</div>

				{/* Toolbar */}
				<div className="flex flex-col md:flex-row md:items-center justify-between gap-3 pt-2">
					<div className="flex flex-wrap items-center gap-3 flex-1">
						<div className="relative flex-1 max-w-sm">
							<Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-[#737373]" />
							<Input
								value={searchQuery}
								onChange={(e) => setSearchQuery(e.target.value)}
								placeholder="Filter projects by name or description..."
								className="pl-9 text-sm h-9 bg-[#141414] border-[#262626] text-[#FAFAFA] placeholder:text-[#737373]"
							/>
						</div>

						<div className="flex items-center gap-1.5 overflow-x-auto py-1">
							{availableTags.map((tag) => (
								<button
									key={tag}
									onClick={() => setSelectedTag(tag)}
									className={`text-xs font-medium px-2.5 py-1 rounded-md transition-colors whitespace-nowrap cursor-pointer ${
										selectedTag === tag
											? 'bg-[#262626] text-[#FAFAFA] border border-white/10'
											: 'text-[#737373] hover:text-[#FAFAFA] hover:bg-[#262626]/40'
									}`}
								>
									{tag}
								</button>
							))}
						</div>
					</div>

					<div className="flex items-center gap-2 shrink-0">
						<div className="flex items-center gap-1 bg-[#141414] p-1 rounded-lg border border-[#262626]">
							<button
								onClick={() => setViewMode('grid')}
								className={`p-1 rounded text-xs transition-colors cursor-pointer ${
									viewMode === 'grid' ? 'bg-[#262626] text-[#FAFAFA]' : 'text-[#737373] hover:text-[#FAFAFA]'
								}`}
								title="Grid View"
							>
								<LayoutGrid className="w-4 h-4" />
							</button>
							<button
								onClick={() => setViewMode('table')}
								className={`p-1 rounded text-xs transition-colors cursor-pointer ${
									viewMode === 'table' ? 'bg-[#262626] text-[#FAFAFA]' : 'text-[#737373] hover:text-[#FAFAFA]'
								}`}
								title="Table View"
							>
								<List className="w-4 h-4" />
							</button>
						</div>
					</div>
				</div>

				{viewMode === 'grid' ? (
					<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
						{filteredProjects.map((project) => {
							const health = getHealthBadge(project.healthStatus);
							return (
								<Card
									key={project.id}
									className="bg-[#171717] border border-[#262626] rounded-xl shadow-2xs hover:border-[#3f3f46] transition-all p-5 flex flex-col justify-between group cursor-pointer"
									onClick={() => navigate(`/projects/${project.id}`)}
								>
									<div>
										<div className="flex items-start justify-between gap-3 mb-2">
											<div className="flex items-center gap-2.5 min-w-0">
												<div className="w-8 h-8 rounded-lg bg-[#262626] border border-white/10 flex items-center justify-center shrink-0">
													<FolderOpen className="w-4 h-4 text-[#FAFAFA]" />
												</div>
												<h2 className="text-base font-semibold text-[#FAFAFA] truncate group-hover:text-white transition-colors">
													{project.name}
												</h2>
											</div>
											<Badge variant="outline" className={`text-[10px] gap-1.5 shrink-0 px-2 py-0.5 font-medium ${health.color}`}>
												<span className={`w-1.5 h-1.5 rounded-full ${health.dot}`} />
												{health.label}
											</Badge>
										</div>

										<p className="text-xs text-[#a1a1aa] line-clamp-2 mb-4 leading-relaxed">
											{project.description || 'No description provided.'}
										</p>

										<div className="flex items-center gap-1.5 flex-wrap mb-4">
											<span className="px-2 py-0.5 rounded border border-[#262626] bg-[#141414] text-[11px] text-[#FAFAFA] font-mono">
												{project.appsCount} Apps
											</span>
											<span className="px-2 py-0.5 rounded border border-[#262626] bg-[#141414] text-[11px] text-[#FAFAFA] font-mono">
												{project.composeCount} Compose
											</span>
											<span className="px-2 py-0.5 rounded border border-[#262626] bg-[#141414] text-[11px] text-[#FAFAFA] font-mono">
												{project.databaseCount} DB
											</span>
										</div>
									</div>

									<div className="pt-3 border-t border-[#262626] flex items-center justify-between text-xs text-[#737373]">
										{project.gitBranch ? (
											<div className="flex items-center gap-1.5 font-mono text-[11px] text-[#a1a1aa]">
												<GitBranch className="w-3.5 h-3.5 text-[#737373]" />
												<span>{project.gitBranch}</span>
											</div>
										) : (
											<span>No Git repo</span>
										)}
										<span className="font-mono text-[11px]">{project.updatedAt}</span>
									</div>
								</Card>
							);
						})}
					</div>
				) : (
					<div className="border border-[#262626] rounded-xl overflow-hidden bg-[#171717]">
						<table className="w-full text-left text-xs">
							<thead className="bg-[#141414] border-b border-[#262626] text-[#737373] uppercase tracking-wider font-semibold">
								<tr>
									<th className="px-5 py-3">PROJECT</th>
									<th className="px-5 py-3">SERVICES</th>
									<th className="px-5 py-3">HEALTH</th>
									<th className="px-5 py-3">GIT BRANCH</th>
									<th className="px-5 py-3">UPDATED</th>
									<th className="px-5 py-3 text-right">ACTION</th>
								</tr>
							</thead>
							<tbody className="divide-y divide-[#262626]">
								{filteredProjects.map((project) => {
									const health = getHealthBadge(project.healthStatus);
									return (
										<tr
											key={project.id}
											className="hover:bg-[#262626]/30 transition-colors cursor-pointer"
											onClick={() => navigate(`/projects/${project.id}`)}
										>
											<td className="px-5 py-3.5 font-semibold text-[#FAFAFA]">
												<div className="flex items-center gap-2.5">
													<FolderOpen className="w-4 h-4 text-[#a1a1aa]" />
													<span>{project.name}</span>
												</div>
											</td>
											<td className="px-5 py-3.5 text-[#a1a1aa] font-mono">
												{project.appsCount} apps · {project.composeCount} compose · {project.databaseCount} db
											</td>
											<td className="px-5 py-3.5">
												<Badge variant="outline" className={`text-[10px] gap-1.5 px-2 py-0.5 font-medium ${health.color}`}>
													<span className={`w-1.5 h-1.5 rounded-full ${health.dot}`} />
													{health.label}
												</Badge>
											</td>
											<td className="px-5 py-3.5 text-[#a1a1aa] font-mono">
												{project.gitBranch || '—'}
											</td>
											<td className="px-5 py-3.5 text-[#737373] font-mono">
												{project.updatedAt}
											</td>
											<td className="px-5 py-3.5 text-right">
												<Button
													variant="ghost"
													size="sm"
													className="h-7 text-xs text-[#FAFAFA]"
													onClick={(e) => {
														e.stopPropagation();
														navigate(`/projects/${project.id}`);
													}}
												>
													Open →
												</Button>
											</td>
										</tr>
									);
								})}
							</tbody>
						</table>
					</div>
				)}
			</main>
		</PageLayout>
	);
}
