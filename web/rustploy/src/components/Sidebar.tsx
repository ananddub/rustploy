import React, { useState, useRef } from 'react';
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
	Bot,
	Tag,
	GitBranch,
	Database,
	Layers,
	Shield,
	Cpu,
	Bell,
	CreditCard,
	KeyRound,
	Lock,
	BookOpen,
	HelpCircle,
	Check,
	ChevronsUpDown,
	Moon,
	LogOut,
	Building2
} from 'lucide-react';
import { useSidebarState } from '$lib/sidebar';
import { getAuthSession, clearAuthSession } from '$lib/auth';
import { toastInfo } from '$lib/toast';

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
	{ label: 'Traefik', icon: Globe, path: '/traefik' },
	{ label: 'Docker', icon: Package, path: '/docker' },
	{ label: 'Swarm', icon: Globe2, path: '/swarm' }
];

const navSettings: SidebarItem[] = [
	{ label: 'Profile', icon: User, path: '/settings/profile' },
	{ label: 'Remote Servers', icon: Server, path: '/remote-servers' },
	{ label: 'Users', icon: Users, path: '/settings/users' },
	{ label: 'Audit Logs', icon: FileText, path: '/settings/audit-logs' },
	{ label: 'SSH Keys', icon: Key, path: '/ssh-keys' },
	{ label: 'AI', icon: Bot, path: '/settings/ai' },
	{ label: 'Tags', icon: Tag, path: '/settings/tags' },
	{ label: 'Git', icon: GitBranch, path: '/settings/git-providers' },
	{ label: 'Registry', icon: Database, path: '/settings/registry' },
	{ label: 'S3 Destinations', icon: Layers, path: '/settings/destinations' },
	{ label: 'Certificates', icon: Shield, path: '/settings/certificates' },
	{ label: 'Cluster', icon: Cpu, path: '/settings/cluster' },
	{ label: 'Notifications', icon: Bell, path: '/settings/notifications' },
	{ label: 'Billing', icon: CreditCard, path: '/settings' },
	{ label: 'License', icon: KeyRound, path: '/settings/server' },
	{ label: 'SSO', icon: Lock, path: '/settings/users' }
];

const navExtra: SidebarItem[] = [
	{ label: 'Documentation', icon: BookOpen, path: 'https://github.com/ananddub/rustploy' },
	{ label: 'Support', icon: HelpCircle, path: 'https://github.com/ananddub/rustploy/issues' }
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

	const navRef = useRef<HTMLElement>(null);
	const [activeOrg, setActiveOrg] = useState(mockOrganizations[0]);
	const [orgMenuOpen, setOrgMenuOpen] = useState(false);
	const [userMenuOpen, setUserMenuOpen] = useState(false);

	const userEmail = session?.user.email || 'dmcbaditya@gmail.com';

	function isActive(path: string): boolean {
		if (path === '/dashboard') return location.pathname === '/dashboard';
		if (path === '/settings') return location.pathname === '/settings';
		return location.pathname.startsWith(path);
	}

	function handleItemClick(item: SidebarItem) {
		if (item.path.startsWith('http')) {
			window.open(item.path, '_blank');
			return;
		}
		navigate(item.path);
	}

	return (
		<aside
			className={`h-screen shrink-0 border-r border-[#272727] bg-[#121212] text-[#FAFAFA] flex flex-col justify-between select-none transition-all duration-200 ${
				collapsed ? 'w-[52px]' : 'w-64'
			}`}
		>
			<div className="flex flex-col min-h-0 flex-1">
				{/* Top Header: Organization Switcher & Notification Bell */}
				<div className={`p-2.5 border-b border-[#272727] relative ${collapsed ? 'px-2' : 'px-3'}`}>
					{collapsed ? (
						<div
							className="w-7 h-7 mx-auto rounded-lg bg-[#1c1c1c] border border-[#272727] flex items-center justify-center cursor-pointer text-xs font-bold text-[#FAFAFA]"
							title={activeOrg.name}
							onClick={() => setOrgMenuOpen(!orgMenuOpen)}
						>
							{activeOrg.name[0]}
						</div>
					) : (
						<div className="flex items-center justify-between gap-1.5">
							<button
								onClick={() => setOrgMenuOpen(!orgMenuOpen)}
								className="flex-1 flex items-center justify-between p-2 rounded-lg bg-[#1c1c1c] hover:bg-[#272727] border border-[#272727] transition-colors cursor-pointer text-left min-w-0"
							>
								<div className="flex items-center gap-2.5 min-w-0">
									<div className="w-6 h-6 rounded-md bg-[#272727] border border-white/10 flex items-center justify-center shrink-0 font-bold text-xs text-[#FAFAFA]">
										<Building2 className="w-3.5 h-3.5 text-[#FAFAFA]" />
									</div>
									<p className="text-xs font-bold text-[#FAFAFA] leading-tight truncate">
										{activeOrg.name}
									</p>
								</div>
								<ChevronsUpDown className="w-3.5 h-3.5 text-[#a1a1aa] shrink-0 ml-1" />
							</button>

							<button
								onClick={() => toastInfo('No unread notifications')}
								className="p-2 rounded-lg bg-[#1c1c1c] hover:bg-[#272727] border border-[#272727] text-[#a1a1aa] hover:text-[#FAFAFA] transition-colors cursor-pointer shrink-0"
								title="Notifications"
							>
								<Bell className="w-4 h-4" />
							</button>
						</div>
					)}

					{/* Org Switcher Dropdown Menu */}
					{orgMenuOpen && (
						<div className="absolute top-full left-2 right-2 mt-1 z-50 bg-[#18181b] border border-[#272727] rounded-xl shadow-2xl p-1.5 space-y-1">
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
											? 'bg-[#272727] text-[#FAFAFA] font-semibold'
											: 'text-[#a1a1aa] hover:bg-[#272727]/60 hover:text-[#FAFAFA]'
									}`}
								>
									<div className="flex items-center gap-2 min-w-0">
										<div className="w-5 h-5 rounded bg-[#272727] flex items-center justify-center text-[10px] font-bold">
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

				{/* Persistent Scrollable Navigation Menu without visible scrollbar */}
				<nav ref={navRef} className="flex-1 overflow-y-auto no-scrollbar px-2 py-3 space-y-5 font-sans text-xs">
					{/* Main Home Section */}
					<div>
						{!collapsed && (
							<p className="px-3 pb-1.5 text-[10px] font-semibold text-[#737373] uppercase tracking-wider">
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
										onClick={() => handleItemClick(item)}
										title={collapsed ? item.label : undefined}
										className={`w-full flex items-center gap-3 rounded-lg transition-all cursor-pointer ${
											collapsed ? 'justify-center p-2' : 'px-3 py-1.5 text-left'
										} ${
											active
												? 'bg-[#272727] text-[#FAFAFA] font-medium border border-white/10'
												: 'text-[#a1a1aa] hover:bg-[#272727]/60 hover:text-[#FAFAFA]'
										}`}
									>
										<Icon className={`shrink-0 ${collapsed ? 'w-5 h-5' : 'w-4 h-4'}`} />
										{!collapsed && (
											<span className="text-xs font-medium truncate">{item.label}</span>
										)}
									</button>
								);
							})}
						</div>
					</div>

					{/* Settings Section */}
					<div>
						{!collapsed && (
							<p className="px-3 pb-1.5 text-[10px] font-semibold text-[#737373] uppercase tracking-wider">
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
										onClick={() => handleItemClick(item)}
										title={collapsed ? item.label : undefined}
										className={`w-full flex items-center gap-3 rounded-lg transition-all cursor-pointer ${
											collapsed ? 'justify-center p-2' : 'px-3 py-1.5 text-left'
										} ${
											active
												? 'bg-[#272727] text-[#FAFAFA] font-medium border border-white/10'
												: 'text-[#a1a1aa] hover:bg-[#272727]/60 hover:text-[#FAFAFA]'
										}`}
									>
										<Icon className={`shrink-0 ${collapsed ? 'w-5 h-5' : 'w-4 h-4'}`} />
										{!collapsed && (
											<span className="text-xs font-medium truncate">{item.label}</span>
										)}
									</button>
								);
							})}
						</div>
					</div>

					{/* Extra Section */}
					<div>
						{!collapsed && (
							<p className="px-3 pb-1.5 text-[10px] font-semibold text-[#737373] uppercase tracking-wider">
								Extra
							</p>
						)}
						<div className="space-y-0.5">
							{navExtra.map((item) => {
								const Icon = item.icon;
								return (
									<button
										key={item.label}
										onClick={() => handleItemClick(item)}
										title={collapsed ? item.label : undefined}
										className={`w-full flex items-center gap-3 rounded-lg transition-all cursor-pointer text-[#a1a1aa] hover:bg-[#272727]/60 hover:text-[#FAFAFA] ${
											collapsed ? 'justify-center p-2' : 'px-3 py-1.5 text-left'
										}`}
									>
										<Icon className={`shrink-0 ${collapsed ? 'w-5 h-5' : 'w-4 h-4'}`} />
										{!collapsed && (
											<span className="text-xs font-medium truncate">{item.label}</span>
										)}
									</button>
								);
							})}
						</div>
					</div>
				</nav>
			</div>

			{/* User Account Footer with Shadcn Popover Menu */}
			<div className={`border-t border-[#272727] py-3 bg-[#121212] shrink-0 relative ${collapsed ? 'px-2 text-center' : 'px-3'}`}>
				{/* Shadcn-Style User Popover Menu */}
				{userMenuOpen && (
					<div className="absolute bottom-full left-2 right-2 mb-2 z-50 bg-[#18181b] border border-[#272727] rounded-xl shadow-2xl p-2 font-sans space-y-1 animate-fade-up">
						<div className="flex items-center justify-between px-2.5 py-2">
							<div className="min-w-0">
								<p className="text-xs font-bold text-[#FAFAFA]">My Account</p>
								<p className="text-[10px] text-[#a1a1aa] font-mono truncate max-w-[150px]">{userEmail}</p>
							</div>
							<button
								onClick={() => toastInfo('Monotone dark theme active')}
								className="p-1.5 rounded-lg border border-[#272727] bg-[#272727]/60 text-[#a1a1aa] hover:text-[#FAFAFA] transition-colors cursor-pointer"
								title="Toggle Theme"
							>
								<Moon className="w-3.5 h-3.5" />
							</button>
						</div>

						<div className="border-t border-[#272727] my-1" />

						<button
							onClick={() => {
								navigate('/settings/profile');
								setUserMenuOpen(false);
							}}
							className="w-full flex items-center gap-2.5 px-2.5 py-1.5 rounded-lg text-xs text-[#FAFAFA] hover:bg-[#272727] transition-colors cursor-pointer font-medium"
						>
							<User className="w-3.5 h-3.5 text-[#a1a1aa]" />
							Profile
						</button>

						<button
							onClick={() => {
								navigate('/projects');
								setUserMenuOpen(false);
							}}
							className="w-full flex items-center gap-2.5 px-2.5 py-1.5 rounded-lg text-xs text-[#FAFAFA] hover:bg-[#272727] transition-colors cursor-pointer font-medium"
						>
							<FolderOpen className="w-3.5 h-3.5 text-[#a1a1aa]" />
							Projects
						</button>

						<button
							onClick={() => {
								navigate('/remote-servers');
								setUserMenuOpen(false);
							}}
							className="w-full flex items-center gap-2.5 px-2.5 py-1.5 rounded-lg text-xs text-[#FAFAFA] hover:bg-[#272727] transition-colors cursor-pointer font-medium"
						>
							<Server className="w-3.5 h-3.5 text-[#a1a1aa]" />
							Servers
						</button>

						<button
							onClick={() => {
								navigate('/settings');
								setUserMenuOpen(false);
							}}
							className="w-full flex items-center gap-2.5 px-2.5 py-1.5 rounded-lg text-xs text-[#FAFAFA] hover:bg-[#272727] transition-colors cursor-pointer font-medium"
						>
							<CreditCard className="w-3.5 h-3.5 text-[#a1a1aa]" />
							Billing
						</button>

						<div className="border-t border-[#272727] my-1" />

						<button
							onClick={() => {
								clearAuthSession();
								navigate('/auth');
							}}
							className="w-full flex items-center gap-2.5 px-2.5 py-1.5 rounded-lg text-xs text-red-400 hover:bg-red-500/10 transition-colors cursor-pointer font-semibold"
						>
							<LogOut className="w-3.5 h-3.5 text-red-400" />
							Log out
						</button>
					</div>
				)}

				{/* Account Button Trigger */}
				{collapsed ? (
					<div
						onClick={() => setUserMenuOpen(!userMenuOpen)}
						className="w-7 h-7 mx-auto rounded-full bg-[#272727] border border-white/20 text-white text-xs font-bold flex items-center justify-center cursor-pointer hover:bg-[#3f3f46] transition-colors"
						title={`Account (${userEmail})`}
					>
						A
					</div>
				) : (
					<div>
						<button
							onClick={() => setUserMenuOpen(!userMenuOpen)}
							className="w-full flex items-center justify-between p-2.5 rounded-xl bg-[#1c1c1c] hover:bg-[#272727] border border-[#272727] transition-all cursor-pointer text-left"
						>
							<div className="flex items-center gap-2.5 min-w-0">
								<div className="w-7 h-7 rounded-full bg-[#272727] border border-white/20 text-[#FAFAFA] font-bold text-xs flex items-center justify-center shrink-0">
									A
								</div>
								<div className="min-w-0">
									<p className="text-xs font-bold text-[#FAFAFA] leading-tight truncate">Account</p>
									<p className="text-[11px] text-[#a1a1aa] leading-tight truncate mt-0.5 font-mono">{userEmail}</p>
								</div>
							</div>
							<ChevronsUpDown className="w-3.5 h-3.5 text-[#a1a1aa] shrink-0" />
						</button>
						<p className="text-[10px] text-[#737373] font-mono text-center mt-2">Version v0.29.12</p>
					</div>
				)}
			</div>
		</aside>
	);
}
