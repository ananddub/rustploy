import React from 'react';
import { cn } from '$lib/utils';

interface StatusBadgeProps {
	status: string;
	pulse?: boolean;
	className?: string;
}

export function StatusBadge({ status, pulse = false, className }: StatusBadgeProps) {
	const normalized = status.toLowerCase();

	let badgeColor = 'bg-[#262626] text-[#a1a1aa] border-[#262626]';
	let dotColor = 'bg-[#71717a]';

	if (normalized === 'done' || normalized === 'running' || normalized === 'active' || normalized === 'healthy' || normalized === 'valid') {
		badgeColor = 'bg-green-500/10 text-green-400 border-green-500/30';
		dotColor = 'bg-green-500';
	} else if (normalized === 'building' || normalized === 'connecting' || normalized === 'expiring') {
		badgeColor = 'bg-blue-500/10 text-blue-400 border-blue-500/30';
		dotColor = 'bg-blue-500';
	} else if (normalized === 'error' || normalized === 'failed' || normalized === 'inactive') {
		badgeColor = 'bg-red-500/10 text-red-400 border-red-500/30';
		dotColor = 'bg-red-500';
	}

	return (
		<span
			className={cn(
				'inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-[10px] font-mono border font-semibold capitalize',
				badgeColor,
				className
			)}
		>
			<span
				className={cn(
					'w-1.5 h-1.5 rounded-full shrink-0',
					dotColor,
					(pulse || normalized === 'building' || normalized === 'connecting') && 'animate-pulse'
				)}
			/>
			{status}
		</span>
	);
}
