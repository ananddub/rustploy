<script lang="ts">
	import { type Snippet } from 'svelte';
	import Sidebar from './Sidebar.svelte';

	type Props = { children: Snippet };
	let { children }: Props = $props();
</script>

<div class="min-h-screen flex bg-background text-foreground">
	<Sidebar />
	<div class="flex-1 flex flex-col min-w-0">
		{@render children()}
	</div>
</div>
