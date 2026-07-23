import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { GitBranch, Plus, Trash2 } from 'lucide-react';
import { PageLayout } from '$lib/../components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { getGitProvidersMock, type GitProviderMock } from '$lib/mocks';
import { Card } from '$lib/../components/ui/card';
import { Button } from '$lib/../components/ui/button';
import { Badge } from '$lib/../components/ui/badge';
import { toastSuccess } from '$lib/toast';

export default function GitProvidersPage() {
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const [providers, setProviders] = useState<GitProviderMock[]>(getGitProvidersMock());

	function disconnect(id: string) {
		setProviders(providers.filter((p) => p.id !== id));
		toastSuccess('Git provider disconnected');
	}

	return (
		<PageLayout>
			<main className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up min-h-0">
				<div className="max-w-5xl mx-auto space-y-6">
					<div className="flex items-center justify-between">
						<div>
							<h1 className="text-3xl font-bold tracking-tight text-[#FAFAFA]">Git Integrations</h1>
							<p className="text-sm text-[#a1a1aa] mt-1">Connect GitHub, GitLab, or Bitbucket for push-to-deploy webhooks</p>
						</div>
						<Button size="sm" className="gap-1.5 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] cursor-pointer">
							<Plus className="w-4 h-4" /> Connect Provider
						</Button>
					</div>

					<div className="grid grid-cols-1 gap-4">
						{providers.map((p) => (
							<Card key={p.id} className="bg-[#171717] border border-[#262626] rounded-xl p-5 flex items-center justify-between hover:border-[#3f3f46] transition-all">
								<div className="flex items-center gap-4">
									<div className="w-10 h-10 rounded-lg bg-[#262626] border border-white/10 flex items-center justify-center font-bold text-sm text-[#FAFAFA]">
										<GitBranch className="w-5 h-5" />
									</div>
									<div>
										<div className="flex items-center gap-2">
											<h2 className="text-base font-semibold text-[#FAFAFA]">{p.provider}</h2>
											<Badge variant="outline" className="text-[10px] border-green-500/30 text-green-400 bg-green-500/10">Connected</Badge>
										</div>
										<p className="text-xs text-[#a1a1aa] font-mono mt-0.5">
											Account: @{p.username} · {p.authType} · {p.repositoriesCount} repositories active
										</p>
									</div>
								</div>
								<button onClick={() => disconnect(p.id)} className="p-2 rounded-lg border border-[#262626] bg-[#262626] text-[#a1a1aa] hover:text-red-400 hover:bg-red-500/10 transition-colors cursor-pointer">
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
