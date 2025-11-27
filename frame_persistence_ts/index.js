"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.snapshotSystem = snapshotSystem;
exports.restoreSystem = restoreSystem;
exports.snapshotToJson = snapshotToJson;
exports.snapshotFromJson = snapshotFromJson;
exports.compareSnapshots = compareSnapshots;
const frame_runtime_ts_1 = require("frame_runtime_ts");
function cloneValue(value) {
    if (Array.isArray(value)) {
        return value.slice();
    }
    if (value && typeof value === "object") {
        return { ...value };
    }
    return value;
}
function defaultDomainKeys(system) {
    if (system == null) {
        return [];
    }
    const keys = Object.keys(system);
    const result = [];
    for (const key of keys) {
        if (key.startsWith("_")) {
            continue;
        }
        const value = system[key];
        if (typeof value === "function") {
            continue;
        }
        result.push(key);
    }
    return result;
}
function snapshotSystem(system, opts) {
    if (system == null) {
        throw new Error("snapshotSystem expects a non-null system object");
    }
    const sysName = opts?.systemName ??
        (system.constructor && system.constructor.name) ??
        "";
    const compartment = system._compartment;
    if (!compartment) {
        throw new Error("snapshotSystem expects a V3 TypeScript system with a '_compartment' field");
    }
    const state = String(compartment.state ?? "");
    const rawStateArgs = compartment.stateArgs ?? {};
    const stateArgs = cloneValue(rawStateArgs);
    let domainState;
    if (opts?.encodeDomain) {
        domainState = { ...opts.encodeDomain(system) };
    }
    else {
        const domainKeys = opts?.domainKeys
            ? Array.from(opts.domainKeys)
            : defaultDomainKeys(system);
        domainState = {};
        for (const key of domainKeys) {
            if (Object.prototype.hasOwnProperty.call(system, key)) {
                domainState[key] = system[key];
            }
        }
    }
    const stackSnapshots = [];
    const stack = system._stack ?? [];
    if (Array.isArray(stack)) {
        for (const comp of stack) {
            if (!comp)
                continue;
            const s = String(comp.state ?? "");
            const rawArgs = comp.stateArgs ?? {};
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
function restoreSystem(snapshot, systemFactory, opts) {
    const sys = systemFactory();
    const comp = new frame_runtime_ts_1.FrameCompartment(snapshot.state, undefined, undefined, snapshot.stateArgs);
    sys._compartment = comp;
    const stack = [];
    for (const frame of snapshot.stack ?? []) {
        const c = new frame_runtime_ts_1.FrameCompartment(frame.state, undefined, undefined, frame.stateArgs);
        stack.push(c);
    }
    sys._stack = stack;
    const domain = snapshot.domainState ?? {};
    const keys = opts?.domainKeys
        ? Array.from(opts.domainKeys)
        : Object.keys(domain);
    for (const key of keys) {
        if (Object.prototype.hasOwnProperty.call(domain, key)) {
            sys[key] = domain[key];
        }
    }
    if (opts?.decodeDomain) {
        opts.decodeDomain(snapshot, sys);
    }
    return sys;
}
function snapshotToJson(snapshot, indent) {
    const spacing = indent !== undefined ? indent : 0;
    return JSON.stringify(snapshot, null, spacing);
}
function snapshotFromJson(text) {
    const raw = JSON.parse(text);
    if (typeof raw !== "object" || raw === null) {
        throw new Error("snapshotFromJson expected a JSON object at the top level");
    }
    const data = raw;
    const schemaVersion = typeof data.schemaVersion === "number" ? data.schemaVersion : 1;
    const systemName = data.systemName !== undefined ? String(data.systemName) : "";
    const state = data.state !== undefined ? String(data.state) : "";
    const stateArgs = data.stateArgs ?? {};
    const domainState = data.domainState ?? {};
    const stackRaw = Array.isArray(data.stack) ? data.stack : [];
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
function compareSnapshots(a, b) {
    const differences = [];
    if (a.schemaVersion !== b.schemaVersion) {
        differences.push(`schemaVersion: ${a.schemaVersion} != ${b.schemaVersion}`);
    }
    if (a.systemName !== b.systemName) {
        differences.push(`systemName: ${a.systemName} != ${b.systemName}`);
    }
    if (a.state !== b.state) {
        differences.push(`state: ${a.state} != ${b.state}`);
    }
    if (JSON.stringify(a.stateArgs) !== JSON.stringify(b.stateArgs)) {
        differences.push(`stateArgs differ: ${JSON.stringify(a.stateArgs)} != ${JSON.stringify(b.stateArgs)}`);
    }
    if (JSON.stringify(a.domainState) !== JSON.stringify(b.domainState)) {
        differences.push(`domainState differ: ${JSON.stringify(a.domainState)} != ${JSON.stringify(b.domainState)}`);
    }
    if (JSON.stringify(a.stack) !== JSON.stringify(b.stack)) {
        differences.push(`stack differ: ${JSON.stringify(a.stack)} != ${JSON.stringify(b.stack)}`);
    }
    return { equal: differences.length === 0, differences };
}
