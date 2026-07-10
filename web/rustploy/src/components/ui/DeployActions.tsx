import { createSignal } from 'solid-js';
import { Rocket, RefreshCw, Hammer, Play, Terminal } from 'lucide-solid';

type Props = {
  onDeploy: () => Promise<void>;
  onReload?: () => Promise<void>;
  onRebuild?: () => Promise<void>;
  onStart?: () => Promise<void>;
  showStop?: boolean;
  onStop?: () => Promise<void>;
  autoDeploy: boolean;
  onAutoDeploy: (v: boolean) => void;
  cleanCache: boolean;
  onCleanCache: (v: boolean) => void;
};

export default function DeployActions(props: Props) {
  const [deploying, setDeploying] = createSignal(false);
  const [reloading, setReloading] = createSignal(false);
  const [rebuilding, setRebuilding] = createSignal(false);
  const [starting, setStarting] = createSignal(false);
  const [stopping, setStopping] = createSignal(false);

  const run = (setter: (v: boolean) => void, fn?: () => Promise<void>) => async () => {
    if (!fn) return;
    setter(true);
    try { await fn(); } finally { setter(false); }
  };

  return (
    <div class="flex flex-wrap items-center gap-2">
      <button class="btn btn-neutral btn-sm gap-1.5" onClick={run(setDeploying, props.onDeploy)} disabled={deploying()}>
        {deploying() ? <span class="loading loading-spinner loading-xs" /> : <Rocket class="w-3.5 h-3.5" />}
        Deploy
      </button>

      <button class="btn btn-ghost btn-sm gap-1.5" onClick={run(setReloading, props.onReload)} disabled={reloading()}>
        {reloading() ? <span class="loading loading-spinner loading-xs" /> : <RefreshCw class="w-3.5 h-3.5" />}
        Reload
      </button>

      <button class="btn btn-ghost btn-sm gap-1.5" onClick={run(setRebuilding, props.onRebuild)} disabled={rebuilding()}>
        {rebuilding() ? <span class="loading loading-spinner loading-xs" /> : <Hammer class="w-3.5 h-3.5" />}
        Rebuild
      </button>

      <button class="btn btn-ghost btn-sm gap-1.5" onClick={run(setStarting, props.onStart)} disabled={starting()}>
        {starting() ? <span class="loading loading-spinner loading-xs" /> : <Play class="w-3.5 h-3.5" />}
        Start
      </button>

      <button class="btn btn-ghost btn-sm gap-1.5">
        <Terminal class="w-3.5 h-3.5" /> Open Terminal
      </button>

      <div class="flex items-center gap-2 ml-2">
        <span class="text-sm text-base-content/60">Autodeploy</span>
        <input
          type="checkbox"
          class="toggle toggle-sm"
          checked={props.autoDeploy}
          onChange={e => props.onAutoDeploy(e.currentTarget.checked)}
        />
      </div>

      <div class="flex items-center gap-2">
        <span class="text-sm text-base-content/60">Clean Cache</span>
        <input
          type="checkbox"
          class="toggle toggle-sm"
          checked={props.cleanCache}
          onChange={e => props.onCleanCache(e.currentTarget.checked)}
        />
      </div>
    </div>
  );
}
