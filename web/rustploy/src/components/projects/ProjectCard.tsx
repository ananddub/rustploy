import { Show } from 'solid-js';
import { FolderOpen, Pencil, Trash2, Check, X } from 'lucide-solid';
import type { ProjectResponseDto } from '../../client/types.gen';

type Props = {
  project: ProjectResponseDto;
  editingId: number | null;
  deletingId: number | null;
  saving: boolean;
  editName: string;
  editDesc: string;
  onNavigate: () => void;
  onStartEdit: () => void;
  onSaveEdit: () => void;
  onCancelEdit: () => void;
  onDelete: () => void;
  onEditName: (v: string) => void;
  onEditDesc: (v: string) => void;
};

const formatDate = (ts: number) =>
  new Date(ts * 1000).toLocaleDateString('en-IN', {
    day: '2-digit', month: 'short', year: 'numeric',
  });

export default function ProjectCard(props: Props) {
  const isEditing = () => props.editingId === props.project.id;
  const isDeleting = () => props.deletingId === props.project.id;

  return (
    <div
      class="bg-base-200 border border-base-300 rounded-lg px-4 py-3 flex items-center gap-4 hover:border-base-content/20 transition-colors cursor-pointer group"
      onClick={() => { if (!isEditing()) props.onNavigate(); }}
    >
      <div class="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center shrink-0">
        <FolderOpen class="w-4 h-4 text-primary" />
      </div>

      <div class="flex-1 min-w-0">
        <Show when={isEditing()} fallback={
          <>
            <p class="font-medium text-sm group-hover:text-primary transition-colors">{props.project.name}</p>
            <p class="text-xs text-base-content/40 mt-0.5">
              {props.project.description ?? <span class="italic">No description</span>}
            </p>
          </>
        }>
          <div class="flex flex-col gap-1.5" onClick={(e) => e.stopPropagation()}>
            <input class="input input-bordered input-sm max-w-xs" value={props.editName} onInput={(e) => props.onEditName(e.currentTarget.value)} placeholder="Name" />
            <input class="input input-bordered input-sm max-w-sm" value={props.editDesc} onInput={(e) => props.onEditDesc(e.currentTarget.value)} placeholder="Description" />
          </div>
        </Show>
      </div>

      <Show when={!isEditing()}>
        <div class="text-right hidden sm:block shrink-0">
          <p class="text-xs font-mono text-base-content/30">{props.project.env_var}</p>
          <p class="text-xs text-base-content/30 mt-0.5">{formatDate(props.project.created_at)}</p>
        </div>
      </Show>

      <div class="flex items-center gap-1 shrink-0" onClick={(e) => e.stopPropagation()}>
        <Show when={isEditing()} fallback={
          <>
            <button class="btn btn-ghost btn-xs text-base-content/40 hover:text-base-content" onClick={props.onStartEdit}>
              <Pencil class="w-3.5 h-3.5" />
            </button>
            <button class="btn btn-ghost btn-xs text-base-content/40 hover:text-error" onClick={props.onDelete} disabled={isDeleting()}>
              <Show when={isDeleting()} fallback={<Trash2 class="w-3.5 h-3.5" />}>
                <span class="loading loading-spinner loading-xs" />
              </Show>
            </button>
          </>
        }>
          <button class="btn btn-ghost btn-xs text-success" onClick={props.onSaveEdit} disabled={props.saving}>
            <Show when={props.saving} fallback={<Check class="w-3.5 h-3.5" />}>
              <span class="loading loading-spinner loading-xs" />
            </Show>
          </button>
          <button class="btn btn-ghost btn-xs text-base-content/40 hover:text-error" onClick={props.onCancelEdit}>
            <X class="w-3.5 h-3.5" />
          </button>
        </Show>
      </div>
    </div>
  );
}
