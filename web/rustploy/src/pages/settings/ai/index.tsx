import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Sparkles, Save } from 'lucide-react';
import { PageLayout } from '$lib/../components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { Card, CardContent, CardHeader, CardTitle, CardDescription, CardFooter } from '$lib/../components/ui/card';
import { Button } from '$lib/../components/ui/button';
import { Input } from '$lib/../components/ui/input';
import { Label } from '$lib/../components/ui/label';
import { Switch } from '$lib/../components/ui/switch';
import { toastSuccess } from '$lib/toast';

export default function AiPage() {
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const [enabled, setEnabled] = useState(true);
	const [apiKey, setApiKey] = useState('sk-proj-••••••••••••••••');
	const [model, setModel] = useState('gpt-4o-mini');
	const [saving, setSaving] = useState(false);

	function save() {
		setSaving(true);
		setTimeout(() => {
			toastSuccess('AI configuration saved');
			setSaving(false);
		}, 400);
	}

	return (
		<PageLayout>
			<header className="flex items-center justify-between px-6 py-3 border-b border-[#262626] text-xs bg-[#0A0A0A] shrink-0">
				<div className="flex items-center gap-2">
					<Sparkles className="w-3.5 h-3.5 text-[#a1a1aa]" />
					<span className="font-medium text-[#FAFAFA]">AI Assistant</span>
				</div>
			</header>

			<main className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up min-h-0">
				<div className="max-w-5xl mx-auto space-y-6">
					<Card className="bg-[#171717] border border-[#262626] rounded-xl shadow-md overflow-hidden">
						<CardHeader className="p-6">
							<CardTitle className="text-base font-semibold text-[#FAFAFA] flex items-center gap-2">
								<Sparkles className="w-4 h-4 text-[#a1a1aa]" />
								AI Assistant Configuration
							</CardTitle>
							<CardDescription className="text-xs text-[#a1a1aa] mt-1">Configure LLM keys for automated Dockerfile generation and log diagnostics</CardDescription>
						</CardHeader>
						<CardContent className="p-6 pt-0 space-y-5">
							<div className="flex items-center justify-between">
								<div>
									<p className="text-xs font-semibold text-[#FAFAFA]">Enable AI Log Summaries</p>
									<p className="text-[11px] text-[#737373]">Automatically analyze build errors and recommend fixes</p>
								</div>
								<Switch checked={enabled} onCheckedChange={(val) => setEnabled(val)} />
							</div>

							<div className="space-y-1.5">
								<Label htmlFor="openai-key" className="text-xs text-[#a1a1aa]">OpenAI API Key</Label>
								<Input id="openai-key" type="password" value={apiKey} onChange={(e) => setApiKey(e.target.value)} placeholder="sk-..." className="bg-[#141414] border-[#262626] text-[#FAFAFA]" />
							</div>

							<div className="space-y-1.5">
								<Label htmlFor="ai-model" className="text-xs text-[#a1a1aa]">Model</Label>
								<Input id="ai-model" value={model} onChange={(e) => setModel(e.target.value)} placeholder="gpt-4o-mini" className="bg-[#141414] border-[#262626] text-[#FAFAFA]" />
							</div>
						</CardContent>
						<CardFooter className="border-t border-[#262626] p-4 flex justify-end bg-[#141414]">
							<Button onClick={save} disabled={saving} size="sm" className="gap-1.5 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] cursor-pointer">
								<Save className="w-3.5 h-3.5" /> {saving ? 'Saving…' : 'Save AI Settings'}
							</Button>
						</CardFooter>
					</Card>
				</div>
			</main>
		</PageLayout>
	);
}
