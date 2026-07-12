<script lang="ts">
	import { onMount } from 'svelte';
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { client } from '$lib/client/client.gen';
	import { getAuthSession, refreshAccessToken, clearAuthSession } from '$lib/auth';
	import { goto } from '$app/navigation';

	let { children } = $props();

	// Setup client interceptors once on mount
	onMount(() => {
		// Request interceptor: add auth token
		client.interceptors.request.use((request: Request) => {
			const session = getAuthSession();
			if (session?.tokens.access_token) {
				request.headers.set('Authorization', `Bearer ${session.tokens.access_token}`);
			}
			return request;
		});

		// Response interceptor: auto-refresh on 401
		client.interceptors.response.use(async (response: Response, request: Request) => {
			if (response.status !== 401) return response;
			const url = (request as Request).url;
			if (url.includes('/auth/')) return response;

			const newToken = await refreshAccessToken();
			if (!newToken) {
				clearAuthSession();
				if (!window.location.pathname.startsWith('/auth')) {
					goto('/auth');
				}
				return response;
			}

			return fetch(request, {
				headers: {
					...Object.fromEntries((request.headers as Headers).entries()),
					Authorization: `Bearer ${newToken}`
				}
			});
		});
	});
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
{@render children()}
