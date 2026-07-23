import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Calendar, Plus, Play, Clock, Terminal, Loader2 } from 'lucide-react';
import { PageLayout } from '@/components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { USE_MOCK_DATA, getSchedulesMock, type ScheduleMock } from '$lib/mocks';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { toastSuccess } from '$lib/toast';

export default function SchedulesPage() {
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const [useMock, setUseMock] = useState(USE_MOCK_DATA);
	const [mockSchedules] = useState<ScheduleMock[]>(getSchedulesMock());
	const [runningId, setRunningId] = useState<number | null>(null);

	const schedules = mockSchedules.map((s, idx) => ({
		id: idx + 1,
		name: s.name,
		description: `${s.targetProject} · ${s.targetService}`,
		cron: s.cronExpression,
		shellType: 'bash',
		command: s.command,
		service: s.targetService,
		enabled: s.status === 'active',
		lastRun: s.lastRun,
		nextRun: s.nextRun
	}));

	function runNow(id: number) {
		setRunningId(id);
		setTimeout(() => {
			toastSuccess('Schedule triggered successfully');
			setRunningId(null);
		}, 800);
	}

	return (
		<PageLayout>
			<main className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up flex flex-col min-h-0 bg-[#171717] border border-[#262626] rounded-2xl shadow-md space-y-6">
				<div className="flex items-center justify-between">
					<div>
						<h1 className="text-3xl font-bold tracking-tight text-[#FAFAFA]">Scheduled Tasks</h1>
						<p className="text-sm text-[#a1a1aa] mt-1">Schedule cron commands to execute automatically against your services</p>
					</div>
					<Button size="sm" className="gap-1.5 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] cursor-pointer">
						<Plus className="w-3.5 h-3.5" /> Add Schedule
					</Button>
				</div>

				<div className="grid xl:grid-cols-2 grid-cols-1 gap-4">
					{schedules.map((s) => (
						<div key={s.id} className="flex flex-col sm:flex-row sm:items-center gap-4 justify-between rounded-xl border border-[#262626] bg-[#171717] p-5 hover:border-[#3f3f46] transition-all">
							<div className="flex items-start gap-3.5 min-w-0 flex-1">
								<div className="shrink-0 w-9 h-9 rounded-lg bg-[#262626] border border-white/10 flex items-center justify-center">
									<Clock className="w-4 h-4 text-[#FAFAFA]" />
								</div>
								<div className="space-y-1.5 min-w-0">
									<div className="flex items-center gap-2 flex-wrap">
										<span className="text-sm font-semibold text-[#FAFAFA]">{s.name}</span>
										<Badge variant="outline" className={`text-[10px] px-2 py-0.5 ${s.enabled ? 'border-green-500/30 text-green-400 bg-green-500/10' : 'border-[#262626] text-[#737373]'}`}>
											{s.enabled ? 'Enabled' : 'Disabled'}
										</Badge>
									</div>
									<p className="text-xs text-[#a1a1aa]">{s.description}</p>
									<div className="flex items-center gap-2 flex-wrap font-mono text-[11px] text-[#a1a1aa]">
										<span className="px-2 py-0.5 rounded border border-[#262626] bg-[#141414]">{s.cron}</span>
										<span className="px-2 py-0.5 rounded border border-[#262626] bg-[#141414]">{s.shellType}</span>
									</div>
									<div className="flex items-start gap-1.5 pt-1">
										<Terminal className="w-3.5 h-3.5 text-[#737373] shrink-0 mt-0.5" />
										<code className="font-mono text-[11px] text-[#a1a1aa] break-all">{s.command}</code>
									</div>
									<div className="flex items-center gap-3 text-[11px] font-mono text-[#737373] pt-1">
										<span>Last: {s.lastRun}</span>
										<span>·</span>
										<span>Next: {s.nextRun}</span>
									</div>
								</div>
							</div>

							<div className="flex items-center gap-2 shrink-0 self-end sm:self-center">
								<button
									className="p-2 rounded-lg border border-[#262626] bg-[#262626] text-[#a1a1aa] hover:text-[#FAFAFA] hover:bg-[#333333] transition-colors cursor-pointer"
									onClick={() => runNow(s.id)} disabled={runningId === s.id} title="Run now"
								>
									{runningId === s.id ? <Loader2 className="w-4 h-4 animate-spin" /> : <Play className="w-4 h-4" />}
								</button>
							</div>
						</div>
					))}
				</div>
			</main>
		</PageLayout>
	);
}
