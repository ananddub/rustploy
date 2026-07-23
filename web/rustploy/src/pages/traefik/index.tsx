import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Globe, FileCode, Folder, FolderOpen, ChevronRight, ChevronDown, Lock, LockOpen, Save } from 'lucide-react';
import Editor from '@monaco-editor/react';
import { PageLayout } from '@/components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { Button } from '@/components/ui/button';
import { toastSuccess } from '$lib/toast';

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
`
};

export default function TraefikPage() {
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const [savedContents, setSavedContents] = useState<Record<string, string>>({ ...fileContents });
	const [expanded, setExpanded] = useState<Set<string>>(new Set(['/etc/traefik', '/etc/traefik/dynamic']));
	const [selectedFile, setSelectedFile] = useState<string>('/etc/traefik/dynamic/http.yml');
	const [locked, setLocked] = useState(true);
	const [saving, setSaving] = useState(false);
	const [skipYaml, setSkipYaml] = useState(false);
	const [currentCode, setCurrentCode] = useState(fileContents['/etc/traefik/dynamic/http.yml']);

	function toggleDir(id: string) {
		setExpanded((prev) => {
			const next = new Set(prev);
			next.has(id) ? next.delete(id) : next.add(id);
			return next;
		});
	}

	function selectFile(path: string) {
		setSelectedFile(path);
		setCurrentCode(savedContents[path] || '');
		setLocked(true);
	}

	function handleSave() {
		setSaving(true);
		setTimeout(() => {
			setSavedContents((prev) => ({ ...prev, [selectedFile]: currentCode }));
			toastSuccess('Traefik config saved successfully');
			setSaving(false);
		}, 400);
	}

	function renderTreeNodes(nodes: TreeNode[], depth: number) {
		return nodes.map((node) => {
			if (node.type === 'dir') {
				const isExp = expanded.has(node.id);
				return (
					<React.Fragment key={node.id}>
						<button
							className="flex items-center gap-1.5 w-full text-left py-1 rounded-md text-xs hover:bg-[#262626] transition-colors text-[#a1a1aa] cursor-pointer"
							style={{ paddingLeft: `${8 + depth * 14}px`, paddingRight: '8px' }}
							onClick={() => toggleDir(node.id)}
						>
							{isExp ? (
								<>
									<ChevronDown className="w-3.5 h-3.5 shrink-0 text-[#737373]" />
									<FolderOpen className="w-3.5 h-3.5 shrink-0 text-amber-400" />
								</>
							) : (
								<>
									<ChevronRight className="w-3.5 h-3.5 shrink-0 text-[#737373]" />
									<Folder className="w-3.5 h-3.5 shrink-0 text-amber-400" />
								</>
							)}
							<span className="truncate">{node.name}</span>
						</button>
						{isExp && node.children && renderTreeNodes(node.children, depth + 1)}
					</React.Fragment>
				);
			}

			const isSelected = selectedFile === node.id;
			return (
				<button
					key={node.id}
					className={`flex items-center gap-1.5 w-full text-left py-1 rounded-md text-xs font-mono transition-colors cursor-pointer ${
						isSelected
							? 'bg-[#262626] text-[#FAFAFA] font-semibold'
							: 'text-[#737373] hover:bg-[#262626] hover:text-[#FAFAFA]'
					}`}
					style={{ paddingLeft: `${8 + depth * 14}px`, paddingRight: '8px' }}
					onClick={() => selectFile(node.id)}
				>
					<FileCode className="w-3.5 h-3.5 shrink-0 opacity-60" />
					<span className="truncate">{node.name}</span>
				</button>
			);
		});
	}

	return (
		<PageLayout>
			<main className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up min-h-0">
				<div className="max-w-6xl mx-auto space-y-6">
					<div className="rounded-xl border border-[#262626] bg-[#171717] overflow-hidden shadow-md">
						<div className="px-6 pt-5 pb-4 border-b border-[#262626]">
							<div className="flex items-center gap-2 mb-1">
								<FileCode className="w-5 h-5 text-[#a1a1aa]" />
								<h1 className="text-xl font-bold text-[#FAFAFA]">Traefik File System</h1>
							</div>
							<p className="text-xs text-[#a1a1aa]">
								Manage routing configurations in{' '}
								<code className="bg-[#262626] text-[#FAFAFA] px-1.5 py-0.5 rounded text-xs font-mono">/etc/traefik</code>
							</p>
						</div>

						<div className="flex flex-col lg:flex-row" style={{ minHeight: '540px' }}>
							{/* File tree */}
							<div className="lg:w-64 w-full shrink-0 border-b lg:border-b-0 lg:border-r border-[#262626] p-3 bg-[#141414]">
								<p className="text-[10px] font-semibold uppercase tracking-widest text-[#737373] px-2 mb-2">Files</p>
								{renderTreeNodes(tree, 0)}
							</div>

							{/* Monaco Editor panel */}
							<div className="flex-1 flex flex-col min-w-0 bg-[#171717]">
								<div className="flex items-center justify-between px-4 py-2.5 border-b border-[#262626] bg-[#141414]">
									<div className="min-w-0">
										<p className="text-[10px] text-[#737373] uppercase tracking-wider">Active configuration</p>
										<p className="text-xs font-mono text-[#FAFAFA] truncate">{selectedFile}</p>
									</div>
									<Button
										variant="secondary"
										size="sm"
										className="gap-1.5 h-7 text-xs bg-[#262626] border-[#3f3f46] text-[#FAFAFA] hover:bg-[#333333] cursor-pointer"
										onClick={() => setLocked(!locked)}
									>
										{locked ? <Lock className="w-3 h-3" /> : <LockOpen className="w-3 h-3" />}
										{locked ? 'Unlock' : 'Lock'}
									</Button>
								</div>

								<div className="relative" style={{ height: '440px' }}>
									<Editor
										height="440px"
										language="yaml"
										theme="vs-dark"
										value={currentCode}
										onChange={(val) => setCurrentCode(val || '')}
										options={{
											readOnly: locked,
											fontSize: 13,
											fontFamily: '"Fira Code", "JetBrains Mono", monospace',
											minimap: { enabled: false },
											scrollBeyondLastLine: false,
											wordWrap: 'on'
										}}
									/>
								</div>

								<div className="border-t border-[#262626] px-4 py-3 flex items-center justify-between bg-[#141414]">
									<label className="flex items-center gap-2 cursor-pointer select-none text-xs text-[#a1a1aa]">
										<input
											type="checkbox"
											checked={skipYaml}
											onChange={(e) => setSkipYaml(e.target.checked)}
											className="rounded border-[#262626] bg-[#171717]"
										/>
										<span>Skip YAML validation</span>
									</label>
									<Button
										size="sm"
										className="gap-1.5 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] cursor-pointer"
										onClick={handleSave}
										disabled={locked || saving}
									>
										<Save className="w-3.5 h-3.5" /> {saving ? 'Saving…' : 'Save Configuration'}
									</Button>
								</div>
							</div>
						</div>
					</div>
				</div>
			</main>
		</PageLayout>
	);
}
