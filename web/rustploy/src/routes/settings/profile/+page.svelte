<script lang="ts">
	import { goto } from '$app/navigation';
	import { User, Loader2, Palette, Shield, ShieldCheck, Eye, EyeOff } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession, setAuthSession } from '$lib/auth';
	import { authState } from '$lib/auth.svelte';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Switch } from '$lib/components/ui/switch';
	import { Avatar, AvatarFallback } from '$lib/components/ui/avatar';
	import { toastSuccess, toastError } from '$lib/toast';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	// ─── Form state ───────────────────────────────────────────────────────────────
	let firstName         = $state(session?.user.first_name ?? '');
	let lastName          = $state(session?.user.last_name ?? '');
	let email             = $state(session?.user.email ?? '');
	let currentPassword   = $state('');
	let newPassword       = $state('');
	let showCurrent       = $state(false);
	let showNew           = $state(false);
	let allowImpersonation= $state(false);
	let saving            = $state(false);

	// ─── Avatar ───────────────────────────────────────────────────────────────────
	// type: 'initials' | 'upload' | 'color' | preset url
	let avatarValue = $state('');  // '' = initials, '#hex' = color, 'data:...' = upload, url = preset
	let colorPickerEl: HTMLInputElement;

	const presetAvatars = [
		'https://api.dicebear.com/7.x/avataaars/svg?seed=Felix',
		'https://api.dicebear.com/7.x/avataaars/svg?seed=Aneka',
		'https://api.dicebear.com/7.x/avataaars/svg?seed=Milo',
		'https://api.dicebear.com/7.x/avataaars/svg?seed=Luna',
		'https://api.dicebear.com/7.x/avataaars/svg?seed=Nova',
		'https://api.dicebear.com/7.x/avataaars/svg?seed=Zara',
		'https://api.dicebear.com/7.x/avataaars/svg?seed=Finn',
		'https://api.dicebear.com/7.x/avataaars/svg?seed=Aria',
		'https://api.dicebear.com/7.x/avataaars/svg?seed=Koda',
		'https://api.dicebear.com/7.x/avataaars/svg?seed=Echo',
		'https://api.dicebear.com/7.x/avataaars/svg?seed=Sage',
		'https://api.dicebear.com/7.x/avataaars/svg?seed=Blaze',
	];

	const isColor  = $derived(avatarValue.startsWith('#'));
	const isUpload = $derived(avatarValue.startsWith('data:'));
	const isPreset = $derived(avatarValue.startsWith('http'));
	const isInitials = $derived(avatarValue === '');

	const initials = $derived(
		((firstName?.[0] ?? '') + (lastName?.[0] ?? email?.[0] ?? '')).toUpperCase() || 'U'
	);

	function handleFileUpload(e: Event) {
		const file = (e.target as HTMLInputElement).files?.[0];
		if (!file) return;
		if (file.size > 2 * 1024 * 1024) { toastError('Image must be under 2MB'); return; }
		const reader = new FileReader();
		reader.onload = (ev) => { avatarValue = ev.target?.result as string; };
		reader.readAsDataURL(file);
	}

	// ─── 2FA state ────────────────────────────────────────────────────────────────
	let twoFAEnabled    = $state(false);
	let show2FAModal    = $state(false);
	let twoFAStep       = $state<'qr'|'verify'|'done'>('qr');
	let twoFACode       = $state('');
	let twoFASaving     = $state(false);
	// Fake QR / secret
	const twoFASecret   = 'JBSWY3DPEHPK3PXP';
	const twoFAQrUrl    = `https://api.qrserver.com/v1/create-qr-code/?size=200x200&data=otpauth://totp/Rustploy:${encodeURIComponent(email)}?secret=${twoFASecret}%26issuer=Rustploy`;

	async function verify2FA() {
		if (twoFACode.length < 6) return;
		twoFASaving = true;
		await new Promise(r => setTimeout(r, 800));
		twoFASaving = false;
		twoFAEnabled = true;
		show2FAModal = false;
		twoFACode = '';
		twoFAStep = 'qr';
		toastSuccess('Two-factor authentication enabled');
	}

	async function disable2FA() {
		twoFAEnabled = false;
		toastSuccess('Two-factor authentication disabled');
	}

	// ─── Save ─────────────────────────────────────────────────────────────────────
	async function save() {
		saving = true;
		try {
			await new Promise(r => setTimeout(r, 700));
			// Sync session user data
			const s = getAuthSession();
			if (s) {
				setAuthSession({ ...s, user: { ...s.user, first_name: firstName, last_name: lastName, email } });
			}
			toastSuccess('Profile updated');
			currentPassword = ''; newPassword = '';
		} catch {
			toastError('Failed to update profile');
		} finally {
			saving = false;
		}
	}
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<User class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Profile</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto">
		<div class="max-w-5xl mx-auto">
			<div class="rounded-xl border border-border bg-card">

				<!-- Card header -->
				<div class="px-6 pt-5 pb-4 border-b border-border flex items-center justify-between flex-wrap gap-3">
					<div>
						<h2 class="text-xl font-bold flex items-center gap-2">
							<User class="w-5 h-5 text-muted-foreground" /> Account
						</h2>
						<p class="text-sm text-muted-foreground mt-0.5">Change the details of your profile here</p>
					</div>
					<!-- 2FA button -->
					{#if !twoFAEnabled}
						<Button variant="outline" size="sm" class="gap-1.5" onclick={() => (show2FAModal = true)}>
							<Shield class="w-4 h-4" /> Enable 2FA
						</Button>
					{:else}
						<Button variant="outline" size="sm" class="gap-1.5 text-green-500 border-green-500/30" onclick={disable2FA}>
							<ShieldCheck class="w-4 h-4" /> 2FA Enabled
						</Button>
					{/if}
				</div>

				<!-- Form body -->
				<div class="px-6 py-5 space-y-6">

					<!-- Avatar picker -->
					<div class="space-y-2">
						<Label class="text-xs text-muted-foreground">Avatar</Label>
						<div class="flex flex-wrap gap-2 items-center">
							<!-- Initials (default) -->
							<button
								class="w-12 h-12 rounded-full border-2 transition-all overflow-hidden {isInitials ? 'border-primary p-0.5' : 'border-transparent hover:border-primary/50'}"
								onclick={() => (avatarValue = '')}
								title="Use initials"
							>
								<div class="w-full h-full rounded-full bg-primary flex items-center justify-center text-primary-foreground text-sm font-bold">
									{initials}
								</div>
							</button>

							<!-- Upload -->
							<button
								class="w-12 h-12 rounded-full border-2 border-dashed transition-all overflow-hidden flex items-center justify-center bg-muted/50
									{isUpload ? 'border-primary p-0.5' : 'border-muted-foreground/40 hover:border-primary/50'}"
								onclick={() => document.getElementById('avatar-upload')?.click()}
								title="Upload photo"
							>
								{#if isUpload}
									<img src={avatarValue} alt="avatar" class="w-full h-full object-cover rounded-full" />
								{:else}
									<svg class="w-5 h-5 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
									</svg>
								{/if}
							</button>
							<input id="avatar-upload" type="file" accept="image/*" class="hidden" onchange={handleFileUpload} />

							<!-- Color picker -->
							<button
								class="w-12 h-12 rounded-full border-2 transition-all overflow-hidden flex items-center justify-center
									{isColor ? 'border-primary p-0.5' : 'border-transparent hover:border-primary/50'}"
								style={isColor ? `background:${avatarValue}` : ''}
								onclick={() => colorPickerEl?.click()}
								title="Choose color"
							>
								{#if !isColor}
									<div class="w-full h-full rounded-full bg-muted flex items-center justify-center">
										<Palette class="w-5 h-5 text-muted-foreground" />
									</div>
								{/if}
							</button>
							<input bind:this={colorPickerEl} type="color" class="sr-only"
								value={isColor ? avatarValue : '#6366f1'}
								oninput={(e) => (avatarValue = (e.target as HTMLInputElement).value)} />

							<!-- Preset avatars -->
							{#each presetAvatars as preset (preset)}
								<button
									class="w-12 h-12 rounded-full border-2 transition-all overflow-hidden
										{avatarValue === preset ? 'border-primary p-0.5' : 'border-transparent hover:border-primary/50'}"
									onclick={() => (avatarValue = preset)}
									title="Select avatar"
								>
									<img src={preset} alt="avatar" class="w-full h-full rounded-full object-cover" />
								</button>
							{/each}
						</div>
					</div>

					<!-- Name + email -->
					<div class="space-y-4">
						<div class="space-y-1.5">
							<Label for="first-name" class="text-xs">First Name</Label>
							<Input id="first-name" bind:value={firstName} placeholder="John" />
						</div>
						<div class="space-y-1.5">
							<Label for="last-name" class="text-xs">Last Name</Label>
							<Input id="last-name" bind:value={lastName} placeholder="Doe" />
						</div>
						<div class="space-y-1.5">
							<Label for="email" class="text-xs">Email</Label>
							<Input id="email" bind:value={email} type="email" placeholder="john@example.com" />
						</div>
					</div>

					<!-- Passwords -->
					<div class="space-y-4">
						<div class="space-y-1.5">
							<Label for="current-pass" class="text-xs">Current Password</Label>
							<div class="relative">
								<Input id="current-pass" type={showCurrent ? 'text' : 'password'}
									bind:value={currentPassword} placeholder="Current Password" class="pr-10" />
								<button type="button"
									class="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
									onclick={() => (showCurrent = !showCurrent)}>
									{#if showCurrent}<EyeOff class="w-4 h-4" />{:else}<Eye class="w-4 h-4" />{/if}
								</button>
							</div>
						</div>
						<div class="space-y-1.5">
							<Label for="new-pass" class="text-xs">New Password</Label>
							<div class="relative">
								<Input id="new-pass" type={showNew ? 'text' : 'password'}
									bind:value={newPassword} placeholder="New Password" class="pr-10" />
								<button type="button"
									class="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
									onclick={() => (showNew = !showNew)}>
									{#if showNew}<EyeOff class="w-4 h-4" />{:else}<Eye class="w-4 h-4" />{/if}
								</button>
							</div>
						</div>
					</div>

					<!-- Allow Impersonation -->
					<div class="flex items-center justify-between rounded-lg border border-border px-4 py-3">
						<div>
							<p class="text-sm font-medium">Allow Impersonation</p>
							<p class="text-xs text-muted-foreground mt-0.5">
								Enable this option to allow administrators to temporarily access your account for troubleshooting and support purposes.
							</p>
						</div>
						<Switch bind:checked={allowImpersonation} />
					</div>

					<!-- Save -->
					<div class="flex justify-end">
						<Button onclick={save} disabled={saving} class="gap-1.5">
							{#if saving}<Loader2 class="w-3.5 h-3.5 animate-spin" /> Saving…{:else}Save{/if}
						</Button>
					</div>
				</div>
			</div>
		</div>
	</main>
</PageLayout>

<!-- ── 2FA Modal ───────────────────────────────────────────────────────────── -->
{#if show2FAModal}
	<div class="fixed inset-0 bg-black/60 z-40" role="button" tabindex="-1"
		onclick={() => { show2FAModal = false; twoFACode = ''; twoFAStep = 'qr'; }} onkeydown={() => {}}></div>
	<div class="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none">
		<div class="bg-card border border-border rounded-xl shadow-2xl w-full max-w-md pointer-events-auto p-6 flex flex-col gap-5">

			<div>
				<h2 class="text-lg font-semibold flex items-center gap-2">
					<Shield class="w-5 h-5 text-primary" /> Enable Two-Factor Authentication
				</h2>
				<p class="text-sm text-muted-foreground mt-1">
					Scan the QR code with your authenticator app, then enter the 6-digit code to confirm.
				</p>
			</div>

			<!-- QR Code -->
			<div class="flex flex-col items-center gap-3">
				<div class="rounded-xl border border-border p-3 bg-white">
					<img src={twoFAQrUrl} alt="2FA QR Code" class="w-48 h-48" />
				</div>
				<div class="text-center">
					<p class="text-xs text-muted-foreground">Or enter this secret manually:</p>
					<code class="text-xs font-mono bg-muted px-2 py-1 rounded mt-1 inline-block tracking-widest">{twoFASecret}</code>
				</div>
			</div>

			<!-- Code input -->
			<div class="space-y-1.5">
				<Label for="totp" class="text-xs">Verification Code</Label>
				<Input id="totp" bind:value={twoFACode} placeholder="000000" maxlength={6}
					class="text-center font-mono text-lg tracking-widest h-12"
					oninput={(e) => { twoFACode = (e.target as HTMLInputElement).value.replace(/\D/g,'').slice(0,6); }} />
			</div>

			<div class="flex justify-end gap-2">
				<Button variant="outline" size="sm"
					onclick={() => { show2FAModal = false; twoFACode = ''; }}>Cancel</Button>
				<Button size="sm" onclick={verify2FA}
					disabled={twoFACode.length < 6 || twoFASaving} class="gap-1.5 min-w-[100px]">
					{#if twoFASaving}<Loader2 class="w-3.5 h-3.5 animate-spin" /> Verifying…{:else}Enable 2FA{/if}
				</Button>
			</div>
		</div>
	</div>
{/if}
