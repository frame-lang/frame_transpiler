import { FrameCompartment } from "frame_runtime_ts";

export interface SystemSnapshot {
  schemaVersion: number;
  systemName: string;
  state: string;
  // Mirrors the runtime FrameCompartment.stateArgs shape (object, array, or
  // primitive), rather than forcing a particular schema up front.
  stateArgs: any;
  domainState: Record<string, any>;
  stack: Array<{ state: string; stateArgs: any }>;
}

function cloneValue(value: any): any {
  if (Array.isArray(value)) {
    return value.slice();
  }
  if (value && typeof value === "object") {
    return { ...value };
  }
  return value;
}

function defaultDomainKeys(system: any): string[] {
  if (system == null) {
    return [];
  }
  const keys = Object.keys(system);
  const result: string[] = [];
  for (const key of keys) {
    if (key.startsWith("_")) {
      continue;
    }
    const value = (system as any)[key];
    if (typeof value === "function") {
      continue;
    }
    result.push(key);
  }
  return result;
}

export interface SnapshotSystemOptions {
  systemName?: string;
  domainKeys?: Iterable<string>;
  encodeDomain?: (system: any) => Record<string, any>;
}

export interface RestoreSystemOptions {
  domainKeys?: Iterable<string>;
  decodeDomain?: (snapshot: SystemSnapshot, system: any) => void;
}

export function snapshotSystem(
  system: any,
  opts?: SnapshotSystemOptions
): SystemSnapshot {
  if (system == null) {
    throw new Error("snapshotSystem expects a non-null system object");
  }

  const sysName =
    opts?.systemName ??
    (system.constructor && system.constructor.name) ??
    "";

  const compartment = (system as any)._compartment;
  if (!compartment) {
    throw new Error(
      "snapshotSystem expects a V3 TypeScript system with a '_compartment' field"
    );
  }

  const state = String(compartment.state ?? "");
  const rawStateArgs = compartment.stateArgs ?? {};
  const stateArgs = cloneValue(rawStateArgs);

  let domainState: Record<string, any>;
  if (opts?.encodeDomain) {
    domainState = { ...opts.encodeDomain(system) };
  } else {
    const domainKeys = opts?.domainKeys
      ? Array.from(opts.domainKeys)
      : defaultDomainKeys(system);
    domainState = {};
    for (const key of domainKeys) {
      if (Object.prototype.hasOwnProperty.call(system, key)) {
        domainState[key] = (system as any)[key];
      }
    }
  }

  const stackSnapshots: Array<{ state: string; stateArgs: any }> = [];
  const stack = (system as any)._stack ?? [];
  if (Array.isArray(stack)) {
    for (const comp of stack) {
      if (!comp) continue;
      const s = String(comp.state ?? "");
      const rawArgs = (comp as any).stateArgs ?? {};
      stackSnapshots.push({ state: s, stateArgs: cloneValue(rawArgs) });
    }
  }

  return {
    schemaVersion: 1,
    systemName: String(sysName),
    state,
    stateArgs,
    domainState,
    stack: stackSnapshots,
  };
}

export function restoreSystem(
  snapshot: SystemSnapshot,
  systemFactory: () => any,
  opts?: RestoreSystemOptions
): any {
  const sys = systemFactory();

  const comp = new FrameCompartment(
    snapshot.state,
    undefined,
    undefined,
    snapshot.stateArgs
  );
  (sys as any)._compartment = comp;

  const stack: FrameCompartment[] = [];
  for (const frame of snapshot.stack ?? []) {
    const c = new FrameCompartment(
      frame.state,
      undefined,
      undefined,
      frame.stateArgs
    );
    stack.push(c);
  }
  (sys as any)._stack = stack;

  const domain = snapshot.domainState ?? {};
  const keys = opts?.domainKeys
    ? Array.from(opts.domainKeys)
    : Object.keys(domain);
  for (const key of keys) {
    if (Object.prototype.hasOwnProperty.call(domain, key)) {
      (sys as any)[key] = (domain as any)[key];
    }
  }

  if (opts?.decodeDomain) {
    opts.decodeDomain(snapshot, sys);
  }

  return sys;
}

export function snapshotToJson(
  snapshot: SystemSnapshot,
  indent?: number
): string {
  const spacing = indent !== undefined ? indent : 0;
  return JSON.stringify(snapshot, null, spacing);
}

export function snapshotFromJson(text: string): SystemSnapshot {
  const raw = JSON.parse(text);
  if (typeof raw !== "object" || raw === null) {
    throw new Error(
      "snapshotFromJson expected a JSON object at the top level"
    );
  }
  const data: any = raw;
  const schemaVersion =
    typeof data.schemaVersion === "number" ? data.schemaVersion : 1;
  const systemName =
    data.systemName !== undefined ? String(data.systemName) : "";
  const state = data.state !== undefined ? String(data.state) : "";
  const stateArgs = data.stateArgs ?? {};
  const domainState =
    (data.domainState as Record<string, any> | undefined) ?? {};
  const stackRaw: any[] = Array.isArray(data.stack) ? data.stack : [];
  const stack = stackRaw.map((frame) => ({
    state: frame && frame.state !== undefined ? String(frame.state) : "",
    stateArgs: frame?.stateArgs ?? {},
  }));

  return {
    schemaVersion,
    systemName,
    state,
    stateArgs,
    domainState,
    stack,
  };
}
