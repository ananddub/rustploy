import React, { useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { Boxes, ArrowLeft, RefreshCw, FileCode } from 'lucide-react';
import Editor from '@monaco-editor/react';
import { PageLayout } from '$lib/../components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { Card } from '$lib/../components/ui/card';
import { Button } from '$lib/../components/ui/button';
import { Tabs, TabsList, TabsTrigger, TabsContent } from '$lib/../components/ui/tabs';
import { toastSuccess } from '$lib/toast';

export default function ComposeDetailPage() {
	const { id, composeId } = useParams<{ id: string; composeId: string }>();
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const [activeTab, setActiveTab] = useState('general');
	const [composeYaml, setComposeYaml] = useState(`version: '3.8'
services:
  web:
    image: nginx:alpine
    ports:
      - "8080:80"
  redis:
    image: redis:alpine
`);

	return (
		<PageLayout>
			<main className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up flex flex-col min-h-0 bg-[#171717] border border-[#262626] rounded-2xl shadow-md space-y-6">
				<div className="flex items-center justify-between">
					<div className="flex items-center gap-3">
						<button onClick={() => navigate(`/projects/${id}`)} className="p-2 rounded-lg border border-[#262626] bg-[#262626] text-[#a1a1aa] hover:text-[#FAFAFA] transition-colors cursor-pointer">
							<ArrowLeft className="w-4 h-4" />
						</button>
						<div>
							<h1 className="text-3xl font-bold tracking-tight text-[#FAFAFA]">Docker Compose Stack</h1>
							<p className="text-xs text-[#a1a1aa] mt-0.5">Compose ID #{composeId} · Multi-container stack</p>
						</div>
					</div>

					<Button onClick={() => toastSuccess('Compose stack deployed')} size="sm" className="gap-1.5 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] cursor-pointer">
						<RefreshCw className="w-3.5 h-3.5" /> Deploy Stack
					</Button>
				</div>

				<Tabs value={activeTab} onValueChange={setActiveTab} className="flex-1 flex flex-col">
					<TabsList className="bg-[#141414] border border-[#262626] p-1 rounded-xl w-fit">
						<TabsTrigger value="general" className="text-xs px-4 py-1.5 cursor-pointer">General (YAML)</TabsTrigger>
						<TabsTrigger value="env" className="text-xs px-4 py-1.5 cursor-pointer">Environment</TabsTrigger>
						<TabsTrigger value="domains" className="text-xs px-4 py-1.5 cursor-pointer">Domains</TabsTrigger>
					</TabsList>

					<TabsContent value="general" className="pt-4 flex-1">
						<Card className="bg-[#171717] border border-[#262626] p-5 space-y-3">
							<h3 className="text-sm font-semibold text-[#FAFAFA]">docker-compose.yml</h3>
							<div className="h-80 rounded-xl border border-[#262626] overflow-hidden">
								<Editor
									height="100%"
									language="yaml"
									theme="vs-dark"
									value={composeYaml}
									onChange={(val) => setComposeYaml(val || '')}
									options={{ fontSize: 13, minimap: { enabled: false } }}
								/>
							</div>
						</Card>
					</TabsContent>
				</Tabs>
			</main>
		</PageLayout>
	);
}
