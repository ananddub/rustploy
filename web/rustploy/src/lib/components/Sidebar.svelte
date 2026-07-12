<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import {
		House,
		FolderOpen,
		Zap,
		ChartLine,
		Calendar,
		Globe,
		Package,
		Globe2,
		Link,
		Cpu,
		Users,
		FileText,
		Key,
		GitBranch,
		Database,
		Layers,
		Shield,
		Bell,
		BellRing,
		Settings,
		BookOpen,
		LogOut,
		ChevronDown,
		Loader
	} from '@lucide/svelte';
	import { Avatar, AvatarFallback } from '$lib/components/ui/avatar';
	import { getAuthSession, signOut } from '$lib/auth';
	import { authState } from '$lib/auth.svelte';
	import { organizationControllerGet } from '$lib/client/sdk.gen';

	const navHome = [
		{ label: 'Home', icon: House, path: '/dashboard' },
		{ label: 'Projects', icon: FolderOpen, path: '/projects' },
		{ label: 'Deployments', icon: Zap },
		{ label: 'Monitoring', icon: ChartLine },
		{ label: 'Schedules', icon: Calendar },
		{ label: 'Traefik File System', icon: Globe },
		{ label: 'Docker', icon: Package },
		{ label: 'Swarm', icon: Globe2 },
		{ label: 'Requests', icon: Link }
	];

	const navSettings = [
		{ label: 'Web Server', icon: Cpu },
		{ label: 'Profile', icon: Users },
		{ label: 'Remote Servers', icon: Cpu, path: '/remote-servers' },
		{ label: 'Deployments', icon: Zap },
		{ label: 'Users', icon: Users },
		{ label: 'Audit Logs', icon: FileText },
		{ label: 'SSH Keys', icon: Key, path: '/ssh-keys' },
		{ label: 'AI', icon: Cpu },
		{ label: 'Tags', icon: FileText },
		{ label: 'Git', icon: GitBranch },
		{ label: 'Registry', icon: Database },
		{ label: 'S3 Destinations', icon: Layers },
		{ label: 'Certificates', icon: Shield },
		{ label: 'Cluster', icon: Globe2 },
		{ label: 'Notifications', icon: BellRing },
		{ label: 'License', icon: FileText },
		{ label: 'SSO', icon: Link },
		{ label: 'Whitelabeling', icon: Settings }
	];

	const session = $derived(authState.session);
	const userName = $derived(
		session?.user.first_name || session?.user.email?.split('@')[0] || 'user'
	);
	const userEmail = $derived(session?.user.email ?? '');
	const initials = $derived(userName.slice(0, 2).toUpperCase());

	let orgName = $state('…');
	let orgLoading = $state(true);

	$effect(() => {
		const s = session;
		if (!s) return;
		organizationControllerGet({ path: { id: s.user.group_id } }).then((res: { data?: { name?: string } | null }) => {
			orgName = res.data?.name ?? '…';
			orgLoading = false;
		});
	});

	const orgInitial = $derived(orgName.slice(0, 1).toUpperCase());

	function isActive(path?: string): boolean {
		if (!path) return false;
		if (path === '/dashboard') return page.url.pathname === '/dashboard';
		return page.url.pathname.startsWith(path);
	}

	function navItemClass(path?: string): string {
		const base =
			'w-full flex items-center gap-2 px-2 py-1.5 rounded-md text-sm transition-all duration-150 text-left outline-none';
		if (isActive(path)) return `${base} bg-accent text-accent-foreground font-medium`;
		return `${base} text-muted-foreground hover:bg-accent hover:text-accent-foreground`;
	}

	async function handleLogout() {
		await signOut();
		goto('/auth');
	}

	// Drag-to-resize
	let width = $state(192);
	let dragging = $state(false);

	function onDragStart(e: MouseEvent) {
		e.preventDefault();
		dragging = true;
		const onMove = (e: MouseEvent) => {
			width = Math.min(320, Math.max(160, e.clientX));
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
	style="width: {width}px; min-width: {width}px"
	class="relative shrink-0 flex flex-col bg-sidebar border-r border-sidebar-border h-screen sticky top-0"
>
	<!-- Org header -->
	<div class="flex items-center justify-between px-3 py-3 border-b border-sidebar-border">
		<div class="flex items-center gap-2 text-sm font-medium truncate">
			<div
				class="w-5 h-5 rounded bg-sidebar-primary/20 flex items-center justify-center text-sidebar-primary text-xs font-bold shrink-0"
			>
				{orgInitial}
			</div>
			<span class="truncate text-sidebar-foreground">
				{#if orgLoading}
					<Loader size={12} class="animate-spin opacity-40" />
				{:else}
					{orgName}
				{/if}
			</span>
		</div>
		<div class="flex gap-1 text-sidebar-foreground/40">
			<ChevronDown size={14} />
			<Bell size={14} />
		</div>
	</div>

	<!-- Nav -->
	<nav class="flex-1 overflow-y-auto py-2 px-2 space-y-0.5">
		<p class="text-[10px] uppercase tracking-widest text-sidebar-foreground/30 px-2 pt-2 pb-1">
			Home
		</p>
		{#each navHome as item, i}
			<button
				class={navItemClass(item.path)}
				style="animation-delay: {i * 20}ms"
				onclick={() => item.path && goto(item.path)}
			>
				<item.icon size={14} class="shrink-0" />
				<span class="truncate">{item.label}</span>
			</button>
		{/each}

		<p class="text-[10px] uppercase tracking-widest text-sidebar-foreground/30 px-2 pt-4 pb-1">
			Settings
		</p>
		{#each navSettings as item, i}
			<button
				class={navItemClass(item.path)}
				style="animation-delay: {(navHome.length + i) * 20}ms"
				onclick={() => item.path && goto(item.path)}
			>
				<item.icon size={14} class="shrink-0" />
				<span class="truncate">{item.label}</span>
			</button>
		{/each}

		<p class="text-[10px] uppercase tracking-widest text-sidebar-foreground/30 px-2 pt-4 pb-1">
			Extra
		</p>
		<button class={navItemClass()}>
			<BookOpen size={14} class="shrink-0" />
			<span>Documentation</span>
		</button>
	</nav>

	<!-- User footer -->
	<div class="border-t border-sidebar-border px-3 py-2.5">
		<div class="flex items-center justify-between">
			<div class="flex items-center gap-2 min-w-0">
				<Avatar class="w-7 h-7 text-xs shrink-0">
					<AvatarFallback class="bg-sidebar-primary text-sidebar-primary-foreground text-xs font-bold">
						{initials}
					</AvatarFallback>
				</Avatar>
				<div class="min-w-0">
					<p class="text-xs font-medium truncate text-sidebar-foreground">Account</p>
					<p class="text-[10px] text-sidebar-foreground/40 truncate">{userEmail}</p>
				</div>
			</div>
			<button
				onclick={handleLogout}
				class="text-sidebar-foreground/40 hover:text-destructive transition-colors ml-1 p-1 rounded hover:bg-destructive/10"
				title="Logout"
			>
				<LogOut size={14} />
			</button>
		</div>
		<p class="text-[10px] text-sidebar-foreground/20 mt-1.5">v0.1.0</p>
	</div>

	<!-- Drag handle -->
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div
		onmousedown={onDragStart}
		role="separator"
		tabindex="-1"
		aria-label="Resize sidebar"
		class="absolute top-0 right-0 h-full w-1 cursor-col-resize z-10 transition-colors hover:bg-sidebar-primary/40 {dragging
			? 'bg-sidebar-primary/60'
			: ''}"
	></div>
</aside>
