import { createResource, createSignal, Show, For } from 'solid-js';
import { useNavigate, useParams } from '@solidjs/router';
import { FolderOpen, Rocket, Plus, Box, Settings2 } from 'lucide-solid';
import { authSession } from '../../lib/auth';
import {
  projectControllerGet,
  environmentControllerListByProject,
  applicationControllerListByEnvironment,
} from '../../client/sdk.gen';
import type { EnvironmentResponseDto, ApplicationResponseDto } from '../../client/types.gen';
import {
  Sidebar,
  EnvDropdown,
  ServiceCard,
  CreateEnvModal,
  CreateServiceDropdown,
  CreateApplicationModal,
  ProjectEnvironmentModal,
} from '../../components';

export default function ProjectDetailPage() {
  const navigate = useNavigate();
  const params = useParams();
  const projectId = () => parseInt(params.id);

  if (!authSession()) navigate('/auth', { replace: true });

  const [selectedEnvId, setSelectedEnvId] = createSignal<number | null>(null);
  const [showCreateEnv, setShowCreateEnv] = createSignal(false);
  const [showEnvSettings, setShowEnvSettings] = createSignal(false);
  const [showCreateApp, setShowCreateApp] = createSignal(false);

  const [project] = createResource(async () => {
    const res = await projectControllerGet({ path: { id: projectId() } });
    return res.data ?? null;
  });

  const [envs, { mutate: mutateEnvs }] = createResource(async () => {
    const res = await environmentControllerListByProject({ path: { project_id: projectId() } });
    const list = (res.data as EnvironmentResponseDto[]) ?? [];
    if (!selectedEnvId()) {
      const def = list.find(e => e.is_default) ?? list[0];
      if (def) setSelectedEnvId(def.id);
    }
    return list;
  });

  const selectedEnv = () => envs()?.find(e => e.id === selectedEnvId()) ?? null;

  const [apps, { mutate: mutateApps }] = createResource(selectedEnvId, async (envId) => {
    if (!envId) return [];
    const res = await applicationControllerListByEnvironment({ path: { environment_id: envId } });
    return (res.data as ApplicationResponseDto[]) ?? [];
  });

  return (
    <div class="min-h-screen flex bg-base-100 text-base-content">
      <Sidebar />

      <div class="flex-1 flex flex-col min-w-0">
        {/* Top bar */}
        <header class="flex items-center justify-between px-6 py-3 border-b border-base-300 text-sm">
          <div class="flex items-center gap-2">
            <Rocket class="w-4 h-4 text-base-content/40" />
            <button onClick={() => navigate('/dashboard')} class="text-base-content/50 hover:text-base-content transition-colors">
              Dashboard
            </button>
            <span class="text-base-content/20">/</span>
            <button onClick={() => navigate('/projects')} class="text-base-content/50 hover:text-base-content transition-colors flex items-center gap-1">
              <FolderOpen class="w-3.5 h-3.5" />{project()?.name ?? '...'}
            </button>
            <span class="text-base-content/20">/</span>
            <EnvDropdown
              envs={envs() ?? []}
              selectedId={selectedEnvId()}
              onSelect={setSelectedEnvId}
              onCreateNew={() => setShowCreateEnv(true)}
            />
          </div>

          <div class="flex items-center gap-2">
            <button
              class="btn btn-ghost btn-sm gap-1.5 text-base-content/60"
              onClick={() => setShowEnvSettings(true)}
            >
              <Settings2 class="w-4 h-4" />
              Project Environment
            </button>
            <CreateServiceDropdown
              onSelect={(type) => {
                if (type === 'application') setShowCreateApp(true);
              }}
            />
          </div>
        </header>

        {/* Subtitle */}
        <div class="px-8 pt-5 pb-3">
          <p class="text-xs text-base-content/40">
            {selectedEnv()?.description ?? 'Production environment'}
          </p>
        </div>

        {/* Services */}
        <main class="flex-1 px-8 pb-8">
          <Show when={!apps.loading} fallback={
            <div class="flex justify-center py-20">
              <span class="loading loading-spinner loading-md text-base-content/40" />
            </div>
          }>
            <Show when={(apps() ?? []).length > 0} fallback={
              <div class="flex flex-col items-center justify-center py-20 text-base-content/30">
                <Box class="w-12 h-12 mb-3" />
                <p class="text-sm">No services yet</p>
                <button class="btn btn-outline btn-sm mt-4" onClick={() => setShowCreateApp(true)}>
                  <Plus class="w-4 h-4" /> Create your first service
                </button>
              </div>
            }>
              <div class="flex flex-wrap gap-3">
                <For each={apps()}>
                  {(app) => <ServiceCard app={app} />}
                </For>
              </div>
            </Show>
          </Show>
        </main>
      </div>

      {/* Create Environment Modal */}
      <Show when={showCreateEnv()}>
        <CreateEnvModal
          projectId={projectId()}
          onClose={() => setShowCreateEnv(false)}
          onCreated={(env) => {
            mutateEnvs(prev => [...(prev ?? []), env]);
            setSelectedEnvId(env.id);
          }}
        />
      </Show>

      {/* Project Environment Settings Modal */}
      <Show when={showEnvSettings() && selectedEnv()}>
        <ProjectEnvironmentModal
          env={selectedEnv()!}
          onClose={() => setShowEnvSettings(false)}
          onUpdated={(updated) => {
            mutateEnvs(prev => prev?.map(e => e.id === updated.id ? updated : e) ?? []);
          }}
        />
      </Show>

      {/* Create Application Modal */}
      <Show when={showCreateApp() && selectedEnvId()}>
        <CreateApplicationModal
          environmentId={selectedEnvId()!}
          onClose={() => setShowCreateApp(false)}
          onCreated={(app) => {
            mutateApps(prev => [...(prev ?? []), app]);
          }}
        />
      </Show>
    </div>
  );
}
