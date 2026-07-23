const AUTH_STORAGE_KEY = 'rustploy-auth-session';

export type TokenPair = {
	access_token: string;
	refresh_token: string;
};

export type JwtSubject = {
	id: string;
	email: string;
	first_name?: string;
	last_name?: string;
	group_id?: string;
};

export type AuthSession = {
	tokens: TokenPair;
	user: JwtSubject;
};

let _session: AuthSession | null = null;

function loadStoredSession(): AuthSession | null {
	if (typeof window === 'undefined') return null;
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
	if (typeof window === 'undefined') return;
	if (session) {
		window.localStorage.setItem(AUTH_STORAGE_KEY, JSON.stringify(session));
	} else {
		window.localStorage.removeItem(AUTH_STORAGE_KEY);
	}
}

if (typeof window !== 'undefined') {
	_session = loadStoredSession();
}

export function getAuthSession(): AuthSession | null {
	if (!_session && typeof window !== 'undefined') {
		_session = loadStoredSession();
	}
	return _session;
}

export function setAuthSession(session: AuthSession | null) {
	_session = session;
	persistSession(session);
}

export function clearAuthSession() {
	_session = null;
	persistSession(null);
}

export function mockLogin(email: string): AuthSession {
	const session: AuthSession = {
		tokens: { access_token: 'mock-access-token', refresh_token: 'mock-refresh-token' },
		user: { id: 'user-01', email, first_name: 'Aditya', last_name: 'Sahu', group_id: 'org-main' }
	};
	setAuthSession(session);
	return session;
}
