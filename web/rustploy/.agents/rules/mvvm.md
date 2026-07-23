---
trigger: always_on
description: "Enforce MVVM architecture and folder structure for SvelteKit development"
---

# MVVM Architecture and Folder Separation Rule

Before rewriting, creating new pages, or modifying files in this workspace, you must enforce and adhere to the Model-View-ViewModel (MVVM) architecture with strict folder/concern separation. Do not place all logic and views in a single file.

## 1. Architecture Separation Guidelines

### View (UI Presentation)
- **Location**: `src/routes/` (for route layouts and pages) and `src/lib/components/` (for reusable UI components).
- **File Types**: `.svelte` files.
- **Guidelines**:
  - Keep Svelte component script blocks as thin as possible.
  - Do NOT call API endpoints or client SDK functions directly inside `.svelte` script tags.
  - The View must bind to properties of a **ViewModel** and call methods on the **ViewModel** to perform actions.
  - Let the View focus strictly on rendering HTML, applying CSS classes, animating elements, and responding to user gestures.
  - **Component Folder Structure (Project specific)**:
    Under `src/lib/components/projects/`, organize components into specific folders:
    - `compose/` (e.g., `CreateComposeModal.svelte`)
    - `database/` (e.g., `CreateDatabaseModal.svelte`)
    - `app/` (e.g., `CreateApplicationModal.svelte`)
    - `env/` (e.g., `CreateEnvModal.svelte`, `EnvDropdown.svelte`, `ProjectEnvironmentModal.svelte`)
    - `dashboard/` (e.g., `CreateProjectModal.svelte`, `ProjectCard.svelte`)
    - `service/` (e.g., `CreateServiceDropdown.svelte`, `ServiceCard.svelte`)

### ViewModel (Presentation Logic & State)
- **Location**: `src/lib/viewmodels/` or localized viewmodel files next to the view (e.g. `src/routes/projects/projects.svelte.ts`).
- **File Types**: `.svelte.ts` files (to allow Svelte compiler to process Svelte 5 runes).
- **Guidelines**:
  - ViewModels must be implemented using Svelte 5 runes (`$state`, `$derived`, `$effect`) to provide reactive state.
  - ViewModels are responsible for API integration (calling Models/SDK), managing view states (e.g. `isLoading`, `errorMsg`, `isSaving`), and validating user inputs.
  - A ViewModel should be class-based or function-based, exportable, and easily instantiable/injectable.

### Model (Business Data & Adapters)
- **Location**: `src/lib/client/` (auto-generated API SDK), `src/lib/models/`, and global state stores like `src/lib/auth.ts`.
- **File Types**: `.ts` files.
- **Guidelines**:
  - Contains pure data interfaces, models, client adapters, and HTTP controllers.
  - Models must not know anything about View configurations or Svelte-specific UI frameworks.

---

## 2. Refactoring Existing Files
- Existing components under `src/lib/components/projects/` have been successfully organized into their respective folders (`compose/`, `database/`, `app/`, `env/`, `dashboard/`, `service/`), and import statements in `src/routes/projects/+page.svelte` and `src/routes/projects/[id]/+page.svelte` have been updated.
- When modifying any existing page that has logic and views mixed, refactor the logic out into a corresponding `.svelte.ts` ViewModel first.
- Example structure:
  - View: `src/routes/projects/+page.svelte`
  - ViewModel: `src/routes/projects/projects.svelte.ts`
