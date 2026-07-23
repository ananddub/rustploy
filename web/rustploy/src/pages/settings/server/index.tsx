import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Settings, Save, RefreshCw, HardDrive } from 'lucide-react';
import { PageLayout } from '$lib/../components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { Card, CardContent, CardHeader, CardTitle, CardDescription, CardFooter } from '$lib/../components/ui/card';
import { Button } from '$lib/../components/ui/button';
import { Input } from '$lib/../components/ui/input';
import { Label } from '$lib/../components/ui/label';
import { toastSuccess } from '$lib/toast';

export default function ServerSettingsPage() {
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const [domain, setDomain] = useState('dashboard.rustploy.dev');
	const [sshPort, setSshPort] = useState('22');
	const [saving, setSaving] = useState(false);
	const [pruning, setPruning] = useState(false);

	function save() {
		setSaving(true);
		setTimeout(() => {
			toastSuccess('Server instance settings saved');
			setSaving(false);
		}, 400);
	}

	function pruneDocker() {
		setPruning(true);
		setTimeout(() => {
			toastSuccess('Docker cleanup completed: Freed 4.2 GB disk space');
			setPruning(false);
		}, 800);
	}

	return (
		<PageLayout>
			<main className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up min-h-0">
				<div className="max-w-5xl mx-auto space-y-6">
					<Card className="bg-[#171717] border border-[#262626] rounded-xl shadow-md overflow-hidden">
						<CardHeader className="p-6">
							<CardTitle className="text-base font-semibold text-[#FAFAFA] flex items-center gap-2">
								<Settings className="w-4 h-4 text-[#a1a1aa]" />
								Rustploy Core Host System
							</CardTitle>
							<CardDescription className="text-xs text-[#a1a1aa] mt-1">Configure global server domain, ports, and automated system maintenance</CardDescription>
						</CardHeader>
						<CardContent className="p-6 pt-0 space-y-5">
							<div className="space-y-1.5">
								<Label htmlFor="server-domain" className="text-xs text-[#a1a1aa]">Rustploy Dashboard Domain</Label>
								<Input id="server-domain" value={domain} onChange={(e) => setDomain(e.target.value)} className="bg-[#141414] border-[#262626] text-[#FAFAFA]" />
							</div>

							<div className="space-y-1.5">
								<Label htmlFor="ssh-port" className="text-xs text-[#a1a1aa]">Host SSH Port</Label>
								<Input id="ssh-port" value={sshPort} onChange={(e) => setSshPort(e.target.value)} className="bg-[#141414] border-[#262626] text-[#FAFAFA]" />
							</div>

							<div className="pt-2 border-t border-[#262626] flex items-center justify-between">
								<div>
									<p className="text-xs font-semibold text-[#FAFAFA]">Docker Garbage Collection</p>
									<p className="text-[11px] text-[#737373]">Prune unused images, build cache, and stopped containers</p>
								</div>
								<Button variant="outline" size="sm" onClick={pruneDocker} disabled={pruning} className="gap-1.5 text-xs bg-[#262626] border-[#3f3f46] text-[#FAFAFA] hover:bg-[#333333] cursor-pointer">
									<HardDrive className="w-3.5 h-3.5" />
									{pruning ? 'Cleaning…' : 'Prune System'}
								</Button>
							</div>
						</CardContent>
						<CardFooter className="border-t border-[#262626] p-4 flex justify-end bg-[#141414]">
							<Button onClick={save} disabled={saving} size="sm" className="gap-1.5 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] cursor-pointer">
								<Save className="w-3.5 h-3.5" /> {saving ? 'Saving…' : 'Save Host Settings'}
							</Button>
						</CardFooter>
					</Card>
				</div>
			</main>
		</PageLayout>
	);
}
