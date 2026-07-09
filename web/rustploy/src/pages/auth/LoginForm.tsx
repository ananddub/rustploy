import { createSignal } from 'solid-js';
import { signInWithPassword } from '../../lib/auth';

export default function LoginForm(props: { onSuccess?: () => void }) {
  const [email, setEmail] = createSignal('');
  const [password, setPassword] = createSignal('');
  const [showPassword, setShowPassword] = createSignal(false);
  const [loading, setLoading] = createSignal(false);
  const [error, setError] = createSignal('');

  const submit = async (e: Event) => {
    e.preventDefault();
    setError('');
    setLoading(true);
    try {
      const session = await signInWithPassword(email(), password());
      console.log('[login] success, session:', session);
      console.log('[login] localStorage:', localStorage.getItem('rustploy-auth-session'));
      props.onSuccess?.();
    } catch (err) {
      console.log('[login] error:', err);
      setError(err instanceof Error ? err.message : 'Login failed');
    } finally {
      setLoading(false);
    }
  };

  return (
    <form onSubmit={submit} class="flex flex-col gap-4">
      <fieldset class="fieldset">
        <legend class="fieldset-legend text-base-content/70">Email</legend>
        <input
          class="input input-bordered w-full"
          type="email"
          value={email()}
          onInput={(e) => setEmail(e.currentTarget.value)}
          placeholder="john@example.com"
          required
        />
      </fieldset>

      <fieldset class="fieldset">
        <legend class="fieldset-legend text-base-content/70">Password</legend>
        <div class="relative">
          <input
            class="input input-bordered w-full pr-10"
            type={showPassword() ? 'text' : 'password'}
            value={password()}
            onInput={(e) => setPassword(e.currentTarget.value)}
            placeholder="Enter your password"
            required
          />
          <button
            type="button"
            class="absolute right-3 top-1/2 -translate-y-1/2 text-base-content/40 hover:text-base-content transition-colors"
            onClick={() => setShowPassword((s) => !s)}
          >
            {showPassword() ? (
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"/>
              </svg>
            ) : (
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21"/>
              </svg>
            )}
          </button>
        </div>
      </fieldset>

      <button
        class="btn btn-neutral w-full mt-1"
        type="submit"
        disabled={loading()}
      >
        {loading() && <span class="loading loading-spinner loading-sm" />}
        {loading() ? 'Signing in…' : 'Login'}
      </button>

      <div class="text-center">
        <a class="text-sm text-base-content/40 hover:text-base-content transition-colors cursor-pointer">
          Lost your password?
        </a>
      </div>

      {error() && (
        <div role="alert" class="alert alert-error text-sm">
          <svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01M12 3a9 9 0 100 18A9 9 0 0012 3z"/>
          </svg>
          <span>{error()}</span>
        </div>
      )}
    </form>
  );
}
