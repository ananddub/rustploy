import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { FileText, Search, Shield, Clock } from 'lucide-react';
import { PageLayout } from '$lib/../components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { Card, CardContent } from '$lib/../components/ui/card';
import { Input } from '$lib/../components/ui/input';

export default function AuditLogsPage() {
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const [logs] = useState([
		{ id: 'log-1', action: 'PROJECT_CREATE', actor: 'admin@rustploy.dev', target: 'web-api-service', ip: '192.168.1.5', time: '10 mins ago' },
		{ id: 'log-2', action: 'SSH_KEY_ADD', actor: 'aman@example.com', target: 'deploy-rsa-key', ip: '10.0.0.12', time: '1 hour ago' },
		{ id: 'log-3', action: 'DEPLOY_TRIGGER', actor: 'system-ci', target: 'rustploy-web #104', ip: '127.0.0.1', time: '3 hours ago' },
		{ id: 'log-4', action: 'TRAEFIK_UPDATE', actor: 'admin@rustploy.dev', target: '/etc/traefik/dynamic/http.yml', ip: '192.168.1.5', time: '1 day ago' }
	]);

	const [query, setQuery] = useState('');

	const filtered = logs.filter((l) =>
		!query || l.action.toLowerCase().includes(query.toLowerCase()) || l.actor.toLowerCase().includes(query.toLowerCase())
	);

	return (
		<PageLayout>
			<header className="flex items-center justify-between px-6 py-3 border-b border-[#262626] text-xs bg-[#0A0A0A] shrink-0">
				<div className="flex items-center gap-2">
					<FileText className="w-3.5 h-3.5 text-[#a1a1aa]" />
					<span className="font-medium text-[#FAFAFA]">Audit Logs</span>
				</div>
			</header>

			<main className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up min-h-0">
				<div className="max-w-5xl mx-auto space-y-6">
					<div className="flex items-center justify-between">
						<div>
							<h1 className="text-3xl font-bold tracking-tight text-[#FAFAFA]">Audit Logs</h1>
							<p className="text-sm text-[#a1a1aa] mt-1">Trace all security events, deployments, and configuration mutations</p>
						</div>
					</div>

					<div className="relative max-w-sm">
						<Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-[#737373]" />
						<Input
							value={query}
							onChange={(e) => setQuery(e.target.value)}
							placeholder="Filter logs by action or user..."
							className="pl-9 text-xs h-9 bg-[#141414] border-[#262626] text-[#FAFAFA]"
						/>
					</div>

					<Card className="bg-[#171717] border border-[#262626] rounded-xl shadow-md overflow-hidden">
						<CardContent className="p-0">
							<table className="w-full text-left text-xs">
								<thead className="bg-[#141414] border-b border-[#262626] text-[#737373] uppercase tracking-wider font-semibold">
									<tr>
										<th className="px-5 py-3">ACTION</th>
										<th className="px-5 py-3">USER / ACTOR</th>
										<th className="px-5 py-3">TARGET</th>
										<th className="px-5 py-3">IP ADDRESS</th>
										<th className="px-5 py-3 text-right">TIME</th>
									</tr>
								</thead>
								<tbody className="divide-y divide-[#262626]">
									{filtered.map((l) => (
										<tr key={l.id} className="hover:bg-[#262626]/30 transition-colors">
											<td className="px-5 py-3.5 font-mono font-semibold text-[#FAFAFA]">{l.action}</td>
											<td className="px-5 py-3.5 text-[#a1a1aa] font-mono">{l.actor}</td>
											<td className="px-5 py-3.5 font-mono text-[#FAFAFA]">{l.target}</td>
											<td className="px-5 py-3.5 font-mono text-[#737373]">{l.ip}</td>
											<td className="px-5 py-3.5 text-right font-mono text-[#737373]">{l.time}</td>
										</tr>
									))}
								</tbody>
							</table>
						</CardContent>
					</Card>
				</div>
			</main>
		</PageLayout>
	);
}
