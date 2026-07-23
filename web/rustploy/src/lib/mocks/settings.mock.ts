import type {
	UserMock,
	AuditLogMock,
	GitProviderMock,
	RegistryMock,
	DestinationMock,
	CertificateMock,
	NotificationChannelMock
} from './types.mock';

export function getUsersMock(): UserMock[] {
	return [
		{
			id: 'usr-1',
			name: 'Aditya Sahu',
			email: 'admin@rustploy.dev',
			role: 'Owner',
			status: 'Active',
			joinedAt: 'Jun 12, 2026',
			lastActive: '2 mins ago'
		},
		{
			id: 'usr-2',
			name: 'Aman Kumar',
			email: 'aman@example.com',
			role: 'Admin',
			status: 'Active',
			joinedAt: 'Jun 15, 2026',
			lastActive: '1 hour ago'
		},
		{
			id: 'usr-3',
			name: 'Sarah Chen',
			email: 'sarah@example.com',
			role: 'Developer',
			status: 'Active',
			joinedAt: 'Jul 01, 2026',
			lastActive: '3 hours ago'
		},
		{
			id: 'usr-4',
			name: 'Alex Rivera',
			email: 'alex@example.com',
			role: 'Viewer',
			status: 'Pending',
			joinedAt: 'Jul 20, 2026',
			lastActive: 'Never'
		}
	];
}

export function getAuditLogsMock(): AuditLogMock[] {
	return [
		{
			id: 'log-101',
			action: 'PROJECT_CREATE',
			actor: 'admin@rustploy.dev',
			target: 'E-Commerce Storefront (web-storefront)',
			ipAddress: '192.168.1.15',
			severity: 'info',
			timestamp: '10 mins ago'
		},
		{
			id: 'log-102',
			action: 'DEPLOY_TRIGGER',
			actor: 'system-ci',
			target: 'Rustploy Core Backend #dep-101',
			ipAddress: '127.0.0.1',
			severity: 'info',
			timestamp: '25 mins ago'
		},
		{
			id: 'log-103',
			action: 'SSH_KEY_ADD',
			actor: 'aman@example.com',
			target: 'prod-rsa-key-2026',
			ipAddress: '10.0.4.12',
			severity: 'info',
			timestamp: '1 hour ago'
		},
		{
			id: 'log-104',
			action: 'TRAEFIK_UPDATE',
			actor: 'admin@rustploy.dev',
			target: '/etc/traefik/dynamic/http.yml',
			ipAddress: '192.168.1.15',
			severity: 'warn',
			timestamp: '3 hours ago'
		},
		{
			id: 'log-105',
			action: 'USER_ROLE_CHANGE',
			actor: 'admin@rustploy.dev',
			target: 'Sarah Chen (Developer -> Admin)',
			ipAddress: '192.168.1.15',
			severity: 'warn',
			timestamp: '1 day ago'
		},
		{
			id: 'log-106',
			action: 'SERVER_CONNECT',
			actor: 'system',
			target: 'production-01 (Ubuntu 24.04)',
			ipAddress: '172.16.0.1',
			severity: 'info',
			timestamp: '2 days ago'
		}
	];
}

export function getGitProvidersMock(): GitProviderMock[] {
	return [
		{
			id: 'git-1',
			provider: 'GitHub',
			username: 'rustploy-org',
			authType: 'OAuth App',
			status: 'connected',
			repositoriesCount: 14,
			connectedAt: 'Jun 12, 2026'
		},
		{
			id: 'git-2',
			provider: 'GitLab',
			username: 'rustploy-devs',
			authType: 'Personal Access Token',
			status: 'connected',
			repositoriesCount: 6,
			connectedAt: 'Jun 18, 2026'
		},
		{
			id: 'git-3',
			provider: 'Bitbucket',
			username: 'rustploy-enterprise',
			authType: 'SSH Key',
			status: 'connected',
			repositoriesCount: 2,
			connectedAt: 'Jul 05, 2026'
		}
	];
}

export function getRegistriesMock(): RegistryMock[] {
	return [
		{
			id: 'reg-1',
			name: 'Docker Hub Official',
			registryUrl: 'registry-1.docker.io',
			username: 'rustployofficial',
			isDefault: true,
			status: 'connected',
			pushedImagesCount: 42,
			createdAt: 'Jun 12, 2026'
		},
		{
			id: 'reg-2',
			name: 'GitHub Container Registry',
			registryUrl: 'ghcr.io',
			username: 'ananddub',
			isDefault: false,
			status: 'connected',
			pushedImagesCount: 18,
			createdAt: 'Jun 20, 2026'
		},
		{
			id: 'reg-3',
			name: 'AWS Elastic Container Registry',
			registryUrl: '123456789.dkr.ecr.us-east-1.amazonaws.com',
			username: 'AWS-IAM-Role',
			isDefault: false,
			status: 'connected',
			pushedImagesCount: 8,
			createdAt: 'Jul 10, 2026'
		}
	];
}

export function getDestinationsMock(): DestinationMock[] {
	return [
		{
			id: 'dest-1',
			name: 'AWS S3 Production Backups',
			provider: 'Amazon S3',
			bucketName: 'rustploy-db-backups-prod',
			region: 'us-east-1',
			accessKeyId: 'AKIAIOSFODNN7EXAMPLE',
			storageUsedGb: 142.8,
			backupsCount: 124,
			createdAt: 'Jun 12, 2026'
		},
		{
			id: 'dest-2',
			name: 'Cloudflare R2 Global Assets',
			provider: 'Cloudflare R2',
			bucketName: 'rustploy-static-assets',
			region: 'auto',
			accessKeyId: 'c3f4e1d2a0b8c6',
			storageUsedGb: 38.4,
			backupsCount: 48,
			createdAt: 'Jun 25, 2026'
		},
		{
			id: 'dest-3',
			name: 'MinIO On-Premises Secondary',
			provider: 'MinIO',
			bucketName: 'minio-local-archive',
			region: 'local',
			accessKeyId: 'minioadmin',
			storageUsedGb: 210.5,
			backupsCount: 200,
			createdAt: 'Jul 02, 2026'
		}
	];
}

export function getCertificatesMock(): CertificateMock[] {
	return [
		{
			id: 'cert-1',
			name: 'Wildcard Production Certificate',
			domain: '*.rustploy.dev',
			type: 'Wildcard',
			status: 'valid',
			expiresAt: '2027-03-15',
			autoRenew: true,
			issuer: "Let's Encrypt Authority X3",
			isChain: true,
			chainLength: 3
		},
		{
			id: 'cert-2',
			name: 'API Gateway SSL Certificate',
			domain: 'api.rustploy.dev',
			type: 'Single Domain',
			status: 'valid',
			expiresAt: '2027-01-20',
			autoRenew: true,
			issuer: "Let's Encrypt Authority X3",
			isChain: false,
			chainLength: 1
		},
		{
			id: 'cert-3',
			name: 'Staging Environment Cert',
			domain: 'staging.rustploy.dev',
			type: 'Single Domain',
			status: 'expiring',
			expiresAt: '2026-08-01',
			autoRenew: false,
			issuer: "ZeroSSL RSA Domain Secure",
			isChain: false,
			chainLength: 1
		}
	];
}

export function getNotificationsMock(): NotificationChannelMock[] {
	return [
		{
			id: 'notif-1',
			name: 'DevOps Discord Channel',
			provider: 'Discord',
			targetUrl: 'https://discord.com/api/webhooks/123456789/abcde-token-key',
			notifyOnSuccess: true,
			notifyOnError: true,
			notifyOnWarning: true,
			status: 'active'
		},
		{
			id: 'notif-2',
			name: 'Engineering Slack #alerts',
			provider: 'Slack',
			targetUrl: 'https://hooks.slack.com/services/T00/B00/XXXXX',
			notifyOnSuccess: false,
			notifyOnError: true,
			notifyOnWarning: true,
			status: 'active'
		},
		{
			id: 'notif-3',
			name: 'Critical Failures Telegram Bot',
			provider: 'Telegram',
			targetUrl: 'https://api.telegram.org/bot12345:token/sendMessage',
			notifyOnSuccess: false,
			notifyOnError: true,
			notifyOnWarning: false,
			status: 'active'
		}
	];
}
