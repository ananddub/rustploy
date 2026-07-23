/**
 * Rustploy Comprehensive Backend DTO & Mock Type Contracts
 * Matching Rust Actix-Web OpenAPI Specs
 */

export interface ServiceSummaryMock {
	id: string;
	name: string;
	type: 'application' | 'compose' | 'database';
	status: 'running' | 'stopped' | 'error' | 'deploying';
	updatedAt: string;
	subDetails?: string;
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

export interface DeploymentMock {
	id: string;
	projectName: string;
	serviceName: string;
	environment: string;
	kind: 'application' | 'compose' | 'database';
	state: 'running' | 'done' | 'error' | 'idle' | 'building';
	commitHash: string;
	commitMessage: string;
	branch: string;
	durationSeconds: number;
	triggeredBy: string;
	logs: string[];
	createdAt: string;
}

export interface ServerMock {
	id: string;
	name: string;
	ipAddress: string;
	port: number;
	user: string;
	status: 'online' | 'offline' | 'connecting' | 'error';
	os: string;
	cpuCores: number;
	cpuUsagePercent: number;
	ramTotalGb: number;
	ramUsagePercent: number;
	diskTotalGb: number;
	diskUsagePercent: number;
	dockerVersion: string;
	sshKeyId: string;
	lastPing: string;
}

export interface ScheduleMock {
	id: string;
	name: string;
	cronExpression: string;
	targetService: string;
	targetProject: string;
	status: 'active' | 'paused' | 'failed';
	lastRun: string;
	nextRun: string;
	command: string;
}

export interface TraefikConfigMock {
	id: string;
	name: string;
	filename: string;
	format: 'yaml' | 'toml';
	content: string;
	status: 'valid' | 'invalid' | 'active';
	updatedAt: string;
}

export interface DockerContainerMock {
	id: string;
	name: string;
	image: string;
	status: 'running' | 'exited' | 'paused' | 'restarting';
	ports: string;
	created: string;
	cpuPercent: number;
	memUsageMb: number;
}

export interface SwarmNodeMock {
	id: string;
	hostname: string;
	role: 'manager' | 'worker';
	status: 'ready' | 'down';
	availability: 'active' | 'pause' | 'drain';
	ipAddress: string;
	dockerEngineVersion: string;
}

export interface SshKeyMock {
	id: string;
	name: string;
	fingerprint: string;
	publicKey: string;
	keyType: 'ed25519' | 'rsa';
	createdAt: string;
}

export interface UserMock {
	id: string;
	name: string;
	email: string;
	role: 'Owner' | 'Admin' | 'Developer' | 'Viewer';
	status: 'Active' | 'Pending' | 'Suspended';
	avatarUrl?: string;
	joinedAt: string;
	lastActive: string;
}

export interface AuditLogMock {
	id: string;
	action: 'PROJECT_CREATE' | 'SSH_KEY_ADD' | 'DEPLOY_TRIGGER' | 'TRAEFIK_UPDATE' | 'USER_ROLE_CHANGE' | 'SERVER_CONNECT';
	actor: string;
	target: string;
	ipAddress: string;
	severity: 'info' | 'warn' | 'error';
	timestamp: string;
}

export interface GitProviderMock {
	id: string;
	provider: 'GitHub' | 'GitLab' | 'Bitbucket';
	username: string;
	authType: 'OAuth App' | 'Personal Access Token' | 'SSH Key';
	status: 'connected' | 'error';
	repositoriesCount: number;
	connectedAt: string;
}

export interface RegistryMock {
	id: string;
	name: string;
	registryUrl: string;
	username: string;
	isDefault: boolean;
	status: 'connected' | 'error';
	pushedImagesCount: number;
	createdAt: string;
}

export interface DestinationMock {
	id: string;
	name: string;
	provider: 'Amazon S3' | 'Cloudflare R2' | 'MinIO' | 'DigitalOcean Spaces';
	bucketName: string;
	region: string;
	accessKeyId: string;
	storageUsedGb: number;
	backupsCount: number;
	createdAt: string;
}

export interface CertificateMock {
	id: string;
	name: string;
	domain: string;
	type: 'Wildcard' | 'Single Domain';
	status: 'valid' | 'expiring' | 'expired';
	expiresAt: string;
	autoRenew: boolean;
	issuer: string;
	isChain: boolean;
	chainLength: number;
}

export interface NotificationChannelMock {
	id: string;
	name: string;
	provider: 'Discord' | 'Slack' | 'Telegram' | 'Email' | 'Webhook';
	targetUrl: string;
	notifyOnSuccess: boolean;
	notifyOnError: boolean;
	notifyOnWarning: boolean;
	status: 'active' | 'disabled';
}
