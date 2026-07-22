/**
 * Rustploy Mock Data Layer & Provider Switch
 * 
 * Set USE_MOCK_DATA = true to load rich realistic mock data during development/demo mode.
 * Set USE_MOCK_DATA = false to seamlessly connect to the Rust backend API SDK.
 */

import { MOCK_PROJECTS } from './projects.mock';
import type { ProjectMock } from './types.mock';

export const USE_MOCK_DATA = true;

export function getProjectsMock(): ProjectMock[] {
	return MOCK_PROJECTS;
}

export function getProjectByIdMock(id: string): ProjectMock | undefined {
	return MOCK_PROJECTS.find((p) => p.id === id);
}

export * from './types.mock';
export * from './projects.mock';
