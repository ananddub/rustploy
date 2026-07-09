import { createEffect } from 'solid-js';
import { useNavigate } from '@solidjs/router';
import { authSession } from '../lib/auth';

export default function Home() {
  const navigate = useNavigate();

  createEffect(() => {
    const session = authSession();
    console.log('[Home] authSession:', session ? 'EXISTS' : 'NULL');
    if (session) {
      console.log('[Home] → /dashboard');
      navigate('/dashboard', { replace: true });
      return;
    }
    console.log('[Home] → /auth');
    navigate('/auth', { replace: true });
  });

  return null;
}
