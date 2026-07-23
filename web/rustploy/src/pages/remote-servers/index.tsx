import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Rocket, Server, Plus, Globe, FileKey, Trash2, Power, TestTube } from 'lucide-react';
import { PageLayout } from '@/components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { USE_MOCK_DATA, getServersMock, type ServerMock } from '$lib/mocks';
import { Progress } from '@/components/ui/progress';

export default function RemoteServersPage() {
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const [useMock, setUseMock] = useState(USE_MOCK_DATA);
	const [mockServers, setMockServers] = useState<ServerMock[]>(getServersMock());
	const [testingId, setTestingId] = useState<string | null>(null);

	const displayServers = mockServers.map((s) => ({
		id: s.id,
		name: s.name,
		ip: s.ipAddress,
		port: s.port,
		user: s.user,
		status: s.status === 'online' ? 'ACTIVE' : s.status === 'connecting' ? 'CONNECTING' : 'INACTIVE',
		cpuPercent: s.cpuUsagePercent,
		ramPercent: s.ramUsagePercent,
		diskPercent: s.diskUsagePercent,
		os: s.os,
		keyId: s.sshKeyId,
		description: `Docker v${s.dockerVersion}`
	}));

	function statusDotClass(status: string) {
		switch (status) {
			case 'ACTIVE': return 'bg-green-500';
			case 'INACTIVE': return 'bg-zinc-500';
			case 'CONNECTING': return 'bg-blue-500 animate-pulse';
			default: return 'bg-red-500';
		}
	}

	function statusBadgeClass(status: string) {
		const map: Record<string, string> = {
			ACTIVE: 'bg-green-500/10 text-green-400 border-green-500/30',
			INACTIVE: 'bg-[#262626] text-[#a1a1aa] border-[#262626]',
			CONNECTING: 'bg-blue-500/10 text-blue-400 border-blue-500/30'
		};
		return map[status] ?? 'bg-red-500/10 text-red-400 border-red-500/30';
	}

	function testConnection(id: string) {
		setTestingId(id);
		setTimeout(() => {
			setTestingId(null);
		}, 600);
	}

	function toggleActive(id: string) {
		setMockServers((prev) =>
			prev.map((s) => (s.id === id ? { ...s, status: s.status === 'online' ? 'offline' : 'online' } : s))
		);
	}

	function deleteServer(id: string) {
		setMockServers((prev) => prev.filter((s) => s.id !== id));
	}

	return (
		<PageLayout>
			<main className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up flex flex-col min-h-0 bg-[#171717] border border-[#262626] rounded-2xl shadow-md space-y-6">
				<div className="flex items-center justify-between">
					<div>
						<h1 className="text-3xl font-bold tracking-tight text-[#FAFAFA]">Remote Servers</h1>
						<p className="text-sm text-[#a1a1aa] mt-1">Manage SSH connections and node health monitoring across your cluster</p>
					</div>
					<button className="inline-flex items-center gap-2 px-3.5 py-2 rounded-lg bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] text-xs font-semibold transition-colors cursor-pointer">
						<Plus className="w-4 h-4" /> Add Server
					</button>
				</div>

				<div className="grid grid-cols-1 gap-4">
					{displayServers.map((server) => (
						<div
							key={server.id}
							className="bg-[#171717] border border-[#262626] rounded-xl p-5 flex flex-col md:flex-row md:items-center justify-between gap-4 hover:border-[#3f3f46] transition-all"
						>
							<div className="flex items-start gap-4">
								<div className="relative shrink-0 mt-0.5">
									<div className="w-10 h-10 rounded-lg bg-[#262626] border border-white/10 flex items-center justify-center">
										<Server className="w-5 h-5 text-[#FAFAFA]" />
									</div>
									<span className={`absolute -bottom-0.5 -right-0.5 w-3 h-3 rounded-full border-2 border-[#171717] ${statusDotClass(server.status)}`} />
								</div>

								<div className="space-y-1">
									<div className="flex items-center gap-2 flex-wrap">
										<h2 className="text-base font-semibold text-[#FAFAFA]">{server.name}</h2>
										<span className={`inline-flex items-center px-2 py-0.5 rounded text-[10px] font-mono border ${statusBadgeClass(server.status)}`}>{server.status}</span>
										<span className="text-[11px] bg-[#262626] text-[#a1a1aa] px-2 py-0.5 rounded font-mono">{server.os}</span>
									</div>

									<div className="flex items-center gap-3 text-xs text-[#a1a1aa] font-mono flex-wrap">
										<span className="flex items-center gap-1"><Globe className="w-3.5 h-3.5 text-[#737373]" />{server.user}@{server.ip}:{server.port}</span>
										<span>·</span>
										<span className="flex items-center gap-1"><FileKey className="w-3.5 h-3.5 text-[#737373]" />{server.keyId}</span>
									</div>

									<div className="grid grid-cols-3 gap-4 pt-2 max-w-lg">
										<div>
											<div className="flex justify-between text-[11px] text-[#a1a1aa] mb-1 font-mono">
												<span>CPU</span>
												<span>{server.cpuPercent}%</span>
											</div>
											<Progress value={server.cpuPercent} className="h-1 bg-[#262626]" />
										</div>
										<div>
											<div className="flex justify-between text-[11px] text-[#a1a1aa] mb-1 font-mono">
												<span>RAM</span>
												<span>{server.ramPercent}%</span>
											</div>
											<Progress value={server.ramPercent} className="h-1 bg-[#262626]" />
										</div>
										<div>
											<div className="flex justify-between text-[11px] text-[#a1a1aa] mb-1 font-mono">
												<span>DISK</span>
												<span>{server.diskPercent}%</span>
											</div>
											<Progress value={server.diskPercent} className="h-1 bg-[#262626]" />
										</div>
									</div>
								</div>
							</div>

							<div className="flex items-center gap-2 shrink-0 self-end md:self-center">
								<button
									onClick={() => testConnection(server.id)}
									disabled={testingId === server.id}
									className="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg border border-[#262626] bg-[#262626] text-xs font-medium text-[#FAFAFA] hover:bg-[#333333] transition-colors cursor-pointer"
								>
									<TestTube className="w-3.5 h-3.5 text-[#a1a1aa]" />
									<span>Test SSH</span>
								</button>
								<button
									onClick={() => toggleActive(server.id)}
									className="p-2 rounded-lg border border-[#262626] bg-[#262626] text-[#a1a1aa] hover:text-[#FAFAFA] hover:bg-[#333333] transition-colors cursor-pointer"
									title="Toggle Connection State"
								>
									<Power className="w-4 h-4" />
								</button>
								<button
									onClick={() => deleteServer(server.id)}
									className="p-2 rounded-lg border border-[#262626] bg-[#262626] text-[#a1a1aa] hover:text-red-400 hover:bg-red-500/10 transition-colors cursor-pointer"
								>
									<Trash2 className="w-4 h-4" />
								</button>
							</div>
						</div>
					))}
				</div>
			</main>
		</PageLayout>
	);
}
