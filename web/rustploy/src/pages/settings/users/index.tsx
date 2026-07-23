import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Users, Plus, Shield, Mail, Trash2, UserPlus } from 'lucide-react';
import { PageLayout } from '$lib/../components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '$lib/../components/ui/card';
import { Button } from '$lib/../components/ui/button';
import { Badge } from '$lib/../components/ui/badge';
import { toastSuccess } from '$lib/toast';

export default function UsersPage() {
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const [users, setUsers] = useState([
		{ id: 'usr-1', name: 'Aditya Sahu', email: 'admin@rustploy.dev', role: 'Owner', status: 'Active', joinedAt: 'Jun 12, 2026' },
		{ id: 'usr-2', name: 'Aman Kumar', email: 'aman@example.com', role: 'Admin', status: 'Active', joinedAt: 'Jun 15, 2026' },
		{ id: 'usr-3', name: 'Sarah Chen', email: 'sarah@example.com', role: 'Developer', status: 'Pending', joinedAt: 'Jul 01, 2026' }
	]);

	function removeUser(id: string) {
		setUsers((prev) => prev.filter((u) => u.id !== id));
		toastSuccess('User removed from organization');
	}

	return (
		<PageLayout>
			<header className="flex items-center justify-between px-6 py-3 border-b border-[#262626] text-xs bg-[#0A0A0A] shrink-0">
				<div className="flex items-center gap-2">
					<Users className="w-3.5 h-3.5 text-[#a1a1aa]" />
					<span className="font-medium text-[#FAFAFA]">Users & Roles</span>
				</div>
			</header>

			<main className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up min-h-0">
				<div className="max-w-5xl mx-auto space-y-6">
					<div className="flex items-center justify-between">
						<div>
							<h1 className="text-3xl font-bold tracking-tight text-[#FAFAFA]">Users & Access Control</h1>
							<p className="text-sm text-[#a1a1aa] mt-1">Manage team members, roles, and invitation status</p>
						</div>
						<Button size="sm" className="gap-1.5 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] cursor-pointer">
							<UserPlus className="w-4 h-4" /> Invite Member
						</Button>
					</div>

					<Card className="bg-[#171717] border border-[#262626] rounded-xl shadow-md overflow-hidden">
						<CardContent className="p-0">
							<table className="w-full text-left text-xs">
								<thead className="bg-[#141414] border-b border-[#262626] text-[#737373] uppercase tracking-wider font-semibold">
									<tr>
										<th className="px-5 py-3">MEMBER</th>
										<th className="px-5 py-3">ROLE</th>
										<th className="px-5 py-3">STATUS</th>
										<th className="px-5 py-3">JOINED</th>
										<th className="px-5 py-3 text-right">ACTION</th>
									</tr>
								</thead>
								<tbody className="divide-y divide-[#262626]">
									{users.map((u) => (
										<tr key={u.id} className="hover:bg-[#262626]/30 transition-colors">
											<td className="px-5 py-3.5 font-semibold text-[#FAFAFA]">
												<div className="flex items-center gap-2.5">
													<div className="w-7 h-7 rounded-full bg-[#262626] border border-white/10 flex items-center justify-center font-bold text-xs text-[#FAFAFA]">
														{u.name[0]}
													</div>
													<div>
														<p>{u.name}</p>
														<p className="text-[10px] font-mono text-[#737373]">{u.email}</p>
													</div>
												</div>
											</td>
											<td className="px-5 py-3.5">
												<Badge variant="outline" className="text-[10px] font-mono border-[#262626] text-[#FAFAFA] bg-[#262626]">
													{u.role}
												</Badge>
											</td>
											<td className="px-5 py-3.5">
												<Badge variant="outline" className={`text-[10px] ${u.status === 'Active' ? 'border-green-500/30 text-green-400 bg-green-500/10' : 'border-amber-500/30 text-amber-400 bg-amber-500/10'}`}>
													{u.status}
												</Badge>
											</td>
											<td className="px-5 py-3.5 font-mono text-[#a1a1aa]">{u.joinedAt}</td>
											<td className="px-5 py-3.5 text-right">
												{u.role !== 'Owner' && (
													<button onClick={() => removeUser(u.id)} className="p-1.5 rounded-lg border border-[#262626] text-[#a1a1aa] hover:text-red-400 hover:bg-red-500/10 transition-colors cursor-pointer">
														<Trash2 className="w-3.5 h-3.5" />
													</button>
												)}
											</td>
										</tr>
									))}
								</tbody>
							</table>
						</CardContent>
					</Card>
				</div>
			</main>
		</PageLayout>
	);
}
