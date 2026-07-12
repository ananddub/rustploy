<script lang="ts">
	import { goto } from '$app/navigation';
	import { RocketIcon } from '@lucide/svelte';
	import { signInWithPassword, getAuthSession } from '$lib/auth';

	let mode = $state<'login' | 'signup'>('login');

	// redirect if already logged in
	if (getAuthSession()) goto('/dashboard', { replaceState: true });

	// Login state
	let email = $state('');
	let password = $state('');
	let showPassword = $state(false);
	let loading = $state(false);
	let error = $state('');

	// Signup extras
	let firstName = $state('');
	let lastName = $state('');

	async function handleLogin(e: SubmitEvent) {
		e.preventDefault();
		error = '';
		loading = true;
		try {
			await signInWithPassword(email, password);
			goto('/dashboard', { replaceState: true });
		} catch (err) {
			error = err instanceof Error ? err.message : 'Login failed';
		} finally {
			loading = false;
		}
	}

	async function handleSignup(e: SubmitEvent) {
		e.preventDefault();
		error = '';
		loading = true;
		try {
			const { signUpWithPassword } = await import('$lib/auth');
			await signUpWithPassword({
				email,
				password,
				first_name: firstName || undefined,
				last_name: lastName || undefined
			});
			goto('/dashboard', { replaceState: true });
		} catch (err) {
			error = err instanceof Error ? err.message : 'Signup failed';
		} finally {
			loading = false;
		}
	}
</script>

<div class="min-h-screen flex">
	<!-- Left Side -->
	<div class="hidden lg:flex lg:w-1/2 bg-muted flex-col p-10">
		<div class="flex items-center gap-2">
			<RocketIcon class="w-6 h-6 text-primary" />
			<span class="font-semibold text-lg">Rustploy</span>
		</div>

		<div class="mt-auto text-muted-foreground text-sm italic">
			"The Open Source alternative to Netlify, Vercel, Heroku."
		</div>

		<div class="mt-6 flex gap-4">
			<a
				href="https://github.com/rustploy/rustploy"
				target="_blank"
				aria-label="GitHub"
				class="text-muted-foreground/60 hover:text-foreground transition-colors"
			>
				<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
					<path
						fill-rule="evenodd"
						d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"
						clip-rule="evenodd"
					/>
				</svg>
			</a>
			<a
				href="https://x.com/rustploy"
				target="_blank"
				aria-label="X (Twitter)"
				class="text-muted-foreground/60 hover:text-foreground transition-colors"
			>
				<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
					<path
						d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"
					/>
				</svg>
			</a>
			<a
				href="https://discord.com/invite/rustploy"
				target="_blank"
				aria-label="Discord"
				class="text-muted-foreground/60 hover:text-foreground transition-colors"
			>
				<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
					<path
						d="M20.317 4.37a19.791 19.791 0 00-4.885-1.515.074.074 0 00-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 00-5.487 0 12.64 12.64 0 00-.617-1.25.077.077 0 00-.079-.037A19.736 19.736 0 003.677 4.37a.07.07 0 00-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 00.031.057 19.9 19.9 0 005.993 3.03.078.078 0 00.084-.028c.462-.63.874-1.295 1.226-1.994a.076.076 0 00-.041-.106 13.107 13.107 0 01-1.872-.892.077.077 0 01-.008-.128 10.2 10.2 0 00.372-.292.074.074 0 01.077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 01.078.01c.12.098.246.198.373.292a.077.077 0 01-.006.127 12.299 12.299 0 01-1.873.892.077.077 0 00-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 00.084.028 19.839 19.839 0 006.002-3.03.077.077 0 00.032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 00-.031-.03zM8.02 15.33c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.956-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.956 2.418-2.157 2.418zm7.975 0c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.955-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.946 2.418-2.157 2.418z"
					/>
				</svg>
			</a>
		</div>
	</div>

	<!-- Right Side -->
	<div class="w-full lg:w-1/2 flex items-center justify-center bg-background p-6">
		<div class="w-full max-w-sm">
			<div class="text-center mb-8">
				<div class="flex items-center justify-center gap-2 mb-2">
					<RocketIcon class="w-5 h-5 text-primary" />
					<h1 class="text-xl font-semibold">
						{mode === 'login' ? 'Sign in' : 'Create account'}
					</h1>
				</div>
				<p class="text-sm text-muted-foreground">
					{mode === 'login'
						? 'Enter your email and password to sign in'
						: 'Fill in the details to create your account'}
				</p>
			</div>

			{#if mode === 'login'}
				<form onsubmit={handleLogin} class="flex flex-col gap-4">
					<div class="flex flex-col gap-1.5">
						<label for="email" class="text-sm font-medium text-muted-foreground">Email</label>
						<input
							id="email"
							class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring"
							type="email"
							bind:value={email}
							placeholder="john@example.com"
							required
						/>
					</div>

					<div class="flex flex-col gap-1.5">
						<label for="password" class="text-sm font-medium text-muted-foreground">Password</label>
						<div class="relative">
							<input
								id="password"
								class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 pr-10 text-sm shadow-sm transition-colors placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring"
								type={showPassword ? 'text' : 'password'}
								bind:value={password}
								placeholder="Enter your password"
								required
							/>
							<button
								type="button"
								class="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground transition-colors"
								aria-label="Toggle password visibility" onclick={() => (showPassword = !showPassword)}
							>
								{#if showPassword}
									<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
									</svg>
								{:else}
									<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21" />
									</svg>
								{/if}
							</button>
						</div>
					</div>

					{#if error}
						<div class="flex items-center gap-2 rounded-md bg-destructive/10 border border-destructive/30 px-3 py-2 text-sm text-destructive">
							<svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01M12 3a9 9 0 100 18A9 9 0 0012 3z" />
							</svg>
							{error}
						</div>
					{/if}

					<button
						class="inline-flex items-center justify-center gap-2 h-9 w-full rounded-md bg-primary text-primary-foreground text-sm font-medium shadow hover:bg-primary/90 transition-colors disabled:opacity-50 mt-1"
						type="submit"
						disabled={loading}
					>
						{#if loading}
							<div class="w-4 h-4 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>
							Signing in…
						{:else}
							Login
						{/if}
					</button>

					<div class="text-center">
						<button type="button" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
							Lost your password?
						</button>
					</div>
				</form>
			{:else}
				<form onsubmit={handleSignup} class="flex flex-col gap-4">
					<div class="grid grid-cols-2 gap-3">
						<div class="flex flex-col gap-1.5">
							<label for="firstName" class="text-sm font-medium text-muted-foreground">First name</label>
							<input
								id="firstName"
								class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring"
								bind:value={firstName}
								placeholder="Aman"
							/>
						</div>
						<div class="flex flex-col gap-1.5">
							<label for="lastName" class="text-sm font-medium text-muted-foreground">Last name</label>
							<input
								id="lastName"
								class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring"
								bind:value={lastName}
								placeholder="Kumar"
							/>
						</div>
					</div>

					<div class="flex flex-col gap-1.5">
						<label for="semail" class="text-sm font-medium text-muted-foreground">Email</label>
						<input
							id="semail"
							class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring"
							type="email"
							bind:value={email}
							placeholder="you@example.com"
							required
						/>
					</div>

					<div class="flex flex-col gap-1.5">
						<label for="spassword" class="text-sm font-medium text-muted-foreground">Password</label>
						<div class="relative">
							<input
								id="spassword"
								class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 pr-10 text-sm shadow-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring"
								type={showPassword ? 'text' : 'password'}
								bind:value={password}
								placeholder="••••••••"
								required
							/>
							<button
								type="button"
								class="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground transition-colors"
								aria-label="Toggle password visibility" onclick={() => (showPassword = !showPassword)}
							>
								<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
								</svg>
							</button>
						</div>
					</div>

					{#if error}
						<div class="flex items-center gap-2 rounded-md bg-destructive/10 border border-destructive/30 px-3 py-2 text-sm text-destructive">
							{error}
						</div>
					{/if}

					<button
						class="inline-flex items-center justify-center gap-2 h-9 w-full rounded-md bg-primary text-primary-foreground text-sm font-medium shadow hover:bg-primary/90 transition-colors disabled:opacity-50 mt-1"
						type="submit"
						disabled={loading}
					>
						{#if loading}
							<div class="w-4 h-4 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>
							Creating…
						{:else}
							Create account
						{/if}
					</button>
				</form>
			{/if}

			<div class="mt-6 text-center">
				<button
					class="text-sm text-muted-foreground hover:text-foreground transition-colors"
					onclick={() => { mode = mode === 'login' ? 'signup' : 'login'; error = ''; }}
				>
					{mode === 'login' ? "Don't have an account? Sign up" : 'Already have an account? Sign in'}
				</button>
			</div>
		</div>
	</div>
</div>
