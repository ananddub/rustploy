import { createEffect } from 'solid-js';
import { useNavigate } from '@solidjs/router';
import { authSession } from '../lib/auth';

export default function Home() {
  const navigate = useNavigate();

  createEffect(() => {
    if (authSession()) {
      navigate('/dashboard', { replace: true });
      return;
    }

    navigate('/auth', { replace: true });
  });

  return null;
}
