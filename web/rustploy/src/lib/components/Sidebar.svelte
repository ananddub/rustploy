<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
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
		ChevronsUpDown,
		Loader
	} from '@lucide/svelte';
	import { getAuthSession } from '$lib/auth';
	import { authState } from '$lib/auth.svelte';
	import { sidebarState } from '$lib/sidebar.svelte';
	import { organizationControllerGet } from '$lib/client/sdk.gen';

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
		{ label: 'Users', icon: Users, path: '/settings/users' },
		{ label: 'Audit Logs', icon: FileText, path: '/settings/audit-logs' },
		{ label: 'SSH Keys', icon: Key, path: '/ssh-keys' },
		{ label: 'AI', icon: Sparkles, path: '/settings/ai' },
		{ label: 'Tags', icon: Tag, path: '/settings/tags' },
		{ label: 'Git', icon: GitBranch, path: '/settings/git-providers' },
		{ label: 'Registry', icon: Database, path: '/settings/registry' },
		{ label: 'S3 Destinations', icon: Layers, path: '/settings/destinations' },
		{ label: 'Certificates', icon: Shield, path: '/settings/certificates' },
		{ label: 'Cluster', icon: Cpu, path: '/settings/cluster' }
	];

	const session = $derived(authState.session);
	const userEmail = $derived(session?.user.email || 'dmcbadhya@gmail.com');

	let orgName = $state('…');
	let orgLoading = $state(true);

	$effect(() => {
		const s = session;
		if (!s) return;
		organizationControllerGet({ path: { id: s.user.group_id } }).then((res: { data?: { name?: string } | null }) => {
			orgName = res.data?.name ?? 'My Organization';
			orgLoading = false;
		});
	});

	function isActive(path?: string): boolean {
		if (!path) return false;
		if (path === '/dashboard') return page.url.pathname === '/dashboard';
		if (path === '/settings') return page.url.pathname === '/settings';
		return page.url.pathname.startsWith(path);
	}

	function navItemClass(path?: string): string {
		const base =
			'w-full flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition-all duration-150 text-left outline-none';
		if (isActive(path)) return `${base} border border-white/15 bg-[#262626] text-[#FAFAFA] font-semibold shadow-2xs`;
		return `${base} text-[#a1a1aa] hover:bg-[#262626]/40 hover:text-[#FAFAFA]`;
	}

	// Drag-to-resize
	let dragging = $state(false);

	function onDragStart(e: MouseEvent) {
		e.preventDefault();
		dragging = true;
		const onMove = (e: MouseEvent) => {
			sidebarState.width = Math.min(320, Math.max(200, e.clientX));
		};
		const onUp = () => {
			dragging = false;
			window.removeEventListener('mousemove', onMove);
			window.removeEventListener('mouseup', onUp);
		};
		window.addEventListener('mousemove', onMove);
		window.addEventListener('mouseup', onUp);
	}
</script>

<aside
	style="width: {sidebarState.collapsed ? '52px' : `${sidebarState.width}px`}; min-width: {sidebarState.collapsed ? '52px' : `${sidebarState.width}px`}"
	class="relative shrink-0 flex flex-col bg-[#0A0A0A] border-r border-[#262626] h-full text-sidebar-foreground overflow-hidden transition-all duration-200 ease-in-out"
>
	<!-- Org header (Dokploy 1:1) -->
	<div class="flex items-center justify-between px-3 py-3 border-b border-[#262626] h-13 shrink-0">
		{#if sidebarState.collapsed}
			<div class="w-full flex items-center justify-center">
				<span class="w-6 h-6 rounded bg-[#262626] border border-zinc-700/60 flex items-center justify-center text-xs font-bold text-[#FAFAFA] shrink-0">
					M
				</span>
			</div>
		{:else}
			<button class="flex items-center gap-2.5 text-sm font-semibold text-[#FAFAFA] truncate hover:opacity-80 transition-opacity min-w-0">
				<span class="w-6 h-6 rounded bg-[#262626] border border-zinc-700/60 flex items-center justify-center text-xs font-bold text-[#FAFAFA] shrink-0">
					M
				</span>
				<span class="truncate">
					{#if orgLoading}
						<Loader size={14} class="animate-spin opacity-40" />
					{:else}
						{orgName}
					{/if}
				</span>
				<ChevronsUpDown size={14} class="text-[#737373] shrink-0 ml-0.5" />
			</button>
			<button class="text-[#737373] hover:text-[#FAFAFA] p-1 rounded-md transition-colors" title="Notifications">
				<Bell size={16} />
			</button>
		{/if}
	</div>

	<!-- Nav List (14px menu text & 16px icons) -->
	<nav class="flex-1 overflow-y-auto py-2.5 px-2 space-y-1">
		{#if sidebarState.collapsed}
			<!-- Collapsed Icon Rail (18px icons) -->
			<button class="w-9 h-9 mx-auto flex items-center justify-center rounded-lg text-[#737373] hover:text-[#FAFAFA] hover:bg-[#262626]/60 transition-colors mb-1" title="Notifications">
				<Bell size={18} />
			</button>
			{#each navHome as item}
				<button
					class="w-9 h-9 mx-auto flex items-center justify-center rounded-lg text-sm font-medium transition-all outline-none {isActive(item.path) ? 'border border-white/15 bg-[#262626] text-[#FAFAFA]' : 'text-[#a1a1aa] hover:bg-[#262626]/40 hover:text-[#FAFAFA]'}"
					title={item.label}
					onclick={() => item.path && goto(item.path)}
				>
					<item.icon size={18} />
				</button>
			{/each}

			<div class="py-1 px-1">
				<div class="h-[1px] bg-[#262626] w-full"></div>
			</div>

			{#each navSettings as item}
				<button
					class="w-9 h-9 mx-auto flex items-center justify-center rounded-lg text-sm font-medium transition-all outline-none {isActive(item.path) ? 'border border-white/15 bg-[#262626] text-[#FAFAFA]' : 'text-[#a1a1aa] hover:bg-[#262626]/40 hover:text-[#FAFAFA]'}"
					title={item.label}
					onclick={() => item.path && goto(item.path)}
				>
					<item.icon size={18} />
				</button>
			{/each}
		{:else}
			<!-- Expanded Nav (14px text & 16px icons) -->
			<p class="text-xs font-semibold text-[#737373] px-3 pt-2 pb-1">
				Home
			</p>
			{#each navHome as item}
				<button
					class={navItemClass(item.path)}
					onclick={() => item.path && goto(item.path)}
				>
					<item.icon size={16} class="shrink-0" />
					<span class="truncate">{item.label}</span>
				</button>
			{/each}

			<div class="py-2.5 px-2">
				<div class="h-[1px] bg-[#262626] w-full"></div>
			</div>

			<p class="text-xs font-semibold text-[#737373] px-3 pt-1 pb-1">
				Settings
			</p>
			{#each navSettings as item}
				<button
					class={navItemClass(item.path)}
					onclick={() => item.path && goto(item.path)}
				>
					<item.icon size={16} class="shrink-0" />
					<span class="truncate">{item.label}</span>
				</button>
			{/each}
		{/if}
	</nav>

	<!-- Account Footer -->
	<div class="border-t border-[#262626] py-3 bg-[#0A0A0A] shrink-0 {sidebarState.collapsed ? 'px-2 text-center' : 'px-3'}">
		{#if sidebarState.collapsed}
			<div class="w-7 h-7 mx-auto rounded-full bg-black border border-white/80 text-white text-xs font-bold flex items-center justify-center cursor-pointer" title="Account (dmcbadhya@gmail.com)">
				A
			</div>
		{:else}
			<div class="flex items-center justify-between gap-2.5">
				<div class="flex items-center gap-2.5 min-w-0">
					<div class="w-7 h-7 rounded-full bg-black border border-white/80 text-white text-xs font-bold flex items-center justify-center shrink-0">
						A
					</div>
					<div class="min-w-0">
						<p class="text-sm font-semibold text-[#FAFAFA] leading-tight truncate">Account</p>
						<p class="text-xs text-[#737373] leading-tight truncate mt-0.5">{userEmail}</p>
					</div>
				</div>
				<ChevronsUpDown size={14} class="text-[#737373] shrink-0" />
			</div>
			<p class="text-[10px] text-[#737373] font-mono text-center mt-2">Version v0.29.12</p>
		{/if}
	</div>

	<!-- Drag handle (only when expanded) -->
	{#if !sidebarState.collapsed}
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div
			onmousedown={onDragStart}
			role="separator"
			tabindex="-1"
			aria-label="Resize sidebar"
			class="absolute top-0 right-0 h-full w-1 cursor-col-resize z-10 transition-colors hover:bg-primary/40 {dragging
				? 'bg-primary/60'
				: ''}"
		></div>
	{/if}
</aside>
