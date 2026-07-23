import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Building2, Save, Trash2 } from 'lucide-react';
import { PageLayout } from '$lib/../components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { Card, CardContent, CardHeader, CardTitle, CardDescription, CardFooter } from '$lib/../components/ui/card';
import { Button } from '$lib/../components/ui/button';
import { Input } from '$lib/../components/ui/input';
import { Label } from '$lib/../components/ui/label';
import { Badge } from '$lib/../components/ui/badge';
import { toastSuccess } from '$lib/toast';

export default function SettingsPage() {
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const [formName, setFormName] = useState('My Organization');
	const [formSlug, setFormSlug] = useState('my-org');
	const [formLogo, setFormLogo] = useState('');
	const [saving, setSaving] = useState(false);

	function save() {
		setSaving(true);
		setTimeout(() => {
			toastSuccess('Organization settings saved');
			setSaving(false);
		}, 400);
	}

	return (
		<PageLayout>
			<header className="flex items-center justify-between px-6 py-3 border-b border-[#262626] text-xs bg-[#0A0A0A] shrink-0">
				<div className="flex items-center gap-2">
					<Building2 className="w-3.5 h-3.5 text-[#a1a1aa]" />
					<span className="font-medium text-[#FAFAFA]">Organization Settings</span>
				</div>
			</header>

			<main className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up min-h-0">
				<div className="max-w-5xl mx-auto space-y-6">
					<Card className="bg-[#171717] border border-[#262626] rounded-xl shadow-md overflow-hidden">
						<CardHeader className="p-6">
							<CardTitle className="text-base font-semibold text-[#FAFAFA] flex items-center gap-2">
								<Building2 className="w-4 h-4 text-[#a1a1aa]" />
								Organization Details
							</CardTitle>
							<CardDescription className="text-xs text-[#a1a1aa] mt-1">Manage your organization name, slug, and branding</CardDescription>
						</CardHeader>
						<CardContent className="p-6 pt-0 space-y-5">
							<div className="flex flex-wrap gap-3 text-xs">
								<div className="flex items-center gap-1.5 text-[#a1a1aa]">
									<span>Created:</span>
									<span className="font-medium text-[#FAFAFA]">Jun 12, 2026</span>
								</div>
								<div className="flex items-center gap-1.5 text-[#a1a1aa]">
									<span>Owner:</span>
									<Badge variant="outline" className="text-[10px] font-mono border-[#262626] text-[#FAFAFA] bg-[#262626]">user-01</Badge>
								</div>
							</div>

							<div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
								<div className="space-y-1.5">
									<Label htmlFor="org-name" className="text-xs text-[#a1a1aa]">Organization Name</Label>
									<Input id="org-name" value={formName} onChange={(e) => setFormName(e.target.value)} placeholder="My Organization" className="bg-[#141414] border-[#262626] text-[#FAFAFA]" />
								</div>
								<div className="space-y-1.5">
									<Label htmlFor="org-slug" className="text-xs text-[#a1a1aa]">Slug</Label>
									<Input id="org-slug" value={formSlug} onChange={(e) => setFormSlug(e.target.value)} placeholder="my-org" className="bg-[#141414] border-[#262626] text-[#FAFAFA]" />
									<p className="text-[10px] text-[#737373]">Used in URLs. Lowercase, no spaces.</p>
								</div>
							</div>
							<div className="space-y-1.5">
								<Label htmlFor="org-logo" className="text-xs text-[#a1a1aa]">Logo URL</Label>
								<Input id="org-logo" value={formLogo} onChange={(e) => setFormLogo(e.target.value)} placeholder="https://example.com/logo.png" className="bg-[#141414] border-[#262626] text-[#FAFAFA]" />
							</div>
						</CardContent>
						<CardFooter className="border-t border-[#262626] p-4 flex items-center justify-end bg-[#141414]">
							<Button onClick={save} disabled={saving || !formName.trim()} size="sm" className="gap-1.5 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] cursor-pointer">
								<Save className="w-3.5 h-3.5" />
								{saving ? 'Saving…' : 'Save Changes'}
							</Button>
						</CardFooter>
					</Card>

					{/* Danger Zone */}
					<Card className="bg-[#171717] border border-red-500/20 rounded-xl shadow-md overflow-hidden">
						<CardHeader className="p-6">
							<CardTitle className="text-base font-semibold flex items-center gap-2 text-red-400">
								<Trash2 className="w-4 h-4" />
								Danger Zone
							</CardTitle>
							<CardDescription className="text-xs text-[#a1a1aa] mt-1">Irreversible actions for your organization</CardDescription>
						</CardHeader>
						<CardContent className="p-6 pt-0">
							<div className="flex items-center justify-between">
								<div>
									<p className="text-xs font-semibold text-[#FAFAFA]">Delete Organization</p>
									<p className="text-[11px] text-[#737373]">Permanently delete this organization and all its data</p>
								</div>
								<Button variant="destructive" size="sm" className="text-xs gap-1.5 bg-red-500/20 text-red-400 border border-red-500/30 hover:bg-red-500/30 cursor-pointer">
									<Trash2 className="w-3.5 h-3.5" />
									Delete
								</Button>
							</div>
						</CardContent>
					</Card>
				</div>
			</main>
		</PageLayout>
	);
}
