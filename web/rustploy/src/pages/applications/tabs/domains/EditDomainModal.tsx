import { createSignal, Show } from 'solid-js';
import { Info, Copy } from 'lucide-solid';
import { domainControllerPatch } from '../../../../client/sdk.gen';
import type { DomainResponseDto, PatchDomainDto } from '../../../../client/types.gen';
import Modal from '../../../../components/ui/Modal';

type Props = {
  domain: DomainResponseDto;
  onClose: () => void;
  onUpdated: (d: DomainResponseDto) => void;
};

const labelCls = 'block text-sm font-medium text-base-content mb-1.5';
const inputCls = 'input input-bordered w-full bg-base-300 border-base-300 focus:border-base-content/30';

export default function EditDomainModal(props: Props) {
  const [host, setHost] = createSignal(props.domain.host);
  const [port, setPort] = createSignal(props.domain.port ? String(props.domain.port) : '3000');
  const [path, setPath] = createSignal(props.domain.path ?? '/');
  const [internalPath, setInternalPath] = createSignal(props.domain.internal_path ?? '/');
  const [https, setHttps] = createSignal(props.domain.https);
  const [stripPath, setStripPath] = createSignal(props.domain.strip_path);
  const [certificateType, setCertificateType] = createSignal(props.domain.certificate_type);
  const [middlewares, setMiddlewares] = createSignal(props.domain.middlewares);
  const [serviceName, setServiceName] = createSignal(props.domain.service_name ?? '');
  const [customCertResolver, setCustomCertResolver] = createSignal(props.domain.custom_cert_resolver ?? '');
  const [customEntrypointEnabled, setCustomEntrypointEnabled] = createSignal(!!props.domain.custom_entrypoint);
  const [customEntrypoint, setCustomEntrypoint] = createSignal(props.domain.custom_entrypoint ?? '');

  const [loading, setLoading] = createSignal(false);
  const [error, setError] = createSignal('');

  const submit = async (e: Event) => {
    e.preventDefault();
    setError('');
    setLoading(true);
    try {
      const body: PatchDomainDto = {
        host: host().trim() || undefined,
        https: https(),
        path: path() || undefined,
        internal_path: internalPath() || undefined,
        port: port() ? parseInt(port()) : undefined,
        strip_path: stripPath(),
        certificate_type: certificateType(),
        middlewares: middlewares(),
        service_name: serviceName() || undefined,
        custom_cert_resolver: customCertResolver() || undefined,
        custom_entrypoint: customEntrypointEnabled() ? (customEntrypoint() || undefined) : undefined,
      };
      const res = await domainControllerPatch({ path: { id: props.domain.id }, body });
      if (res.error || !res.data)
        throw new Error((res.error as any)?.message ?? 'Failed to update domain');
      props.onUpdated(res.data);
      props.onClose();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Something went wrong');
    } finally {
      setLoading(false);
    }
  };

  return (
    <Modal
      title="Domain"
      subtitle="In this section you can edit this domain"
      onClose={props.onClose}
      width="max-w-lg"
    >
      <form onSubmit={submit} class="flex flex-col gap-5">

        {/* Info banner */}
        <div class="flex items-start gap-3 bg-blue-500/10 border border-blue-500/30 rounded-lg px-4 py-3">
          <Info class="w-4 h-4 text-blue-400 mt-0.5 shrink-0" />
          <p class="text-sm text-blue-400 leading-snug">
            Whenever you make changes to domains, remember to redeploy your application to apply the changes.
          </p>
        </div>

        {/* Host */}
        <div>
          <label class={labelCls}>Host</label>
          <div class="flex items-center gap-2">
            <input
              class={`${inputCls} flex-1`}
              placeholder="api.example.com"
              value={host()}
              onInput={e => setHost(e.currentTarget.value)}
              required
              autofocus
            />
            <button type="button" class="btn btn-square btn-sm btn-ghost border border-base-300 bg-base-300" title="Copy">
              <Copy class="w-4 h-4 text-base-content/50" />
            </button>
          </div>
        </div>

        {/* Path */}
        <div>
          <label class={labelCls}>Path</label>
          <input
            class={inputCls}
            placeholder="/"
            value={path()}
            onInput={e => setPath(e.currentTarget.value)}
          />
        </div>

        {/* Internal Path */}
        <div>
          <label class={labelCls}>Internal Path</label>
          <p class="text-xs text-base-content/40 mb-1.5">
            The path where your application expects to receive requests internally (defaults to "/")
          </p>
          <input
            class={inputCls}
            placeholder="/"
            value={internalPath()}
            onInput={e => setInternalPath(e.currentTarget.value)}
          />
        </div>

        {/* Strip Path */}
        <div class="border border-base-300 rounded-lg px-4 py-3 flex items-start justify-between gap-4 bg-base-300/40">
          <div>
            <p class="text-sm font-medium text-base-content">Strip Path</p>
            <p class="text-xs text-base-content/50 mt-0.5">
              Remove the external path from the request before forwarding to the application
            </p>
          </div>
          <input
            id="edit-strip-path"
            type="checkbox"
            class="toggle toggle-sm mt-0.5 shrink-0"
            checked={stripPath()}
            onChange={e => setStripPath(e.currentTarget.checked)}
          />
        </div>

        {/* Container Port */}
        <div>
          <label class={labelCls}>Container Port</label>
          <p class="text-xs text-base-content/40 mb-1.5">
            The port where your application is running inside the container (e.g., 3000 for Node.js, 80 for Nginx, 8080 for Java)
          </p>
          <input
            class={inputCls}
            placeholder="3000"
            type="number"
            min="1"
            max="65535"
            value={port()}
            onInput={e => setPort(e.currentTarget.value)}
          />
        </div>

        {/* Custom Entrypoint */}
        <div class="border border-base-300 rounded-lg px-4 py-3 flex items-start justify-between gap-4 bg-base-300/40">
          <div>
            <p class="text-sm font-medium text-base-content">Custom Entrypoint</p>
            <p class="text-xs text-base-content/50 mt-0.5">
              Use custom entrypoint for domain<br />
              "web" and/or "websecure" is used by default.
            </p>
          </div>
          <input
            id="edit-custom-ep"
            type="checkbox"
            class="toggle toggle-sm mt-0.5 shrink-0"
            checked={customEntrypointEnabled()}
            onChange={e => setCustomEntrypointEnabled(e.currentTarget.checked)}
          />
        </div>

        <Show when={customEntrypointEnabled()}>
          <div>
            <label class={labelCls}>Entrypoint</label>
            <input
              class={inputCls}
              placeholder="websecure"
              value={customEntrypoint()}
              onInput={e => setCustomEntrypoint(e.currentTarget.value)}
            />
          </div>
        </Show>

        {/* HTTPS */}
        <div class="border border-base-300 rounded-lg px-4 py-3 flex items-start justify-between gap-4 bg-base-300/40">
          <div>
            <p class="text-sm font-medium text-base-content">HTTPS</p>
            <p class="text-xs text-base-content/50 mt-0.5">
              Automatically provision SSL Certificate.
            </p>
          </div>
          <input
            id="edit-https"
            type="checkbox"
            class="toggle toggle-sm mt-0.5 shrink-0"
            checked={https()}
            onChange={e => setHttps(e.currentTarget.checked)}
          />
        </div>

        <Show when={https()}>
          <div>
            <label class={labelCls}>Certificate Type</label>
            <select
              class="select select-bordered w-full bg-base-300 border-base-300 focus:border-base-content/30"
              value={certificateType()}
              onChange={e => setCertificateType(e.currentTarget.value)}
            >
              <option value="none">None</option>
              <option value="letsencrypt">Let's Encrypt</option>
              <option value="custom">Custom</option>
            </select>
          </div>
          <Show when={certificateType() === 'custom'}>
            <div>
              <label class={labelCls}>Custom Cert Resolver</label>
              <input
                class={inputCls}
                placeholder="myresolver"
                value={customCertResolver()}
                onInput={e => setCustomCertResolver(e.currentTarget.value)}
              />
            </div>
          </Show>
        </Show>

        {/* Middlewares */}
        <div>
          <label class={labelCls}>
            Middlewares{' '}
            <span class="text-base-content/40 text-xs font-normal ml-1">?</span>
          </label>
          <div class="flex items-center gap-2">
            <input
              class={`${inputCls} flex-1`}
              placeholder="e.g., rate-limit@file, auth@file"
              value={middlewares()}
              onInput={e => setMiddlewares(e.currentTarget.value)}
            />
            <button
              type="button"
              class="btn btn-sm btn-ghost border border-base-300 bg-base-300 text-base-content/70 px-4"
            >
              Add
            </button>
          </div>
        </div>

        {/* Domain Type — read only */}
        <div>
          <label class={labelCls}>Domain Type</label>
          <input
            class={`${inputCls} opacity-50 cursor-not-allowed`}
            value={props.domain.domain_type}
            disabled
            title="Domain type cannot be changed after creation"
          />
        </div>

        <Show when={error()}>
          <div class="alert alert-error text-sm py-2">
            <span>{error()}</span>
          </div>
        </Show>

        <div class="flex justify-end pt-1">
          <button
            type="submit"
            class="btn btn-neutral btn-sm px-6 gap-1.5"
            disabled={loading()}
          >
            {loading() && <span class="loading loading-spinner loading-xs" />}
            {loading() ? 'Saving…' : 'Save Changes'}
          </button>
        </div>

      </form>
    </Modal>
  );
}
