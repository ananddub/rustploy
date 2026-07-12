<script lang="ts">
	import { goto } from '$app/navigation';
	import { RocketIcon, Clock, ArrowRight } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	const userName =
		session?.user.first_name || session?.user.email?.split('@')[0] || 'user';

	const stats = [
		{ label: 'PROJECTS', value: '1', sub: '1 environment' },
		{ label: 'SERVICES', value: '3', sub: '1 apps · 1 compose · 1 db' },
		{ label: 'DEPLOYS / 7D', value: '1', sub: 'no prior data' }
	];

	const statuses = [
		{ count: 1, label: 'running', color: 'bg-green-500' },
		{ count: 1, label: 'errored', color: 'bg-red-500' },
		{ count: 1, label: 'idle', color: 'bg-muted-foreground/30' }
	];

	const deployments = [
		{ name: 'nginx', sub: 'backend · production', state: 'done', time: '3 days ago' }
	];

	const now = new Date().toLocaleTimeString('en-GB');
</script>

<PageLayout>
	<!-- Top bar -->
	<header class="flex items-center justify-between px-6 py-3 border-b border-border bg-background">
		<div class="flex items-center gap-2 text-sm text-muted-foreground">
			<RocketIcon class="w-4 h-4" />
			<span>Home</span>
		</div>
		<div class="flex items-center gap-1 text-xs text-muted-foreground">
			<Clock class="w-3.5 h-3.5" />
			<span>Server Time: {now} UTC | UTC+00:00</span>
		</div>
	</header>

	<main class="flex-1 p-6">
		<div class="flex items-center justify-between mb-6">
			<h1 class="text-2xl font-semibold">Welcome back, {userName}</h1>
			<button
				class="inline-flex items-center gap-1 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 transition-colors"
				onclick={() => goto('/projects')}
			>
				Go to projects <ArrowRight class="w-3.5 h-3.5" />
			</button>
		</div>

		<!-- Stats -->
		<div class="grid grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
			{#each stats as s}
				<div class="bg-card rounded-lg p-4 border border-border">
					<p class="text-[10px] uppercase tracking-widest text-muted-foreground mb-2">{s.label}</p>
					<p class="text-3xl font-semibold">{s.value}</p>
					<p class="text-xs text-muted-foreground mt-1">{s.sub}</p>
				</div>
			{/each}

			<!-- Status card -->
			<div class="bg-card rounded-lg p-4 border border-border">
				<p class="text-[10px] uppercase tracking-widest text-muted-foreground mb-2">STATUS</p>
				<div class="flex flex-col gap-1.5 mt-1">
					{#each statuses as s}
						<div class="flex items-center gap-2 text-sm">
							<span class="w-2 h-2 rounded-full shrink-0 {s.color}"></span>
							<span class="font-medium">{s.count}</span>
							<span class="text-muted-foreground">{s.label}</span>
						</div>
					{/each}
				</div>
			</div>
		</div>

		<!-- Recent deployments -->
		<div class="bg-card rounded-lg border border-border">
			<div class="flex items-center justify-between px-4 py-3 border-b border-border">
				<div class="flex items-center gap-2 text-sm font-medium">
					<RocketIcon class="w-4 h-4 text-muted-foreground" />
					<span>Recent deployments</span>
				</div>
				<button class="text-xs text-muted-foreground hover:text-foreground transition-colors">
					view all →
				</button>
			</div>

			<div class="divide-y divide-border">
				{#each deployments as d}
					<div class="flex items-center justify-between px-4 py-3">
						<div class="flex items-center gap-3">
							<span class="w-2 h-2 rounded-full bg-green-500 shrink-0"></span>
							<div>
								<p class="text-sm font-medium">{d.name}</p>
								<p class="text-xs text-muted-foreground">{d.sub}</p>
							</div>
						</div>
						<div class="flex items-center gap-6 text-xs text-muted-foreground">
							<div class="flex items-center gap-1">
								<RocketIcon class="w-3 h-3" />
								<span>Rustploy</span>
							</div>
							<span>{d.state}</span>
							<span>{d.time}</span>
							<button class="hover:text-foreground transition-colors">logs →</button>
						</div>
					</div>
				{/each}
			</div>
		</div>
	</main>
</PageLayout>
