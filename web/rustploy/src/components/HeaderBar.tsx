import React, { useState } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import {
	PanelLeft,
	Search,
	Home,
	FolderOpen,
	Zap,
	Clock,
	Globe,
	Package,
	Globe2,
	User,
	Server,
	Users,
	FileText,
	Key,
	Sparkles,
	Tag,
	GitBranch,
	Database,
	Layers,
	Shield,
	Cpu,
	Bell,
	Building2,
	Settings,
	Plus
} from 'lucide-react';
import { useSidebarState } from '$lib/sidebar';
import { USE_MOCK_DATA } from '$lib/mocks';

const routeTitleMap: Record<string, { title: string; icon: React.ComponentType<{ className?: string }> }> = {
	'/dashboard': { title: 'Home', icon: Home },
	'/projects': { title: 'Projects', icon: FolderOpen },
	'/deployments': { title: 'Deployments', icon: Zap },
	'/schedules': { title: 'Schedules', icon: Clock },
	'/traefik': { title: 'Traefik File System', icon: Globe },
	'/docker': { title: 'Docker Containers', icon: Package },
	'/swarm': { title: 'Docker Swarm Cluster', icon: Globe2 },
	'/ssh-keys': { title: 'SSH Keys', icon: Key },
	'/remote-servers': { title: 'Remote Servers', icon: Server },
	'/settings': { title: 'Organization Settings', icon: Building2 },
	'/settings/profile': { title: 'Settings / Profile', icon: User },
	'/settings/users': { title: 'Settings / Users & Roles', icon: Users },
	'/settings/audit-logs': { title: 'Settings / Audit Logs', icon: FileText },
	'/settings/ai': { title: 'Settings / AI Assistant', icon: Sparkles },
	'/settings/tags': { title: 'Settings / Tags', icon: Tag },
	'/settings/git-providers': { title: 'Settings / Git Integrations', icon: GitBranch },
	'/settings/registry': { title: 'Settings / Docker Registry', icon: Database },
	'/settings/destinations': { title: 'Settings / S3 Destinations', icon: Layers },
	'/settings/certificates': { title: 'Settings / Certificates', icon: Shield },
	'/settings/cluster': { title: 'Settings / Cluster Config', icon: Cpu },
	'/settings/notifications': { title: 'Settings / Notifications', icon: Bell },
	'/settings/server': { title: 'Settings / Server Settings', icon: Settings }
};

export function HeaderBar() {
	const location = useLocation();
	const navigate = useNavigate();
	const { toggle } = useSidebarState();
	const [useMock, setUseMock] = useState(USE_MOCK_DATA);

	const routeInfo = routeTitleMap[location.pathname] || {
		title: location.pathname.replace(/^\//, '').replace(/\//g, ' / '),
		icon: FolderOpen
	};

	const Icon = routeInfo.icon;

	return (
		<header className="flex items-center justify-between px-6 py-3 border-b border-[#262626] text-xs bg-[#0A0A0A] shrink-0 gap-4">
			{/* Left: Sidebar Collapse Toggle Button & Page Breadcrumb */}
			<div className="flex items-center gap-3 min-w-0">
				<button
					onClick={() => toggle()}
					className="text-[#a1a1aa] hover:text-[#FAFAFA] transition-colors p-1.5 rounded-lg hover:bg-[#262626] cursor-pointer"
					title="Toggle Sidebar (⌘[|])"
				>
					<PanelLeft className="w-4 h-4" />
				</button>
				<div className="flex items-center gap-2 min-w-0">
					<Icon className="w-3.5 h-3.5 text-[#a1a1aa] shrink-0" />
					<span className="font-semibold text-[#FAFAFA] truncate text-xs">{routeInfo.title}</span>
				</div>
			</div>

			{/* Center: Command Palette Trigger */}
			<button
				onClick={() => window.dispatchEvent(new CustomEvent('open-command-palette'))}
				className="hidden md:flex items-center gap-2.5 px-3.5 py-1.5 rounded-lg border border-[#262626] bg-[#141414] hover:bg-[#262626]/60 text-xs text-[#a1a1aa] transition-colors cursor-pointer w-64 justify-between"
			>
				<div className="flex items-center gap-2">
					<Search className="w-3.5 h-3.5 text-[#737373]" />
					<span>Search or command...</span>
				</div>
				<kbd className="px-1.5 py-0.5 text-[10px] font-mono rounded bg-[#262626] border border-white/10 text-[#FAFAFA]">
					⌘K
				</kbd>
			</button>

			{/* Right: Mock Data Switcher & Pulse Tag */}
			<div className="flex items-center gap-3 shrink-0">
				<div className="hidden lg:flex items-center gap-2 px-3 py-1 rounded-full bg-[#141414] border border-[#262626]">
					<span className="text-[11px] text-[#a1a1aa]">Data:</span>
					<button
						onClick={() => setUseMock(!useMock)}
						className={`text-[11px] font-semibold px-2 py-0.5 rounded transition-colors cursor-pointer ${
							useMock
								? 'bg-[#262626] text-[#FAFAFA] border border-white/10'
								: 'text-[#737373] hover:text-[#FAFAFA]'
						}`}
					>
						{useMock ? 'Mock Demo Data' : 'Live Rust Backend'}
					</button>
				</div>

				<div className="hidden sm:flex items-center gap-2 px-2.5 py-1 rounded-full border border-green-500/20 bg-green-500/5 text-xs text-green-400 font-medium">
					<span className="relative flex h-2 w-2">
						<span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75" />
						<span className="relative inline-flex rounded-full h-2 w-2 bg-green-500" />
					</span>
					<span>System Normal</span>
				</div>
			</div>
		</header>
	);
}
