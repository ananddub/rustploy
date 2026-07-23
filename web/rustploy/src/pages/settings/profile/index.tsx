import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { User, Save, Camera, Key, Shield, Loader2 } from 'lucide-react';
import { PageLayout } from '@/components/PageLayout';
import { getAuthSession } from '$lib/auth';
import { Card, CardContent, CardHeader, CardTitle, CardDescription, CardFooter } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Switch } from '@/components/ui/switch';
import { toastSuccess } from '$lib/toast';

export default function ProfilePage() {
	const navigate = useNavigate();
	const session = getAuthSession();

	if (!session) {
		setTimeout(() => navigate('/auth', { replace: true }), 0);
	}

	const [firstName, setFirstName] = useState(session?.user.first_name || 'Aditya');
	const [lastName, setLastName] = useState(session?.user.last_name || 'Sahu');
	const [email] = useState(session?.user.email || 'admin@rustploy.dev');
	const [currentPassword, setCurrentPassword] = useState('');
	const [newPassword, setNewPassword] = useState('');
	const [confirmPassword, setConfirmPassword] = useState('');
	const [saving, setSaving] = useState(false);
	const [savingPassword, setSavingPassword] = useState(false);
	const [enable2FA, setEnable2FA] = useState(false);
	const [allowImpersonation, setAllowImpersonation] = useState(false);

	const initials = ((firstName?.[0] || '') + (lastName?.[0] || email?.[0] || '')).toUpperCase();

	function saveProfile() {
		setSaving(true);
		setTimeout(() => {
			toastSuccess('Profile information updated');
			setSaving(false);
		}, 400);
	}

	function changePassword() {
		if (newPassword !== confirmPassword) return;
		setSavingPassword(true);
		setTimeout(() => {
			toastSuccess('Password updated successfully');
			setSavingPassword(false);
			setCurrentPassword('');
			setNewPassword('');
			setConfirmPassword('');
		}, 400);
	}

	return (
		<PageLayout>
			<main className="flex-1 m-3.5 overflow-y-auto p-7 animate-fade-up min-h-0">
				<div className="max-w-5xl mx-auto space-y-6">
					<Card className="bg-[#171717] border border-[#262626] rounded-xl shadow-md overflow-hidden">
						<CardHeader className="p-6">
							<CardTitle className="text-base font-semibold text-[#FAFAFA] flex items-center gap-2">
								<User className="w-4 h-4 text-[#a1a1aa]" />
								Profile Information
							</CardTitle>
							<CardDescription className="text-xs text-[#a1a1aa] mt-1">Update your account details and personal credentials</CardDescription>
						</CardHeader>
						<CardContent className="p-6 pt-0 space-y-5">
							<div className="flex items-center gap-4">
								<div className="w-14 h-14 rounded-full bg-[#262626] border border-white/10 flex items-center justify-center text-lg font-bold text-[#FAFAFA]">
									{initials}
								</div>
								<div className="space-y-1">
									<Button variant="outline" size="sm" className="gap-1.5 text-xs h-8 bg-[#141414] border-[#262626] text-[#FAFAFA] hover:bg-[#262626] cursor-pointer">
										<Camera className="w-3.5 h-3.5" />
										Upload Avatar
									</Button>
									<p className="text-[11px] text-[#737373]">JPG, PNG or GIF. Max 2MB.</p>
								</div>
							</div>

							<div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
								<div className="space-y-1.5">
									<Label htmlFor="first-name" className="text-xs text-[#a1a1aa]">First Name</Label>
									<Input id="first-name" value={firstName} onChange={(e) => setFirstName(e.target.value)} placeholder="First Name" className="bg-[#141414] border-[#262626] text-[#FAFAFA]" />
								</div>
								<div className="space-y-1.5">
									<Label htmlFor="last-name" className="text-xs text-[#a1a1aa]">Last Name</Label>
									<Input id="last-name" value={lastName} onChange={(e) => setLastName(e.target.value)} placeholder="Last Name" className="bg-[#141414] border-[#262626] text-[#FAFAFA]" />
								</div>
							</div>
							<div className="space-y-1.5">
								<Label htmlFor="email" className="text-xs text-[#a1a1aa]">Email Address</Label>
								<Input id="email" value={email} type="email" disabled className="bg-[#141414] border-[#262626] text-[#a1a1aa] opacity-70" />
							</div>

							<div className="h-[1px] bg-[#262626] my-4" />

							<div className="flex items-center justify-between">
								<div>
									<p className="text-xs font-semibold text-[#FAFAFA]">Allow Admin Impersonation</p>
									<p className="text-[11px] text-[#737373]">Allow workspace admins to temporarily log in to troubleshoot issue reports</p>
								</div>
								<Switch checked={allowImpersonation} onCheckedChange={(val) => setAllowImpersonation(val)} />
							</div>
						</CardContent>
						<CardFooter className="border-t border-[#262626] p-4 flex justify-end bg-[#141414]">
							<Button onClick={saveProfile} disabled={saving} size="sm" className="gap-1.5 text-xs font-semibold bg-[#FAFAFA] hover:bg-[#e4e4e7] text-[#0A0A0A] cursor-pointer">
								{saving ? <Loader2 className="w-3.5 h-3.5 animate-spin" /> : <Save className="w-3.5 h-3.5" />}
								{saving ? 'Saving…' : 'Save Changes'}
							</Button>
						</CardFooter>
					</Card>

					<Card className="bg-[#171717] border border-[#262626] rounded-xl shadow-md overflow-hidden">
						<CardHeader className="p-6">
							<CardTitle className="text-base font-semibold text-[#FAFAFA] flex items-center gap-2">
								<Shield className="w-4 h-4 text-[#a1a1aa]" />
								Security & Password
							</CardTitle>
							<CardDescription className="text-xs text-[#a1a1aa] mt-1">Manage two-factor authentication and password updates</CardDescription>
						</CardHeader>
						<CardContent className="p-6 pt-0 space-y-5">
							<div className="flex items-center justify-between">
								<div>
									<p className="text-xs font-semibold text-[#FAFAFA]">Two-Factor Authentication (2FA)</p>
									<p className="text-[11px] text-[#737373]">Enforce TOTP authenticator code verification on sign in</p>
								</div>
								<Switch checked={enable2FA} onCheckedChange={(val) => setEnable2FA(val)} />
							</div>

							<div className="h-[1px] bg-[#262626] my-4" />

							<div className="space-y-3">
								<h3 className="text-xs font-semibold text-[#FAFAFA]">Change Password</h3>
								<div className="grid grid-cols-1 sm:grid-cols-3 gap-3">
									<div className="space-y-1.5">
										<Label htmlFor="current-pass" className="text-xs text-[#a1a1aa]">Current Password</Label>
										<Input id="current-pass" type="password" value={currentPassword} onChange={(e) => setCurrentPassword(e.target.value)} placeholder="••••••••" className="bg-[#141414] border-[#262626] text-[#FAFAFA]" />
									</div>
									<div className="space-y-1.5">
										<Label htmlFor="new-pass" className="text-xs text-[#a1a1aa]">New Password</Label>
										<Input id="new-pass" type="password" value={newPassword} onChange={(e) => setNewPassword(e.target.value)} placeholder="••••••••" className="bg-[#141414] border-[#262626] text-[#FAFAFA]" />
									</div>
									<div className="space-y-1.5">
										<Label htmlFor="confirm-pass" className="text-xs text-[#a1a1aa]">Confirm Password</Label>
										<Input id="confirm-pass" type="password" value={confirmPassword} onChange={(e) => setConfirmPassword(e.target.value)} placeholder="••••••••" className="bg-[#141414] border-[#262626] text-[#FAFAFA]" />
									</div>
								</div>
							</div>
						</CardContent>
						<CardFooter className="border-t border-[#262626] p-4 flex justify-end bg-[#141414]">
							<Button variant="outline" size="sm" onClick={changePassword} disabled={savingPassword || !currentPassword || !newPassword || newPassword !== confirmPassword} className="gap-1.5 text-xs bg-[#262626] border-[#3f3f46] text-[#FAFAFA] hover:bg-[#333333] cursor-pointer">
								{savingPassword ? <Loader2 className="w-3.5 h-3.5 animate-spin" /> : <Key className="w-3.5 h-3.5" />}
								Update Password
							</Button>
						</CardFooter>
					</Card>
				</div>
			</main>
		</PageLayout>
	);
}
