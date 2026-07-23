import type { DockerContainerMock } from './types.mock';

export const MOCK_DOCKER_CONTAINERS: DockerContainerMock[] = [
	{
		id: 'c-01a2b3',
		name: 'rustploy-api-prod',
		image: 'rustploy/core:v0.29.12',
		status: 'running',
		ports: '0.0.0.0:8080->8080/tcp',
		created: '2 hours ago',
		cpuPercent: 1.4,
		memUsageMb: 84.2
	},
	{
		id: 'c-04c5d6',
		name: 'traefik-proxy-ingress',
		image: 'traefik:v3.0',
		status: 'running',
		ports: '80:80, 443:443',
		created: '5 days ago',
		cpuPercent: 0.8,
		memUsageMb: 42.1
	},
	{
		id: 'c-07e8f9',
		name: 'postgres-16-main',
		image: 'postgres:16-alpine',
		status: 'running',
		ports: '127.0.0.1:5432->5432/tcp',
		created: '12 days ago',
		cpuPercent: 2.1,
		memUsageMb: 240.5
	}
];

export function getDockerContainersMock(): DockerContainerMock[] {
	return MOCK_DOCKER_CONTAINERS;
}
