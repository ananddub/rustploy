# AGENTS.md — Repository Guide for AI Agents

Welcome to **Rustploy Web**, the frontend dashboard for the Rustploy deployment management platform. This document provides high-level architectural context, technology stack specifications, directory layout, commands, and engineering guidelines for AI agents working in this repository.

---

## 1. Executive Summary & Tech Stack

Rustploy Web is a modern, single-page dashboard application for managing deployments, servers, Docker instances, Traefik routing, Swarm clusters, SSH keys, and project schedules.

- **Framework**: [SvelteKit 2](https://svelte.dev/docs/kit) + [Svelte 5](https://svelte.dev/docs/svelte) (with **Runes mode** enforced via `vite.config.ts`)
- **Build Tool**: [Vite 8](https://vitejs.dev/)
- **Styling**: [Tailwind CSS v4](https://tailwindcss.com/) (`@tailwindcss/vite`, `@tailwindcss/forms`, `@tailwindcss/typography`, `tw-animate-css`)
- **UI Components**: [Bits UI](https://bits-ui.com/), [Shadcn Svelte](https://shadcn-svelte.com/), `@lucide/svelte` icons
- **State & Runes**: Svelte 5 Runes (`$state`, `$derived`, `$props`, `$effect`). Reactive state modules outside `.svelte` components require `.svelte.ts` extensions (e.g., `src/lib/auth.svelte.ts`).
- **API Client**: Auto-generated TypeScript SDK using [`@hey-api/openapi-ts`](https://github.com/hey-api/openapi-ts) from the Rustploy OpenAPI spec (`src/lib/client/`).
- **Data Visualization**: `layerchart`, `d3-scale`, `d3-shape`
- **Code Editor**: `@monaco-editor/loader`
- **Notifications**: `svelte-sonner`
- **Package Manager**: `pnpm` (lockfile: `pnpm-lock.yaml`)

---

## 2. Directory & Architecture Overview

The repository follows a feature-oriented layout co-locating page components, tab views, and routing:

```
rustploy/
├── .svelte-kit/              # SvelteKit generated sync types and build artifacts
├── openapi-ts.config.ts      # Config for generating API SDK from OpenAPI spec
├── package.json              # Dependencies and scripts
├── pnpm-lock.yaml            # Lockfile
├── vite.config.ts            # Vite config (enforces Svelte 5 runes)
├── tsconfig.json             # TypeScript configuration
├── static/                   # Static public assets
└── src/
    ├── app.html              # HTML shell template
    ├── app.d.ts              # Global ambient TypeScript definitions
    ├── lib/                  # Shared libraries, utilities, and global state
    │   ├── auth.ts           # Auth session logic, token refresh, and storage
    │   ├── auth.svelte.ts    # Svelte 5 reactive auth state rune (`authState`)
    │   ├── client/           # Generated OpenAPI SDK (`sdk.gen.ts`, `types.gen.ts`, `hey-api.ts`)
    │   ├── components/       # Shared non-route components (e.g. CommandPalette)
    │   ├── helpers.ts        # Formatting & data transformations
    │   ├── toast.ts          # Toast helper utilities (`svelte-sonner`)
    │   └── utils.ts          # UI utilities (`cn` with `clsx` & `tailwind-merge`)
    ├── components/           # UI Primitives & Domain Components
    │   ├── ui/               # Shadcn/Bits UI primitives (button, dialog, card, badge, etc.)
    │   ├── projects/         # Project-specific UI components
    │   └── tabs/             # Tab navigation components
    ├── pages/                # Modular Page Views & Complex Tab Implementations
    │   ├── applications/     # Application view components
    │   ├── auth/             # Login / Register forms & views
    │   ├── compose/          # Docker Compose configuration views
    │   ├── dashboard/        # Main dashboard overview metrics & cards
    │   ├── projects/         # Project listing & management views
    │   ├── remote-servers/   # Server management & SSH status
    │   └── ssh-keys/         # SSH key setup & listing
    ├── routes/               # SvelteKit File-based Routes
    │   ├── +layout.svelte    # Top-level layout (auth interceptors, toaster, command palette)
    │   ├── layout.css        # Global CSS, Tailwind v4 imports, theme tokens
    │   ├── auth/             # Route `/auth`
    │   ├── dashboard/        # Route `/dashboard`
    │   ├── deployments/      # Route `/deployments`
    │   ├── docker/           # Route `/docker`
    │   ├── monitoring/       # Route `/monitoring`
    │   ├── projects/         # Route `/projects` (and sub-routes `[id]`, `app/[appId]`, etc.)
    │   ├── remote-servers/   # Route `/remote-servers`
    │   ├── requests/         # Route `/requests`
    │   ├── schedules/        # Route `/schedules`
    │   ├── settings/         # Route `/settings`
    │   ├── ssh-keys/         # Route `/ssh-keys`
    │   ├── swarm/            # Route `/swarm`
    │   └── traefik/          # Route `/traefik`
    └── errors/               # Application error views & handlers
```

---

## 3. Essential Commands

Always use `pnpm` for running scripts and managing dependencies.

| Task | Command | Description |
| :--- | :--- | :--- |
| **Development** | `pnpm dev` | Starts Vite dev server |
| **Production Build** | `pnpm build` | Builds SvelteKit app for production |
| **Preview Build** | `pnpm preview` | Serves the production build locally |
| **Svelte Sync** | `pnpm prepare` | Generates `.svelte-kit/tsconfig.json` & route types |
| **Type Check** | `pnpm check` | Runs `svelte-kit sync` followed by `svelte-check` |
| **Type Check (Watch)** | `pnpm check:watch` | Runs Svelte type check in watch mode |
| **Linting** | `pnpm lint` | Verifies code format with `prettier` and `eslint` |
| **Formatting** | `pnpm format` | Formats all files in workspace using `prettier` |
| **Generate API SDK** | `npx @hey-api/openapi-ts` | Regenerates `$lib/client` from OpenAPI spec |

---

## 4. Engineering Rules & Coding Standards

### Svelte 5 & Runes Best Practices
1. **Svelte 5 Runes**: Always use Svelte 5 runes (`$state`, `$derived`, `$props`, `$effect`) for reactivity. Do NOT use Svelte 4 legacy store patterns (`writable`, `derived`) or `let` exports (`export let prop`).
2. **Reactive Files (`.svelte.ts`)**: Any TypeScript file outside of a `.svelte` file that uses runes MUST have a `.svelte.ts` extension (e.g. `auth.svelte.ts`). Standard `.ts` files cannot compile Svelte runes.
3. **State Referencing**: When initializing `$state()` from component props or object properties, beware of capturing initial values only. Use `$derived()` or update state explicitly when the source object changes.

### API Integration & Authentication
1. **Generated Client**: All API requests should use the SDK in `src/lib/client/sdk.gen.ts`. Avoid writing custom `fetch` wrappers when generated endpoints exist.
2. **Interceptors**: HTTP requests automatically attach JWT bearer tokens via request interceptors configured in `src/routes/+layout.svelte`. Response 401s trigger automatic token refreshes.

### UI & Styling
1. **Tailwind CSS v4**: Use Tailwind utility classes for styling. Use `cn(...)` from `$lib/utils` when merging conditional classes.
2. **UI Primitives**: Reusable atomic UI components are located in `src/components/ui/`. Keep primitive styling clean and decoupled from business logic.
3. **Icons**: Use `@lucide/svelte` for all application icons.

### Type Safety & Code Quality
1. **Strict Types**: Always define explicit interfaces and types for props, API models, and function parameters. Never use `any`.
2. **Verification**: Always run `pnpm check` and `pnpm lint` to verify type safety and formatting before finalizing changes.

---

## 5. Helpful Tips for AI Agents

- If SvelteKit type definitions or route types seem out of sync, run `pnpm prepare` (`svelte-kit sync`).
- If OpenAPI specifications change, update `openapi-ts.config.ts` if needed and run `npx @hey-api/openapi-ts` to refresh `src/lib/client/`.
- Maintain feature-first co-location: keep sub-components and logic close to their corresponding route or page view.
