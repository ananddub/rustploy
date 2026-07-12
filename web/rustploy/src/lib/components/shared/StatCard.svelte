<script lang="ts">
	import { type Snippet } from 'svelte';

	type Props = {
		class?: string;
		children: Snippet;
	};

	let { class: className = '', children }: Props = $props();
</script>

<div class="flex flex-col gap-1 animate-fade-up {className}">
	{@render children()}
</div>
