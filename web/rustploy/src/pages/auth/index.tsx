import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Rocket, Eye, EyeOff, Loader2 } from 'lucide-react';
import { mockLogin, getAuthSession } from '$lib/auth';

export default function AuthPage() {
	const navigate = useNavigate();
	const [mode, setMode] = useState<'login' | 'signup'>('login');
	const [email, setEmail] = useState('admin@rustploy.dev');
	const [password, setPassword] = useState('password');
	const [showPassword, setShowPassword] = useState(false);
	const [loading, setLoading] = useState(false);
	const [error, setError] = useState('');

	if (getAuthSession()) {
		setTimeout(() => navigate('/dashboard', { replace: true }), 0);
	}

	function handleSubmit(e: React.FormEvent) {
		e.preventDefault();
		setError('');
		setLoading(true);
		setTimeout(() => {
			mockLogin(email);
			setLoading(false);
			navigate('/dashboard', { replace: true });
		}, 400);
	}

	return (
		<div className="min-h-screen flex bg-[#0A0A0A] text-[#FAFAFA]">
			<div className="hidden lg:flex lg:w-1/2 bg-[#141414] flex-col p-10 border-r border-[#262626]">
				<div className="flex items-center gap-2">
					<Rocket className="w-6 h-6 text-[#FAFAFA]" />
					<span className="font-semibold text-lg text-[#FAFAFA]">Rustploy</span>
				</div>
				<div className="mt-auto text-[#a1a1aa] text-sm italic">
					"The Open Source alternative to Netlify, Vercel, Heroku."
				</div>
			</div>

			<div className="w-full lg:w-1/2 flex items-center justify-center p-6">
				<div className="w-full max-w-sm">
					<div className="text-center mb-8">
						<div className="flex items-center justify-center gap-2 mb-2">
							<Rocket className="w-5 h-5 text-[#FAFAFA]" />
							<h1 className="text-xl font-semibold text-[#FAFAFA]">
								{mode === 'login' ? 'Sign in' : 'Create account'}
							</h1>
						</div>
						<p className="text-xs text-[#a1a1aa]">
							{mode === 'login'
								? 'Enter your email and password to sign in'
								: 'Fill in the details to create your account'}
						</p>
					</div>

					<form onSubmit={handleSubmit} className="flex flex-col gap-4">
						<div className="flex flex-col gap-1.5">
							<label htmlFor="email" className="text-xs font-medium text-[#a1a1aa]">
								Email
							</label>
							<input
								id="email"
								type="email"
								value={email}
								onChange={(e) => setEmail(e.target.value)}
								placeholder="admin@rustploy.dev"
								className="flex h-9 w-full rounded-md border border-[#262626] bg-[#141414] px-3 py-1 text-xs text-[#FAFAFA] placeholder:text-[#737373] focus:outline-none focus:ring-1 focus:ring-[#3f3f46]"
								required
							/>
						</div>

						<div className="flex flex-col gap-1.5">
							<label htmlFor="password" className="text-xs font-medium text-[#a1a1aa]">
								Password
							</label>
							<div className="relative">
								<input
									id="password"
									type={showPassword ? 'text' : 'password'}
									value={password}
									onChange={(e) => setPassword(e.target.value)}
									placeholder="••••••••"
									className="flex h-9 w-full rounded-md border border-[#262626] bg-[#141414] px-3 py-1 pr-10 text-xs text-[#FAFAFA] placeholder:text-[#737373] focus:outline-none focus:ring-1 focus:ring-[#3f3f46]"
									required
								/>
								<button
									type="button"
									onClick={() => setShowPassword(!showPassword)}
									className="absolute right-3 top-1/2 -translate-y-1/2 text-[#737373] hover:text-[#FAFAFA]"
								>
									{showPassword ? <EyeOff className="w-4 h-4" /> : <Eye className="w-4 h-4" />}
								</button>
							</div>
						</div>

						{error && (
							<div className="rounded-md bg-red-500/10 border border-red-500/30 px-3 py-2 text-xs text-red-400">
								{error}
							</div>
						)}

						<button
							type="submit"
							disabled={loading}
							className="inline-flex items-center justify-center gap-2 h-9 w-full rounded-md bg-[#FAFAFA] text-[#0A0A0A] text-xs font-semibold hover:bg-[#e4e4e7] transition-colors disabled:opacity-50 mt-1 cursor-pointer"
						>
							{loading ? (
								<>
									<Loader2 className="w-4 h-4 animate-spin" /> Signing in…
								</>
							) : mode === 'login' ? (
								'Login'
							) : (
								'Create Account'
							)}
						</button>
					</form>

					<div className="mt-6 text-center">
						<button
							onClick={() => setMode(mode === 'login' ? 'signup' : 'login')}
							className="text-xs text-[#a1a1aa] hover:text-[#FAFAFA] transition-colors cursor-pointer"
						>
							{mode === 'login' ? "Don't have an account? Sign up" : 'Already have an account? Sign in'}
						</button>
					</div>
				</div>
			</div>
		</div>
	);
}
