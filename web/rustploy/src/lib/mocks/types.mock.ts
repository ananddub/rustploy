/**
 * Mock Data Type Definitions matching Rustploy OpenAPI / Backend DTO Specifications
 */

export interface ServiceSummaryMock {
	id: string;
	name: string;
	type: 'application' | 'compose' | 'database';
	status: 'running' | 'stopped' | 'error' | 'deploying';
	updatedAt: string;
	subDetails?: string; // e.g. "PostgreSQL 16", "Docker Compose", "Next.js 14"
}

export interface EnvironmentMock {
	id: string;
	name: string;
	slug: string;
	isProduction: boolean;
	servicesCount: number;
}

export interface ProjectMock {
	id: string;
	name: string;
	description: string;
	tags: string[];
	environments: EnvironmentMock[];
	services: ServiceSummaryMock[];
	appsCount: number;
	composeCount: number;
	databaseCount: number;
	healthStatus: 'healthy' | 'warning' | 'error' | 'deploying';
	gitRepo?: string;
	gitBranch?: string;
	lastCommitMsg?: string;
	lastCommitHash?: string;
	updatedAt: string;
	createdAt: string;
}
