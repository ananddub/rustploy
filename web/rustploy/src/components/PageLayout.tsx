import React from 'react';
import { Sidebar } from './Sidebar';

interface PageLayoutProps {
	children: React.ReactNode;
}

export function PageLayout({ children }: PageLayoutProps) {
	return (
		<div className="h-screen w-screen flex bg-[#0A0A0A] text-[#FAFAFA] overflow-hidden font-sans antialiased box-border">
			<Sidebar />
			<div className="flex-1 flex flex-col min-w-0 h-full overflow-hidden bg-[#0A0A0A]">
				{children}
			</div>
		</div>
	);
}
