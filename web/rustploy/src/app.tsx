import { Suspense, type Component } from 'solid-js';
import { client } from './client/client.gen';
import { authSession, refreshAccessToken, clearAuthSession } from './lib/auth';

// Setup interceptors once when app mounts
client.interceptors.request.use((request) => {
  const token = authSession()?.tokens.access_token;
  if (token) {
    request.headers.set('Authorization', `Bearer ${token}`);
  }
  return request;
});

// Auto-refresh on 401 — retry once with new token
// Skip auth endpoints to avoid infinite loops
client.interceptors.response.use(async (response, request) => {
  if (response.status !== 401) return response;

  const url = request.url;
  if (url.includes('/auth/')) return response; // never intercept auth endpoints

  const { refreshAccessToken, clearAuthSession } = await import('./lib/auth');
  const newToken = await refreshAccessToken();

  if (!newToken) {
    clearAuthSession();
    // Only redirect if not already on auth page
    if (!window.location.pathname.startsWith('/auth')) {
      window.location.href = '/auth';
    }
    return response;
  }

  return fetch(request, {
    headers: {
      ...Object.fromEntries((request.headers as Headers).entries()),
      Authorization: `Bearer ${newToken}`,
    },
  });
});

const App: Component<{ children: Element }> = (props) => {
  return (
    <main>
      <Suspense>{props.children}</Suspense>
    </main>
  );
};

export default App;
