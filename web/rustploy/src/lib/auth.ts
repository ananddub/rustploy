import { createSignal } from 'solid-js';
import type { JwtSubject, TokenPair } from '../client/types.gen';
import { authControllerLogin, authControllerLogout, authControllerSignup, authControllerWhoAmI } from '../client/sdk.gen';

const AUTH_STORAGE_KEY = 'rustploy-auth-session';

export type AuthSession = {
  tokens: TokenPair;
  user: JwtSubject;
};

const loadStoredSession = (): AuthSession | null => {
  if (typeof window === 'undefined') {
    return null;
  }

  const raw = window.localStorage.getItem(AUTH_STORAGE_KEY);
  if (!raw) {
    return null;
  }

  try {
    return JSON.parse(raw) as AuthSession;
  } catch {
    window.localStorage.removeItem(AUTH_STORAGE_KEY);
    return null;
  }
};

const persistSession = (session: AuthSession | null) => {
  if (typeof window === 'undefined') {
    return;
  }

  if (session) {
    window.localStorage.setItem(AUTH_STORAGE_KEY, JSON.stringify(session));
    return;
  }

  window.localStorage.removeItem(AUTH_STORAGE_KEY);
};

const extractErrorMessage = (error: unknown): string => {
  if (typeof error === 'string') {
    return error;
  }

  if (error && typeof error === 'object' && 'message' in error && typeof error.message === 'string') {
    return error.message;
  }

  return 'Authentication failed. Please try again.';
};

export const [authSession, setAuthSession] = createSignal<AuthSession | null>(loadStoredSession());
export const [authReady, setAuthReady] = createSignal(false);

export async function restoreAuthSession() {
  const storedSession = authSession();

  if (!storedSession) {
    setAuthReady(true);
    return false;
  }

  try {
    const response = await authControllerWhoAmI({ auth: storedSession.tokens.access_token });

    if (response.error || !response.data) {
      clearAuthSession();
      return false;
    }

    setAuthSession({ ...storedSession, user: response.data });
    persistSession(authSession());
    setAuthReady(true);
    return true;
  } catch (error) {
    console.error(error);
    clearAuthSession();
    return false;
  }
}

export function clearAuthSession() {
  setAuthSession(null);
  persistSession(null);
  setAuthReady(true);
}

export async function signInWithPassword(email: string, password: string) {
  const response = await authControllerLogin({ body: { email, password } });

  if (response.error || !response.data) {
    throw new Error(extractErrorMessage(response.error));
  }

  const nextSession: AuthSession = {
    tokens: response.data.tokens,
    user: response.data.user,
  };

  setAuthSession(nextSession);
  persistSession(nextSession);
  return nextSession;
}

export async function signUpWithPassword(input: { email: string; password: string; first_name?: string; last_name?: string }) {
  const response = await authControllerSignup({ body: input });

  if (response.error || !response.data) {
    throw new Error(extractErrorMessage(response.error));
  }

  const nextSession: AuthSession = {
    tokens: response.data.tokens,
    user: response.data.user,
  };

  setAuthSession(nextSession);
  persistSession(nextSession);
  return nextSession;
}

export async function signOut() {
  const currentSession = authSession();

  try {
    if (currentSession?.tokens.access_token) {
      await authControllerLogout({ auth: currentSession.tokens.access_token });
    }
  } catch (error) {
    console.error(error);
  }

  clearAuthSession();
}
