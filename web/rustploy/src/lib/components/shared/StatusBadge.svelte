<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';

	type Props = {
		status: string;
		pulse?: boolean;
	};

	let { status, pulse = false }: Props = $props();

	const variantMap: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
		running: 'default',
		success: 'default',
		healthy: 'default',
		active: 'default',
		error: 'destructive',
		failed: 'destructive',
		unhealthy: 'destructive',
		warning: 'secondary',
		deploying: 'secondary',
		building: 'secondary',
		pending: 'secondary',
		stopped: 'outline',
		idle: 'outline',
		cancelled: 'outline',
		exited: 'outline'
	};

	const dotColorMap: Record<string, string> = {
		running: 'bg-green-400',
		success: 'bg-green-400',
		healthy: 'bg-green-400',
		active: 'bg-green-400',
		error: 'bg-red-400',
		failed: 'bg-red-400',
		unhealthy: 'bg-red-400',
		warning: 'bg-yellow-400',
		deploying: 'bg-yellow-400',
		building: 'bg-yellow-400',
		pending: 'bg-yellow-400',
		stopped: 'bg-muted-foreground/50',
		idle: 'bg-muted-foreground/50',
		cancelled: 'bg-muted-foreground/50',
		exited: 'bg-muted-foreground/50'
	};

	const lower = $derived(status?.toLowerCase() ?? 'idle');
	const variant = $derived(variantMap[lower] ?? 'outline');
	const dotColor = $derived(dotColorMap[lower] ?? 'bg-muted-foreground/50');
</script>

<Badge {variant} class="gap-1.5 capitalize text-[11px]">
	<span class="w-1.5 h-1.5 rounded-full shrink-0 {dotColor} {pulse ? 'animate-pulse' : ''}"></span>
	{lower}
</Badge>
