import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Package } from 'lucide-react';
import { PageLayout } from '$lib/../components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { USE_MOCK_DATA, getDockerContainersMock, type DockerContainerMock } from '$lib/mocks';
import { Badge } from '$lib/../components/ui/badge';
import { Button } from '$lib/../components/ui/button';

export default function DockerPage() {
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const [useMock, setUseMock] = useState(USE_MOCK_DATA);
	const [containers] = useState<DockerContainerMock[]>(getDockerContainersMock());

	return (
		<PageLayout>
			<header className="flex items-center justify-between px-6 py-3 border-b border-[#262626] text-xs bg-[#0A0A0A] shrink-0">
				<div className="flex items-center gap-2">
					<Package className="w-3.5 h-3.5 text-[#a1a1aa]" />
					<span className="font-medium text-[#FAFAFA]">Docker Containers</span>
				</div>

				<div className="flex items-center gap-2 px-3 py-1 rounded-full bg-[#141414] border border-[#262626]">
					<span className="text-[11px] text-[#a1a1aa]">Data Source:</span>
					<button
						onClick={() => setUseMock(!useMock)}
						className={`text-[11px] font-semibold px-2 py-0.5 rounded transition-colors cursor-pointer ${
							useMock
								? 'bg-[#262626] text-[#FAFAFA] border border-white/10'
								: 'text-[#737373] hover:text-[#FAFAFA]'
						}`}
					>
						{useMock ? 'Mock Demo Data' : 'Live Rust Backend API'}
					</button>
				</div>
			</header>

			<main className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up flex flex-col min-h-0 bg-[#171717] border border-[#262626] rounded-2xl shadow-md space-y-6">
				<div className="flex items-center justify-between">
					<div>
						<h1 className="text-3xl font-bold tracking-tight text-[#FAFAFA]">Docker Containers</h1>
						<p className="text-sm text-[#a1a1aa] mt-1">Live container engine stats, ports, and memory usage</p>
					</div>
				</div>

				<div className="border border-[#262626] rounded-xl overflow-hidden bg-[#171717]">
					<table className="w-full text-left text-xs">
						<thead className="bg-[#141414] border-b border-[#262626] text-[#737373] uppercase tracking-wider font-semibold">
							<tr>
								<th className="px-5 py-3">CONTAINER</th>
								<th className="px-5 py-3">IMAGE</th>
								<th className="px-5 py-3">STATUS</th>
								<th className="px-5 py-3">PORTS</th>
								<th className="px-5 py-3">CPU / MEM</th>
								<th className="px-5 py-3 text-right">ACTION</th>
							</tr>
						</thead>
						<tbody className="divide-y divide-[#262626]">
							{containers.map((c) => (
								<tr key={c.id} className="hover:bg-[#262626]/30 transition-colors">
									<td className="px-5 py-3.5 font-semibold text-[#FAFAFA]">
										<div className="flex items-center gap-2.5">
											<Package className="w-4 h-4 text-[#a1a1aa]" />
											<div>
												<p>{c.name}</p>
												<p className="text-[10px] font-mono text-[#737373]">{c.id}</p>
											</div>
										</div>
									</td>
									<td className="px-5 py-3.5 font-mono text-[#a1a1aa]">{c.image}</td>
									<td className="px-5 py-3.5">
										<Badge variant="outline" className="text-[10px] px-2 py-0.5 border-green-500/30 text-green-400 bg-green-500/10">
											{c.status}
										</Badge>
									</td>
									<td className="px-5 py-3.5 font-mono text-[#a1a1aa]">{c.ports}</td>
									<td className="px-5 py-3.5 font-mono text-[#a1a1aa]">
										{c.cpuPercent}% · {c.memUsageMb} MB
									</td>
									<td className="px-5 py-3.5 text-right">
										<Button variant="ghost" size="sm" className="h-7 text-xs text-[#FAFAFA] cursor-pointer">Logs</Button>
									</td>
								</tr>
							))}
						</tbody>
					</table>
				</div>
			</main>
		</PageLayout>
	);
}
