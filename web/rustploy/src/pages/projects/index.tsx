import { createSignal, createResource, Show, For } from 'solid-js';
import { useNavigate } from '@solidjs/router';
import { Plus, Trash2, Pencil, Check, X, FolderOpen, Rocket } from 'lucide-solid';
import { authSession } from '../../lib/auth';
import {
  projectControllerListByOrganization,
  projectControllerDelete,
  projectControllerPatch,
} from '../../client/sdk.gen';
import type { ProjectResponseDto } from '../../client/types.gen';
import { Sidebar } from '../../components';
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
      mutate((prev) => prev?.filter((p) => p.id !== id) ?? []);
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
      if (res.data) mutate((prev) => prev?.map((p) => (p.id === id ? res.data! : p)) ?? []);
      setEditingId(null);
    } finally {
      setSaving(false);
    }
  };

  const formatDate = (ts: number) =>
    new Date(ts * 1000).toLocaleDateString('en-IN', { day: '2-digit', month: 'short', year: 'numeric' });

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

          {/* Content */}
          <main class="flex-1 px-8 py-8">
            <div class="flex items-center justify-between mb-8">
              <div>
                <h1 class="text-2xl font-semibold">Projects</h1>
                <p class="text-sm text-base-content/40 mt-1">Manage your organization's projects</p>
              </div>
              <button class="btn btn-neutral btn-sm gap-2" onClick={() => setShowCreate(true)}>
                <Plus class="w-4 h-4" /> New Project
              </button>
            </div>

            <Show when={!projects.loading} fallback={
              <div class="flex justify-center py-32">
                <span class="loading loading-spinner loading-md text-base-content/40" />
              </div>
            }>
              <Show when={(projects() ?? []).length > 0} fallback={
                <div class="flex flex-col items-center justify-center py-32 gap-3 text-base-content/30">
                  <FolderOpen class="w-16 h-16" />
                  <p class="text-base">No projects yet</p>
                  <button class="btn btn-outline btn-sm mt-2" onClick={() => setShowCreate(true)}>
                    Create your first project
                  </button>
                </div>
              }>
                <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
                  <For each={projects()}>
                    {(project) => (
                      <div class="bg-base-200 border border-base-300 rounded-lg p-4 flex flex-col gap-3 hover:border-base-content/20 transition-colors">
                        <div class="flex items-start justify-between gap-2">
                          <div class="flex items-center gap-3 min-w-0">
                            <div class="w-9 h-9 rounded-lg bg-primary/10 flex items-center justify-center shrink-0">
                              <FolderOpen class="w-4 h-4 text-primary" />
                            </div>
                            <Show when={editingId() === project.id} fallback={
                              <div class="min-w-0">
                                <p class="font-medium text-sm truncate">{project.name}</p>
                                <p class="text-xs text-base-content/40 truncate mt-0.5">
                                  {project.description ?? <span class="italic">No description</span>}
                                </p>
                              </div>
                            }>
                              <div class="flex flex-col gap-1.5 flex-1">
                                <input class="input input-bordered input-xs w-full" value={editName()} onInput={(e) => setEditName(e.currentTarget.value)} placeholder="Name" />
                                <input class="input input-bordered input-xs w-full" value={editDesc()} onInput={(e) => setEditDesc(e.currentTarget.value)} placeholder="Description" />
                              </div>
                            </Show>
                          </div>

                          <div class="flex items-center gap-1 shrink-0">
                            <Show when={editingId() === project.id} fallback={
                              <>
                                <button class="btn btn-ghost btn-xs text-base-content/40 hover:text-base-content" onClick={() => startEdit(project)}>
                                  <Pencil class="w-3.5 h-3.5" />
                                </button>
                                <button class="btn btn-ghost btn-xs text-base-content/40 hover:text-error" onClick={() => deleteProject(project.id)} disabled={deletingId() === project.id}>
                                  <Show when={deletingId() === project.id} fallback={<Trash2 class="w-3.5 h-3.5" />}>
                                    <span class="loading loading-spinner loading-xs" />
                                  </Show>
                                </button>
                              </>
                            }>
                              <button class="btn btn-ghost btn-xs text-success" onClick={() => saveEdit(project.id)} disabled={saving()}>
                                <Show when={saving()} fallback={<Check class="w-3.5 h-3.5" />}>
                                  <span class="loading loading-spinner loading-xs" />
                                </Show>
                              </button>
                              <button class="btn btn-ghost btn-xs text-base-content/40 hover:text-error" onClick={() => setEditingId(null)}>
                                <X class="w-3.5 h-3.5" />
                              </button>
                            </Show>
                          </div>
                        </div>

                        <div class="flex items-center justify-between pt-1 border-t border-base-300">
                          <span class="text-xs font-mono text-base-content/30">{project.env_var}</span>
                          <span class="text-xs text-base-content/30">{formatDate(project.created_at)}</span>
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

      <Show when={showCreate()}>
        <CreateProjectModal
          onClose={() => setShowCreate(false)}
          onCreated={(p) => { mutate((prev) => [...(prev ?? []), p]); refetch(); }}
        />
      </Show>
    </>
  );
}
