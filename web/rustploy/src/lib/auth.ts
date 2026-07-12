import { browser } from '$app/environment';
import {
	authControllerLogin,
	authControllerLogout,
	authControllerRefresh,
	authControllerSignup,
	authControllerWhoAmI
} from '$lib/client/sdk.gen';
import type { JwtSubject, TokenPair } from '$lib/client/types.gen';

const AUTH_STORAGE_KEY = 'rustploy-auth-session';

export type AuthSession = {
	tokens: TokenPair;
	user: JwtSubject;
};

// Plain module-level variable — no $state (auth.ts is a plain TS file, not a .svelte)
let _session: AuthSession | null = null;

function loadStoredSession(): AuthSession | null {
	if (!browser) return null;
	const raw = window.localStorage.getItem(AUTH_STORAGE_KEY);
	if (!raw) return null;
	try {
		return JSON.parse(raw) as AuthSession;
	} catch {
		window.localStorage.removeItem(AUTH_STORAGE_KEY);
		return null;
	}
}

function persistSession(session: AuthSession | null) {
	if (!browser) return;
	if (session) {
		window.localStorage.setItem(AUTH_STORAGE_KEY, JSON.stringify(session));
	} else {
		window.localStorage.removeItem(AUTH_STORAGE_KEY);
	}
}

const extractErrorMessage = (error: unknown): string => {
	if (typeof error === 'string') return error;
	if (
		error &&
		typeof error === 'object' &&
		'message' in error &&
		typeof (error as { message: unknown }).message === 'string'
	)
		return (error as { message: string }).message;
	return 'Authentication failed. Please try again.';
};

// Initialize from localStorage on first import (browser only)
if (browser) {
	_session = loadStoredSession();
	// Sync reactive store after it loads
	import('./auth.svelte').then(({ authState }) => {
		authState.session = _session;
	});
}

export function getAuthSession(): AuthSession | null {
	return _session;
}

export function setAuthSession(session: AuthSession | null) {
	_session = session;
	persistSession(session);
	// Sync reactive store (only runs in browser where Svelte has compiled the rune)
	if (browser) {
		import('./auth.svelte').then(({ authState }) => {
			authState.session = session;
		});
	}
}

export function clearAuthSession() {
	_session = null;
	persistSession(null);
	if (browser) {
		import('./auth.svelte').then(({ authState }) => {
			authState.session = null;
		});
	}
}

// Deduplicate concurrent refresh calls
let refreshPromise: Promise<string | null> | null = null;

export async function refreshAccessToken(): Promise<string | null> {
	if (refreshPromise) return refreshPromise;

	refreshPromise = (async () => {
		const session = _session;
		if (!session?.tokens.refresh_token) return null;

		try {
			const res = await authControllerRefresh({
				body: { refresh_token: session.tokens.refresh_token }
			});

			if (res.error || !res.data) {
				clearAuthSession();
				return null;
			}

			const updated: AuthSession = {
				tokens: res.data.tokens,
				user: res.data.user
			};
			setAuthSession(updated);
			return updated.tokens.access_token;
		} catch {
			clearAuthSession();
			return null;
		} finally {
			refreshPromise = null;
		}
	})();

	return refreshPromise;
}

export async function restoreAuthSession(): Promise<boolean> {
	const storedSession = _session;
	if (!storedSession) return false;

	try {
		const response = await authControllerWhoAmI({
			auth: storedSession.tokens.access_token
		});

		if (response.error || !response.data) {
			const newToken = await refreshAccessToken();
			if (!newToken) {
				clearAuthSession();
				return false;
			}
			return true;
		}

		setAuthSession({ ...storedSession, user: response.data });
		return true;
	} catch {
		clearAuthSession();
		return false;
	}
}

export async function signInWithPassword(email: string, password: string): Promise<AuthSession> {
	const response = await authControllerLogin({ body: { email, password } });
	if (response.error || !response.data) throw new Error(extractErrorMessage(response.error));

	const nextSession: AuthSession = {
		tokens: response.data.tokens,
		user: response.data.user
	};
	setAuthSession(nextSession);
	return nextSession;
}

export async function signUpWithPassword(input: {
	email: string;
	password: string;
	first_name?: string;
	last_name?: string;
}): Promise<AuthSession> {
	const response = await authControllerSignup({ body: input });
	if (response.error || !response.data) throw new Error(extractErrorMessage(response.error));

	const nextSession: AuthSession = {
		tokens: response.data.tokens,
		user: response.data.user
	};
	setAuthSession(nextSession);
	return nextSession;
}

export async function signOut() {
	const currentSession = _session;
	try {
		if (currentSession?.tokens.access_token) {
			await authControllerLogout({ auth: currentSession.tokens.access_token });
		}
	} catch {
		/* ignore */
	}
	clearAuthSession();
}
