import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Globe2, Server } from 'lucide-react';
import { PageLayout } from '$lib/../components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { USE_MOCK_DATA, getSwarmNodesMock, type SwarmNodeMock } from '$lib/mocks';
import { Badge } from '$lib/../components/ui/badge';

export default function SwarmPage() {
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const [useMock, setUseMock] = useState(USE_MOCK_DATA);
	const [nodes] = useState<SwarmNodeMock[]>(getSwarmNodesMock());

	return (
		<PageLayout>
			<main className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up min-h-0 space-y-6">
				<div className="flex items-center justify-between">
					<div>
						<h1 className="text-3xl font-bold tracking-tight text-[#FAFAFA]">Docker Swarm Cluster</h1>
						<p className="text-sm text-[#a1a1aa] mt-1">Multi-node Docker Swarm manager and worker node status</p>
					</div>
				</div>

				<div className="border border-[#262626] rounded-xl overflow-hidden bg-[#171717]">
					<table className="w-full text-left text-xs">
						<thead className="bg-[#141414] border-b border-[#262626] text-[#737373] uppercase tracking-wider font-semibold">
							<tr>
								<th className="px-5 py-3">NODE HOSTNAME</th>
								<th className="px-5 py-3">ROLE</th>
								<th className="px-5 py-3">STATUS</th>
								<th className="px-5 py-3">IP ADDRESS</th>
								<th className="px-5 py-3">ENGINE VERSION</th>
							</tr>
						</thead>
						<tbody className="divide-y divide-[#262626]">
							{nodes.map((n) => (
								<tr key={n.id} className="hover:bg-[#262626]/30 transition-colors">
									<td className="px-5 py-3.5 font-semibold text-[#FAFAFA]">
										<div className="flex items-center gap-2.5">
											<Server className="w-4 h-4 text-[#a1a1aa]" />
											<div>
												<p>{n.hostname}</p>
												<p className="text-[10px] font-mono text-[#737373]">{n.id}</p>
											</div>
										</div>
									</td>
									<td className="px-5 py-3.5">
										<Badge variant="outline" className="text-[10px] uppercase font-mono border-[#262626] text-[#FAFAFA] bg-[#262626]">
											{n.role}
										</Badge>
									</td>
									<td className="px-5 py-3.5">
										<Badge variant="outline" className="text-[10px] px-2 py-0.5 border-green-500/30 text-green-400 bg-green-500/10 uppercase font-mono">
											{n.status}
										</Badge>
									</td>
									<td className="px-5 py-3.5 font-mono text-[#a1a1aa]">{n.ipAddress}</td>
									<td className="px-5 py-3.5 font-mono text-[#a1a1aa]">v{n.dockerEngineVersion}</td>
								</tr>
							))}
						</tbody>
					</table>
				</div>
			</main>
		</PageLayout>
	);
}
