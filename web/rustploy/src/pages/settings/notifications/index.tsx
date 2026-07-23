import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Bell, Save, Plus, Trash2 } from 'lucide-react';
import { PageLayout } from '$lib/../components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { Card, CardContent, CardHeader, CardTitle, CardDescription, CardFooter } from '$lib/../components/ui/card';
import { Button } from '$lib/../components/ui/button';
import { Input } from '$lib/../components/ui/input';
import { Label } from '$lib/../components/ui/label';
import { Switch } from '$lib/../components/ui/switch';
import { toastSuccess } from '$lib/toast';

export default function NotificationsPage() {
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const [discordWebhook, setDiscordWebhook] = useState('https://discord.com/api/webhooks/12345/abcde');
	const [slackWebhook, setSlackWebhook] = useState('');
	const [notifyOnSuccess, setNotifyOnSuccess] = useState(true);
	const [notifyOnError, setNotifyOnError] = useState(true);
	const [saving, setSaving] = useState(false);

	function save() {
		setSaving(true);
		setTimeout(() => {
			toastSuccess('Notification webhooks updated');
			setSaving(false);
		}, 400);
	}

	return (
		<PageLayout>
			<header className="flex items-center justify-between px-6 py-3 border-b border-[#262626] text-xs bg-[#0A0A0A] shrink-0">
				<div className="flex items-center gap-2">
					<Bell className="w-3.5 h-3.5 text-[#a1a1aa]" />
					<span className="font-medium text-[#FAFAFA]">Notifications</span>
				</div>
			</header>

			<main className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up min-h-0">
				<div className="max-w-5xl mx-auto space-y-6">
					<Card className="bg-[#171717] border border-[#262626] rounded-xl shadow-md overflow-hidden">
						<CardHeader className="p-6">
							<CardTitle className="text-base font-semibold text-[#FAFAFA] flex items-center gap-2">
								<Bell className="w-4 h-4 text-[#a1a1aa]" />
								Notification Channels & Webhooks
							</CardTitle>
							<CardDescription className="text-xs text-[#a1a1aa] mt-1">Receive real-time build notifications in Discord, Slack, Telegram, or custom webhooks</CardDescription>
						</CardHeader>
						<CardContent className="p-6 pt-0 space-y-5">
							<div className="flex items-center justify-between">
								<div>
									<p className="text-xs font-semibold text-[#FAFAFA]">Notify on Successful Deployment</p>
									<p className="text-[11px] text-[#737373]">Send a message whenever a service builds and deploys cleanly</p>
								</div>
								<Switch checked={notifyOnSuccess} onCheckedChange={(val) => setNotifyOnSuccess(val)} />
							</div>

							<div className="flex items-center justify-between">
								<div>
									<p className="text-xs font-semibold text-[#FAFAFA]">Notify on Deployment Error</p>
									<p className="text-[11px] text-[#737373]">Send an urgent message when a build fails or container crashes</p>
								</div>
								<Switch checked={notifyOnError} onCheckedChange={(val) => setNotifyOnError(val)} />
							</div>

							<div className="space-y-1.5">
								<Label htmlFor="discord" className="text-xs text-[#a1a1aa]">Discord Webhook URL</Label>
								<Input id="discord" value={discordWebhook} onChange={(e) => setDiscordWebhook(e.target.value)} placeholder="https://discord.com/api/webhooks/..." className="bg-[#141414] border-[#262626] text-[#FAFAFA]" />
							</div>

							<div className="space-y-1.5">
								<Label htmlFor="slack" className="text-xs text-[#a1a1aa]">Slack Webhook URL</Label>
								<Input id="slack" value={slackWebhook} onChange={(e) => setSlackWebhook(e.target.value)} placeholder="https://hooks.slack.com/services/..." className="bg-[#141414] border-[#262626] text-[#FAFAFA]" />
							</div>
						</CardContent>
						<CardFooter className="border-t border-[#262626] p-4 flex justify-end bg-[#141414]">
							<Button onClick={save} disabled={saving} size="sm" className="gap-1.5 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] cursor-pointer">
								<Save className="w-3.5 h-3.5" /> {saving ? 'Saving…' : 'Save Notification Rules'}
							</Button>
						</CardFooter>
					</Card>
				</div>
			</main>
		</PageLayout>
	);
}
