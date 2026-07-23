import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Search, UserPlus, Trash2 } from 'lucide-react';
import { PageLayout } from '@/components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { getUsersMock, type UserMock } from '$lib/mocks';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Badge } from '@/components/ui/badge';
import { toastSuccess } from '$lib/toast';

export default function UsersPage() {
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const [users, setUsers] = useState<UserMock[]>(getUsersMock());
	const [searchQuery, setSearchQuery] = useState('');

	const filteredUsers = users.filter((u) =>
		!searchQuery ||
		u.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
		u.email.toLowerCase().includes(searchQuery.toLowerCase()) ||
		u.role.toLowerCase().includes(searchQuery.toLowerCase())
	);

	function removeUser(id: string) {
		setUsers((prev) => prev.filter((u) => u.id !== id));
		toastSuccess('User removed from organization');
	}

	return (
		<PageLayout>
			<main className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up min-h-0">
				<div className="max-w-5xl mx-auto space-y-6">
					<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
						<div>
							<h1 className="text-3xl font-bold tracking-tight text-[#FAFAFA]">Users & Access Control</h1>
							<p className="text-sm text-[#a1a1aa] mt-1">Manage team members, RBAC roles, and invitation status</p>
						</div>
						<Button size="sm" className="gap-1.5 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] cursor-pointer self-start sm:self-auto">
							<UserPlus className="w-4 h-4" /> Invite Member
						</Button>
					</div>

					<div className="relative max-w-sm">
						<Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-[#737373]" />
						<Input
							value={searchQuery}
							onChange={(e) => setSearchQuery(e.target.value)}
							placeholder="Search by user name or email..."
							className="pl-9 text-xs h-9 bg-[#141414] border-[#262626] text-[#FAFAFA]"
						/>
					</div>

					<Card className="bg-[#171717] border border-[#262626] rounded-xl shadow-md overflow-hidden">
						<CardContent className="p-0">
							<table className="w-full text-left text-xs">
								<thead className="bg-[#141414] border-b border-[#262626] text-[#737373] uppercase tracking-wider font-semibold">
									<tr>
										<th className="px-5 py-3">MEMBER</th>
										<th className="px-5 py-3">ROLE</th>
										<th className="px-5 py-3">STATUS</th>
										<th className="px-5 py-3">LAST ACTIVE</th>
										<th className="px-5 py-3">JOINED</th>
										<th className="px-5 py-3 text-right">ACTION</th>
									</tr>
								</thead>
								<tbody className="divide-y divide-[#262626]">
									{filteredUsers.map((u) => (
										<tr key={u.id} className="hover:bg-[#262626]/30 transition-colors">
											<td className="px-5 py-3.5 font-semibold text-[#FAFAFA]">
												<div className="flex items-center gap-2.5">
													<div className="w-8 h-8 rounded-full bg-[#262626] border border-white/10 flex items-center justify-center font-bold text-xs text-[#FAFAFA]">
														{u.name[0]}
													</div>
													<div>
														<p className="text-xs">{u.name}</p>
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
												<Badge
													variant="outline"
													className={`text-[10px] ${
														u.status === 'Active'
															? 'border-green-500/30 text-green-400 bg-green-500/10'
															: 'border-amber-500/30 text-amber-400 bg-amber-500/10'
													}`}
												>
													{u.status}
												</Badge>
											</td>
											<td className="px-5 py-3.5 font-mono text-[#a1a1aa]">{u.lastActive}</td>
											<td className="px-5 py-3.5 font-mono text-[#737373]">{u.joinedAt}</td>
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
