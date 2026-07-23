import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Bell, Plus, Trash2 } from 'lucide-react';
import { PageLayout } from '@/components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { getNotificationsMock, type NotificationChannelMock } from '$lib/mocks';
import { Card } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { toastSuccess } from '$lib/toast';

export default function NotificationsPage() {
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const [channels, setChannels] = useState<NotificationChannelMock[]>(getNotificationsMock());

	function removeChannel(id: string) {
		setChannels(channels.filter((c) => c.id !== id));
		toastSuccess('Notification channel removed');
	}

	return (
		<PageLayout>
			<main className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up min-h-0">
				<div className="max-w-5xl mx-auto space-y-6">
					<div className="flex items-center justify-between">
						<div>
							<h1 className="text-3xl font-bold tracking-tight text-[#FAFAFA]">Notification Channels & Webhooks</h1>
							<p className="text-sm text-[#a1a1aa] mt-1">Configure Discord, Slack, Telegram, or custom webhooks for real-time build alerts</p>
						</div>
						<Button size="sm" className="gap-1.5 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] cursor-pointer">
							<Plus className="w-4 h-4" /> Add Channel
						</Button>
					</div>

					<div className="grid grid-cols-1 gap-4">
						{channels.map((c) => (
							<Card key={c.id} className="bg-[#171717] border border-[#262626] rounded-xl p-5 flex items-center justify-between hover:border-[#3f3f46] transition-all">
								<div className="flex items-center gap-4">
									<div className="w-10 h-10 rounded-lg bg-[#262626] border border-white/10 flex items-center justify-center font-bold text-sm text-[#FAFAFA]">
										<Bell className="w-5 h-5" />
									</div>
									<div>
										<div className="flex items-center gap-2">
											<h2 className="text-base font-semibold text-[#FAFAFA]">{c.name}</h2>
											<Badge variant="outline" className="text-[10px] border-green-500/30 text-green-400 bg-green-500/10">{c.provider}</Badge>
										</div>
										<p className="text-xs text-[#a1a1aa] font-mono mt-0.5">
											Target: {c.targetUrl.slice(0, 45)}… · {c.notifyOnSuccess ? 'Success & Errors' : 'Errors Only'}
										</p>
									</div>
								</div>
								<button onClick={() => removeChannel(c.id)} className="p-2 rounded-lg border border-[#262626] bg-[#262626] text-[#a1a1aa] hover:text-red-400 hover:bg-red-500/10 transition-colors cursor-pointer">
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
