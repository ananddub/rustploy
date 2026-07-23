import * as React from 'react';
import { cn } from '$lib/utils';

const Input = React.forwardRef<HTMLInputElement, React.ComponentProps<'input'>>(
	({ className, type, ...props }, ref) => {
		return (
			<input
				type={type}
				className={cn(
					'flex h-9 w-full rounded-md border border-[#262626] bg-[#141414] px-3 py-1 text-xs text-[#FAFAFA] shadow-xs transition-colors file:border-0 file:bg-transparent file:text-xs file:font-medium placeholder:text-[#737373] focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-[#3f3f46] disabled:cursor-not-allowed disabled:opacity-50',
					className
				)}
				ref={ref}
				{...props}
			/>
		);
	}
);
Input.displayName = 'Input';

export { Input };
