import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Tag, Plus, Trash2 } from 'lucide-react';
import { PageLayout } from '$lib/../components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '$lib/../components/ui/card';
import { Button } from '$lib/../components/ui/button';
import { Input } from '$lib/../components/ui/input';
import { Badge } from '$lib/../components/ui/badge';
import { toastSuccess } from '$lib/toast';

export default function TagsPage() {
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const [tags, setTags] = useState([
		{ id: 'tag-1', name: 'Production', color: 'bg-green-500/10 text-green-400 border-green-500/30' },
		{ id: 'tag-2', name: 'Staging', color: 'bg-amber-500/10 text-amber-400 border-amber-500/30' },
		{ id: 'tag-3', name: 'Frontend', color: 'bg-blue-500/10 text-blue-400 border-blue-500/30' },
		{ id: 'tag-4', name: 'Backend', color: 'bg-purple-500/10 text-purple-400 border-purple-500/30' }
	]);
	const [newTagName, setNewTagName] = useState('');

	function addTag() {
		if (!newTagName.trim()) return;
		setTags([
			...tags,
			{ id: `tag-${Date.now()}`, name: newTagName.trim(), color: 'bg-[#262626] text-[#FAFAFA] border-white/10' }
		]);
		setNewTagName('');
		toastSuccess('Tag created');
	}

	function removeTag(id: string) {
		setTags(tags.filter((t) => t.id !== id));
	}

	return (
		<PageLayout>
			<header className="flex items-center justify-between px-6 py-3 border-b border-[#262626] text-xs bg-[#0A0A0A] shrink-0">
				<div className="flex items-center gap-2">
					<Tag className="w-3.5 h-3.5 text-[#a1a1aa]" />
					<span className="font-medium text-[#FAFAFA]">Tags</span>
				</div>
			</header>

			<main className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up min-h-0">
				<div className="max-w-5xl mx-auto space-y-6">
					<Card className="bg-[#171717] border border-[#262626] rounded-xl shadow-md overflow-hidden">
						<CardHeader className="p-6">
							<CardTitle className="text-base font-semibold text-[#FAFAFA] flex items-center gap-2">
								<Tag className="w-4 h-4 text-[#a1a1aa]" />
								Organization Tags
							</CardTitle>
							<CardDescription className="text-xs text-[#a1a1aa] mt-1">Tags categorize projects and services for filtering and access policies</CardDescription>
						</CardHeader>
						<CardContent className="p-6 pt-0 space-y-5">
							<div className="flex items-center gap-3">
								<Input
									value={newTagName}
									onChange={(e) => setNewTagName(e.target.value)}
									placeholder="New tag name (e.g. Microservices)"
									className="bg-[#141414] border-[#262626] text-[#FAFAFA] max-w-sm text-xs"
								/>
								<Button onClick={addTag} size="sm" className="gap-1.5 text-xs bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] cursor-pointer">
									<Plus className="w-3.5 h-3.5" /> Add Tag
								</Button>
							</div>

							<div className="flex flex-wrap gap-2 pt-2">
								{tags.map((t) => (
									<div key={t.id} className="flex items-center gap-2 px-3 py-1.5 rounded-lg border bg-[#141414] border-[#262626]">
										<Badge variant="outline" className={`text-xs ${t.color}`}>{t.name}</Badge>
										<button onClick={() => removeTag(t.id)} className="text-[#737373] hover:text-red-400 transition-colors cursor-pointer">
											<Trash2 className="w-3.5 h-3.5" />
										</button>
									</div>
								))}
							</div>
						</CardContent>
					</Card>
				</div>
			</main>
		</PageLayout>
	);
}
