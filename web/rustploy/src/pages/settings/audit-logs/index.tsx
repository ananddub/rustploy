import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Search } from 'lucide-react';
import { PageLayout } from '$lib/../components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { getAuditLogsMock, type AuditLogMock } from '$lib/mocks';
import { Card, CardContent } from '$lib/../components/ui/card';
import { Input } from '$lib/../components/ui/input';
import { Badge } from '$lib/../components/ui/badge';

export default function AuditLogsPage() {
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const [logs] = useState<AuditLogMock[]>(getAuditLogsMock());
	const [searchQuery, setSearchQuery] = useState('');

	const filteredLogs = logs.filter((l) =>
		!searchQuery ||
		l.action.toLowerCase().includes(searchQuery.toLowerCase()) ||
		l.actor.toLowerCase().includes(searchQuery.toLowerCase()) ||
		l.target.toLowerCase().includes(searchQuery.toLowerCase())
	);

	function severityBadge(sev: string) {
		if (sev === 'warn') return 'border-amber-500/30 text-amber-400 bg-amber-500/10';
		if (sev === 'error') return 'border-red-500/30 text-red-400 bg-red-500/10';
		return 'border-blue-500/30 text-blue-400 bg-blue-500/10';
	}

	return (
		<PageLayout>
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
							value={searchQuery}
							onChange={(e) => setSearchQuery(e.target.value)}
							placeholder="Filter logs by action, target, or user..."
							className="pl-9 text-xs h-9 bg-[#141414] border-[#262626] text-[#FAFAFA]"
						/>
					</div>

					<Card className="bg-[#171717] border border-[#262626] rounded-xl shadow-md overflow-hidden">
						<CardContent className="p-0">
							<table className="w-full text-left text-xs">
								<thead className="bg-[#141414] border-b border-[#262626] text-[#737373] uppercase tracking-wider font-semibold">
									<tr>
										<th className="px-5 py-3">ACTION</th>
										<th className="px-5 py-3">SEVERITY</th>
										<th className="px-5 py-3">USER / ACTOR</th>
										<th className="px-5 py-3">TARGET</th>
										<th className="px-5 py-3">IP ADDRESS</th>
										<th className="px-5 py-3 text-right">TIME</th>
									</tr>
								</thead>
								<tbody className="divide-y divide-[#262626]">
									{filteredLogs.map((l) => (
										<tr key={l.id} className="hover:bg-[#262626]/30 transition-colors">
											<td className="px-5 py-3.5 font-mono font-semibold text-[#FAFAFA]">{l.action}</td>
											<td className="px-5 py-3.5">
												<Badge variant="outline" className={`text-[10px] uppercase font-mono ${severityBadge(l.severity)}`}>
													{l.severity}
												</Badge>
											</td>
											<td className="px-5 py-3.5 text-[#a1a1aa] font-mono">{l.actor}</td>
											<td className="px-5 py-3.5 font-mono text-[#FAFAFA]">{l.target}</td>
											<td className="px-5 py-3.5 font-mono text-[#737373]">{l.ipAddress}</td>
											<td className="px-5 py-3.5 text-right font-mono text-[#737373]">{l.timestamp}</td>
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
