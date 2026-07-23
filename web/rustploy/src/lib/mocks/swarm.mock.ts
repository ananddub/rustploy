import type { SwarmNodeMock, SshKeyMock } from './types.mock';

export const MOCK_SWARM_NODES: SwarmNodeMock[] = [
	{
		id: 'sw-node-01',
		hostname: 'manager-01.rustploy.cluster',
		role: 'manager',
		status: 'ready',
		availability: 'active',
		ipAddress: '159.65.234.12',
		dockerEngineVersion: '26.1.4'
	},
	{
		id: 'sw-node-02',
		hostname: 'worker-01.rustploy.cluster',
		role: 'worker',
		status: 'ready',
		availability: 'active',
		ipAddress: '167.99.142.89',
		dockerEngineVersion: '26.0.2'
	}
];

export const MOCK_SSH_KEYS: SshKeyMock[] = [
	{
		id: 'key-ed25519-prod',
		name: 'Production Deploy Key',
		fingerprint: 'SHA256:4a8f9c1b2e3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9',
		publicKey: 'ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIG5... root@rustploy',
		keyType: 'ed25519',
		createdAt: '2026-06-01'
	},
	{
		id: 'key-rsa-backup',
		name: 'Backup Storage Key',
		fingerprint: 'SHA256:9f8e7d6c5b4a3f2e1d0c9b8a7f6e5d4c3b2a1f0e9d8',
		publicKey: 'ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQC... admin@rustploy',
		keyType: 'rsa',
		createdAt: '2026-06-15'
	}
];

export function getSwarmNodesMock(): SwarmNodeMock[] {
	return MOCK_SWARM_NODES;
}

export function getSshKeysMock(): SshKeyMock[] {
	return MOCK_SSH_KEYS;
}
