import { Show, createSignal, createEffect } from 'solid-js';
import { useNavigate } from '@solidjs/router';
import { Rocket, Clock } from 'lucide-solid';
import { authSession } from '../../lib/auth';
import { Sidebar } from '../../components';

export default function DashboardPage() {
  const navigate = useNavigate();
  const [dragging, setDragging] = createSignal(false);

  createEffect(() => {
    if (!authSession()) navigate('/auth', { replace: true });
  });

  const userName = () =>
    authSession()?.user.first_name ||
    authSession()?.user.email?.split('@')[0] ||
    'user';

  const stats = [
    { label: 'PROJECTS', value: '1', sub: '1 environment' },
    { label: 'SERVICES', value: '3', sub: '1 apps · 1 compose · 1 db' },
    { label: 'DEPLOYS / 7D', value: '1', sub: 'no prior data' },
  ];

  const statuses = [
    { count: 1, label: 'running', color: 'bg-success' },
    { count: 1, label: 'errored', color: 'bg-error' },
    { count: 1, label: 'idle', color: 'bg-base-content/30' },
  ];

  const deployments = [
    { name: 'nginx', sub: 'backend · production', state: 'done', time: '3 days ago' },
  ];

  return (
    <div class={`min-h-screen flex bg-base-100 text-base-content ${dragging() ? 'cursor-col-resize select-none' : ''}`}>

        <Sidebar onWidthChange={(w) => setDragging(w > 0)} />

        <div class="flex-1 flex flex-col min-w-0">
          {/* Top bar */}
          <header class="flex items-center justify-between px-6 py-3 border-b border-base-300 bg-base-100">
            <div class="flex items-center gap-2 text-sm text-base-content/50">
              <Rocket class="w-4 h-4" />
              <span>Home</span>
            </div>
            <div class="flex items-center gap-1 text-xs text-base-content/40">
              <Clock class="w-3.5 h-3.5" />
              <span>Server Time: {new Date().toLocaleTimeString('en-GB')} UTC | UTC+00:00</span>
            </div>
          </header>

          {/* Page content */}
          <main class="flex-1 p-6">
            <div class="flex items-center justify-between mb-6">
              <h1 class="text-2xl font-semibold">Welcome back, {userName()}</h1>
              <button
                class="btn btn-sm btn-neutral gap-1"
                onClick={() => navigate('/projects')}
              >
                Go to projects →
              </button>
            </div>

            {/* Stats */}
            <div class="grid grid-cols-4 gap-4 mb-6">
              {stats.map((s) => (
                <div class="bg-base-200 rounded-lg p-4 border border-base-300">
                  <p class="text-[10px] uppercase tracking-widest text-base-content/40 mb-2">{s.label}</p>
                  <p class="text-3xl font-semibold">{s.value}</p>
                  <p class="text-xs text-base-content/40 mt-1">{s.sub}</p>
                </div>
              ))}

              {/* Status card */}
              <div class="bg-base-200 rounded-lg p-4 border border-base-300">
                <p class="text-[10px] uppercase tracking-widest text-base-content/40 mb-2">STATUS</p>
                <div class="flex flex-col gap-1.5 mt-1">
                  {statuses.map((s) => (
                    <div class="flex items-center gap-2 text-sm">
                      <span class={`w-2 h-2 rounded-full shrink-0 ${s.color}`} />
                      <span class="font-medium">{s.count}</span>
                      <span class="text-base-content/50">{s.label}</span>
                    </div>
                  ))}
                </div>
              </div>
            </div>

            {/* Recent deployments */}
            <div class="bg-base-200 rounded-lg border border-base-300">
              <div class="flex items-center justify-between px-4 py-3 border-b border-base-300">
                <div class="flex items-center gap-2 text-sm font-medium">
                  <Rocket class="w-4 h-4 text-base-content/60" />
                  <span>Recent deployments</span>
                </div>
                <button class="text-xs text-base-content/40 hover:text-base-content transition-colors">
                  view all →
                </button>
              </div>

              <div class="divide-y divide-base-300">
                {deployments.map((d) => (
                  <div class="flex items-center justify-between px-4 py-3">
                    <div class="flex items-center gap-3">
                      <span class="w-2 h-2 rounded-full bg-success shrink-0" />
                      <div>
                        <p class="text-sm font-medium">{d.name}</p>
                        <p class="text-xs text-base-content/40">{d.sub}</p>
                      </div>
                    </div>
                    <div class="flex items-center gap-6 text-xs text-base-content/40">
                      <div class="flex items-center gap-1">
                        <Rocket class="w-3 h-3" />
                        <span>Rustploy</span>
                      </div>
                      <span>{d.state}</span>
                      <span>{d.time}</span>
                      <button class="hover:text-base-content transition-colors">logs →</button>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </main>
        </div>
      </div>
  );
}
