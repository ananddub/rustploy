<script lang="ts">
	import { goto } from '$app/navigation';
	import { User, Save, Camera, Key, Shield, Loader2 } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Separator } from '$lib/components/ui/separator';
	import { Switch } from '$lib/components/ui/switch';
	import { Avatar, AvatarFallback } from '$lib/components/ui/avatar';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	let firstName = $state(session?.user.first_name ?? '');
	let lastName = $state(session?.user.last_name ?? '');
	let email = $state(session?.user.email ?? '');
	let currentPassword = $state('');
	let newPassword = $state('');
	let confirmPassword = $state('');
	let saving = $state(false);
	let savingPassword = $state(false);
	let enable2FA = $state(false);
	let allowImpersonation = $state(false);

	const initials = $derived(
		((firstName?.[0] ?? '') + (lastName?.[0] ?? email?.[0] ?? '')).toUpperCase()
	);

	async function saveProfile() {
		saving = true;
		await new Promise((r) => setTimeout(r, 800));
		saving = false;
	}

	async function changePassword() {
		if (newPassword !== confirmPassword) return;
		savingPassword = true;
		await new Promise((r) => setTimeout(r, 800));
		savingPassword = false;
		currentPassword = '';
		newPassword = '';
		confirmPassword = '';
	}
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<User class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Profile</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto animate-fade-up">
		<div class="w-full space-y-6">
			<!-- Profile Information -->
			<Card.Root>
				<Card.Header>
					<Card.Title class="text-base flex items-center gap-2">
						<User class="w-4 h-4 text-muted-foreground" />
						Profile Information
					</Card.Title>
					<Card.Description>Update your account details and personal information</Card.Description>
				</Card.Header>
				<Card.Content class="space-y-5">
					<!-- Avatar Row -->
					<div class="flex items-center gap-4">
						<Avatar class="w-14 h-14 text-lg">
							<AvatarFallback class="bg-primary text-primary-foreground font-bold">{initials}</AvatarFallback>
						</Avatar>
						<div class="space-y-1">
							<Button variant="outline" size="sm" class="gap-1.5 text-xs h-8">
								<Camera class="w-3.5 h-3.5" />
								Upload Photo
							</Button>
							<p class="text-[11px] text-muted-foreground">JPG, PNG or GIF. Max 2MB.</p>
						</div>
					</div>

					<!-- Form fields -->
					<div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
						<div class="space-y-1.5">
							<Label for="first-name" class="text-xs">First Name</Label>
							<Input id="first-name" bind:value={firstName} placeholder="John" />
						</div>
						<div class="space-y-1.5">
							<Label for="last-name" class="text-xs">Last Name</Label>
							<Input id="last-name" bind:value={lastName} placeholder="Doe" />
						</div>
					</div>
					<div class="space-y-1.5">
						<Label for="email" class="text-xs">Email</Label>
						<Input id="email" bind:value={email} type="email" disabled class="opacity-60" />
						<p class="text-[11px] text-muted-foreground">Contact support to change your email address</p>
					</div>

					<Separator />

					<!-- Impersonation -->
					<div class="flex items-center justify-between">
						<div>
							<p class="text-sm font-medium">Allow Impersonation</p>
							<p class="text-[11px] text-muted-foreground">Allow administrators to impersonate your account for support</p>
						</div>
						<Switch bind:checked={allowImpersonation} />
					</div>
				</Card.Content>
				<Card.Footer class="border-t pt-4 flex justify-end">
					<Button onclick={saveProfile} disabled={saving} size="sm" class="gap-1.5 text-xs">
						{#if saving}
							<Loader2 class="w-3.5 h-3.5 animate-spin" />
							Saving...
						{:else}
							<Save class="w-3.5 h-3.5" />
							Save Changes
						{/if}
					</Button>
				</Card.Footer>
			</Card.Root>

			<!-- Security -->
			<Card.Root>
				<Card.Header>
					<Card.Title class="text-base flex items-center gap-2">
						<Shield class="w-4 h-4 text-muted-foreground" />
						Security
					</Card.Title>
					<Card.Description>Manage your account security and authentication</Card.Description>
				</Card.Header>
				<Card.Content class="space-y-5">
					<!-- 2FA -->
					<div class="flex items-center justify-between">
						<div>
							<p class="text-sm font-medium">Two-Factor Authentication</p>
							<p class="text-[11px] text-muted-foreground">Add an extra layer of security using TOTP</p>
						</div>
						<Switch bind:checked={enable2FA} />
					</div>

					<Separator />

					<!-- Change Password -->
					<div class="space-y-3">
						<h3 class="text-sm font-medium">Change Password</h3>
						<div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
							<div class="space-y-1.5">
								<Label for="current-pass" class="text-xs">Current Password</Label>
								<Input id="current-pass" type="password" bind:value={currentPassword} placeholder="••••••••" />
							</div>
							<div class="space-y-1.5">
								<Label for="new-pass" class="text-xs">New Password</Label>
								<Input id="new-pass" type="password" bind:value={newPassword} placeholder="••••••••" />
							</div>
							<div class="space-y-1.5">
								<Label for="confirm-pass" class="text-xs">Confirm Password</Label>
								<Input id="confirm-pass" type="password" bind:value={confirmPassword} placeholder="••••••••" />
							</div>
						</div>
					</div>
				</Card.Content>
				<Card.Footer class="border-t pt-4 flex justify-end">
					<Button variant="outline" size="sm" onclick={changePassword} disabled={savingPassword || !currentPassword || !newPassword || newPassword !== confirmPassword} class="gap-1.5 text-xs">
						{#if savingPassword}
							<Loader2 class="w-3.5 h-3.5 animate-spin" />
						{:else}
							<Key class="w-3.5 h-3.5" />
						{/if}
						Update Password
					</Button>
				</Card.Footer>
			</Card.Root>
		</div>
	</main>
</PageLayout>
