import { Show, createEffect } from 'solid-js';
import { useNavigate } from '@solidjs/router';
import { authSession, signOut } from '../lib/auth';

export default function DashboardPage() {
  const navigate = useNavigate();

  createEffect(() => {
    if (!authSession()) {
      navigate('/auth', { replace: true });
    }
  });

  const handleLogout = async () => {
    await signOut();
    navigate('/auth', { replace: true });
  };

  return (
    <Show when={authSession()}>
      <div class="min-h-screen bg-base-200 p-6 text-base-content">
        <div class="mx-auto flex max-w-5xl flex-col gap-6">
          <div class="rounded-box border border-base-300 bg-base-100 p-6 shadow-xl">
            <div class="flex flex-wrap items-center justify-between gap-4">
              <div>
                <p class="text-sm uppercase tracking-[0.3em] text-primary">Authenticated</p>
                <h1 class="text-3xl font-semibold">Welcome, {authSession()?.user.first_name || authSession()?.user.email || 'user'}!</h1>
                <p class="mt-2 text-sm text-base-content/70">Your session is saved locally and the dashboard is ready for your next deployment flow.</p>
              </div>
              <button class="btn btn-outline" onClick={handleLogout}>Logout</button>
            </div>
          </div>

          <div class="grid gap-6 md:grid-cols-2">
            <div class="rounded-box border border-base-300 bg-base-100 p-6 shadow-xl">
              <h2 class="text-xl font-semibold">Quick overview</h2>
              <p class="mt-3 text-sm text-base-content/70">This section can later host your applications, environments, or deployment widgets.</p>
            </div>
            <div class="rounded-box border border-base-300 bg-base-100 p-6 shadow-xl">
              <h2 class="text-xl font-semibold">Session details</h2>
              <div class="mt-3 space-y-2 text-sm text-base-content/70">
                <p>Email: {authSession()?.user.email}</p>
                <p>Role: {authSession()?.user.role || 'user'}</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </Show>
  );
}
