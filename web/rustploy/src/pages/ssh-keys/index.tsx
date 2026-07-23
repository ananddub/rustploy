import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Key, Plus, FileKey, CheckCircle, Copy, Clock, Trash2 } from 'lucide-react';
import { PageLayout } from '@/components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { USE_MOCK_DATA, getSshKeysMock, type SshKeyMock } from '$lib/mocks';
import { toastSuccess } from '$lib/toast';

export default function SshKeysPage() {
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const [useMock, setUseMock] = useState(USE_MOCK_DATA);
	const [mockKeys, setMockKeys] = useState<SshKeyMock[]>(getSshKeysMock());
	const [copied, setCopied] = useState<string | null>(null);

	const displayKeys = mockKeys.map((k) => ({
		id: k.id,
		name: k.name,
		fingerprint: k.fingerprint,
		publicKey: k.publicKey,
		keyType: k.keyType,
		createdAt: k.createdAt,
		hasPrivate: true
	}));

	function deleteKey(id: string) {
		setMockKeys((prev) => prev.filter((k) => k.id !== id));
	}

	async function copyPublicKey(id: string, publicKey: string) {
		await navigator.clipboard.writeText(publicKey);
		setCopied(id);
		toastSuccess('Public key copied to clipboard');
		setTimeout(() => setCopied(null), 2000);
	}

	function truncateKey(key: string, len = 54) {
		return key.length > len ? key.slice(0, len) + '…' : key;
	}

	return (
		<PageLayout>
			<main className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up min-h-0 space-y-6">
				<div className="max-w-5xl mx-auto space-y-6">
					<div className="flex items-center justify-between">
						<div>
							<h1 className="text-3xl font-bold tracking-tight text-[#FAFAFA]">SSH Keys</h1>
							<p className="text-sm text-[#a1a1aa] mt-1">Manage SSH key pairs for authenticating remote server deployments</p>
						</div>
						<button className="inline-flex items-center gap-2 px-3.5 py-2 rounded-lg bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] text-xs font-semibold transition-colors cursor-pointer">
							<Plus className="w-4 h-4" /> Add SSH Key
						</button>
					</div>

					<div className="flex flex-col gap-3.5">
						{displayKeys.map((key) => (
							<div key={key.id} className="bg-[#171717] border border-[#262626] rounded-xl p-5 flex items-start gap-4 hover:border-[#3f3f46] transition-all">
								<div className="w-10 h-10 rounded-lg bg-[#262626] border border-white/10 flex items-center justify-center shrink-0 mt-0.5">
									<FileKey className="w-5 h-5 text-[#FAFAFA]" />
								</div>
								<div className="flex-1 min-w-0">
									<div className="flex items-center gap-2 flex-wrap">
										<h2 className="text-base font-semibold text-[#FAFAFA]">{key.name}</h2>
										<span className="inline-flex items-center gap-1 text-[10px] font-mono bg-green-500/10 text-green-400 border border-green-500/30 px-2 py-0.5 rounded">
											<CheckCircle className="w-3 h-3" /> Private Key Active
										</span>
										<span className="text-[10px] font-mono bg-[#262626] text-[#a1a1aa] px-2 py-0.5 rounded uppercase">{key.keyType}</span>
									</div>

									<div className="mt-2.5 flex items-center gap-2 bg-[#141414] border border-[#262626] rounded-lg px-3 py-2">
										<p className="font-mono text-xs text-[#a1a1aa] truncate flex-1">{truncateKey(key.publicKey)}</p>
										<button onClick={() => copyPublicKey(key.id, key.publicKey)} className="inline-flex items-center gap-1 text-xs text-[#a1a1aa] hover:text-[#FAFAFA] transition-colors shrink-0 cursor-pointer">
											{copied === key.id ? <CheckCircle className="w-3.5 h-3.5 text-green-400" /> : <Copy className="w-3.5 h-3.5" />}
											{copied === key.id ? 'Copied' : 'Copy'}
										</button>
									</div>

									<div className="flex items-center gap-4 mt-2.5 text-[11px] font-mono text-[#737373]">
										<span className="flex items-center gap-1"><Clock className="w-3 h-3 text-[#737373]" /> Created {key.createdAt}</span>
										<span>·</span>
										<span>Fingerprint: {key.fingerprint}</span>
									</div>
								</div>

								<div className="flex items-center gap-1 shrink-0">
									<button onClick={() => deleteKey(key.id)} className="p-2 rounded-lg border border-[#262626] bg-[#262626] text-[#a1a1aa] hover:text-red-400 hover:bg-red-500/10 transition-colors cursor-pointer">
										<Trash2 className="w-4 h-4" />
									</button>
								</div>
							</div>
						))}
					</div>
				</div>
			</main>
		</PageLayout>
	);
}
