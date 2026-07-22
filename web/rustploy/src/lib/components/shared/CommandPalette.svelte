<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import {
		House,
		FolderOpen,
		Zap,
		ChartLine,
		Package,
		User,
		Server,
		Shield,
		Bell,
		GitBranch,
		Database,
		Tag,
		Search
	} from '@lucide/svelte';
	import * as Command from '$lib/components/ui/command';

	let open = $state(false);

	const routes = [
		{ label: 'Dashboard', path: '/dashboard', icon: House, group: 'Navigation' },
		{ label: 'Projects', path: '/projects', icon: FolderOpen, group: 'Navigation' },
		{ label: 'Deployments', path: '/deployments', icon: Zap, group: 'Navigation' },
		{ label: 'Monitoring', path: '/monitoring', icon: ChartLine, group: 'Navigation' },
		{ label: 'Docker', path: '/docker', icon: Package, group: 'Navigation' },
		{ label: 'Profile', path: '/settings/profile', icon: User, group: 'Settings' },
		{ label: 'Servers', path: '/settings/servers', icon: Server, group: 'Settings' },
		{ label: 'Certificates', path: '/settings/certificates', icon: Shield, group: 'Settings' },
		{ label: 'Notifications', path: '/settings/notifications', icon: Bell, group: 'Settings' },
		{ label: 'Registry', path: '/settings/registry', icon: Database, group: 'Settings' },
		{ label: 'Git Providers', path: '/settings/git-providers', icon: GitBranch, group: 'Settings' },
		{ label: 'Tags', path: '/settings/tags', icon: Tag, group: 'Settings' }
	];

	function navigate(path: string) {
		open = false;
		goto(path);
	}

	function handleKeydown(e: KeyboardEvent) {
		if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
			e.preventDefault();
			open = !open;
		}
	}

	onMount(() => {
		const handleCustomOpen = () => {
			open = true;
		};
		window.addEventListener('open-command-palette', handleCustomOpen);
		return () => {
			window.removeEventListener('open-command-palette', handleCustomOpen);
		};
	});
</script>

<svelte:window onkeydown={handleKeydown} />

<Command.Dialog bind:open>
	<Command.Input placeholder="Type a command or search..." />
	<Command.List>
		<Command.Empty>No results found.</Command.Empty>
		<Command.Group heading="Navigation">
			{#each routes.filter((r) => r.group === 'Navigation') as route}
				<Command.Item onSelect={() => navigate(route.path)}>
					<route.icon class="mr-2 h-4 w-4 text-muted-foreground" />
					<span>{route.label}</span>
				</Command.Item>
			{/each}
		</Command.Group>
		<Command.Separator />
		<Command.Group heading="Settings">
			{#each routes.filter((r) => r.group === 'Settings') as route}
				<Command.Item onSelect={() => navigate(route.path)}>
					<route.icon class="mr-2 h-4 w-4 text-muted-foreground" />
					<span>{route.label}</span>
				</Command.Item>
			{/each}
		</Command.Group>
	</Command.List>
</Command.Dialog>
