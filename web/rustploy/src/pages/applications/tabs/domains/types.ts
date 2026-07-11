import type { DomainResponseDto } from '../../../../client/types.gen';

export type { DomainResponseDto };

export function domainUrl(d: DomainResponseDto): string {
  const scheme = d.https ? 'https' : 'http';
  const portSuffix =
    d.port && ((d.https && d.port !== 443) || (!d.https && d.port !== 80))
      ? `:${d.port}`
      : '';
  const pathSuffix = d.path && d.path !== '/' ? d.path : '';
  return `${scheme}://${d.host}${portSuffix}${pathSuffix}`;
}

export function formatDate(ts: number): string {
  return new Date(ts * 1000).toLocaleString(undefined, {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  });
}
