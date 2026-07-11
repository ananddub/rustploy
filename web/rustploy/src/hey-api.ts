import type { CreateClientConfig } from './client/client.gen';

/**
 * Custom query serializer that respects OpenAPI `style: form, explode: true` on objects.
 *
 * The default hey-api serializer uses `deepObject` style for all objects, which produces
 * `?key[field]=value`. But some endpoints (e.g. /public/git/list_branches) use
 * `style: form, explode: true` which should produce flat `?field=value` pairs from
 * the object's own properties.
 *
 * This serializer uses form-explode for objects (flat) and falls back to standard
 * behaviour for arrays and primitives. It survives client regeneration since it lives
 * outside the generated files.
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
  querySerializer: formExplodeQuerySerializer,
});
