import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Shield, Plus, Trash2, RefreshCw, AlertTriangle } from 'lucide-react';
import { PageLayout } from '$lib/../components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { USE_MOCK_DATA, getCertificatesMock, type CertificateMock } from '$lib/mocks';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '$lib/../components/ui/card';
import { Button } from '$lib/../components/ui/button';
import { Badge } from '$lib/../components/ui/badge';
import { toastSuccess } from '$lib/toast';

export default function CertificatesPage() {
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const [useMock, setUseMock] = useState(USE_MOCK_DATA);
	const [certificates, setCertificates] = useState<CertificateMock[]>(getCertificatesMock());

	function statusColor(status: string): string {
		if (status === 'valid') return 'text-green-400';
		if (status === 'expiring') return 'text-amber-400';
		return 'text-red-400';
	}

	function deleteCert(id: string) {
		setCertificates((prev) => prev.filter((c) => c.id !== id));
		toastSuccess('Certificate removed');
	}

	return (
		<PageLayout>
			<header className="flex items-center justify-between px-6 py-3 border-b border-[#262626] text-xs bg-[#0A0A0A] shrink-0">
				<div className="flex items-center gap-2">
					<Shield className="w-3.5 h-3.5 text-[#a1a1aa]" />
					<span className="font-medium text-[#FAFAFA]">Certificates</span>
				</div>

				<div className="flex items-center gap-2 px-3 py-1 rounded-full bg-[#141414] border border-[#262626]">
					<span className="text-[11px] text-[#a1a1aa]">Data Source:</span>
					<button
						onClick={() => setUseMock(!useMock)}
						className={`text-[11px] font-semibold px-2 py-0.5 rounded transition-colors cursor-pointer ${
							useMock
								? 'bg-[#262626] text-[#FAFAFA] border border-white/10'
								: 'text-[#737373] hover:text-[#FAFAFA]'
						}`}
					>
						{useMock ? 'Mock Demo Data' : 'Live Rust Backend API'}
					</button>
				</div>
			</header>

			<main className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up min-h-0">
				<div className="max-w-5xl mx-auto space-y-6">
					<Card className="bg-[#171717] border border-[#262626] rounded-xl shadow-md overflow-hidden">
						<CardHeader className="p-6 pb-4">
							<div className="flex items-center justify-between">
								<div>
									<CardTitle className="text-lg font-semibold text-[#FAFAFA] flex items-center gap-2">
										<Shield className="w-5 h-5 text-[#a1a1aa]" />
										TLS / SSL Certificates
									</CardTitle>
									<CardDescription className="text-xs text-[#a1a1aa] mt-1">Managed Let's Encrypt and custom SSL certificates in Traefik directory</CardDescription>
								</div>
								<Button size="sm" className="gap-1.5 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] cursor-pointer">
									<Plus className="w-3.5 h-3.5" />
									Add Certificate
								</Button>
							</div>

							<div className="mt-4 p-3.5 rounded-lg bg-amber-500/10 border border-amber-500/20 flex items-start gap-3 text-amber-400">
								<AlertTriangle className="h-4 w-4 shrink-0 mt-0.5" />
								<div>
									<p className="text-xs font-semibold">Warning</p>
									<p className="text-[11px] text-amber-400/90 mt-0.5 leading-relaxed">
										Certificates are stored in the Traefik directory. Using invalid certificates can break your Traefik proxy instance, preventing access to host routes.
									</p>
								</div>
							</div>
						</CardHeader>

						<CardContent className="p-6 pt-2 border-t border-[#262626]">
							<div className="flex flex-col gap-2.5 pt-2">
								{certificates.map((cert) => (
									<div key={cert.id} className="flex items-center justify-between p-4 rounded-xl border border-[#262626] bg-[#141414] hover:border-[#3f3f46] transition-all">
										<div className="flex items-center gap-3.5">
											<Shield className={`w-4 h-4 ${statusColor(cert.status)} shrink-0`} />
											<div className="flex flex-col gap-0.5">
												<div className="flex items-center gap-2">
													<span className="text-xs font-semibold text-[#FAFAFA]">{cert.name}</span>
													<Badge variant="outline" className="text-[10px] border-[#262626] text-[#a1a1aa] bg-[#262626]">{cert.type}</Badge>
													{cert.isChain && (
														<Badge variant="outline" className="text-[10px] border-[#262626] text-[#a1a1aa]">Chain ({cert.chainLength})</Badge>
													)}
												</div>
												<div className="flex items-center gap-2 text-[11px] text-[#737373] font-mono">
													<span>{cert.domain}</span>
													<span>·</span>
													<span>{cert.issuer}</span>
												</div>
											</div>
										</div>
										<div className="flex items-center gap-3">
											<div className="text-right">
												<p className={`text-xs ${statusColor(cert.status)} font-semibold font-mono`}>
													{cert.status === 'valid' ? 'Valid' : 'Expiring Soon'}
												</p>
												<p className="text-[10px] text-[#737373] font-mono">Expires {cert.expiresAt}</p>
											</div>
											{cert.autoRenew && (
												<Badge variant="outline" className="text-[9px] text-green-400 border-green-500/30 bg-green-500/10 font-mono">Auto-Renew</Badge>
											)}
											<button onClick={() => deleteCert(cert.id)} className="p-1.5 rounded-lg border border-[#262626] text-[#a1a1aa] hover:text-red-400 hover:bg-red-500/10 transition-colors cursor-pointer">
												<Trash2 className="w-3.5 h-3.5" />
											</button>
										</div>
									</div>
								))}
							</div>
						</CardContent>
					</Card>
				</div>
			</main>
		</PageLayout>
	);
}
