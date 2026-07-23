import React from 'react';
import { Outlet } from 'react-router-dom';
import { Toaster } from 'sonner';
import { CommandPalette } from '$lib/../components/CommandPalette';

export default function AppLayout() {
	return (
		<>
			<Outlet />
			<CommandPalette />
			<Toaster theme="dark" position="top-right" />
		</>
	);
}
