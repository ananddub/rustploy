import { lazy } from 'solid-js';
import type { RouteDefinition } from '@solidjs/router';

import Home from './pages/home';
import AuthPage from './pages/auth';
import DashboardPage from './pages/dashboard';

export const routes: RouteDefinition[] = [
  {
    path: '/',
    component: Home,
  },
  {
    path: '/auth',
    component: AuthPage,
  },
  {
    path: '/dashboard',
    component: DashboardPage,
  },
  {
    path: '/projects',
    component: lazy(() => import('./pages/projects')),
  },
  {
    path: '/projects/:id',
    component: lazy(() => import('./pages/projects/detail')),
  },
  {
    path: '/projects/:id/env/:envId',
    component: lazy(() => import('./pages/projects/env')),
  },
  {
    path: '/projects/:projectId/app/:appId',
    component: lazy(() => import('./pages/applications')),
  },
  {
    path: '/projects/:projectId/compose/:composeId',
    component: lazy(() => import('./pages/compose')),
  },
  {
    path: '/remote-servers',
    component: lazy(() => import('./pages/remote-servers')),
  },
  {
    path: '/ssh-keys',
    component: lazy(() => import('./pages/ssh-keys')),
  },
  {
    path: '/about',
    component: lazy(() => import('./pages/about')),
  },
  {
    path: '**',
    component: lazy(() => import('./errors/404')),
  },
];
