import type { DeploymentMock } from './types.mock';

export const MOCK_DEPLOYMENTS: DeploymentMock[] = [
	{
		id: 'dep-101',
		projectName: 'Rustploy Core Backend',
		serviceName: 'api-server',
		environment: 'Production',
		kind: 'application',
		state: 'done',
		commitHash: '8f2a10b',
		commitMessage: 'fix(engine): optimize Traefik container label generation',
		branch: 'main',
		durationSeconds: 42,
		triggeredBy: 'Aditya Sahu (Webhook)',
		logs: [
			'[INFO] Pulling source code from github.com/rustploy/core (branch: main)...',
			'[INFO] Building Nixpacks Rust release binary...',
			'[INFO] Cargo build finished in 34.2s.',
			'[INFO] Deploying container rustploy-api-v0.29.12...',
			'[INFO] Health check HTTP GET /health returned 200 OK.',
			'[SUCCESS] Service deployment completed successfully!'
		],
		createdAt: '2m ago'
	},
	{
		id: 'dep-102',
		projectName: 'E-Commerce Storefront',
		serviceName: 'web-storefront',
		environment: 'Production',
		kind: 'application',
		state: 'done',
		commitHash: '3c9f11a',
		commitMessage: 'feat(cart): add instant checkout drawer animations',
		branch: 'main',
		durationSeconds: 65,
		triggeredBy: 'GitHub Actions',
		logs: [
			'[INFO] Fetching Next.js 14 repository...',
			'[INFO] Running pnpm install & pnpm build...',
			'[INFO] Next.js static pages generated (18 routes).',
			'[SUCCESS] Container store-web-prod running on port 3000.'
		],
		createdAt: '14m ago'
	},
	{
		id: 'dep-103',
		projectName: 'AI Model Inference Service',
		serviceName: 'fastapi-inference',
		environment: 'Staging',
		kind: 'application',
		state: 'building',
		commitHash: '9a7d22f',
		commitMessage: 'feat(model): switch to bge-m3 embedding model',
		branch: 'feature/embeddings',
		durationSeconds: 18,
		triggeredBy: 'Aditya Sahu (Manual)',
		logs: [
			'[INFO] Downloading PyTorch model weights...',
			'[INFO] Compiling FastAPI dependencies...'
		],
		createdAt: 'Just now'
	},
	{
		id: 'dep-104',
		projectName: 'Legacy Auth Gateway',
		serviceName: 'go-auth-service',
		environment: 'Production',
		kind: 'application',
		state: 'error',
		commitHash: '11b88e0',
		commitMessage: 'fix: handle token expiration corner cases',
		branch: 'main',
		durationSeconds: 28,
		triggeredBy: 'Git Push Hook',
		logs: [
			'[INFO] Compiling Go main.go package...',
			'[ERROR] panic: fatal runtime error: nil pointer dereference at auth.go:142',
			'[FATAL] Deployment failed with exit code 1.'
		],
		createdAt: '45m ago'
	}
];
