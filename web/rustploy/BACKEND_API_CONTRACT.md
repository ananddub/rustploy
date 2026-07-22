# 🚀 Rustploy Backend REST API Specification & Data Contract

> **Target Backend Stack**: Rust (Actix-Web / Axum), PostgreSQL, Traefik v3, Docker Engine SDK  
> **OpenAPI Generation Target**: `@hey-api/openapi-ts` -> `$lib/client/`  
> **Frontend Framework**: SvelteKit 2 + Svelte 5 Runes

---

## 1. Overview & Standard Protocol Rules

### Authentication & Authorization
- **Header**: `Authorization: Bearer <JWT_ACCESS_TOKEN>`
- **Token Refresh**: `POST /api/auth/refresh` with `{ refresh_token: string }`
- **Error Status Codes**: `401 Unauthorized`, `403 Forbidden`, `404 Not Found`, `422 Unprocessable Entity`, `500 Internal Server Error`

### Standard Error Response Shape
All failing requests MUST return a JSON object conforming to this exact shape:
```json
{
  "code": "PROJECT_NOT_FOUND",
  "message": "Project with ID 'proj-99' was not found or has been deleted.",
  "details": null
}
```

---

## 2. API Endpoints & DTO Contracts

### A. Projects & Services (`/api/projects`)

#### `GET /api/projects`
List all projects for the authenticated organization.

- **Query Parameters**:
  - `organization_id` (string, required)
  - `tag` (string, optional)
- **Response Shape `200 OK`**:
```json
[
  {
    "id": "proj-01",
    "name": "Rustploy Core Backend",
    "description": "Main deployment engine and REST API",
    "tags": ["Rust", "Backend", "Production"],
    "environments": [
      {
        "id": "env-01",
        "name": "Production",
        "slug": "production",
        "is_production": true,
        "services_count": 3
      }
    ],
    "apps_count": 1,
    "compose_count": 0,
    "database_count": 2,
    "health_status": "healthy",
    "git_repo": "github.com/rustploy/core",
    "git_branch": "main",
    "last_commit_msg": "fix(engine): optimize Traefik container labels",
    "last_commit_hash": "8f2a10b",
    "updated_at": "2026-07-23T04:00:00Z",
    "created_at": "2026-06-12T10:00:00Z"
  }
]
```

#### `POST /api/projects`
Create a new project container.

- **Request Body**:
```json
{
  "name": "E-Commerce Storefront",
  "description": "Next.js web application",
  "tags": ["Frontend", "Next.js"]
}
```
- **Response Shape `201 Created`**: Project DTO.

---

### B. Deployments & Activity Stream (`/api/deployments`)

#### `GET /api/deployments`
List recent deployments across projects.

- **Query Parameters**:
  - `project_id` (string, optional)
  - `limit` (integer, default: 20)
  - `state` (string, optional: `running` | `done` | `error` | `building`)
- **Response Shape `200 OK`**:
```json
[
  {
    "id": "dep-101",
    "project_name": "Rustploy Core Backend",
    "service_name": "api-server",
    "environment": "Production",
    "kind": "application",
    "state": "done",
    "commit_hash": "8f2a10b",
    "commit_message": "fix(engine): optimize Traefik container labels",
    "branch": "main",
    "duration_seconds": 42,
    "triggered_by": "Aditya Sahu (Webhook)",
    "logs": [
      "[INFO] Pulling source code...",
      "[INFO] Building Nixpacks release binary...",
      "[SUCCESS] Service deployment completed successfully!"
    ],
    "created_at": "2026-07-23T04:10:00Z"
  }
]
```

#### `POST /api/deployments/trigger`
Trigger an instant service build and container deployment.

- **Request Body**:
```json
{
  "service_id": "srv-01",
  "branch": "main",
  "clear_cache": false
}
```

---

### C. Server Resources & Monitoring (`/api/servers`)

#### `GET /api/servers`
List connected remote server nodes and health metrics.

- **Response Shape `200 OK`**:
```json
[
  {
    "id": "srv-node-1",
    "name": "Production Worker Node 01",
    "ip_address": "159.65.234.12",
    "port": 22,
    "user": "root",
    "status": "online",
    "os": "Ubuntu 24.04 LTS (x86_64)",
    "cpu_cores": 8,
    "cpu_usage_percent": 14.2,
    "ram_total_gb": 16.0,
    "ram_usage_percent": 32.5,
    "disk_total_gb": 160.0,
    "disk_usage_percent": 24.1,
    "docker_version": "26.1.4",
    "ssh_key_id": "key-ed25519-prod",
    "last_ping": "2026-07-23T04:36:58Z"
  }
]
```

#### `GET /api/servers/metrics/realtime`
Server Sent Events (SSE) stream for real-time CPU, RAM, and Disk metrics.

---

### D. Cron Schedules (`/api/schedules`)

#### `GET /api/schedules`
List configured cron tasks.

- **Response Shape `200 OK`**:
```json
[
  {
    "id": "sched-01",
    "name": "Nightly Database Backup",
    "cron_expression": "0 2 * * *",
    "target_service": "postgres-db",
    "target_project": "Rustploy Core Backend",
    "status": "active",
    "last_run": "2026-07-23T02:00:00Z",
    "next_run": "2026-07-24T02:00:00Z",
    "command": "pg_dumpall -U postgres | gzip > /backups/db-latest.sql.gz"
  }
]
```

---

### E. Traefik Dynamic Configurations (`/api/traefik`)

#### `GET /api/traefik/configs`
List dynamic YAML/TOML routing configuration files.

- **Response Shape `200 OK`**:
```json
[
  {
    "id": "tr-01",
    "name": "dynamic-routers.yml",
    "filename": "dynamic-routers.yml",
    "format": "yaml",
    "status": "active",
    "updated_at": "2026-07-23T03:00:00Z",
    "content": "http:\n  routers:\n    api-router:\n      rule: \"Host(`api.rustploy.dev`)\""
  }
]
```

---

## 3. OpenAPI Spec Handoff Checklist for Rust Team

1. **Serve Swagger / OpenAPI JSON**: Serve the generated `openapi.json` spec at `GET /api/docs/openapi.json`.
2. **SDK Generation**: Run `npx @hey-api/openapi-ts` to auto-regenerate `$lib/client/sdk.gen.ts` whenever endpoints change.
3. **Mock Data Sync**: The Svelte frontend uses `src/lib/mocks/` with `USE_MOCK_DATA = true`. Flipping `USE_MOCK_DATA = false` will route 100% of frontend traffic directly to these Rust API contracts!
