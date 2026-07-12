<script lang="ts">
	import { X, Database } from '@lucide/svelte';
	import { databaseControllerCreate } from '$lib/client/sdk.gen';
	import type { DatabaseResponseDto } from '$lib/client/types.gen';

	type Props = {
		environmentId: number;
		onClose: () => void;
		onCreated: (db: DatabaseResponseDto) => void;
	};
	let { environmentId, onClose, onCreated }: Props = $props();

	const DB_KINDS = [
		{ id: 'postgres',  label: 'PostgreSQL',  defaultImage: 'postgres:17', color: 'text-blue-400' },
		{ id: 'mysql',     label: 'MySQL',        defaultImage: 'mysql:8',    color: 'text-orange-400' },
		{ id: 'mariadb',   label: 'MariaDB',      defaultImage: 'mariadb:11', color: 'text-orange-400' },
		{ id: 'mongo',     label: 'MongoDB',      defaultImage: 'mongo:7',    color: 'text-green-500' },
		{ id: 'redis',     label: 'Redis',        defaultImage: 'redis:7',    color: 'text-red-400' },
		{ id: 'libsql',    label: 'LibSQL/Turso', defaultImage: 'ghcr.io/tursodatabase/libsql-server:latest', color: 'text-purple-400' },
	];

	let selectedKind = $state('postgres');
	let name = $state('');
	let description = $state('');
	let dbName = $state('');
	let dbUser = $state('');
	let dbPassword = $state('');
	let dockerImage = $state('postgres:17');
	let externalPort = $state('');
	let loading = $state(false);
	let error = $state('');

	function selectKind(kind: string) {
		selectedKind = kind;
		const k = DB_KINDS.find(d => d.id === kind);
		if (k) dockerImage = k.defaultImage;
	}

	async function submit(e: SubmitEvent) {
		e.preventDefault(); error = ''; loading = true;
		try {
			const res = await databaseControllerCreate({
				path: { kind: selectedKind },
				body: {
					name: name.trim(),
					description: description.trim() || undefined,
					environment_id: environmentId,
					docker_image: dockerImage || undefined,
					database_name: dbName.trim() || undefined,
					database_user: dbUser.trim() || undefined,
					database_password: dbPassword || undefined,
					external_port: externalPort ? parseInt(externalPort) : undefined
				}
			});
			if (res.error || !res.data) throw new Error((res.error as any)?.message ?? 'Failed to create database');
			onCreated(res.data as DatabaseResponseDto);
			onClose();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Something went wrong';
		} finally { loading = false; }
	}

	const inputCls = 'flex h-9 w-full rounded-md border border-input bg-secondary px-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring';
</script>

<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1" aria-label="Close" onclick={onClose} onkeydown={() => {}}></div>
<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
	<div class="bg-card border border-border rounded-lg w-full max-w-lg shadow-2xl flex flex-col max-h-[90vh] pointer-events-auto">
		<div class="flex items-start justify-between px-5 py-4 border-b border-border shrink-0">
			<div>
				<h2 class="font-semibold">Create Database</h2>
				<p class="text-sm text-muted-foreground mt-0.5">Add a managed database to this environment</p>
			</div>
			<button onclick={onClose} class="text-muted-foreground hover:text-foreground p-0.5 rounded hover:bg-accent ml-4">
				<X size={16} />
			</button>
		</div>

		<form onsubmit={submit} class="overflow-y-auto flex-1 px-5 py-4 flex flex-col gap-4">
			<!-- Database type selection -->
			<div class="flex flex-col gap-2">
				<p class="text-sm font-medium text-muted-foreground">Database Type <span class="text-destructive">*</span></p>
				<div class="grid grid-cols-3 gap-2">
					{#each DB_KINDS as k}
						<button
							type="button"
							onclick={() => selectKind(k.id)}
							class="flex items-center gap-2 px-3 py-2 rounded-md border text-sm transition-colors {selectedKind === k.id ? 'border-primary bg-primary/10 text-primary font-medium' : 'border-border text-muted-foreground hover:border-foreground/30 hover:text-foreground'}"
						>
							<Database size={14} class={k.color} />
							{k.label}
						</button>
					{/each}
				</div>
			</div>

			<div class="flex flex-col gap-1.5">
				<label for="db-name" class="text-sm font-medium text-muted-foreground">Service Name <span class="text-destructive">*</span></label>
				<input id="db-name" class={inputCls} placeholder="my-postgres" bind:value={name} required />
			</div>

			<div class="flex flex-col gap-1.5">
				<label for="db-desc" class="text-sm font-medium text-muted-foreground">Description</label>
				<input id="db-desc" class={inputCls} placeholder="Optional" bind:value={description} />
			</div>

			<div class="flex flex-col gap-1.5">
				<label for="db-image" class="text-sm font-medium text-muted-foreground">Docker Image</label>
				<input id="db-image" class={inputCls} bind:value={dockerImage} />
				<p class="text-xs text-muted-foreground">Leave default or pin a specific version</p>
			</div>

			{#if selectedKind !== 'redis' && selectedKind !== 'libsql'}
				<div class="grid grid-cols-2 gap-4">
					<div class="flex flex-col gap-1.5">
						<label for="db-dbname" class="text-sm font-medium text-muted-foreground">Database Name</label>
						<input id="db-dbname" class={inputCls} placeholder="{name || 'mydb'}" bind:value={dbName} />
					</div>
					<div class="flex flex-col gap-1.5">
						<label for="db-user" class="text-sm font-medium text-muted-foreground">Database User</label>
						<input id="db-user" class={inputCls} placeholder="{name || 'admin'}" bind:value={dbUser} />
					</div>
				</div>
				<div class="flex flex-col gap-1.5">
					<label for="db-pass" class="text-sm font-medium text-muted-foreground">Database Password</label>
					<input id="db-pass" class={inputCls} type="password" placeholder="••••••••" bind:value={dbPassword} />
				</div>
			{/if}

			<div class="flex flex-col gap-1.5">
				<label for="db-port" class="text-sm font-medium text-muted-foreground">External Port</label>
				<input id="db-port" class={inputCls} type="number" placeholder="Leave empty to auto-assign" bind:value={externalPort} />
				<p class="text-xs text-muted-foreground">Expose on host port (optional)</p>
			</div>

			{#if error}
				<div class="rounded-md bg-destructive/10 border border-destructive/30 px-3 py-2 text-sm text-destructive">{error}</div>
			{/if}

			<div class="flex justify-end gap-2 pt-1">
				<button type="button" onclick={onClose} disabled={loading} class="px-3 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent transition-colors">Cancel</button>
				<button type="submit" disabled={loading || !name.trim()} class="inline-flex items-center gap-2 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 disabled:opacity-50 transition-colors">
					{#if loading}
						<div class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></div>Creating…
					{:else}
						<Database size={13} />Create Database
					{/if}
				</button>
			</div>
		</form>
	</div>
</div>
