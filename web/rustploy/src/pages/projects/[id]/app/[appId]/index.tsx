import React, { useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { Rocket, ArrowLeft, Globe, Terminal, Settings, Shield, RefreshCw } from 'lucide-react';
import Editor from '@monaco-editor/react';
import { PageLayout } from '$lib/../components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { Card, CardContent } from '$lib/../components/ui/card';
import { Button } from '$lib/../components/ui/button';
import { Badge } from '$lib/../components/ui/badge';
import { Tabs, TabsList, TabsTrigger, TabsContent } from '$lib/../components/ui/tabs';
import { toastSuccess } from '$lib/toast';

export default function AppDetailPage() {
	const { id, appId } = useParams<{ id: string; appId: string }>();
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const [activeTab, setActiveTab] = useState('general');
	const [deploying, setDeploying] = useState(false);
	const [envCode, setEnvCode] = useState(`PORT=3000\nNODE_ENV=production\nDATABASE_URL=postgres://root:password@postgres:5432/production`);

	function redeploy() {
		setDeploying(true);
		setTimeout(() => {
			toastSuccess('Deployment triggered successfully');
			setDeploying(false);
		}, 800);
	}

	return (
		<PageLayout>
			<header className="flex items-center justify-between px-6 py-3 border-b border-[#262626] text-xs bg-[#0A0A0A] shrink-0">
				<div className="flex items-center gap-2">
					<button onClick={() => navigate(`/projects/${id}`)} className="text-[#737373] hover:text-[#FAFAFA] transition-colors flex items-center gap-1.5 cursor-pointer">
						<Rocket className="w-3.5 h-3.5" /> Project #{id}
					</button>
					<span className="text-[#737373]">/</span>
					<span className="font-medium text-[#FAFAFA]">Application #{appId}</span>
				</div>
			</header>

			<main className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up flex flex-col min-h-0 bg-[#171717] border border-[#262626] rounded-2xl shadow-md space-y-6">
				<div className="flex items-center justify-between">
					<div className="flex items-center gap-3">
						<button onClick={() => navigate(`/projects/${id}`)} className="p-2 rounded-lg border border-[#262626] bg-[#262626] text-[#a1a1aa] hover:text-[#FAFAFA] transition-colors cursor-pointer">
							<ArrowLeft className="w-4 h-4" />
						</button>
						<div>
							<h1 className="text-3xl font-bold tracking-tight text-[#FAFAFA]">Web Application Service</h1>
							<p className="text-xs text-[#a1a1aa] mt-0.5">App ID #{appId} · Branch: main · Production</p>
						</div>
					</div>

					<Button onClick={redeploy} disabled={deploying} size="sm" className="gap-1.5 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] cursor-pointer">
						<RefreshCw className={`w-3.5 h-3.5 ${deploying ? 'animate-spin' : ''}`} />
						{deploying ? 'Building…' : 'Deploy Service'}
					</Button>
				</div>

				<Tabs value={activeTab} onValueChange={setActiveTab} className="flex-1 flex flex-col">
					<TabsList className="bg-[#141414] border border-[#262626] p-1 rounded-xl w-fit">
						<TabsTrigger value="general" className="text-xs px-4 py-1.5 cursor-pointer">General</TabsTrigger>
						<TabsTrigger value="env" className="text-xs px-4 py-1.5 cursor-pointer">Environment</TabsTrigger>
						<TabsTrigger value="domains" className="text-xs px-4 py-1.5 cursor-pointer">Domains</TabsTrigger>
						<TabsTrigger value="previews" className="text-xs px-4 py-1.5 cursor-pointer">Preview Deployments</TabsTrigger>
						<TabsTrigger value="advanced" className="text-xs px-4 py-1.5 cursor-pointer">Advanced</TabsTrigger>
					</TabsList>

					<TabsContent value="general" className="pt-4 flex-1 space-y-4">
						<Card className="bg-[#171717] border border-[#262626] p-5">
							<h3 className="text-sm font-semibold text-[#FAFAFA] mb-2">Service Status & Build Configuration</h3>
							<div className="grid grid-cols-2 gap-4 text-xs font-mono text-[#a1a1aa]">
								<div><span className="text-[#737373]">Repository:</span> github.com/rustploy/web</div>
								<div><span className="text-[#737373]">Branch:</span> main</div>
								<div><span className="text-[#737373]">Build Type:</span> Nixpacks / Dockerfile</div>
								<div><span className="text-[#737373]">Port:</span> 3000</div>
							</div>
						</Card>
					</TabsContent>

					<TabsContent value="env" className="pt-4 flex-1">
						<Card className="bg-[#171717] border border-[#262626] p-5 space-y-3">
							<h3 className="text-sm font-semibold text-[#FAFAFA]">Environment Variables (.env)</h3>
							<div className="h-64 rounded-xl border border-[#262626] overflow-hidden">
								<Editor
									height="100%"
									language="ini"
									theme="vs-dark"
									value={envCode}
									onChange={(val) => setEnvCode(val || '')}
									options={{ fontSize: 13, minimap: { enabled: false } }}
								/>
							</div>
						</Card>
					</TabsContent>

					<TabsContent value="domains" className="pt-4 flex-1">
						<Card className="bg-[#171717] border border-[#262626] p-5 space-y-3">
							<h3 className="text-sm font-semibold text-[#FAFAFA]">Traefik Domain Routing</h3>
							<div className="flex items-center justify-between p-3 rounded-lg bg-[#141414] border border-[#262626] text-xs">
								<div className="flex items-center gap-2">
									<Globe className="w-4 h-4 text-green-400" />
									<span className="font-mono text-[#FAFAFA]">https://app.rustploy.dev</span>
								</div>
								<Badge variant="outline" className="border-green-500/30 text-green-400 bg-green-500/10 text-[10px]">Active SSL</Badge>
							</div>
						</Card>
					</TabsContent>

					<TabsContent value="previews" className="pt-4 flex-1">
						<Card className="bg-[#171717] border border-[#262626] p-5">
							<h3 className="text-sm font-semibold text-[#FAFAFA] mb-2">Pull Request Preview Deployments</h3>
							<p className="text-xs text-[#a1a1aa]">Automatically spin up isolated preview URLs for every pull request opened on GitHub.</p>
						</Card>
					</TabsContent>

					<TabsContent value="advanced" className="pt-4 flex-1">
						<Card className="bg-[#171717] border border-[#262626] p-5">
							<h3 className="text-sm font-semibold text-[#FAFAFA] mb-2">Advanced Container Health & Resource Limits</h3>
							<p className="text-xs text-[#a1a1aa]">Set custom Docker restart policies, CPU shares, and memory reservation thresholds.</p>
						</Card>
					</TabsContent>
				</Tabs>
			</main>
		</PageLayout>
	);
}
