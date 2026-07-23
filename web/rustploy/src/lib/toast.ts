import { toast } from 'sonner';

export function toastSuccess(message: string, description?: string) {
	toast.success(message, { description });
}

export function toastError(message: string, description?: string) {
	toast.error(message, { description });
}

export function toastInfo(message: string, description?: string) {
	toast.info(message, { description });
}

export function toastLoading(message: string) {
	return toast.loading(message);
}

export function toastDismiss(id: string | number) {
	toast.dismiss(id);
}

export async function withToast<T>(
	fn: () => Promise<T>,
	opts: {
		loading?: string;
		success: string;
		error?: string;
		successDescription?: string;
	}
): Promise<T | null> {
	const id = opts.loading ? toast.loading(opts.loading) : undefined;
	try {
		const result = await fn();
		if (id != null) toast.dismiss(id);
		toast.success(opts.success, { description: opts.successDescription });
		return result;
	} catch (e: any) {
		if (id != null) toast.dismiss(id);
		toast.error(opts.error ?? 'Something went wrong', {
			description: e?.message ?? String(e)
		});
		return null;
	}
}
