import { useState, useEffect } from 'react';

type SidebarListener = (collapsed: boolean) => void;
const listeners = new Set<SidebarListener>();

let _collapsed = false;

export const sidebarState = {
	get collapsed() {
		return _collapsed;
	},
	set collapsed(val: boolean) {
		_collapsed = val;
		listeners.forEach((l) => l(_collapsed));
	},
	toggle() {
		this.collapsed = !this.collapsed;
	},
	subscribe(listener: SidebarListener) {
		listeners.add(listener);
		return () => {
			listeners.delete(listener);
		};
	}
};

export function useSidebarState() {
	const [collapsed, setCollapsed] = useState(sidebarState.collapsed);

	useEffect(() => {
		return sidebarState.subscribe((newVal) => setCollapsed(newVal));
	}, []);

	return {
		collapsed,
		toggle: () => sidebarState.toggle()
	};
}
