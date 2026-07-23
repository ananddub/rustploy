import React, { useState } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import {
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
	ChevronDown,
	Check,
	ChevronsUpDown,
	Settings
} from 'lucide-react';
import { useSidebarState } from '$lib/sidebar';
import { getAuthSession } from '$lib/auth';

interface SidebarItem {
	label: string;
	icon: React.ComponentType<{ className?: string }>;
	path: string;
}

const navHome: SidebarItem[] = [
	{ label: 'Home', icon: Home, path: '/dashboard' },
	{ label: 'Projects', icon: FolderOpen, path: '/projects' },
	{ label: 'Deployments', icon: Zap, path: '/deployments' },
	{ label: 'Schedules', icon: Clock, path: '/schedules' },
	{ label: 'Traefik File System', icon: Globe, path: '/traefik' },
	{ label: 'Docker', icon: Package, path: '/docker' },
	{ label: 'Swarm', icon: Globe2, path: '/swarm' }
];

const navSettings: SidebarItem[] = [
	{ label: 'Profile', icon: User, path: '/settings/profile' },
	{ label: 'Remote Servers', icon: Server, path: '/remote-servers' },
	{ label: 'Users & Roles', icon: Users, path: '/settings/users' },
	{ label: 'Audit Logs', icon: FileText, path: '/settings/audit-logs' },
	{ label: 'SSH Keys', icon: Key, path: '/ssh-keys' },
	{ label: 'AI Assistant', icon: Sparkles, path: '/settings/ai' },
	{ label: 'Tags', icon: Tag, path: '/settings/tags' },
	{ label: 'Git Integrations', icon: GitBranch, path: '/settings/git-providers' },
	{ label: 'Docker Registry', icon: Database, path: '/settings/registry' },
	{ label: 'S3 Destinations', icon: Layers, path: '/settings/destinations' },
	{ label: 'Certificates', icon: Shield, path: '/settings/certificates' },
	{ label: 'Cluster Config', icon: Cpu, path: '/settings/cluster' },
	{ label: 'Notifications', icon: Bell, path: '/settings/notifications' },
	{ label: 'Server Settings', icon: Settings, path: '/settings/server' },
	{ label: 'Organization Settings', icon: Building2, path: '/settings' }
];

const mockOrganizations = [
	{ id: 'org-1', name: 'My Organization', slug: 'my-org', role: 'Owner' },
	{ id: 'org-2', name: 'Acme Corp', slug: 'acme-corp', role: 'Member' },
	{ id: 'org-3', name: 'Staging Env', slug: 'staging-env', role: 'Admin' }
];

export function Sidebar() {
	const location = useLocation();
	const navigate = useNavigate();
	const { collapsed } = useSidebarState();
	const session = getAuthSession();

	const [activeOrg, setActiveOrg] = useState(mockOrganizations[0]);
	const [orgMenuOpen, setOrgMenuOpen] = useState(false);

	const userEmail = session?.user.email || 'admin@rustploy.dev';

	function isActive(path: string): boolean {
		if (path === '/dashboard') return location.pathname === '/dashboard';
		if (path === '/settings') return location.pathname === '/settings';
		return location.pathname.startsWith(path);
	}

	return (
		<aside
			className={`h-screen shrink-0 border-r border-[#262626] bg-[#0A0A0A] text-[#FAFAFA] flex flex-col justify-between select-none transition-all duration-200 ${
				collapsed ? 'w-[52px]' : 'w-64'
			}`}
		>
			<div className="flex flex-col min-h-0 flex-1">
				{/* Top Header: Organization Switcher */}
				<div className={`p-2.5 border-b border-[#262626] relative ${collapsed ? 'px-2' : 'px-3'}`}>
					{collapsed ? (
						<div
							className="w-7 h-7 mx-auto rounded-lg bg-[#1c1c1c] border border-[#262626] flex items-center justify-center cursor-pointer text-xs font-bold text-[#FAFAFA]"
							title={activeOrg.name}
							onClick={() => setOrgMenuOpen(!orgMenuOpen)}
						>
							{activeOrg.name[0]}
						</div>
					) : (
						<button
							onClick={() => setOrgMenuOpen(!orgMenuOpen)}
							className="w-full flex items-center justify-between p-2 rounded-lg bg-[#141414] hover:bg-[#262626]/80 border border-[#262626] transition-colors cursor-pointer text-left"
						>
							<div className="flex items-center gap-2.5 min-w-0">
								<div className="w-6 h-6 rounded-md bg-[#262626] border border-white/10 flex items-center justify-center shrink-0 font-bold text-xs text-[#FAFAFA]">
									{activeOrg.name[0]}
								</div>
								<div className="min-w-0">
									<p className="text-sm font-bold text-[#FAFAFA] leading-tight truncate">
										{activeOrg.name}
									</p>
									<p className="text-[10px] text-[#a1a1aa] leading-tight font-medium mt-0.5 truncate">
										{activeOrg.role}
									</p>
								</div>
							</div>
							<ChevronsUpDown className="w-4 h-4 text-[#a1a1aa] shrink-0" />
						</button>
					)}

					{/* Org Switcher Dropdown Menu */}
					{orgMenuOpen && (
						<div className="absolute top-full left-2 right-2 mt-1 z-50 bg-[#171717] border border-[#262626] rounded-xl shadow-xl p-1.5 space-y-1">
							<p className="text-[10px] font-semibold uppercase tracking-wider text-[#a1a1aa] px-2 py-1">
								Organizations
							</p>
							{mockOrganizations.map((org) => (
								<button
									key={org.id}
									onClick={() => {
										setActiveOrg(org);
										setOrgMenuOpen(false);
									}}
									className={`w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg text-xs transition-colors cursor-pointer ${
										activeOrg.id === org.id
											? 'bg-[#262626] text-[#FAFAFA] font-semibold'
											: 'text-[#a1a1aa] hover:bg-[#262626]/50 hover:text-[#FAFAFA]'
									}`}
								>
									<div className="flex items-center gap-2 min-w-0">
										<div className="w-5 h-5 rounded bg-[#262626] flex items-center justify-center text-[10px] font-bold">
											{org.name[0]}
										</div>
										<span className="truncate">{org.name}</span>
									</div>
									{activeOrg.id === org.id && <Check className="w-3.5 h-3.5 text-[#FAFAFA]" />}
								</button>
							))}
						</div>
					)}
				</div>

				{/* Scrollable Navigation Menu */}
				<nav className="flex-1 overflow-y-auto custom-scrollbar px-2 py-3 space-y-4 font-sans text-sm font-bold">
					{/* Main Home Section */}
					<div>
						{!collapsed && (
							<p className="px-3 pb-1.5 text-[11px] font-bold text-[#a1a1aa] uppercase tracking-wider">
								Home
							</p>
						)}
						<div className="space-y-0.5">
							{navHome.map((item) => {
								const active = isActive(item.path);
								const Icon = item.icon;
								return (
									<button
										key={item.path}
										onClick={() => navigate(item.path)}
										title={collapsed ? item.label : undefined}
										className={`w-full flex items-center gap-3 rounded-lg transition-colors cursor-pointer ${
											collapsed ? 'justify-center p-2' : 'px-3 py-2 text-left'
										} ${
											active
												? 'bg-[#262626] text-[#FAFAFA] border border-white/10 font-bold'
												: 'text-[#a1a1aa] hover:bg-[#262626]/50 hover:text-[#FAFAFA]'
										}`}
									>
										<Icon className={`shrink-0 ${collapsed ? 'w-5 h-5' : 'w-4 h-4'}`} />
										{!collapsed && (
											<span className="text-sm font-bold truncate tracking-tight">{item.label}</span>
										)}
									</button>
								);
							})}
						</div>
					</div>

					{/* Settings & Admin Section */}
					<div>
						{!collapsed && (
							<p className="px-3 pb-1.5 text-[11px] font-bold text-[#a1a1aa] uppercase tracking-wider">
								Settings
							</p>
						)}
						<div className="space-y-0.5">
							{navSettings.map((item) => {
								const active = isActive(item.path);
								const Icon = item.icon;
								return (
									<button
										key={item.path}
										onClick={() => navigate(item.path)}
										title={collapsed ? item.label : undefined}
										className={`w-full flex items-center gap-3 rounded-lg transition-colors cursor-pointer ${
											collapsed ? 'justify-center p-2' : 'px-3 py-2 text-left'
										} ${
											active
												? 'bg-[#262626] text-[#FAFAFA] border border-white/10 font-bold'
												: 'text-[#a1a1aa] hover:bg-[#262626]/50 hover:text-[#FAFAFA]'
										}`}
									>
										<Icon className={`shrink-0 ${collapsed ? 'w-5 h-5' : 'w-4 h-4'}`} />
										{!collapsed && (
											<span className="text-sm font-bold truncate tracking-tight">{item.label}</span>
										)}
									</button>
								);
							})}
						</div>
					</div>
				</nav>
			</div>

			{/* User Account Footer */}
			<div className={`border-t border-[#262626] py-3 bg-[#0A0A0A] shrink-0 ${collapsed ? 'px-2 text-center' : 'px-3'}`}>
				{collapsed ? (
					<div
						className="w-7 h-7 mx-auto rounded-full bg-black border border-white/80 text-white text-xs font-bold flex items-center justify-center cursor-pointer"
						title={`Account (${userEmail})`}
					>
						A
					</div>
				) : (
					<div>
						<div className="flex items-center justify-between gap-2.5">
							<div className="flex items-center gap-2.5 min-w-0">
								<div className="w-7 h-7 rounded-full bg-black border border-white/80 text-white text-xs font-bold flex items-center justify-center shrink-0">
									A
								</div>
								<div className="min-w-0">
									<p className="text-sm font-bold text-[#FAFAFA] leading-tight truncate">Account</p>
									<p className="text-xs text-[#a1a1aa] leading-tight truncate mt-0.5">{userEmail}</p>
								</div>
							</div>
						</div>
						<p className="text-[10px] text-[#a1a1aa] font-mono text-center mt-2">Version v0.29.12</p>
					</div>
				)}
			</div>
		</aside>
	);
}
