import React, { useState } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import {
	House,
	FolderOpen,
	Zap,
	Clock,
	Globe,
	Package,
	Globe2,
	User,
	Server,
	Key,
	Shield,
	Cpu,
	ChevronsUpDown
} from 'lucide-react';
import { useSidebarState } from '$lib/sidebar';
import { getAuthSession } from '$lib/auth';

const navHome = [
	{ label: 'Home', icon: House, path: '/dashboard' },
	{ label: 'Projects', icon: FolderOpen, path: '/projects' },
	{ label: 'Deployments', icon: Zap, path: '/deployments' },
	{ label: 'Schedules', icon: Clock, path: '/schedules' },
	{ label: 'Traefik File System', icon: Globe, path: '/traefik' },
	{ label: 'Docker', icon: Package, path: '/docker' },
	{ label: 'Swarm', icon: Globe2, path: '/swarm' }
];

const navSettings = [
	{ label: 'Profile', icon: User, path: '/settings/profile' },
	{ label: 'Remote Servers', icon: Server, path: '/remote-servers' },
	{ label: 'SSH Keys', icon: Key, path: '/ssh-keys' },
	{ label: 'Certificates', icon: Shield, path: '/settings/certificates' },
	{ label: 'Organization Settings', icon: Cpu, path: '/settings' }
];

export function Sidebar() {
	const location = useLocation();
	const navigate = useNavigate();
	const { collapsed } = useSidebarState();
	const [width] = useState(240);
	const session = getAuthSession();
	const userEmail = session?.user.email || 'admin@rustploy.dev';

	function isActive(path: string): boolean {
		if (path === '/dashboard') return location.pathname === '/dashboard';
		if (path === '/settings') return location.pathname === '/settings';
		return location.pathname.startsWith(path);
	}

	function navItemClass(path: string): string {
		const base =
			'w-full flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition-all duration-150 text-left outline-none cursor-pointer';
		if (isActive(path))
			return `${base} border border-white/15 bg-[#262626] text-[#FAFAFA] font-semibold shadow-2xs`;
		return `${base} text-[#a1a1aa] hover:bg-[#262626]/40 hover:text-[#FAFAFA]`;
	}

	return (
		<aside
			style={{
				width: collapsed ? '52px' : `${width}px`,
				minWidth: collapsed ? '52px' : `${width}px`
			}}
			className="relative shrink-0 flex flex-col bg-[#0A0A0A] border-r border-[#262626] h-full text-[#a1a1aa] overflow-hidden transition-all duration-200 ease-in-out select-none"
		>
			<div className="flex items-center justify-between px-3 py-3 border-b border-[#262626] h-13 shrink-0">
				{collapsed ? (
					<div className="w-full flex items-center justify-center">
						<span className="w-6 h-6 rounded bg-[#262626] border border-zinc-700/60 flex items-center justify-center text-xs font-bold text-[#FAFAFA] shrink-0">
							R
						</span>
					</div>
				) : (
					<button className="flex items-center gap-2.5 text-sm font-semibold text-[#FAFAFA] truncate hover:opacity-80 transition-opacity min-w-0">
						<span className="w-6 h-6 rounded bg-[#262626] border border-zinc-700/60 flex items-center justify-center text-xs font-bold text-[#FAFAFA] shrink-0">
							R
						</span>
						<span className="truncate">My Organization</span>
						<ChevronsUpDown size={14} className="text-[#737373] shrink-0 ml-0.5" />
					</button>
				)}
			</div>

			<nav className="flex-1 overflow-y-auto py-2.5 px-2 space-y-1">
				{collapsed ? (
					<>
						{navHome.map((item) => {
							const Icon = item.icon;
							return (
								<button
									key={item.path}
									title={item.label}
									onClick={() => navigate(item.path)}
									className={`w-9 h-9 mx-auto flex items-center justify-center rounded-lg text-sm font-medium transition-all outline-none cursor-pointer ${
										isActive(item.path)
											? 'border border-white/15 bg-[#262626] text-[#FAFAFA]'
											: 'text-[#a1a1aa] hover:bg-[#262626]/40 hover:text-[#FAFAFA]'
									}`}
								>
									<Icon size={18} />
								</button>
							);
						})}
						<div className="py-1 px-1">
							<div className="h-[1px] bg-[#262626] w-full" />
						</div>
						{navSettings.map((item) => {
							const Icon = item.icon;
							return (
								<button
									key={item.path}
									title={item.label}
									onClick={() => navigate(item.path)}
									className={`w-9 h-9 mx-auto flex items-center justify-center rounded-lg text-sm font-medium transition-all outline-none cursor-pointer ${
										isActive(item.path)
											? 'border border-white/15 bg-[#262626] text-[#FAFAFA]'
											: 'text-[#a1a1aa] hover:bg-[#262626]/40 hover:text-[#FAFAFA]'
									}`}
								>
									<Icon size={18} />
								</button>
							);
						})}
					</>
				) : (
					<>
						<p className="text-xs font-semibold text-[#737373] px-3 pt-2 pb-1">Home</p>
						{navHome.map((item) => {
							const Icon = item.icon;
							return (
								<button
									key={item.path}
									onClick={() => navigate(item.path)}
									className={navItemClass(item.path)}
								>
									<Icon size={16} className="shrink-0" />
									<span className="truncate">{item.label}</span>
								</button>
							);
						})}
						<div className="py-2.5 px-2">
							<div className="h-[1px] bg-[#262626] w-full" />
						</div>
						<p className="text-xs font-semibold text-[#737373] px-3 pt-1 pb-1">Settings</p>
						{navSettings.map((item) => {
							const Icon = item.icon;
							return (
								<button
									key={item.path}
									onClick={() => navigate(item.path)}
									className={navItemClass(item.path)}
								>
									<Icon size={16} className="shrink-0" />
									<span className="truncate">{item.label}</span>
								</button>
							);
						})}
					</>
				)}
			</nav>

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
									<p className="text-sm font-semibold text-[#FAFAFA] leading-tight truncate">Account</p>
									<p className="text-xs text-[#737373] leading-tight truncate mt-0.5">{userEmail}</p>
								</div>
							</div>
						</div>
						<p className="text-[10px] text-[#737373] font-mono text-center mt-2">Version v0.29.12</p>
					</div>
				)}
			</div>
		</aside>
	);
}
