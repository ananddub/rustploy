import React from 'react';
import { PanelLeft, Search } from 'lucide-react';
import { sidebarState } from '$lib/sidebar';

interface HeaderBarProps {
	title: string;
	icon?: React.ComponentType<{ className?: string }>;
}

export function HeaderBar({ title, icon: Icon }: HeaderBarProps) {
	return (
		<header className="flex items-center justify-between px-6 py-3 border-b border-[#262626] text-xs bg-[#0A0A0A] shrink-0">
			<div className="flex items-center gap-3">
				<button
					onClick={() => sidebarState.toggle()}
					className="text-[#a1a1aa] hover:text-[#FAFAFA] transition-colors p-1 rounded-md hover:bg-[#262626] cursor-pointer"
					title="Toggle Sidebar (⌘[|])"
				>
					<PanelLeft className="w-4 h-4" />
				</button>
				<div className="flex items-center gap-2">
					{Icon && <Icon className="w-3.5 h-3.5 text-[#a1a1aa]" />}
					<span className="font-medium text-[#FAFAFA]">{title}</span>
				</div>
			</div>

			<button
				onClick={() => window.dispatchEvent(new CustomEvent('open-command-palette'))}
				className="flex items-center gap-2 px-3 py-1.5 rounded-lg bg-[#141414] border border-[#262626] text-[#737373] hover:text-[#FAFAFA] transition-colors cursor-pointer text-xs"
			>
				<Search className="w-3.5 h-3.5" />
				<span>Search or type command...</span>
				<kbd className="px-1.5 py-0.5 text-[10px] font-mono bg-[#262626] text-[#a1a1aa] rounded border border-white/10 ml-2">
					⌘K
				</kbd>
			</button>
		</header>
	);
}
