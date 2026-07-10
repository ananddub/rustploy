import { createSignal, onCleanup, createResource } from 'solid-js';
import { useNavigate, useLocation } from '@solidjs/router';
import {
  Home, FolderOpen, Zap, Activity, Calendar, Globe, Network, Link2,
  Server, Users, FileText, FileKey, Bot, Tag, GitBranch,
  Database, Layers, Shield, BellRing, Settings, BookOpen,
  LogOut, Bell, ChevronDown, Container, Rocket,
} from 'lucide-solid';
import { authSession, signOut } from '../../lib/auth';
import { organizationControllerGet } from '../../client/sdk.gen';

const navHome = [
  { label: 'Home', icon: Home, path: '/dashboard' },
  { label: 'Projects', icon: FolderOpen, path: '/projects' },
  { label: 'Deployments', icon: Zap },
  { label: 'Monitoring', icon: Activity },
  { label: 'Schedules', icon: Calendar },
  { label: 'Traefik File System', icon: Globe },
  { label: 'Docker', icon: Container },
  { label: 'Swarm', icon: Network },
  { label: 'Requests', icon: Link2 },
];

const navSettings = [
  { label: 'Web Server', icon: Server },
  { label: 'Profile', icon: Users },
  { label: 'Remote Servers', icon: Globe },
  { label: 'Deployments', icon: Zap },
  { label: 'Users', icon: Users },
  { label: 'Audit Logs', icon: FileText },
  { label: 'SSH Keys', icon: FileKey },
  { label: 'AI', icon: Bot },
  { label: 'Tags', icon: Tag },
  { label: 'Git', icon: GitBranch },
  { label: 'Registry', icon: Database },
  { label: 'S3 Destinations', icon: Layers },
  { label: 'Certificates', icon: Shield },
  { label: 'Cluster', icon: Network },
  { label: 'Notifications', icon: BellRing },
  { label: 'License', icon: FileText },
  { label: 'SSO', icon: Link2 },
  { label: 'Whitelabeling', icon: Settings },
];

type Props = {
  onWidthChange?: (w: number) => void;
};

export default function Sidebar(props: Props) {
  const navigate = useNavigate();
  const location = useLocation();
  const [width, setWidth] = createSignal(192);
  const [dragging, setDragging] = createSignal(false);

  const session = () => authSession();
  const userName = () => session()?.user.first_name || session()?.user.email?.split('@')[0] || 'user';
  const userEmail = () => session()?.user.email || '';
  const initials = () => userName().slice(0, 2).toUpperCase();

  // Fetch org
  const [org] = createResource(async () => {
    const s = session();
    if (!s) return null;
    const res = await organizationControllerGet({
      path: { id: s.user.group_id },
    });
    return res.data ?? null;
  });

  const orgName = () => org()?.name ?? '...';
  const orgInitial = () => orgName().slice(0, 1).toUpperCase();

  const handleLogout = async () => {
    await signOut();
    navigate('/auth', { replace: true });
  };

  const onDragStart = (e: MouseEvent) => {
    e.preventDefault();
    setDragging(true);
    const onMove = (e: MouseEvent) => {
      const w = Math.min(320, Math.max(160, e.clientX));
      setWidth(w);
      props.onWidthChange?.(w);
    };
    const onUp = () => {
      setDragging(false);
      window.removeEventListener('mousemove', onMove);
      window.removeEventListener('mouseup', onUp);
    };
    window.addEventListener('mousemove', onMove);
    window.addEventListener('mouseup', onUp);
  };

  onCleanup(() => setDragging(false));

  const isActive = (path?: string) => {
    if (!path) return false;
    if (path === '/dashboard') return location.pathname === '/dashboard';
    return location.pathname.startsWith(path);
  };

  return (
    <aside
      style={{ width: `${width()}px`, 'min-width': `${width()}px` }}
      class="shrink-0 flex flex-col bg-base-100 border-r border-base-300 h-screen sticky top-0 relative"
    >
      {/* Org header */}
      <div class="flex items-center justify-between px-3 py-3 border-b border-base-300">
        <div class="flex items-center gap-2 text-sm font-medium truncate">
          <div class="w-5 h-5 rounded bg-primary/20 flex items-center justify-center text-primary text-xs font-bold shrink-0">
            {orgInitial()}
          </div>
          <span class="truncate">{orgName()}</span>
        </div>
        <div class="flex gap-1 text-base-content/40">
          <ChevronDown class="w-3.5 h-3.5" />
          <Bell class="w-3.5 h-3.5" />
        </div>
      </div>

      {/* Nav */}
      <nav class="flex-1 overflow-y-auto py-2 px-2 space-y-0.5">
        <p class="text-[10px] uppercase tracking-widest text-base-content/30 px-2 pt-2 pb-1">Home</p>
        {navHome.map((item) => (
          <button
            class={`w-full flex items-center gap-2 px-2 py-1.5 rounded text-sm transition-colors text-left
              ${isActive(item.path)
                ? 'bg-base-300 text-base-content font-medium'
                : 'text-base-content/60 hover:bg-base-200 hover:text-base-content'}`}
            onClick={() => item.path && navigate(item.path)}
          >
            <item.icon class="w-3.5 h-3.5 shrink-0" />
            <span class="truncate">{item.label}</span>
          </button>
        ))}

        <p class="text-[10px] uppercase tracking-widest text-base-content/30 px-2 pt-4 pb-1">Settings</p>
        {navSettings.map((item) => (
          <button class="w-full flex items-center gap-2 px-2 py-1.5 rounded text-sm text-base-content/60 hover:bg-base-200 hover:text-base-content transition-colors text-left">
            <item.icon class="w-3.5 h-3.5 shrink-0" />
            <span class="truncate">{item.label}</span>
          </button>
        ))}

        <p class="text-[10px] uppercase tracking-widest text-base-content/30 px-2 pt-4 pb-1">Extra</p>
        <button class="w-full flex items-center gap-2 px-2 py-1.5 rounded text-sm text-base-content/60 hover:bg-base-200 hover:text-base-content transition-colors text-left">
          <BookOpen class="w-3.5 h-3.5 shrink-0" />
          <span>Documentation</span>
        </button>
      </nav>

      {/* User footer */}
      <div class="border-t border-base-300 px-3 py-2">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2 min-w-0">
            <div class="w-7 h-7 rounded-full bg-primary text-primary-content flex items-center justify-center text-xs font-bold shrink-0">
              {initials()}
            </div>
            <div class="min-w-0">
              <p class="text-xs font-medium truncate">Account</p>
              <p class="text-[10px] text-base-content/40 truncate">{userEmail()}</p>
            </div>
          </div>
          <button onClick={handleLogout} class="text-base-content/40 hover:text-error transition-colors ml-1" title="Logout">
            <LogOut class="w-3.5 h-3.5" />
          </button>
        </div>
        <p class="text-[10px] text-base-content/20 mt-1">Version v0.1.0</p>
      </div>

      {/* Drag handle */}
      <div
        onMouseDown={onDragStart}
        class={`absolute top-0 right-0 h-full w-1 cursor-col-resize z-10 hover:bg-primary/50 transition-colors ${dragging() ? 'bg-primary/70' : ''}`}
      />
    </aside>
  );
}
