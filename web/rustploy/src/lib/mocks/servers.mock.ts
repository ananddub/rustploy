import type { ServerMock } from './types.mock';

export const MOCK_SERVERS: ServerMock[] = [
	{
		id: 'srv-node-1',
		name: 'Production Worker Node 01',
		ipAddress: '159.65.234.12',
		port: 22,
		user: 'root',
		status: 'online',
		os: 'Ubuntu 24.04 LTS (x86_64)',
		cpuCores: 8,
		cpuUsagePercent: 14,
		ramTotalGb: 16,
		ramUsagePercent: 32,
		diskTotalGb: 160,
		diskUsagePercent: 24,
		dockerVersion: '26.1.4',
		sshKeyId: 'key-ed25519-prod',
		lastPing: '2s ago'
	},
	{
		id: 'srv-node-2',
		name: 'Staging & Preview Server',
		ipAddress: '167.99.142.89',
		port: 22,
		user: 'deploy',
		status: 'online',
		os: 'Debian 12 Bookworm',
		cpuCores: 4,
		cpuUsagePercent: 28,
		ramTotalGb: 8,
		ramUsagePercent: 54,
		diskTotalGb: 80,
		diskUsagePercent: 41,
		dockerVersion: '26.0.2',
		sshKeyId: 'key-ed25519-prod',
		lastPing: '5s ago'
	},
	{
		id: 'srv-node-3',
		name: 'Database Backup Replica',
		ipAddress: '138.197.80.44',
		port: 2222,
		user: 'admin',
		status: 'connecting',
		os: 'Ubuntu 22.04 LTS',
		cpuCores: 4,
		cpuUsagePercent: 5,
		ramTotalGb: 8,
		ramUsagePercent: 18,
		diskTotalGb: 320,
		diskUsagePercent: 68,
		dockerVersion: '25.0.3',
		sshKeyId: 'key-rsa-backup',
		lastPing: '45s ago'
	}
];

export function getServersMock(): ServerMock[] {
	return MOCK_SERVERS;
}
