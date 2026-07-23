import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { Search, Rocket, Boxes, Server, Shield, Key, Terminal, Calendar, X } from 'lucide-react';

export function CommandPalette() {
	const [open, setOpen] = useState(false);
	const [query, setQuery] = useState('');
	const navigate = useNavigate();

	useEffect(() => {
		const handleKeyDown = (e: KeyboardEvent) => {
			if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
				e.preventDefault();
				setOpen((prev) => !prev);
			}
			if (e.key === 'Escape') setOpen(false);
		};

		const handleCustomEvent = () => setOpen(true);

		window.addEventListener('keydown', handleKeyDown);
		window.addEventListener('open-command-palette', handleCustomEvent);
		return () => {
			window.removeEventListener('keydown', handleKeyDown);
			window.removeEventListener('open-command-palette', handleCustomEvent);
		};
	}, []);

	if (!open) return null;

	const actions = [
		{ title: 'Dashboard', icon: Rocket, path: '/dashboard' },
		{ title: 'Projects Overview', icon: Boxes, path: '/projects' },
		{ title: 'Deployments', icon: Terminal, path: '/deployments' },
		{ title: 'Remote Servers', icon: Server, path: '/remote-servers' },
		{ title: 'Cron Schedules', icon: Calendar, path: '/schedules' },
		{ title: 'SSH Keys', icon: Key, path: '/ssh-keys' },
		{ title: 'Certificates', icon: Shield, path: '/settings/certificates' }
	].filter((a) => a.title.toLowerCase().includes(query.toLowerCase()));

	return (
		<div className="fixed inset-0 z-50 flex items-start justify-center pt-24 bg-black/70 backdrop-blur-xs p-4">
			<div className="bg-[#171717] border border-[#262626] rounded-xl shadow-2xl w-full max-w-lg overflow-hidden flex flex-col animate-fade-up">
				<div className="flex items-center px-4 py-3 border-b border-[#262626]">
					<Search className="w-4 h-4 text-[#737373] mr-2 shrink-0" />
					<input
						type="text"
						autoFocus
						value={query}
						onChange={(e) => setQuery(e.target.value)}
						placeholder="Search commands, projects, servers... (⌘K)"
						className="w-full bg-transparent text-xs text-[#FAFAFA] placeholder:text-[#737373] focus:outline-none"
					/>
					<button onClick={() => setOpen(false)} className="text-[#737373] hover:text-[#FAFAFA] p-1">
						<X className="w-4 h-4" />
					</button>
				</div>
				<div className="max-h-80 overflow-y-auto p-2 space-y-1">
					{actions.length === 0 ? (
						<p className="text-xs text-[#737373] py-6 text-center">No commands found</p>
					) : (
						actions.map((act) => {
							const Icon = act.icon;
							return (
								<button
									key={act.path}
									onClick={() => {
										navigate(act.path);
										setOpen(false);
									}}
									className="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-xs text-[#FAFAFA] hover:bg-[#262626] transition-colors text-left"
								>
									<Icon className="w-4 h-4 text-[#a1a1aa]" />
									<span className="font-medium">{act.title}</span>
								</button>
							);
						})
					)}
				</div>
			</div>
		</div>
	);
}
