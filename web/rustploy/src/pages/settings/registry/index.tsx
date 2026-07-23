import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Database, Plus, Trash2 } from 'lucide-react';
import { PageLayout } from '$lib/../components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { Card, CardContent } from '$lib/../components/ui/card';
import { Button } from '$lib/../components/ui/button';
import { Badge } from '$lib/../components/ui/badge';
import { toastSuccess } from '$lib/toast';

export default function RegistryPage() {
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const [registries, setRegistries] = useState([
		{ id: 'reg-1', name: 'Docker Hub', url: 'registry-1.docker.io', user: 'rustployofficial', isDefault: true },
		{ id: 'reg-2', name: 'GitHub Container Registry', url: 'ghcr.io', user: 'ananddub', isDefault: false }
	]);

	function removeRegistry(id: string) {
		setRegistries(registries.filter((r) => r.id !== id));
		toastSuccess('Registry credentials removed');
	}

	return (
		<PageLayout>
			<header className="flex items-center justify-between px-6 py-3 border-b border-[#262626] text-xs bg-[#0A0A0A] shrink-0">
				<div className="flex items-center gap-2">
					<Database className="w-3.5 h-3.5 text-[#a1a1aa]" />
					<span className="font-medium text-[#FAFAFA]">Docker Registries</span>
				</div>
			</header>

			<main className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up min-h-0">
				<div className="max-w-5xl mx-auto space-y-6">
					<div className="flex items-center justify-between">
						<div>
							<h1 className="text-3xl font-bold tracking-tight text-[#FAFAFA]">Docker Container Registries</h1>
							<p className="text-sm text-[#a1a1aa] mt-1">Configure credentials for pulling private Docker images</p>
						</div>
						<Button size="sm" className="gap-1.5 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] cursor-pointer">
							<Plus className="w-4 h-4" /> Add Registry
						</Button>
					</div>

					<div className="grid grid-cols-1 gap-4">
						{registries.map((r) => (
							<Card key={r.id} className="bg-[#171717] border border-[#262626] rounded-xl p-5 flex items-center justify-between">
								<div className="flex items-center gap-4">
									<div className="w-10 h-10 rounded-lg bg-[#262626] border border-white/10 flex items-center justify-center font-bold text-sm text-[#FAFAFA]">
										<Database className="w-5 h-5" />
									</div>
									<div>
										<div className="flex items-center gap-2">
											<h2 className="text-base font-semibold text-[#FAFAFA]">{r.name}</h2>
											{r.isDefault && <Badge variant="outline" className="text-[10px] border-[#262626] text-[#FAFAFA] bg-[#262626]">Default</Badge>}
										</div>
										<p className="text-xs text-[#a1a1aa] font-mono mt-0.5">{r.user} @ {r.url}</p>
									</div>
								</div>
								<button onClick={() => removeRegistry(r.id)} className="p-2 rounded-lg border border-[#262626] bg-[#262626] text-[#a1a1aa] hover:text-red-400 hover:bg-red-500/10 transition-colors cursor-pointer">
									<Trash2 className="w-4 h-4" />
								</button>
							</Card>
						))}
					</div>
				</div>
			</main>
		</PageLayout>
	);
}
