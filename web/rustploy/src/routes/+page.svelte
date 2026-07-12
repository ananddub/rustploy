<script lang="ts">
	import { goto } from '$app/navigation';
	import { getAuthSession } from '$lib/auth';

	const session = getAuthSession();
	if (session) {
		goto('/dashboard', { replaceState: true });
	} else {
		goto('/auth', { replaceState: true });
	}
</script>
