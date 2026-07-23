import type { ProjectMock } from './types.mock';

export const MOCK_PROJECTS: ProjectMock[] = [
	{
		id: 'proj-01',
		name: 'Rustploy Core Backend',
		description: 'Main deployment engine and REST API built with Rust, Actix-Web, and Traefik routing.',
		tags: ['Rust', 'Backend', 'Production'],
		environments: [
			{ id: 'env-01', name: 'Production', slug: 'production', isProduction: true, servicesCount: 3 },
			{ id: 'env-02', name: 'Staging', slug: 'staging', isProduction: false, servicesCount: 2 }
		],
		services: [
			{ id: 'srv-01', name: 'api-server', type: 'application', status: 'running', updatedAt: '2m ago', subDetails: 'Rust Actix v0.29' },
			{ id: 'srv-02', name: 'postgres-db', type: 'database', status: 'running', updatedAt: '1h ago', subDetails: 'PostgreSQL 16' },
			{ id: 'srv-03', name: 'redis-cache', type: 'database', status: 'running', updatedAt: '3h ago', subDetails: 'Redis 7' }
		],
		appsCount: 1,
		composeCount: 0,
		databaseCount: 2,
		healthStatus: 'healthy',
		gitRepo: 'github.com/rustploy/core',
		gitBranch: 'main',
		lastCommitMsg: 'fix(engine): optimize Traefik container label generation',
		lastCommitHash: '8f2a10b',
		updatedAt: '2m ago',
		createdAt: '2026-06-12'
	},
	{
		id: 'proj-02',
		name: 'E-Commerce Storefront',
		description: 'High-traffic Next.js 14 web application with Tailwind CSS and Stripe checkout integration.',
		tags: ['Frontend', 'Next.js', 'Production'],
		environments: [
			{ id: 'env-03', name: 'Production', slug: 'production', isProduction: true, servicesCount: 2 },
			{ id: 'env-04', name: 'Preview', slug: 'preview', isProduction: false, servicesCount: 1 }
		],
		services: [
			{ id: 'srv-04', name: 'web-storefront', type: 'application', status: 'running', updatedAt: '14m ago', subDetails: 'Next.js 14' },
			{ id: 'srv-05', name: 'search-indexer', type: 'application', status: 'running', updatedAt: '5h ago', subDetails: 'Node.js 20' }
		],
		appsCount: 2,
		composeCount: 0,
		databaseCount: 0,
		healthStatus: 'healthy',
		gitRepo: 'github.com/rustploy/storefront',
		gitBranch: 'main',
		lastCommitMsg: 'feat(cart): add instant checkout drawer animations',
		lastCommitHash: '3c9f11a',
		updatedAt: '14m ago',
		createdAt: '2026-06-15'
	},
	{
		id: 'proj-03',
		name: 'Analytics Data Pipeline',
		description: 'Docker Compose stack running ClickHouse, Vector logs collector, and Grafana dashboard.',
		tags: ['Compose', 'Analytics', 'Monitoring'],
		environments: [
			{ id: 'env-05', name: 'Production', slug: 'production', isProduction: true, servicesCount: 4 }
		],
		services: [
			{ id: 'srv-06', name: 'analytics-stack', type: 'compose', status: 'running', updatedAt: '1h ago', subDetails: 'Docker Compose v2' }
		],
		appsCount: 0,
		composeCount: 1,
		databaseCount: 0,
		healthStatus: 'healthy',
		gitRepo: 'github.com/rustploy/analytics',
		gitBranch: 'ops/prod',
		lastCommitMsg: 'chore: bump ClickHouse buffer limit to 128MB',
		lastCommitHash: 'e41b90d',
		updatedAt: '1h ago',
		createdAt: '2026-06-20'
	},
	{
		id: 'proj-04',
		name: 'AI Model Inference Service',
		description: 'Python FastAPI microservice serving local LLM embeddings and vector search queries.',
		tags: ['AI', 'Python', 'Staging'],
		environments: [
			{ id: 'env-06', name: 'Staging', slug: 'staging', isProduction: false, servicesCount: 2 }
		],
		services: [
			{ id: 'srv-07', name: 'fastapi-inference', type: 'application', status: 'deploying', updatedAt: 'Just now', subDetails: 'Python 3.11' },
			{ id: 'srv-08', name: 'qdrant-vector-db', type: 'database', status: 'running', updatedAt: '2d ago', subDetails: 'Qdrant Vector' }
		],
		appsCount: 1,
		composeCount: 0,
		databaseCount: 1,
		healthStatus: 'deploying',
		gitRepo: 'github.com/rustploy/ai-inference',
		gitBranch: 'feature/embeddings',
		lastCommitMsg: 'feat(model): switch to bge-m3 embedding model',
		lastCommitHash: '9a7d22f',
		updatedAt: 'Just now',
		createdAt: '2026-07-01'
	},
	{
		id: 'proj-05',
		name: 'Legacy Auth Gateway',
		description: 'Go authentication service handling JWT session management and OAuth2 providers.',
		tags: ['Go', 'Auth', 'Warning'],
		environments: [
			{ id: 'env-07', name: 'Production', slug: 'production', isProduction: true, servicesCount: 1 }
		],
		services: [
			{ id: 'srv-09', name: 'go-auth-service', type: 'application', status: 'error', updatedAt: '45m ago', subDetails: 'Go 1.22' }
		],
		appsCount: 1,
		composeCount: 0,
		databaseCount: 0,
		healthStatus: 'error',
		gitRepo: 'github.com/rustploy/auth-service',
		gitBranch: 'main',
		lastCommitMsg: 'fix: handle token expiration corner cases',
		lastCommitHash: '11b88e0',
		updatedAt: '45m ago',
		createdAt: '2026-05-10'
	}
];

export function getProjectsMock(): ProjectMock[] {
	return MOCK_PROJECTS;
}
