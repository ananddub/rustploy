import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Cpu, Save } from 'lucide-react';
import { PageLayout } from '@/components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { Card, CardContent, CardHeader, CardTitle, CardDescription, CardFooter } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Switch } from '@/components/ui/switch';
import { toastSuccess } from '$lib/toast';

export default function ClusterPage() {
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const [clusterName, setClusterName] = useState('production-us-east');
	const [autoScale, setAutoScale] = useState(true);
	const [maxReplicas, setMaxReplicas] = useState('10');
	const [saving, setSaving] = useState(false);

	function save() {
		setSaving(true);
		setTimeout(() => {
			toastSuccess('Cluster configuration updated');
			setSaving(false);
		}, 400);
	}

	return (
		<PageLayout>
			<main className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up min-h-0">
				<div className="max-w-5xl mx-auto space-y-6">
					<Card className="bg-[#171717] border border-[#262626] rounded-xl shadow-md overflow-hidden">
						<CardHeader className="p-6">
							<CardTitle className="text-base font-semibold text-[#FAFAFA] flex items-center gap-2">
								<Cpu className="w-4 h-4 text-[#a1a1aa]" />
								Cluster Infrastructure Settings
							</CardTitle>
							<CardDescription className="text-xs text-[#a1a1aa] mt-1">Manage global node orchestration and auto-scaling rules</CardDescription>
						</CardHeader>
						<CardContent className="p-6 pt-0 space-y-5">
							<div className="space-y-1.5">
								<Label htmlFor="cluster-name" className="text-xs text-[#a1a1aa]">Cluster Identifier</Label>
								<Input id="cluster-name" value={clusterName} onChange={(e) => setClusterName(e.target.value)} className="bg-[#141414] border-[#262626] text-[#FAFAFA]" />
							</div>

							<div className="flex items-center justify-between">
								<div>
									<p className="text-xs font-semibold text-[#FAFAFA]">Auto-Scale Container Replicas</p>
									<p className="text-[11px] text-[#737373]">Automatically spawn instances when CPU load exceeds 80%</p>
								</div>
								<Switch checked={autoScale} onCheckedChange={(val) => setAutoScale(val)} />
							</div>

							<div className="space-y-1.5">
								<Label htmlFor="max-replicas" className="text-xs text-[#a1a1aa]">Max Replicas Per Service</Label>
								<Input id="max-replicas" type="number" value={maxReplicas} onChange={(e) => setMaxReplicas(e.target.value)} className="bg-[#141414] border-[#262626] text-[#FAFAFA]" />
							</div>
						</CardContent>
						<CardFooter className="border-t border-[#262626] p-4 flex justify-end bg-[#141414]">
							<Button onClick={save} disabled={saving} size="sm" className="gap-1.5 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] cursor-pointer">
								<Save className="w-3.5 h-3.5" /> {saving ? 'Saving…' : 'Save Cluster Config'}
							</Button>
						</CardFooter>
					</Card>
				</div>
			</main>
		</PageLayout>
	);
}
