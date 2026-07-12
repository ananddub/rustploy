<script lang="ts">
	import { goto } from '$app/navigation';
	import { Users, Plus, Trash2, Shield, Mail, Loader2, MoreVertical } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import * as Select from '$lib/components/ui/select';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	let showInvite = $state(false);

	const users = [
		{ id: 1, email: 'admin@example.com', name: 'Admin User', role: 'owner', status: 'active', createdAt: '2026-01-15' },
		{ id: 2, email: 'dev@example.com', name: 'Dev User', role: 'admin', status: 'active', createdAt: '2026-03-20' },
		{ id: 3, email: 'ops@example.com', name: 'Ops User', role: 'member', status: 'active', createdAt: '2026-05-10' },
		{ id: 4, email: 'invited@example.com', name: '—', role: 'member', status: 'pending', createdAt: '2026-07-01' }
	];

	const invitations = [
		{ id: 1, email: 'newdev@example.com', role: 'member', expiresAt: '2026-07-20' }
	];

	const roleColors: Record<string, string> = {
		owner: 'default',
		admin: 'secondary',
		member: 'outline'
	};
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<Users class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Users</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto animate-fade-up">
		<div class="w-full space-y-6">
			<!-- Users Table -->
			<Card.Root>
				<Card.Header>
					<div class="flex items-center justify-between">
						<div>
							<Card.Title class="text-base flex items-center gap-2">
								<Users class="w-4 h-4 text-muted-foreground" />
								Team Members
							</Card.Title>
							<Card.Description>Manage who has access to your organization</Card.Description>
						</div>
						<Button size="sm" class="gap-1.5 text-xs" onclick={() => (showInvite = true)}>
							<Plus class="w-3.5 h-3.5" />
							Invite Member
						</Button>
					</div>
				</Card.Header>
				<Card.Content class="p-0">
					<Table.Root>
						<Table.Header>
							<Table.Row>
								<Table.Head>User</Table.Head>
								<Table.Head>Role</Table.Head>
								<Table.Head>Status</Table.Head>
								<Table.Head>Joined</Table.Head>
								<Table.Head class="w-12"></Table.Head>
							</Table.Row>
						</Table.Header>
						<Table.Body>
							{#each users as user (user.id)}
								<Table.Row>
									<Table.Cell>
										<div>
											<p class="text-sm font-medium">{user.name}</p>
											<p class="text-xs text-muted-foreground">{user.email}</p>
										</div>
									</Table.Cell>
									<Table.Cell>
										<Badge variant={roleColors[user.role] === 'default' ? 'default' : roleColors[user.role] === 'secondary' ? 'secondary' : 'outline'} class="text-[10px] capitalize">{user.role}</Badge>
									</Table.Cell>
									<Table.Cell>
										{#if user.status === 'active'}
											<Badge variant="default" class="text-[10px]">Active</Badge>
										{:else}
											<Badge variant="secondary" class="text-[10px]">Pending</Badge>
										{/if}
									</Table.Cell>
									<Table.Cell class="text-xs text-muted-foreground">{user.createdAt}</Table.Cell>
									<Table.Cell>
										<DropdownMenu.Root>
											<DropdownMenu.Trigger>
												<Button variant="ghost" size="sm" class="h-7 w-7 p-0">
													<MoreVertical class="w-3.5 h-3.5" />
												</Button>
											</DropdownMenu.Trigger>
											<DropdownMenu.Content align="end">
												<DropdownMenu.Item>Change Role</DropdownMenu.Item>
												<DropdownMenu.Item>Permissions</DropdownMenu.Item>
												<DropdownMenu.Separator />
												<DropdownMenu.Item class="text-destructive">Remove</DropdownMenu.Item>
											</DropdownMenu.Content>
										</DropdownMenu.Root>
									</Table.Cell>
								</Table.Row>
							{/each}
						</Table.Body>
					</Table.Root>
				</Card.Content>
			</Card.Root>

			<!-- Pending Invitations -->
			<Card.Root>
				<Card.Header>
					<Card.Title class="text-base flex items-center gap-2">
						<Mail class="w-4 h-4 text-muted-foreground" />
						Pending Invitations
					</Card.Title>
					<Card.Description>Invitations that haven't been accepted yet</Card.Description>
				</Card.Header>
				<Card.Content>
					{#if invitations.length === 0}
						<p class="text-sm text-muted-foreground text-center py-6">No pending invitations</p>
					{:else}
						<div class="flex flex-col gap-2">
							{#each invitations as inv (inv.id)}
								<div class="flex items-center justify-between p-3 rounded-lg border">
									<div class="flex items-center gap-3">
										<Mail class="w-4 h-4 text-muted-foreground" />
										<div>
											<p class="text-sm font-medium">{inv.email}</p>
											<p class="text-[11px] text-muted-foreground">Expires: {inv.expiresAt} · Role: {inv.role}</p>
										</div>
									</div>
									<Button variant="ghost" size="sm" class="h-7 w-7 p-0">
										<Trash2 class="w-3.5 h-3.5 text-destructive" />
									</Button>
								</div>
							{/each}
						</div>
					{/if}
				</Card.Content>
			</Card.Root>
		</div>
	</main>
</PageLayout>

<Dialog.Root bind:open={showInvite}>
	<Dialog.Content class="sm:max-w-md">
		<Dialog.Header>
			<Dialog.Title>Invite Member</Dialog.Title>
			<Dialog.Description>Send an invitation to join your organization</Dialog.Description>
		</Dialog.Header>
		<div class="space-y-4 py-4">
			<div class="space-y-1.5">
				<Label class="text-xs">Email Address</Label>
				<Input type="email" placeholder="user@example.com" />
			</div>
			<div class="space-y-1.5">
				<Label class="text-xs">Role</Label>
				<Select.Root type="single">
					<Select.Trigger class="w-full"><span class="text-sm text-muted-foreground">Select role...</span></Select.Trigger>
					<Select.Content>
						<Select.Item value="admin">Admin</Select.Item>
						<Select.Item value="member">Member</Select.Item>
					</Select.Content>
				</Select.Root>
			</div>
		</div>
		<Dialog.Footer>
			<Button variant="outline" size="sm" onclick={() => (showInvite = false)}>Cancel</Button>
			<Button size="sm" class="gap-1.5"><Mail class="w-3.5 h-3.5" />Send Invite</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
