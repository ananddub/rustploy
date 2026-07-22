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

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	let firstName = $state(session?.user.first_name ?? 'Aditya');
	let lastName = $state(session?.user.last_name ?? 'Sahu');
	let email = $state(session?.user.email ?? 'admin@rustploy.dev');
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
	<!-- Top Breadcrumb Header Bar -->
	<header class="flex items-center justify-between px-6 py-3 border-b border-[#262626] text-xs bg-[#0A0A0A] shrink-0">
		<div class="flex items-center gap-2">
			<User class="w-3.5 h-3.5 text-[#a1a1aa]" />
			<span class="font-medium text-[#FAFAFA]">User Profile</span>
		</div>
	</header>

	<main class="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up min-h-0">
		<!-- Constrained Max-Width Shell (Dokploy Standard max-w-5xl) -->
		<div class="max-w-5xl space-y-6">
			<!-- Profile Information -->
			<Card.Root class="bg-[#171717] border border-[#262626] rounded-xl shadow-md overflow-hidden">
				<Card.Header class="p-6">
					<Card.Title class="text-base font-semibold text-[#FAFAFA] flex items-center gap-2">
						<User class="w-4 h-4 text-[#a1a1aa]" />
						Profile Information
					</Card.Title>
					<Card.Description class="text-xs text-[#a1a1aa] mt-1">Update your account details and personal credentials</Card.Description>
				</Card.Header>
				<Card.Content class="p-6 pt-0 space-y-5">
					<!-- Avatar Row -->
					<div class="flex items-center gap-4">
						<div class="w-14 h-14 rounded-full bg-[#262626] border border-white/10 flex items-center justify-center text-lg font-bold text-[#FAFAFA]">
							{initials}
						</div>
						<div class="space-y-1">
							<Button variant="outline" size="sm" class="gap-1.5 text-xs h-8 bg-[#141414] border-[#262626] text-[#FAFAFA] hover:bg-[#262626]">
								<Camera class="w-3.5 h-3.5" />
								Upload Avatar
							</Button>
							<p class="text-[11px] text-[#737373]">JPG, PNG or GIF. Max 2MB.</p>
						</div>
					</div>

					<div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
						<div class="space-y-1.5">
							<Label for="first-name" class="text-xs text-[#a1a1aa]">First Name</Label>
							<Input id="first-name" bind:value={firstName} placeholder="First Name" class="bg-[#141414] border-[#262626] text-[#FAFAFA]" />
						</div>
						<div class="space-y-1.5">
							<Label for="last-name" class="text-xs text-[#a1a1aa]">Last Name</Label>
							<Input id="last-name" bind:value={lastName} placeholder="Last Name" class="bg-[#141414] border-[#262626] text-[#FAFAFA]" />
						</div>
					</div>
					<div class="space-y-1.5">
						<Label for="email" class="text-xs text-[#a1a1aa]">Email Address</Label>
						<Input id="email" bind:value={email} type="email" disabled class="bg-[#141414] border-[#262626] text-[#a1a1aa] opacity-70" />
					</div>

					<Separator class="bg-[#262626]" />

					<div class="flex items-center justify-between">
						<div>
							<p class="text-xs font-semibold text-[#FAFAFA]">Allow Admin Impersonation</p>
							<p class="text-[11px] text-[#737373]">Allow workspace admins to temporarily log in to troubleshoot issue reports</p>
						</div>
						<Switch bind:checked={allowImpersonation} />
					</div>
				</Card.Content>
				<Card.Footer class="border-t border-[#262626] p-4 flex justify-end bg-[#141414]">
					<Button onclick={saveProfile} disabled={saving} size="sm" class="gap-1.5 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A]">
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
			<Card.Root class="bg-[#171717] border border-[#262626] rounded-xl shadow-md overflow-hidden">
				<Card.Header class="p-6">
					<Card.Title class="text-base font-semibold text-[#FAFAFA] flex items-center gap-2">
						<Shield class="w-4 h-4 text-[#a1a1aa]" />
						Security & Password
					</Card.Title>
					<Card.Description class="text-xs text-[#a1a1aa] mt-1">Manage two-factor authentication and password updates</Card.Description>
				</Card.Header>
				<Card.Content class="p-6 pt-0 space-y-5">
					<div class="flex items-center justify-between">
						<div>
							<p class="text-xs font-semibold text-[#FAFAFA]">Two-Factor Authentication (2FA)</p>
							<p class="text-[11px] text-[#737373]">Enforce TOTP authenticator code verification on sign in</p>
						</div>
						<Switch bind:checked={enable2FA} />
					</div>

					<Separator class="bg-[#262626]" />

					<div class="space-y-3">
						<h3 class="text-xs font-semibold text-[#FAFAFA]">Change Password</h3>
						<div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
							<div class="space-y-1.5">
								<Label for="current-pass" class="text-xs text-[#a1a1aa]">Current Password</Label>
								<Input id="current-pass" type="password" bind:value={currentPassword} placeholder="••••••••" class="bg-[#141414] border-[#262626] text-[#FAFAFA]" />
							</div>
							<div class="space-y-1.5">
								<Label for="new-pass" class="text-xs text-[#a1a1aa]">New Password</Label>
								<Input id="new-pass" type="password" bind:value={newPassword} placeholder="••••••••" class="bg-[#141414] border-[#262626] text-[#FAFAFA]" />
							</div>
							<div class="space-y-1.5">
								<Label for="confirm-pass" class="text-xs text-[#a1a1aa]">Confirm Password</Label>
								<Input id="confirm-pass" type="password" bind:value={confirmPassword} placeholder="••••••••" class="bg-[#141414] border-[#262626] text-[#FAFAFA]" />
							</div>
						</div>
					</div>
				</Card.Content>
				<Card.Footer class="border-t border-[#262626] p-4 flex justify-end bg-[#141414]">
					<Button variant="outline" size="sm" onclick={changePassword} disabled={savingPassword || !currentPassword || !newPassword || newPassword !== confirmPassword} class="gap-1.5 text-xs bg-[#262626] border-[#3f3f46] text-[#FAFAFA] hover:bg-[#333333]">
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
