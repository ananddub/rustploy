<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import {
		Globe, FileCode, Folder, FolderOpen, ChevronRight, ChevronDown,
		MousePointerClick, Lock, LockOpen, Save, Loader2, AlertTriangle
	} from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import { Button } from '$lib/components/ui/button';
	import { toastSuccess, toastError } from '$lib/toast';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	type TreeNode = { id: string; name: string; type: 'file' | 'dir'; children?: TreeNode[] };

	const tree: TreeNode[] = [
		{
			id: '/etc/traefik', name: 'traefik', type: 'dir', children: [
				{
					id: '/etc/traefik/dynamic', name: 'dynamic', type: 'dir', children: [
						{ id: '/etc/traefik/dynamic/http.yml',        name: 'http.yml',        type: 'file' },
						{ id: '/etc/traefik/dynamic/tls.yml',         name: 'tls.yml',         type: 'file' },
						{ id: '/etc/traefik/dynamic/middlewares.yml', name: 'middlewares.yml', type: 'file' },
					]
				},
				{ id: '/etc/traefik/traefik.yml', name: 'traefik.yml', type: 'file' },
			]
		}
	];

	const fileContents: Record<string, string> = {
		'/etc/traefik/dynamic/http.yml': `# HTTP Routers and Services
http:
  routers:
    my-app:
      rule: "Host(\`app.example.com\`)"
      service: my-app-service
      entryPoints:
        - websecure
      tls: {}
      middlewares:
        - redirect-to-https
`,
		'/etc/traefik/dynamic/tls.yml': `# TLS Configuration
tls:
  options:
    default:
      minVersion: VersionTLS12
      sniStrict: true
`,
		'/etc/traefik/dynamic/middlewares.yml': `# Middleware Definitions
http:
  middlewares:
    redirect-to-https:
      redirectScheme:
        scheme: https
        permanent: true
`,
		'/etc/traefik/traefik.yml': `# Traefik Static Configuration
api:
  dashboard: true
  insecure: false
`,
	};

	const savedContents: Record<string, string> = { ...fileContents };

	let expanded    = $state<Set<string>>(new Set(['/etc/traefik', '/etc/traefik/dynamic']));
	let selectedFile = $state<string | null>('/etc/traefik/dynamic/http.yml');

	function toggleDir(id: string) {
		const s = new Set(expanded);
		s.has(id) ? s.delete(id) : s.add(id);
		expanded = s;
	}

	let editorEl: HTMLDivElement;
	let monacoInst: any = null;
	let editor: any    = null;
	let monacoReady    = $state(false);
	let locked         = $state(true);
	let saving         = $state(false);
	let skipYaml       = $state(false);
	let yamlError      = $state('');

	onMount(() => {
		let isMounted = true;
		(async () => {
			const loaderModule = await import('@monaco-editor/loader');
			const loader = loaderModule.default;
			loader.config({ paths: { vs: 'https://cdn.jsdelivr.net/npm/monaco-editor@0.52.0/min/vs' } });
			monacoInst = await loader.init();
			if (!isMounted || !editorEl) return;

			editor = monacoInst.editor.create(editorEl, {
				value: savedContents['/etc/traefik/dynamic/http.yml'],
				language: 'yaml',
				theme: 'vs-dark',
				readOnly: true,
				fontSize: 13,
				fontFamily: '"Fira Code", "JetBrains Mono", monospace',
				minimap: { enabled: false },
				scrollBeyondLastLine: false,
				lineNumbers: 'on',
				wordWrap: 'on',
				automaticLayout: true,
				padding: { top: 16, bottom: 16 },
				renderLineHighlight: 'line',
				smoothScrolling: true,
				stickyScroll: { enabled: false },
			});

			monacoReady = true;
			if (selectedFile) loadFileInEditor(selectedFile);
		})();

		return () => {
			isMounted = false;
			editor?.dispose();
		};
	});

	function loadFileInEditor(path: string) {
		if (!editor || !monacoInst) return;
		const content = savedContents[path] ?? '';
		const oldModel = editor.getModel();
		const newModel = monacoInst.editor.createModel(content, 'yaml');
		editor.setModel(newModel);
		oldModel?.dispose();
		editor.updateOptions({ readOnly: true });
		locked = true;
		yamlError = '';
	}

	function selectFile(path: string) {
		selectedFile = path;
		if (monacoReady) loadFileInEditor(path);
	}

	function toggleLock() {
		locked = !locked;
		editor?.updateOptions({ readOnly: locked });
		if (!locked) editor?.focus();
	}

	async function save() {
		if (!selectedFile || !editor) return;
		const content = editor.getValue();
		saving = true;
		try {
			await new Promise(r => setTimeout(r, 500));
			savedContents[selectedFile] = content;
			toastSuccess('Traefik config saved');
		} catch {
			toastError('Failed to save');
		} finally {
			saving = false;
		}
	}
</script>

<PageLayout>
	<!-- Top Breadcrumb Header Bar -->
	<header class="flex items-center justify-between px-6 py-3 border-b border-[#262626] text-xs bg-[#0A0A0A] shrink-0">
		<div class="flex items-center gap-2">
			<Globe class="w-3.5 h-3.5 text-[#a1a1aa]" />
			<span class="font-medium text-[#FAFAFA]">Traefik File System</span>
		</div>
	</header>

	<main class="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up min-h-0">
		<!-- Constrained Max-Width Shell (Dokploy Standard max-w-6xl) -->
		<div class="max-w-6xl space-y-6">
			<div class="rounded-xl border border-[#262626] bg-[#171717] overflow-hidden shadow-md">
				<div class="px-6 pt-5 pb-4 border-b border-[#262626]">
					<div class="flex items-center gap-2 mb-1">
						<FileCode class="w-5 h-5 text-[#a1a1aa]" />
						<h1 class="text-xl font-bold text-[#FAFAFA]">Traefik File System</h1>
					</div>
					<p class="text-xs text-[#a1a1aa]">
						Manage routing configurations in
						<code class="bg-[#262626] text-[#FAFAFA] px-1.5 py-0.5 rounded text-xs font-mono">/etc/traefik</code>
					</p>
				</div>

				<div class="flex flex-col lg:flex-row" style="min-height: 540px">
					<!-- Left: file tree -->
					<div class="lg:w-64 w-full shrink-0 border-b lg:border-b-0 lg:border-r border-[#262626] p-3 bg-[#141414]">
						<p class="text-[10px] font-semibold uppercase tracking-widest text-[#737373] px-2 mb-2">Files</p>
						{#snippet renderTree(nodes: typeof tree, depth: number)}
							{#each nodes as node (node.id)}
								{#if node.type === 'dir'}
									<button
										class="flex items-center gap-1.5 w-full text-left py-1 rounded-md text-xs hover:bg-[#262626] transition-colors text-[#a1a1aa]"
										style="padding-left:{8 + depth * 14}px; padding-right:8px"
										onclick={() => toggleDir(node.id)}
									>
										{#if expanded.has(node.id)}
											<ChevronDown class="w-3.5 h-3.5 shrink-0 text-[#737373]" />
											<FolderOpen class="w-3.5 h-3.5 shrink-0 text-amber-400" />
										{:else}
											<ChevronRight class="w-3.5 h-3.5 shrink-0 text-[#737373]" />
											<Folder class="w-3.5 h-3.5 shrink-0 text-amber-400" />
										{/if}
										<span class="truncate">{node.name}</span>
									</button>
									{#if expanded.has(node.id) && node.children}
										{@render renderTree(node.children, depth + 1)}
									{/if}
								{:else}
									<button
										class="flex items-center gap-1.5 w-full text-left py-1 rounded-md text-xs font-mono transition-colors
											{selectedFile === node.id
												? 'bg-[#262626] text-[#FAFAFA] font-semibold'
												: 'text-[#737373] hover:bg-[#262626] hover:text-[#FAFAFA]'}"
										style="padding-left:{8 + depth * 14}px; padding-right:8px"
										onclick={() => selectFile(node.id)}
									>
										<FileCode class="w-3.5 h-3.5 shrink-0 opacity-60" />
										<span class="truncate">{node.name}</span>
									</button>
								{/if}
							{/each}
						{/snippet}

						{@render renderTree(tree, 0)}
					</div>

					<!-- Right: editor panel -->
					<div class="flex-1 flex flex-col min-w-0 bg-[#171717]">
						<div class="flex items-center justify-between px-4 py-2.5 border-b border-[#262626] bg-[#141414]">
							<div class="min-w-0">
								<p class="text-[10px] text-[#737373] uppercase tracking-wider">Active configuration</p>
								<p class="text-xs font-mono text-[#FAFAFA] truncate">{selectedFile ?? ''}</p>
							</div>
							<Button variant="secondary" size="sm" class="gap-1.5 h-7 text-xs bg-[#262626] border-[#3f3f46] text-[#FAFAFA] hover:bg-[#333333]" onclick={toggleLock}>
								{#if locked}<Lock class="w-3 h-3" /> Unlock{:else}<LockOpen class="w-3 h-3" /> Lock{/if}
							</Button>
						</div>

						<div class="relative" style="height:440px">
							{#if !monacoReady}
								<div class="absolute inset-0 z-10 flex items-center justify-center gap-3 text-[#a1a1aa] bg-[#171717]">
									<Loader2 class="w-5 h-5 animate-spin" />
									<span class="text-sm">Loading Monaco editor…</span>
								</div>
							{/if}
							<div bind:this={editorEl} class="w-full h-full"></div>
						</div>

						<div class="border-t border-[#262626] px-4 py-3 flex items-center justify-between bg-[#141414]">
							<label class="flex items-center gap-2 cursor-pointer select-none text-xs text-[#a1a1aa]">
								<input type="checkbox" class="rounded border-[#262626] bg-[#171717]" bind:checked={skipYaml} />
								<span>Skip YAML validation</span>
							</label>
							<Button size="sm" class="gap-1.5 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A]" onclick={save} disabled={locked || saving}>
								{#if saving}<Loader2 class="w-3.5 h-3.5 animate-spin" /> Saving…
								{:else}<Save class="w-3.5 h-3.5" /> Save Configuration{/if}
							</Button>
						</div>
					</div>
				</div>
			</div>
		</div>
	</main>
</PageLayout>
