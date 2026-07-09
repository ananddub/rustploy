import { lazy } from 'solid-js';
import type { RouteDefinition } from '@solidjs/router';

import Home from './pages/home';
import AuthPage from './pages/auth';
import DashboardPage from './pages/dashboard';
import AboutData from './pages/about.data';

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
    path: '/about',
    component: lazy(() => import('./pages/about')),
    data: AboutData,
  },
  {
    path: '**',
    component: lazy(() => import('./errors/404')),
  },
];
