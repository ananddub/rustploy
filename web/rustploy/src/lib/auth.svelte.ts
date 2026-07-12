/**
 * Reactive auth session store using Svelte 5 runes.
 * This file MUST have the .svelte.ts extension so the Svelte compiler
 * processes it and allows $state/$derived runes.
 */
import type { AuthSession } from './auth';

export const authState = $state<{ session: AuthSession | null }>({ session: null });
