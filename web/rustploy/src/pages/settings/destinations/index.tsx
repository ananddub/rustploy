import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Layers, Plus, Trash2 } from 'lucide-react';
import { PageLayout } from '$lib/../components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { getDestinationsMock, type DestinationMock } from '$lib/mocks';
import { Card } from '$lib/../components/ui/card';
import { Button } from '$lib/../components/ui/button';
import { toastSuccess } from '$lib/toast';

export default function DestinationsPage() {
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const [destinations, setDestinations] = useState<DestinationMock[]>(getDestinationsMock());

	function removeDest(id: string) {
		setDestinations(destinations.filter((d) => d.id !== id));
		toastSuccess('Destination removed');
	}

	return (
		<PageLayout>
			<main className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up min-h-0">
				<div className="max-w-5xl mx-auto space-y-6">
					<div className="flex items-center justify-between">
						<div>
							<h1 className="text-3xl font-bold tracking-tight text-[#FAFAFA]">S3 Backup Destinations</h1>
							<p className="text-sm text-[#a1a1aa] mt-1">Configure automated database export and backup storage locations</p>
						</div>
						<Button size="sm" className="gap-1.5 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] cursor-pointer">
							<Plus className="w-4 h-4" /> Add S3 Bucket
						</Button>
					</div>

					<div className="grid grid-cols-1 gap-4">
						{destinations.map((d) => (
							<Card key={d.id} className="bg-[#171717] border border-[#262626] rounded-xl p-5 flex items-center justify-between hover:border-[#3f3f46] transition-all">
								<div className="flex items-center gap-4">
									<div className="w-10 h-10 rounded-lg bg-[#262626] border border-white/10 flex items-center justify-center font-bold text-sm text-[#FAFAFA]">
										<Layers className="w-5 h-5" />
									</div>
									<div>
										<h2 className="text-base font-semibold text-[#FAFAFA]">{d.name}</h2>
										<p className="text-xs text-[#a1a1aa] font-mono mt-0.5">
											{d.provider} · Bucket: s3://{d.bucketName} ({d.region}) · {d.storageUsedGb} GB ({d.backupsCount} backups)
										</p>
									</div>
								</div>
								<button onClick={() => removeDest(d.id)} className="p-2 rounded-lg border border-[#262626] bg-[#262626] text-[#a1a1aa] hover:text-red-400 hover:bg-red-500/10 transition-colors cursor-pointer">
									<Trash2 className="w-4 h-4" />
								</button>
							</Card>
						))}
					</div>
				</div>
			</main>
		</PageLayout>
	);
}
