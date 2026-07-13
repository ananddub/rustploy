<script lang="ts">
	import { goto } from '$app/navigation';
	import {
		Users, Mail, MoreHorizontal, Loader2, Plus,
		Copy, Trash2, Shield, CheckCircle2, XCircle
	} from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Table from '$lib/components/ui/table';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Select from '$lib/components/ui/select';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { toastSuccess, toastError } from '$lib/toast';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	const currentUserId = session?.user.id ?? '';

	// ─── Types ────────────────────────────────────────────────────────────────────
	type Role = 'owner' | 'admin' | 'member';
	type Member = {
		id: string; userId: string; email: string; firstName: string; lastName: string;
		role: Role; banned: boolean; twoFactorEnabled: boolean; createdAt: string;
	};
	type Invitation = {
		id: string; email: string; role: Role;
		status: 'pending' | 'accepted' | 'canceled';
		expiresAt: string;
	};

	// ─── Static data ──────────────────────────────────────────────────────────────
	let members = $state<Member[]>([
		{ id:'m1', userId:'u1', email:'admin@example.com',  firstName:'Admin',  lastName:'User',  role:'owner',  banned:false, twoFactorEnabled:true,  createdAt:'2026-01-15T10:00:00Z' },
		{ id:'m2', userId:'u2', email:'dev@example.com',    firstName:'Dev',    lastName:'User',  role:'admin',  banned:false, twoFactorEnabled:false, createdAt:'2026-03-20T09:00:00Z' },
		{ id:'m3', userId:'u3', email:'ops@example.com',    firstName:'Ops',    lastName:'User',  role:'member', banned:false, twoFactorEnabled:false, createdAt:'2026-05-10T11:00:00Z' },
		{ id:'m4', userId:'u4', email:'banned@example.com', firstName:'Banned', lastName:'User',  role:'member', banned:true,  twoFactorEnabled:false, createdAt:'2026-06-01T08:00:00Z' },
	]);

	let invitations = $state<Invitation[]>([
		{ id:'i1', email:'newdev@example.com',  role:'member', status:'pending',  expiresAt:'2026-07-20T00:00:00Z' },
		{ id:'i2', email:'olddev@example.com',  role:'admin',  status:'canceled', expiresAt:'2026-07-10T00:00:00Z' },
	]);

	// ─── Helpers ──────────────────────────────────────────────────────────────────
	function fmtDate(iso: string) {
		return new Date(iso).toLocaleString(undefined, { month:'short', day:'numeric', year:'numeric', hour:'2-digit', minute:'2-digit' });
	}

	function isExpired(iso: string) { return new Date(iso) < new Date(); }

	function roleBadgeVariant(role: Role): 'default'|'secondary'|'outline' {
		if (role === 'owner') return 'default';
		if (role === 'admin') return 'secondary';
		return 'outline';
	}

	// ─── Change Role ──────────────────────────────────────────────────────────────
	let changeRoleId    = $state<string|null>(null);
	let changeRoleValue = $state<Role>('member');
	let changingRole    = $state(false);

	function openChangeRole(m: Member) { changeRoleId=m.id; changeRoleValue=m.role; }

	async function submitChangeRole() {
		if (!changeRoleId) return;
		changingRole = true;
		await new Promise(r => setTimeout(r, 400));
		members = members.map(m => m.id === changeRoleId ? { ...m, role:changeRoleValue } : m);
		changingRole = false; changeRoleId = null;
		toastSuccess('Role updated');
	}

	// ─── Delete member ────────────────────────────────────────────────────────────
	let confirmDeleteMemberId = $state<string|null>(null);
	let deletingMemberId      = $state<string|null>(null);

	async function deleteMember(id: string) {
		deletingMemberId = id;
		await new Promise(r => setTimeout(r, 400));
		members = members.filter(m => m.id !== id);
		deletingMemberId = null; confirmDeleteMemberId = null;
		toastSuccess('User deleted');
	}

	// ─── Invite modal ─────────────────────────────────────────────────────────────
	let showInvite  = $state(false);
	let iMode       = $state<'invitation'|'credentials'>('invitation');
	let iEmail      = $state('');
	let iRole       = $state<Role>('member');
	let iPassword   = $state('');
	let iConfirm    = $state('');
	let iSaving     = $state(false);
	let iError      = $state('');

	function openInvite() { iEmail=''; iRole='member'; iMode='invitation'; iPassword=''; iConfirm=''; iError=''; showInvite=true; }

	async function submitInvite(e: SubmitEvent) {
		e.preventDefault(); iError='';
		if (!iEmail.trim()) { iError='Email is required'; return; }
		if (iMode === 'credentials' && iPassword.length < 8) { iError='Password must be at least 8 characters'; return; }
		if (iMode === 'credentials' && iPassword !== iConfirm) { iError='Passwords do not match'; return; }
		iSaving = true;
		await new Promise(r => setTimeout(r, 500));
		invitations = [...invitations, { id:`i${Date.now()}`, email:iEmail.trim(), role:iRole, status:'pending', expiresAt:new Date(Date.now()+7*86400000).toISOString() }];
		iSaving = false; showInvite = false;
		toastSuccess('Invitation sent');
	}

	// ─── Invitation actions ───────────────────────────────────────────────────────
	function copyInvitation(id: string) {
		navigator.clipboard.writeText(`${location.origin}/invitation?token=${id}`);
		toastSuccess('Invitation copied to clipboard');
	}

	async function cancelInvitation(id: string) {
		invitations = invitations.map(i => i.id===id ? {...i, status:'canceled'} : i);
		toastSuccess('Invitation canceled');
	}

	async function removeInvitation(id: string) {
		invitations = invitations.filter(i => i.id !== id);
		toastSuccess('Invitation removed');
	}
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<Users class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Users</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto">
		<div class="w-full space-y-6">

			<!-- ── Users Card ─────────────────────────────────────────────────── -->
			<div class="rounded-xl border border-border bg-card">
				<div class="px-6 pt-5 pb-4 border-b border-border">
					<h2 class="text-xl font-bold flex items-center gap-2">
						<Users class="w-5 h-5 text-muted-foreground" /> Users
					</h2>
					<p class="text-sm text-muted-foreground mt-0.5">Add your users to your account</p>
				</div>
				<div class="px-6 py-4">
					{#if members.length === 0}
						<div class="flex flex-col items-center justify-center gap-3 min-h-[25vh] text-muted-foreground">
							<Users class="w-8 h-8 opacity-40" />
							<p class="text-base">Invite users to your organization</p>
						</div>
					{:else}
						<div class="rounded-md border border-border overflow-hidden">
							<Table.Root>
								<Table.Header>
									<Table.Row>
										<Table.Head class="pl-4">Email</Table.Head>
										<Table.Head class="text-center">Role</Table.Head>
										<Table.Head class="text-center">Status</Table.Head>
										<Table.Head class="text-center">2FA</Table.Head>
										<Table.Head class="text-center">Created At</Table.Head>
										<Table.Head class="text-right pr-4">Actions</Table.Head>
									</Table.Row>
								</Table.Header>
								<Table.Body>
									{#each members as member (member.id)}
										{@const isMe       = member.userId === currentUserId}
										{@const isOwner    = member.role === 'owner'}
										{@const canChange  = !isOwner && !isMe}
										{@const canDelete  = !isOwner && !isMe}

										<Table.Row>
											<Table.Cell class="text-sm pl-4">
												{member.email}
												{#if isMe}<span class="text-muted-foreground ml-1 text-xs">(You)</span>{/if}
											</Table.Cell>
											<Table.Cell class="text-center">
												<Badge variant={roleBadgeVariant(member.role)} class="text-[10px] capitalize">{member.role}</Badge>
											</Table.Cell>
											<Table.Cell class="text-center">
												<Badge variant={member.banned ? 'destructive' : 'outline'} class="text-[10px]">
													{member.banned ? 'Deactivated' : 'Active'}
												</Badge>
											</Table.Cell>
											<Table.Cell class="text-center text-xs text-muted-foreground">
												{member.twoFactorEnabled ? 'Enabled' : 'Disabled'}
											</Table.Cell>
											<Table.Cell class="text-center text-xs text-muted-foreground whitespace-nowrap">
												{fmtDate(member.createdAt)}
											</Table.Cell>
											<Table.Cell class="text-right pr-4">
												{#if canChange || canDelete}
													<DropdownMenu.Root>
														<DropdownMenu.Trigger>
															<Button variant="ghost" size="icon" class="h-8 w-8">
																<MoreHorizontal class="w-4 h-4" />
															</Button>
														</DropdownMenu.Trigger>
														<DropdownMenu.Content align="end">
															<DropdownMenu.Label class="text-xs">Actions</DropdownMenu.Label>
															{#if canChange}
																<DropdownMenu.Item class="text-xs cursor-pointer" onclick={() => openChangeRole(member)}>
																	Change Role
																</DropdownMenu.Item>
																<DropdownMenu.Item class="text-xs cursor-pointer">
																	Edit Permissions
																</DropdownMenu.Item>
															{/if}
															{#if canDelete}
																<DropdownMenu.Separator />
																<DropdownMenu.Item
																	class="text-xs cursor-pointer text-destructive focus:text-destructive"
																	onclick={() => (confirmDeleteMemberId = member.id)}>
																	Delete User
																</DropdownMenu.Item>
															{/if}
														</DropdownMenu.Content>
													</DropdownMenu.Root>
												{:else}
													<Button variant="ghost" size="icon" class="h-8 w-8" disabled>
														<MoreHorizontal class="w-4 h-4 text-muted-foreground/40" />
													</Button>
												{/if}
											</Table.Cell>
										</Table.Row>
									{/each}
								</Table.Body>
							</Table.Root>
						</div>
					{/if}
				</div>
			</div>

			<!-- ── Invitations Card ───────────────────────────────────────────── -->
			<div class="rounded-xl border border-border bg-card">
				<div class="px-6 pt-5 pb-4 border-b border-border">
					<h2 class="text-xl font-bold flex items-center gap-2">
						<Mail class="w-5 h-5 text-muted-foreground" /> Invitations
					</h2>
					<p class="text-sm text-muted-foreground mt-0.5">Create invitations to your organization</p>
				</div>
				<div class="px-6 py-4">
					{#if invitations.length === 0}
						<div class="flex flex-col items-center justify-center gap-3 min-h-[25vh] text-muted-foreground">
							<Users class="w-8 h-8 opacity-40" />
							<p class="text-base">Invite users to your organization</p>
							<Button size="sm" class="gap-1.5 mt-1" onclick={openInvite}>
								<Plus class="w-4 h-4" /> Add Invitation
							</Button>
						</div>
					{:else}
						<div class="rounded-md border border-border overflow-hidden">
							<Table.Root>
								<Table.Header>
									<Table.Row>
										<Table.Head class="pl-4">Email</Table.Head>
										<Table.Head class="text-center">Role</Table.Head>
										<Table.Head class="text-center">Status</Table.Head>
										<Table.Head class="text-center">Expires At</Table.Head>
										<Table.Head class="text-right pr-4">Actions</Table.Head>
									</Table.Row>
								</Table.Header>
								<Table.Body>
									{#each invitations as inv (inv.id)}
										{@const expired = isExpired(inv.expiresAt)}
										<Table.Row>
											<Table.Cell class="text-sm pl-4">{inv.email}</Table.Cell>
											<Table.Cell class="text-center">
												<Badge variant={roleBadgeVariant(inv.role)} class="text-[10px] capitalize">{inv.role}</Badge>
											</Table.Cell>
											<Table.Cell class="text-center">
												<Badge
													variant={inv.status === 'pending' ? 'secondary' : inv.status === 'canceled' ? 'destructive' : 'default'}
													class="text-[10px] capitalize"
												>{inv.status}</Badge>
											</Table.Cell>
											<Table.Cell class="text-center text-xs text-muted-foreground whitespace-nowrap">
												{fmtDate(inv.expiresAt)}
												{#if expired}<span class="ml-1">(Expired)</span>{/if}
											</Table.Cell>
											<Table.Cell class="text-right pr-4">
												<DropdownMenu.Root>
													<DropdownMenu.Trigger>
														<Button variant="ghost" size="icon" class="h-8 w-8">
															<MoreHorizontal class="w-4 h-4" />
														</Button>
													</DropdownMenu.Trigger>
													<DropdownMenu.Content align="end">
														<DropdownMenu.Label class="text-xs">Actions</DropdownMenu.Label>
														{#if !expired && inv.status === 'pending'}
															<DropdownMenu.Item class="text-xs cursor-pointer gap-2" onclick={() => copyInvitation(inv.id)}>
																<Copy class="w-3.5 h-3.5" /> Copy Invitation
															</DropdownMenu.Item>
															<DropdownMenu.Item class="text-xs cursor-pointer" onclick={() => cancelInvitation(inv.id)}>
																Cancel Invitation
															</DropdownMenu.Item>
														{/if}
														<DropdownMenu.Item class="text-xs cursor-pointer text-destructive focus:text-destructive" onclick={() => removeInvitation(inv.id)}>
															Remove Invitation
														</DropdownMenu.Item>
													</DropdownMenu.Content>
												</DropdownMenu.Root>
											</Table.Cell>
										</Table.Row>
									{/each}
								</Table.Body>
							</Table.Root>
						</div>

						<div class="flex justify-end pt-4">
							<Button size="sm" class="gap-1.5" onclick={openInvite}>
								<Plus class="w-4 h-4" /> Add Invitation
							</Button>
						</div>
					{/if}
				</div>
			</div>

		</div>
	</main>
</PageLayout>

<!-- ── Add Invitation Modal ───────────────────────────────────────────────── -->
{#if showInvite}
	<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1"
		onclick={() => (showInvite=false)} onkeydown={() => {}}></div>
	<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
		<form onsubmit={submitInvite}
			class="bg-card border border-border rounded-xl shadow-2xl w-full max-w-md pointer-events-auto flex flex-col gap-4 p-6 max-h-[90vh] overflow-y-auto">

			<div class="flex items-start justify-between">
				<div>
					<h2 class="text-base font-semibold">Add Invitation</h2>
					<p class="text-xs text-muted-foreground mt-0.5">Invite a user to join your organization</p>
				</div>
				<button type="button" onclick={() => (showInvite=false)}
					class="text-muted-foreground hover:text-foreground p-1 rounded hover:bg-accent">✕</button>
			</div>

			<!-- Mode toggle -->
			<div class="flex rounded-lg border border-border overflow-hidden">
				<button type="button"
					class="flex-1 py-2 text-xs font-medium transition-colors {iMode === 'invitation' ? 'bg-primary text-primary-foreground' : 'text-muted-foreground hover:bg-accent'}"
					onclick={() => (iMode='invitation')}>
					Invitation Link
				</button>
				<button type="button"
					class="flex-1 py-2 text-xs font-medium transition-colors {iMode === 'credentials' ? 'bg-primary text-primary-foreground' : 'text-muted-foreground hover:bg-accent'}"
					onclick={() => (iMode='credentials')}>
					Set Credentials
				</button>
			</div>

			<div class="space-y-1.5">
				<Label for="i-email" class="text-xs">Email <span class="text-destructive">*</span></Label>
				<Input id="i-email" type="email" bind:value={iEmail} placeholder="user@example.com" required />
			</div>

			<div class="space-y-1.5">
				<Label class="text-xs">Role <span class="text-destructive">*</span></Label>
				<Select.Root type="single" value={iRole} onValueChange={(v) => (iRole = (v ?? 'member') as Role)}>
					<Select.Trigger class="w-full">
						<span class="text-sm capitalize">{iRole}</span>
					</Select.Trigger>
					<Select.Content>
						<Select.Item value="admin">Admin</Select.Item>
						<Select.Item value="member">Member</Select.Item>
					</Select.Content>
				</Select.Root>
			</div>

			{#if iMode === 'credentials'}
				<div class="space-y-1.5">
					<Label for="i-pass" class="text-xs">Password <span class="text-destructive">*</span></Label>
					<Input id="i-pass" type="password" bind:value={iPassword} placeholder="Min 8 characters" />
				</div>
				<div class="space-y-1.5">
					<Label for="i-confirm" class="text-xs">Confirm Password <span class="text-destructive">*</span></Label>
					<Input id="i-confirm" type="password" bind:value={iConfirm} placeholder="Confirm password" />
				</div>
			{/if}

			{#if iError}
				<p class="text-xs text-destructive bg-destructive/10 border border-destructive/20 rounded px-3 py-2">{iError}</p>
			{/if}

			<div class="flex justify-end gap-2 pt-1">
				<Button type="button" variant="outline" size="sm" onclick={() => (showInvite=false)}>Cancel</Button>
				<Button type="submit" size="sm" disabled={iSaving} class="gap-1.5 min-w-[120px]">
					{#if iSaving}<Loader2 class="w-3.5 h-3.5 animate-spin" /> Sending…
					{:else}<Mail class="w-3.5 h-3.5" /> Send Invitation{/if}
				</Button>
			</div>
		</form>
	</div>
{/if}

<!-- ── Change Role Modal ──────────────────────────────────────────────────── -->
{#if changeRoleId}
	{@const target = members.find(m => m.id === changeRoleId)}
	<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1"
		onclick={() => (changeRoleId=null)} onkeydown={() => {}}></div>
	<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
		<div class="bg-card border border-border rounded-xl shadow-2xl w-full max-w-sm pointer-events-auto flex flex-col gap-4 p-6">
			<div>
				<h2 class="text-base font-semibold">Change Role</h2>
				<p class="text-xs text-muted-foreground mt-0.5">Change the role for <strong class="text-foreground">{target?.email}</strong></p>
			</div>
			<div class="space-y-1.5">
				<Label class="text-xs">Role</Label>
				<Select.Root type="single" value={changeRoleValue} onValueChange={(v) => (changeRoleValue = (v ?? 'member') as Role)}>
					<Select.Trigger class="w-full">
						<span class="text-sm capitalize">{changeRoleValue}</span>
					</Select.Trigger>
					<Select.Content>
						<Select.Item value="admin">Admin</Select.Item>
						<Select.Item value="member">Member</Select.Item>
					</Select.Content>
				</Select.Root>
			</div>
			<div class="flex justify-end gap-2">
				<Button variant="outline" size="sm" onclick={() => (changeRoleId=null)}>Cancel</Button>
				<Button size="sm" onclick={submitChangeRole} disabled={changingRole} class="gap-1.5 min-w-[80px]">
					{#if changingRole}<Loader2 class="w-3.5 h-3.5 animate-spin" />{:else}Save{/if}
				</Button>
			</div>
		</div>
	</div>
{/if}

<!-- ── Delete Confirm ─────────────────────────────────────────────────────── -->
{#if confirmDeleteMemberId}
	{@const target = members.find(m => m.id === confirmDeleteMemberId)}
	<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1"
		onclick={() => (confirmDeleteMemberId=null)} onkeydown={() => {}}></div>
	<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
		<div class="bg-card border border-border rounded-xl shadow-2xl w-full max-w-sm pointer-events-auto flex flex-col gap-4 p-6">
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 rounded-full bg-destructive/10 flex items-center justify-center shrink-0">
					<Trash2 class="w-5 h-5 text-destructive" />
				</div>
				<div>
					<h2 class="text-sm font-semibold">Delete User</h2>
					<p class="text-xs text-muted-foreground mt-0.5">
						Delete <strong class="text-foreground">{target?.email}</strong>? This cannot be undone.
					</p>
				</div>
			</div>
			<div class="flex justify-end gap-2">
				<Button variant="outline" size="sm" onclick={() => (confirmDeleteMemberId=null)}>Cancel</Button>
				<Button variant="destructive" size="sm"
					onclick={() => deleteMember(confirmDeleteMemberId!)}
					disabled={deletingMemberId !== null} class="gap-1.5">
					{#if deletingMemberId}<Loader2 class="w-3.5 h-3.5 animate-spin" /> Deleting…
					{:else}<Trash2 class="w-3.5 h-3.5" /> Delete{/if}
				</Button>
			</div>
		</div>
	</div>
{/if}
