import { createSignal, onCleanup, createResource } from 'solid-js';
import { useNavigate, useLocation } from '@solidjs/router';
import {
  House, FolderOpen, Lightning, ChartLine, Calendar, Globe, Terminal,
  HardDrives, GlobeSimple, LinkSimple, Cpu, Users, FileText, Key,
  GitBranch, Database, Stack, Shield, BellRinging, Gear, BookOpen,
  SignOut, Bell, CaretDown, Package, RocketLaunch, Tag, Spinner,
} from 'phosphor-solid';
import { authSession, signOut } from '../../lib/auth';
import { organizationControllerGet } from '../../client/sdk.gen';

const navHome = [
  { label: 'Home',               icon: House,       path: '/dashboard'       },
  { label: 'Projects',           icon: FolderOpen,  path: '/projects'        },
  { label: 'Deployments',        icon: Lightning                              },
  { label: 'Monitoring',         icon: ChartLine                              },
  { label: 'Schedules',          icon: Calendar                               },
  { label: 'Traefik File System',icon: Globe                                  },
  { label: 'Docker',             icon: Package                                },
  { label: 'Swarm',              icon: GlobeSimple                            },
  { label: 'Requests',           icon: LinkSimple                             },
];

const navSettings = [
  { label: 'Web Server',         icon: Cpu                                    },
  { label: 'Profile',            icon: Users                                  },
  { label: 'Remote Servers',     icon: HardDrives,  path: '/remote-servers'  },
  { label: 'Deployments',        icon: Lightning                              },
  { label: 'Users',              icon: Users                                  },
  { label: 'Audit Logs',         icon: FileText                               },
  { label: 'SSH Keys',           icon: Key,         path: '/ssh-keys'        },
  { label: 'AI',                 icon: Terminal                               },
  { label: 'Tags',               icon: Tag                                    },
  { label: 'Git',                icon: GitBranch                              },
  { label: 'Registry',           icon: Database                               },
  { label: 'S3 Destinations',    icon: Stack                                  },
  { label: 'Certificates',       icon: Shield                                 },
  { label: 'Cluster',            icon: GlobeSimple                            },
  { label: 'Notifications',      icon: BellRinging                            },
  { label: 'License',            icon: FileText                               },
  { label: 'SSO',                icon: LinkSimple                             },
  { label: 'Whitelabeling',      icon: Gear                                   },
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

  const [org] = createResource(async () => {
    const s = session();
    if (!s) return null;
    const res = await organizationControllerGet({ path: { id: s.user.group_id } });
    return res.data ?? null;
  });

  const orgName = () => org()?.name ?? '…';
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

  const navItemCls = (path?: string) =>
    `w-full flex items-center gap-2 px-2 py-1.5 rounded-md text-sm transition-all duration-150 text-left outline-none ${
      isActive(path)
        ? 'bg-base-300 text-base-content font-medium'
        : 'text-base-content/55 hover:bg-base-200 hover:text-base-content'
    }`;

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
          <span class="truncate">
            {org.loading ? <Spinner size={12} class="animate-spin opacity-40" /> : orgName()}
          </span>
        </div>
        <div class="flex gap-1 text-base-content/40">
          <CaretDown size={14} />
          <Bell size={14} />
        </div>
      </div>

      {/* Nav */}
      <nav class="flex-1 overflow-y-auto py-2 px-2 space-y-0.5">
        <p class="text-[10px] uppercase tracking-widest text-base-content/30 px-2 pt-2 pb-1">Home</p>
        {navHome.map((item, i) => (
          <button
            class={navItemCls(item.path)}
            style={{ 'animation-delay': `${i * 20}ms` }}
            onClick={() => item.path && navigate(item.path)}
          >
            <item.icon size={14} weight={isActive(item.path) ? 'fill' : 'regular'} class="shrink-0" />
            <span class="truncate">{item.label}</span>
          </button>
        ))}

        <p class="text-[10px] uppercase tracking-widest text-base-content/30 px-2 pt-4 pb-1">Settings</p>
        {navSettings.map((item, i) => (
          <button
            class={navItemCls(item.path)}
            style={{ 'animation-delay': `${(navHome.length + i) * 20}ms` }}
            onClick={() => item.path && navigate(item.path)}
          >
            <item.icon size={14} weight={isActive(item.path) ? 'fill' : 'regular'} class="shrink-0" />
            <span class="truncate">{item.label}</span>
          </button>
        ))}

        <p class="text-[10px] uppercase tracking-widest text-base-content/30 px-2 pt-4 pb-1">Extra</p>
        <button class={navItemCls()}>
          <BookOpen size={14} class="shrink-0" />
          <span>Documentation</span>
        </button>
      </nav>

      {/* User footer */}
      <div class="border-t border-base-300 px-3 py-2.5">
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
          <button
            onClick={handleLogout}
            class="text-base-content/40 hover:text-error transition-colors ml-1 p-1 rounded hover:bg-error/10"
            title="Logout"
          >
            <SignOut size={14} />
          </button>
        </div>
        <p class="text-[10px] text-base-content/20 mt-1.5">v0.1.0</p>
      </div>

      {/* Drag handle */}
      <div
        onMouseDown={onDragStart}
        class={`absolute top-0 right-0 h-full w-1 cursor-col-resize z-10 transition-colors hover:bg-primary/40 ${dragging() ? 'bg-primary/60' : ''}`}
      />
    </aside>
  );
}
