import { createSignal } from 'solid-js';
import { signUpWithPassword } from '../../lib/auth';

export default function SignupForm(props: { onSuccess?: () => void }) {
  const [email, setEmail] = createSignal('');
  const [password, setPassword] = createSignal('');
  const [showPassword, setShowPassword] = createSignal(false);
  const [firstName, setFirstName] = createSignal('');
  const [lastName, setLastName] = createSignal('');
  const [loading, setLoading] = createSignal(false);
  const [error, setError] = createSignal('');

  const submit = async (e: Event) => {
    e.preventDefault();
    setError('');
    setLoading(true);
    try {
      await signUpWithPassword({
        email: email(),
        password: password(),
        first_name: firstName() || undefined,
        last_name: lastName() || undefined,
      });
      props.onSuccess?.();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Signup failed');
    } finally {
      setLoading(false);
    }
  };

  return (
    <form onSubmit={submit} class="flex flex-col gap-4">
      <div class="grid grid-cols-2 gap-3">
        <div>
          <label class="block text-sm text-gray-300 mb-1.5">First name</label>
          <input
            class="w-full px-3 py-2 rounded-md bg-[#0d0d0d] border border-gray-700 text-white placeholder-gray-600 text-sm focus:outline-none focus:ring-1 focus:ring-gray-500 focus:border-gray-500 transition-all"
            value={firstName()}
            onInput={(e) => setFirstName(e.currentTarget.value)}
            placeholder="Aman"
          />
        </div>
        <div>
          <label class="block text-sm text-gray-300 mb-1.5">Last name</label>
          <input
            class="w-full px-3 py-2 rounded-md bg-[#0d0d0d] border border-gray-700 text-white placeholder-gray-600 text-sm focus:outline-none focus:ring-1 focus:ring-gray-500 focus:border-gray-500 transition-all"
            value={lastName()}
            onInput={(e) => setLastName(e.currentTarget.value)}
            placeholder="Kumar"
          />
        </div>
      </div>

      <div>
        <label class="block text-sm text-gray-300 mb-1.5">Email</label>
        <input
          class="w-full px-3 py-2 rounded-md bg-[#0d0d0d] border border-gray-700 text-white placeholder-gray-600 text-sm focus:outline-none focus:ring-1 focus:ring-gray-500 focus:border-gray-500 transition-all"
          type="email"
          value={email()}
          onInput={(e) => setEmail(e.currentTarget.value)}
          placeholder="you@example.com"
          required
        />
      </div>

      <div>
        <label class="block text-sm text-gray-300 mb-1.5">Password</label>
        <div class="relative">
          <input
            class="w-full px-3 py-2 rounded-md bg-[#0d0d0d] border border-gray-700 text-white placeholder-gray-600 text-sm focus:outline-none focus:ring-1 focus:ring-gray-500 focus:border-gray-500 transition-all pr-10"
            type={showPassword() ? 'text' : 'password'}
            value={password()}
            onInput={(e) => setPassword(e.currentTarget.value)}
            placeholder="••••••••"
            required
          />
          <button
            type="button"
            class="absolute right-3 top-1/2 -translate-y-1/2 text-gray-500 hover:text-gray-300 transition-colors"
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
      </div>

      <button
        class="w-full py-2 mt-1 bg-white text-black text-sm font-medium rounded-md hover:bg-gray-200 transition-colors disabled:opacity-40 disabled:cursor-not-allowed"
        type="submit"
        disabled={loading()}
      >
        {loading() ? 'Creating…' : 'Create account'}
      </button>

      {error() && (
        <div class="p-3 bg-red-950/40 border border-red-800 rounded-md text-red-400 text-sm">
          {error()}
        </div>
      )}
    </form>
  );
}
