import { createResource, Show, For } from 'solid-js';
import { useNavigate, useParams } from '@solidjs/router';
import { FolderOpen, Rocket, Layers, Plus, Box, ChevronRight } from 'lucide-solid';
import { authSession } from '../../../lib/auth';
import {
  projectControllerGet,
  environmentControllerGet,
  applicationControllerListByEnvironment,
} from '../../../client/sdk.gen';
import type { ApplicationResponseDto } from '../../../client/types.gen';
import { Sidebar } from '../../../components';

const statusColor = (status: string) => {
  switch (status?.toLowerCase()) {
    case 'running': return 'bg-success';
    case 'stopped':
    case 'exited': return 'bg-error';
    case 'idle': return 'bg-base-content/30';
    default: return 'bg-warning';
  }
};

export default function EnvDetailPage() {
  const navigate = useNavigate();
  const params = useParams();
  const projectId = () => parseInt(params.id);
  const envId = () => parseInt(params.envId);

  if (!authSession()) navigate('/auth', { replace: true });

  const [project] = createResource(async () => {
    const res = await projectControllerGet({ path: { id: projectId() } });
    return res.data ?? null;
  });

  const [env] = createResource(async () => {
    const res = await environmentControllerGet({ path: { id: envId() } });
    return res.data ?? null;
  });

  const [apps] = createResource(async () => {
    const res = await applicationControllerListByEnvironment({
      path: { environment_id: envId() },
    });
    return (res.data as ApplicationResponseDto[]) ?? [];
  });

  const formatDate = (ts: number) =>
    new Date(ts * 1000).toLocaleDateString('en-IN', {
      day: '2-digit', month: 'short', year: 'numeric',
    });

  return (
    <div class="min-h-screen flex bg-base-100 text-base-content">
      <Sidebar />

      <div class="flex-1 flex flex-col min-w-0">
        {/* Breadcrumb */}
        <header class="flex items-center gap-2 px-6 py-3 border-b border-base-300 text-sm flex-wrap">
          <Rocket class="w-4 h-4 text-base-content/40" />
          <button onClick={() => navigate('/dashboard')} class="text-base-content/50 hover:text-base-content transition-colors">Dashboard</button>
          <span class="text-base-content/20">/</span>
          <button onClick={() => navigate('/projects')} class="text-base-content/50 hover:text-base-content transition-colors flex items-center gap-1">
            <FolderOpen class="w-3.5 h-3.5" /> Projects
          </button>
          <span class="text-base-content/20">/</span>
          <button onClick={() => navigate(`/projects/${projectId()}`)} class="text-base-content/50 hover:text-base-content transition-colors flex items-center gap-1">
            <FolderOpen class="w-3.5 h-3.5" /> {project()?.name ?? '...'}
          </button>
          <span class="text-base-content/20">/</span>
          <span class="font-medium text-base-content flex items-center gap-1">
            <Layers class="w-3.5 h-3.5" /> {env()?.name ?? '...'}
          </span>
        </header>

        {/* Content */}
        <main class="flex-1 px-8 py-8">
          <div class="flex items-center justify-between mb-8">
            <div>
              <div class="flex items-center gap-2">
                <h1 class="text-2xl font-semibold">{env()?.name ?? '...'}</h1>
                <Show when={env()?.is_default}>
                  <span class="badge badge-neutral badge-sm">default</span>
                </Show>
              </div>
              <p class="text-sm text-base-content/40 mt-1">
                {env()?.description ?? project()?.name ?? ''}
              </p>
            </div>
            <button class="btn btn-neutral btn-sm gap-2">
              <Plus class="w-4 h-4" /> Create Service
            </button>
          </div>

          {/* Services */}
          <div class="mb-4">
            <p class="text-sm font-medium text-base-content/60 uppercase tracking-widest">Services</p>
          </div>

          <Show when={!apps.loading} fallback={
            <div class="flex justify-center py-20">
              <span class="loading loading-spinner loading-md text-base-content/40" />
            </div>
          }>
            <Show when={(apps() ?? []).length > 0} fallback={
              <div class="flex flex-col items-center justify-center py-20 text-base-content/30">
                <Box class="w-12 h-12 mb-3" />
                <p class="text-sm">No services yet</p>
                <button class="btn btn-outline btn-sm mt-4">
                  <Plus class="w-4 h-4" /> Create your first service
                </button>
              </div>
            }>
              <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-3">
                <For each={apps()}>
                  {(app) => (
                    <div class="bg-base-200 border border-base-300 rounded-lg p-4 flex flex-col gap-3 hover:border-base-content/20 transition-colors cursor-pointer group">
                      <div class="flex items-start justify-between">
                        <div class="flex items-center gap-3 min-w-0">
                          <div class="w-9 h-9 rounded-lg bg-primary/10 flex items-center justify-center shrink-0 relative">
                            <Box class="w-4 h-4 text-primary" />
                            <span class={`absolute -top-0.5 -right-0.5 w-2.5 h-2.5 rounded-full border-2 border-base-200 ${statusColor(app.app_status)}`} />
                          </div>
                          <div class="min-w-0">
                            <p class="font-medium text-sm truncate group-hover:text-primary transition-colors">{app.app_name}</p>
                            <p class="text-xs text-base-content/40 truncate">{app.build_type}</p>
                          </div>
                        </div>
                      </div>

                      <div class="flex items-center justify-between pt-2 border-t border-base-300">
                        <span class={`text-xs flex items-center gap-1.5`}>
                          <span class={`w-1.5 h-1.5 rounded-full ${statusColor(app.app_status)}`} />
                          <span class="text-base-content/50">{app.app_status ?? 'unknown'}</span>
                        </span>
                        <span class="text-xs text-base-content/30">{formatDate(app.created_at)}</span>
                      </div>
                    </div>
                  )}
                </For>
              </div>
            </Show>
          </Show>
        </main>
      </div>
    </div>
  );
}
