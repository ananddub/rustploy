import { createSignal, createResource, Show, For } from 'solid-js';
import { useNavigate } from '@solidjs/router';
import { Plus, FolderOpen, Rocket } from 'lucide-solid';
import { authSession } from '../../lib/auth';
import {
  projectControllerListByOrganization,
  projectControllerDelete,
  projectControllerPatch,
} from '../../client/sdk.gen';
import type { ProjectResponseDto } from '../../client/types.gen';
import { Sidebar } from '../../components';
import ProjectCard from '../../components/projects/ProjectCard';
import CreateProjectModal from './CreateProjectModal';

export default function ProjectsPage() {
  const navigate = useNavigate();
  const [showCreate, setShowCreate] = createSignal(false);
  const [editingId, setEditingId] = createSignal<number | null>(null);
  const [editName, setEditName] = createSignal('');
  const [editDesc, setEditDesc] = createSignal('');
  const [deletingId, setDeletingId] = createSignal<number | null>(null);
  const [saving, setSaving] = createSignal(false);

  const session = () => authSession();
  if (!session()) navigate('/auth', { replace: true });

  const [projects, { mutate, refetch }] = createResource(async () => {
    const s = session();
    if (!s) return [];
    const res = await projectControllerListByOrganization({
      path: { organization_id: s.user.group_id },
    });
    return (res.data as ProjectResponseDto[]) ?? [];
  });

  const deleteProject = async (id: number) => {
    setDeletingId(id);
    try {
      await projectControllerDelete({ path: { id } });
      mutate(prev => prev?.filter(p => p.id !== id) ?? []);
    } finally {
      setDeletingId(null);
    }
  };

  const startEdit = (p: ProjectResponseDto) => {
    setEditingId(p.id);
    setEditName(p.name);
    setEditDesc(p.description ?? '');
  };

  const saveEdit = async (id: number) => {
    setSaving(true);
    try {
      const res = await projectControllerPatch({
        path: { id },
        body: { name: editName(), description: editDesc() || undefined },
      });
      if (res.data) mutate(prev => prev?.map(p => p.id === id ? res.data! : p) ?? []);
      setEditingId(null);
    } finally {
      setSaving(false);
    }
  };

  return (
    <>
      <div class="min-h-screen flex bg-base-100 text-base-content">
        <Sidebar />

        <div class="flex-1 flex flex-col min-w-0">
          {/* Breadcrumb */}
          <header class="flex items-center gap-2 px-6 py-3 border-b border-base-300 text-sm">
            <Rocket class="w-4 h-4 text-base-content/40" />
            <button onClick={() => navigate('/dashboard')} class="text-base-content/50 hover:text-base-content transition-colors">
              Dashboard
            </button>
            <span class="text-base-content/20">/</span>
            <span class="font-medium flex items-center gap-1.5">
              <FolderOpen class="w-4 h-4" /> Projects
            </span>
          </header>

          <main class="flex-1 px-8 py-8">
            <div class="flex items-center justify-between mb-6">
              <div>
                <h1 class="text-2xl font-semibold">Projects</h1>
                <p class="text-sm text-base-content/40 mt-1">Manage your organization's projects</p>
              </div>
              <button class="btn btn-neutral btn-sm gap-2" onClick={() => setShowCreate(true)}>
                <Plus class="w-4 h-4" /> New Project
              </button>
            </div>

            <Show when={!projects.loading} fallback={
              <div class="flex justify-center py-20">
                <span class="loading loading-spinner loading-md text-base-content/40" />
              </div>
            }>
              <Show when={(projects() ?? []).length > 0} fallback={
                <div class="flex flex-col items-center justify-center py-20 text-base-content/30">
                  <FolderOpen class="w-12 h-12 mb-3" />
                  <p class="text-sm">No projects yet</p>
                  <button class="btn btn-outline btn-sm mt-4" onClick={() => setShowCreate(true)}>
                    Create your first project
                  </button>
                </div>
              }>
                <div class="flex flex-col gap-2">
                  <For each={projects()}>
                    {(project) => (
                      <ProjectCard
                        project={project}
                        editingId={editingId()}
                        deletingId={deletingId()}
                        saving={saving()}
                        editName={editName()}
                        editDesc={editDesc()}
                        onNavigate={() => navigate(`/projects/${project.id}`)}
                        onStartEdit={() => startEdit(project)}
                        onSaveEdit={() => saveEdit(project.id)}
                        onCancelEdit={() => setEditingId(null)}
                        onDelete={() => deleteProject(project.id)}
                        onEditName={setEditName}
                        onEditDesc={setEditDesc}
                      />
                    )}
                  </For>
                </div>
              </Show>
            </Show>
          </main>
        </div>
      </div>

      <Show when={showCreate()}>
        <CreateProjectModal
          onClose={() => setShowCreate(false)}
          onCreated={(p) => { mutate(prev => [...(prev ?? []), p]); refetch(); }}
        />
      </Show>
    </>
  );
}
