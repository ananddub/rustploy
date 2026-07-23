import React from 'react';
import { Outlet, useLocation } from 'react-router-dom';
import { Toaster } from 'sonner';
import { Sidebar } from '@/components/Sidebar';
import { HeaderBar } from '@/components/HeaderBar';
import { CommandPalette } from '@/components/CommandPalette';

export default function AppLayout() {
	const location = useLocation();
	const isAuthPage = location.pathname.startsWith('/auth');

	if (isAuthPage) {
		return (
			<>
				<Outlet />
				<Toaster theme="dark" position="top-right" />
			</>
		);
	}

	return (
		<div className="h-screen w-screen flex bg-[#0A0A0A] text-[#FAFAFA] overflow-hidden font-sans antialiased box-border">
			<Sidebar />
			<div className="flex-1 flex flex-col min-w-0 h-full overflow-hidden bg-[#0A0A0A]">
				<HeaderBar />
				<div className="flex-1 flex flex-col min-h-0 overflow-y-auto custom-scrollbar">
					<Outlet />
				</div>
			</div>
			<CommandPalette />
			<Toaster theme="dark" position="top-right" />
		</div>
	);
}
