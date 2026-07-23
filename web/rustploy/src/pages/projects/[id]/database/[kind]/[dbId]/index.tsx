import React from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { Database, ArrowLeft, RefreshCw, HardDrive } from 'lucide-react';
import { PageLayout } from '$lib/../components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { Card } from '$lib/../components/ui/card';
import { Button } from '$lib/../components/ui/button';
import { Badge } from '$lib/../components/ui/badge';
import { toastSuccess } from '$lib/toast';

export default function DatabaseDetailPage() {
	const { id, kind, dbId } = useParams<{ id: string; kind: string; dbId: string }>();
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	return (
		<PageLayout>
			<header className="flex items-center justify-between px-6 py-3 border-b border-[#262626] text-xs bg-[#0A0A0A] shrink-0">
				<div className="flex items-center gap-2">
					<button onClick={() => navigate(`/projects/${id}`)} className="text-[#737373] hover:text-[#FAFAFA] transition-colors flex items-center gap-1.5 cursor-pointer">
						<Database className="w-3.5 h-3.5" /> Project #{id}
					</button>
					<span className="text-[#737373]">/</span>
					<span className="font-medium text-[#FAFAFA] capitalize">{kind || 'postgres'} #{dbId}</span>
				</div>
			</header>

			<main className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up flex flex-col min-h-0 bg-[#171717] border border-[#262626] rounded-2xl shadow-md space-y-6">
				<div className="flex items-center justify-between">
					<div className="flex items-center gap-3">
						<button onClick={() => navigate(`/projects/${id}`)} className="p-2 rounded-lg border border-[#262626] bg-[#262626] text-[#a1a1aa] hover:text-[#FAFAFA] transition-colors cursor-pointer">
							<ArrowLeft className="w-4 h-4" />
						</button>
						<div>
							<h1 className="text-3xl font-bold tracking-tight text-[#FAFAFA] capitalize">{kind || 'PostgreSQL'} Database</h1>
							<p className="text-xs text-[#a1a1aa] mt-0.5">Database ID #{dbId} · Port: 5432 · Engine v16.2</p>
						</div>
					</div>

					<Button onClick={() => toastSuccess('Database backup initiated')} size="sm" className="gap-1.5 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] cursor-pointer">
						<HardDrive className="w-3.5 h-3.5" /> Export Backup
					</Button>
				</div>

				<Card className="bg-[#171717] border border-[#262626] p-5 space-y-4">
					<h3 className="text-sm font-semibold text-[#FAFAFA]">Connection Credentials</h3>
					<div className="grid grid-cols-2 gap-4 text-xs font-mono text-[#a1a1aa]">
						<div><span className="text-[#737373]">Host:</span> localhost</div>
						<div><span className="text-[#737373]">Port:</span> 5432</div>
						<div><span className="text-[#737373]">Database Name:</span> production_db</div>
						<div><span className="text-[#737373]">User:</span> postgres</div>
					</div>
				</Card>
			</main>
		</PageLayout>
	);
}
