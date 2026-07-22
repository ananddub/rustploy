import type { TraefikConfigMock } from './types.mock';

export const MOCK_TRAEFIK_CONFIGS: TraefikConfigMock[] = [
	{
		id: 'tr-01',
		name: 'dynamic-routers.yml',
		filename: 'dynamic-routers.yml',
		format: 'yaml',
		status: 'active',
		updatedAt: '1h ago',
		content: `http:
  routers:
    api-router:
      rule: "Host(\`api.rustploy.dev\`)"
      service: "api-service"
      entryPoints:
        - "websecure"
      tls:
        certResolver: "letsencrypt"

  services:
    api-service:
      loadBalancer:
        servers:
          - url: "http://127.0.0.1:8080"`
	},
	{
		id: 'tr-02',
		name: 'middlewares-security.yml',
		filename: 'middlewares-security.yml',
		format: 'yaml',
		status: 'active',
		updatedAt: '2d ago',
		content: `http:
  middlewares:
    sec-headers:
      headers:
        frameDeny: true
        sslRedirect: true
        browserXssFilter: true
        contentTypeNosniff: true`
	}
];
