<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import {
		House,
		FolderOpen,
		Zap,
		ChartLine,
		Package,
		Cpu,
		Users,
		Key,
		GitBranch,
		Database,
		Shield,
		Bell,
		Settings,
		BookOpen,
		LogOut,
		ChevronDown,
		Loader,
		Tag,
		User,
		Server,
		Command,
		Calendar,
		Globe,
		Globe2,
		Link,
		Layers,
		FileText
	} from '@lucide/svelte';
	import { Avatar, AvatarFallback } from '$lib/components/ui/avatar';
	import { Button } from '$lib/components/ui/button';
	import { Separator } from '$lib/components/ui/separator';
	import { getAuthSession, signOut } from '$lib/auth';
	import { authState } from '$lib/auth.svelte';
	import { organizationControllerGet } from '$lib/client/sdk.gen';

	const navHome = [
		{ label: 'Home', icon: House, path: '/dashboard' },
		{ label: 'Projects', icon: FolderOpen, path: '/projects' },
		{ label: 'Deployments', icon: Zap, path: '/deployments' },
		{ label: 'Monitoring', icon: ChartLine, path: '/monitoring' },
		{ label: 'Schedules', icon: Calendar, path: '/schedules' },
		{ label: 'Traefik', icon: Globe, path: '/traefik' },
		{ label: 'Docker', icon: Package, path: '/docker' },
		{ label: 'Swarm', icon: Globe2, path: '/swarm' },
		{ label: 'Requests', icon: Link, path: '/requests' }
	];

	const navSettings = [
		{ label: 'Web Server', icon: Cpu, path: '/settings/server' },
		{ label: 'Profile', icon: User, path: '/settings/profile' },
		{ label: 'Servers', icon: Server, path: '/settings/servers' },
		{ label: 'Remote Servers', icon: Cpu, path: '/remote-servers' },
		{ label: 'Users', icon: Users, path: '/settings/users' },
		{ label: 'Audit Logs', icon: FileText, path: '/settings/audit-logs' },
		{ label: 'SSH Keys', icon: Key, path: '/ssh-keys' },
		{ label: 'Organization', icon: Settings, path: '/settings' },
		{ label: 'Tags', icon: Tag, path: '/settings/tags' },
		{ label: 'Git Providers', icon: GitBranch, path: '/settings/git-providers' },
		{ label: 'Registry', icon: Database, path: '/settings/registry' },
		{ label: 'S3 Destinations', icon: Layers, path: '/settings/destinations' },
		{ label: 'Certificates', icon: Shield, path: '/settings/certificates' },
		{ label: 'Notifications', icon: Bell, path: '/settings/notifications' }
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
		if (path === '/settings') return page.url.pathname === '/settings';
		return page.url.pathname.startsWith(path);
	}

	function navItemClass(path?: string): string {
		const base =
			'w-full flex items-center gap-2.5 px-2.5 py-1.5 rounded-md text-sm transition-all duration-150 text-left outline-none';
		if (isActive(path)) return `${base} bg-primary/10 text-primary font-medium`;
		return `${base} text-muted-foreground hover:bg-accent hover:text-accent-foreground`;
	}

	async function handleLogout() {
		await signOut();
		goto('/auth');
	}

	// Drag-to-resize
	let width = $state(220);
	let dragging = $state(false);

	function onDragStart(e: MouseEvent) {
		e.preventDefault();
		dragging = true;
		const onMove = (e: MouseEvent) => {
			width = Math.min(320, Math.max(180, e.clientX));
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
				class="w-6 h-6 rounded-md bg-primary/20 flex items-center justify-center text-primary text-xs font-bold shrink-0"
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
		<ChevronDown size={14} class="text-sidebar-foreground/40" />
	</div>

	<!-- Quick search hint -->
	<button
		class="mx-3 mt-3 flex items-center gap-2 px-2.5 py-1.5 rounded-md border border-sidebar-border bg-sidebar text-xs text-muted-foreground hover:bg-accent hover:text-accent-foreground transition-colors"
		onclick={() => {
			const event = new KeyboardEvent('keydown', { key: 'k', metaKey: true, bubbles: true });
			window.dispatchEvent(event);
		}}
	>
		<Command size={12} />
		<span class="flex-1 text-left">Search...</span>
		<kbd class="text-[9px] bg-muted px-1 py-0.5 rounded border border-border font-mono">⌘K</kbd>
	</button>

	<!-- Nav -->
	<nav class="flex-1 overflow-y-auto py-3 px-2 space-y-0.5">
		<p class="text-[10px] uppercase tracking-widest text-sidebar-foreground/30 px-2.5 pt-1 pb-1.5 font-medium">
			Platform
		</p>
		{#each navHome as item, i}
			<button
				class={navItemClass(item.path)}
				style="animation-delay: {i * 20}ms"
				onclick={() => item.path && goto(item.path)}
			>
				<item.icon size={15} class="shrink-0" />
				<span class="truncate">{item.label}</span>
			</button>
		{/each}

		<div class="py-2 px-2.5">
			<Separator />
		</div>

		<p class="text-[10px] uppercase tracking-widest text-sidebar-foreground/30 px-2.5 pt-1 pb-1.5 font-medium">
			Settings
		</p>
		{#each navSettings as item, i}
			<button
				class={navItemClass(item.path)}
				style="animation-delay: {(navHome.length + i) * 20}ms"
				onclick={() => item.path && goto(item.path)}
			>
				<item.icon size={15} class="shrink-0" />
				<span class="truncate">{item.label}</span>
			</button>
		{/each}

		<div class="py-2 px-2.5">
			<Separator />
		</div>

		<button class={navItemClass()}>
			<BookOpen size={15} class="shrink-0" />
			<span>Documentation</span>
		</button>
	</nav>

	<!-- User footer -->
	<div class="border-t border-sidebar-border px-3 py-2.5">
		<div class="flex items-center justify-between">
			<div class="flex items-center gap-2 min-w-0">
				<Avatar class="w-7 h-7 text-xs shrink-0">
					<AvatarFallback class="bg-primary text-primary-foreground text-xs font-bold">
						{initials}
					</AvatarFallback>
				</Avatar>
				<div class="min-w-0">
					<p class="text-xs font-medium truncate text-sidebar-foreground">{userName}</p>
					<p class="text-[10px] text-sidebar-foreground/40 truncate">{userEmail}</p>
				</div>
			</div>
			<Button
				variant="ghost"
				size="sm"
				class="h-7 w-7 p-0 text-sidebar-foreground/40 hover:text-destructive"
				onclick={handleLogout}
			>
				<LogOut size={14} />
			</Button>
		</div>
		<p class="text-[9px] text-sidebar-foreground/20 mt-1.5 font-mono">rustploy v0.1.0</p>
	</div>

	<!-- Drag handle -->
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
</aside>
