import React from 'react';
import { Outlet, useLocation } from 'react-router-dom';
import { Toaster } from 'sonner';
import { Sidebar } from '$lib/../components/Sidebar';
import { CommandPalette } from '$lib/../components/CommandPalette';

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
				<Outlet />
			</div>
			<CommandPalette />
			<Toaster theme="dark" position="top-right" />
		</div>
	);
}
