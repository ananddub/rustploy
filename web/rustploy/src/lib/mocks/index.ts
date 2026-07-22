/**
 * Rustploy Universal Mock Data System
 * 
 * Set USE_MOCK_DATA = true for full UI demo mode across all pages.
 * Set USE_MOCK_DATA = false to route calls to the live Rust backend.
 */

import { MOCK_PROJECTS } from './projects.mock';
import { MOCK_DEPLOYMENTS } from './deployments.mock';
import { MOCK_SERVERS } from './servers.mock';
import { MOCK_SCHEDULES } from './schedules.mock';
import { MOCK_TRAEFIK_CONFIGS } from './traefik.mock';
import { MOCK_DOCKER_CONTAINERS } from './docker.mock';
import { MOCK_SWARM_NODES, MOCK_SSH_KEYS } from './swarm.mock';

export const USE_MOCK_DATA = true;

// Getters
export function getProjectsMock() { return MOCK_PROJECTS; }
export function getDeploymentsMock() { return MOCK_DEPLOYMENTS; }
export function getServersMock() { return MOCK_SERVERS; }
export function getSchedulesMock() { return MOCK_SCHEDULES; }
export function getTraefikConfigsMock() { return MOCK_TRAEFIK_CONFIGS; }
export function getDockerContainersMock() { return MOCK_DOCKER_CONTAINERS; }
export function getSwarmNodesMock() { return MOCK_SWARM_NODES; }
export function getSshKeysMock() { return MOCK_SSH_KEYS; }

export * from './types.mock';
export * from './projects.mock';
export * from './deployments.mock';
export * from './servers.mock';
export * from './schedules.mock';
export * from './traefik.mock';
export * from './docker.mock';
export * from './swarm.mock';
