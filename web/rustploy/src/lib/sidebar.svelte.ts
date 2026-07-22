/**
 * Reactive sidebar collapse/expand state store using Svelte 5 runes.
 * Must have .svelte.ts extension so Svelte compiler processes $state/$derived.
 */
export const sidebarState = $state({
	collapsed: false,
	width: 240,
	toggle() {
		this.collapsed = !this.collapsed;
	}
});
