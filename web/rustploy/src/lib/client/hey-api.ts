import type { CreateClientConfig } from './client.gen';

/**
 * Custom query serializer: form-explode style for objects (flat key=value),
 * standard for arrays and primitives.
 */
function formExplodeQuerySerializer(params: Record<string, unknown>): string {
	const parts: string[] = [];

	for (const [key, value] of Object.entries(params)) {
		if (value === undefined || value === null) continue;

		if (typeof value === 'object' && !Array.isArray(value)) {
			for (const [field, fieldVal] of Object.entries(value as Record<string, unknown>)) {
				if (fieldVal === undefined || fieldVal === null) continue;
				parts.push(`${encodeURIComponent(field)}=${encodeURIComponent(String(fieldVal))}`);
			}
		} else if (Array.isArray(value)) {
			for (const item of value) {
				parts.push(`${encodeURIComponent(key)}=${encodeURIComponent(String(item))}`);
			}
		} else {
			parts.push(`${encodeURIComponent(key)}=${encodeURIComponent(String(value))}`);
		}
	}

	return parts.join('&');
}

export const createClientConfig: CreateClientConfig = (config) => ({
	...config,
	baseUrl: 'http://das.tail25b5a0.ts.net:4000',
	querySerializer: formExplodeQuerySerializer
});
