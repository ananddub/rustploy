import React, { useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { FolderOpen, Rocket, Plus, Boxes, Database, Shield, Terminal, ArrowLeft } from 'lucide-react';
import { PageLayout } from '@/components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { getProjectsMock } from '$lib/mocks';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Tabs, TabsList, TabsTrigger, TabsContent } from '@/components/ui/tabs';

export default function ProjectDetailPage() {
	const { id } = useParams<{ id: string }>();
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const mockProjects = getProjectsMock();
	const project = mockProjects.find((p) => String(p.id) === String(id)) || mockProjects[0];
	const [activeTab, setActiveTab] = useState('apps');

	return (
		<PageLayout>
			<main className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up flex flex-col min-h-0 bg-[#171717] border border-[#262626] rounded-2xl shadow-md space-y-6">
				<div className="flex items-center justify-between">
					<div className="flex items-center gap-3">
						<button onClick={() => navigate('/projects')} className="p-2 rounded-lg border border-[#262626] bg-[#262626] text-[#a1a1aa] hover:text-[#FAFAFA] transition-colors cursor-pointer">
							<ArrowLeft className="w-4 h-4" />
						</button>
						<div>
							<h1 className="text-3xl font-bold tracking-tight text-[#FAFAFA]">{project.name}</h1>
							<p className="text-xs text-[#a1a1aa] mt-0.5">{project.description || 'Project environment and service stacks'}</p>
						</div>
					</div>

					<Button size="sm" className="gap-1.5 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] cursor-pointer">
						<Plus className="w-4 h-4" /> Add Service
					</Button>
				</div>

				<Tabs value={activeTab} onValueChange={setActiveTab} className="flex-1 flex flex-col">
					<TabsList className="bg-[#141414] border border-[#262626] p-1 rounded-xl w-fit">
						<TabsTrigger value="apps" className="text-xs px-4 py-1.5 cursor-pointer">Applications ({project.appsCount})</TabsTrigger>
						<TabsTrigger value="compose" className="text-xs px-4 py-1.5 cursor-pointer">Compose Stacks ({project.composeCount})</TabsTrigger>
						<TabsTrigger value="databases" className="text-xs px-4 py-1.5 cursor-pointer">Databases ({project.databaseCount})</TabsTrigger>
						<TabsTrigger value="env" className="text-xs px-4 py-1.5 cursor-pointer">Environment Variables</TabsTrigger>
					</TabsList>

					<TabsContent value="apps" className="pt-4 flex-1">
						<div className="grid grid-cols-1 md:grid-cols-2 gap-4">
							{project.services.filter((s) => s.type === 'application').map((svc) => (
								<Card
									key={svc.id}
									className="bg-[#171717] border border-[#262626] rounded-xl p-5 hover:border-[#3f3f46] transition-all cursor-pointer"
									onClick={() => navigate(`/projects/${project.id}/app/${svc.id}`)}
								>
									<div className="flex items-start justify-between">
										<div className="flex items-center gap-3">
											<div className="w-9 h-9 rounded-lg bg-[#262626] border border-white/10 flex items-center justify-center">
												<Rocket className="w-4.5 h-4.5 text-[#FAFAFA]" />
											</div>
											<div>
												<h3 className="text-sm font-semibold text-[#FAFAFA]">{svc.name}</h3>
												<p className="text-xs text-[#a1a1aa] font-mono mt-0.5">{svc.subDetails || 'main · v1.0'}</p>
											</div>
										</div>
										<Badge variant="outline" className="text-[10px] border-green-500/30 text-green-400 bg-green-500/10">
											{svc.status}
										</Badge>
									</div>
								</Card>
							))}
						</div>
					</TabsContent>

					<TabsContent value="compose" className="pt-4 flex-1">
						<div className="grid grid-cols-1 md:grid-cols-2 gap-4">
							{project.services.filter((s) => s.type === 'compose').map((svc) => (
								<Card
									key={svc.id}
									className="bg-[#171717] border border-[#262626] rounded-xl p-5 hover:border-[#3f3f46] transition-all cursor-pointer"
									onClick={() => navigate(`/projects/${project.id}/compose/${svc.id}`)}
								>
									<div className="flex items-start justify-between">
										<div className="flex items-center gap-3">
											<div className="w-9 h-9 rounded-lg bg-[#262626] border border-white/10 flex items-center justify-center">
												<Boxes className="w-4.5 h-4.5 text-[#FAFAFA]" />
											</div>
											<div>
												<h3 className="text-sm font-semibold text-[#FAFAFA]">{svc.name}</h3>
												<p className="text-xs text-[#a1a1aa] font-mono mt-0.5">docker-compose.yml</p>
											</div>
										</div>
										<Badge variant="outline" className="text-[10px] border-green-500/30 text-green-400 bg-green-500/10">
											{svc.status}
										</Badge>
									</div>
								</Card>
							))}
						</div>
					</TabsContent>

					<TabsContent value="databases" className="pt-4 flex-1">
						<div className="grid grid-cols-1 md:grid-cols-2 gap-4">
							{project.services.filter((s) => s.type === 'database').map((svc) => (
								<Card
									key={svc.id}
									className="bg-[#171717] border border-[#262626] rounded-xl p-5 hover:border-[#3f3f46] transition-all cursor-pointer"
									onClick={() => navigate(`/projects/${project.id}/database/postgres/${svc.id}`)}
								>
									<div className="flex items-start justify-between">
										<div className="flex items-center gap-3">
											<div className="w-9 h-9 rounded-lg bg-[#262626] border border-white/10 flex items-center justify-center">
												<Database className="w-4.5 h-4.5 text-[#FAFAFA]" />
											</div>
											<div>
												<h3 className="text-sm font-semibold text-[#FAFAFA]">{svc.name}</h3>
												<p className="text-xs text-[#a1a1aa] font-mono mt-0.5">PostgreSQL 16</p>
											</div>
										</div>
										<Badge variant="outline" className="text-[10px] border-green-500/30 text-green-400 bg-green-500/10">
											{svc.status}
										</Badge>
									</div>
								</Card>
							))}
						</div>
					</TabsContent>

					<TabsContent value="env" className="pt-4 flex-1">
						<Card className="bg-[#171717] border border-[#262626] p-5">
							<h3 className="text-sm font-semibold text-[#FAFAFA] mb-2">Global Project Environment Variables</h3>
							<pre className="p-4 rounded-xl bg-[#141414] border border-[#262626] font-mono text-xs text-[#a1a1aa] overflow-x-auto">
{`DATABASE_URL=postgres://user:password@localhost:5432/main_db
REDIS_URL=redis://localhost:6379
NODE_ENV=production`}
							</pre>
						</Card>
					</TabsContent>
				</Tabs>
			</main>
		</PageLayout>
	);
}
