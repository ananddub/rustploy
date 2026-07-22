<script lang="ts">
	import { type Snippet } from 'svelte';
	import Sidebar from './Sidebar.svelte';

	type Props = { children: Snippet };
	let { children }: Props = $props();
</script>

<!-- Dokploy Outer Canvas (#0A0A0A) -->
<div class="h-screen w-screen flex bg-[#0A0A0A] text-[#FAFAFA] overflow-hidden font-sans antialiased box-border">
	<!-- Left Sidebar Panel -->
	<Sidebar />
	<!-- Right Column (Header on top of #0A0A0A, Main Content Card Floating Underneath) -->
	<div class="flex-1 flex flex-col min-w-0 h-full overflow-hidden bg-[#0A0A0A]">
		{@render children()}
	</div>
</div>
