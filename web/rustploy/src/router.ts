// Generouted, changes to this file will be overridden
/* eslint-disable */

import { components, hooks, utils } from '@generouted/react-router/client'

export type Path =
  | `/`
  | `/auth`
  | `/dashboard`
  | `/deployments`
  | `/docker`
  | `/monitoring`
  | `/projects`
  | `/projects/:id`
  | `/projects/:id/app/:appId`
  | `/projects/:id/compose/:composeId`
  | `/projects/:id/database/:kind/:dbId`
  | `/remote-servers`
  | `/schedules`
  | `/settings`
  | `/settings/ai`
  | `/settings/audit-logs`
  | `/settings/certificates`
  | `/settings/cluster`
  | `/settings/destinations`
  | `/settings/git-providers`
  | `/settings/notifications`
  | `/settings/profile`
  | `/settings/registry`
  | `/settings/server`
  | `/settings/tags`
  | `/settings/users`
  | `/ssh-keys`
  | `/swarm`
  | `/traefik`

export type Params = {
  '/projects/:id': { id: string }
  '/projects/:id/app/:appId': { id: string; appId: string }
  '/projects/:id/compose/:composeId': { id: string; composeId: string }
  '/projects/:id/database/:kind/:dbId': { id: string; kind: string; dbId: string }
}

export type ModalPath = never

export const { Link, Navigate } = components<Path, Params>()
export const { useModals, useNavigate, useParams } = hooks<Path, Params, ModalPath>()
export const { redirect } = utils<Path, Params>()
