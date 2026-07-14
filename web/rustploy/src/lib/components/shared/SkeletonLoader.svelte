<script lang="ts">
	import { Skeleton } from '$lib/components/ui/skeleton';

	type Props = {
		rows?: number;
		class?: string;
	};

	let { rows = 3, class: className = '' }: Props = $props();
</script>

<div class="flex flex-col gap-4 {className}">
	{#each Array(rows) as _, i}
		<div class="space-y-2" style="animation-delay: {i * 80}ms">
			<Skeleton class="h-4 w-full" />
			<Skeleton class="h-3 w-3/4" />
		</div>
	{/each}
</div>
