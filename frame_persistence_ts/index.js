"use strict";
var __assign = (this && this.__assign) || function () {
    __assign = Object.assign || function(t) {
        for (var s, i = 1, n = arguments.length; i < n; i++) {
            s = arguments[i];
            for (var p in s) if (Object.prototype.hasOwnProperty.call(s, p))
                t[p] = s[p];
        }
        return t;
    };
    return __assign.apply(this, arguments);
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.snapshotSystem = snapshotSystem;
exports.restoreSystem = restoreSystem;
exports.snapshotToJson = snapshotToJson;
exports.snapshotFromJson = snapshotFromJson;
exports.compareSnapshots = compareSnapshots;
var frame_runtime_ts_1 = require("frame_runtime_ts");
function cloneValue(value) {
    if (Array.isArray(value)) {
        return value.slice();
    }
    if (value && typeof value === "object") {
        return __assign({}, value);
    }
    return value;
}
function defaultDomainKeys(system) {
    if (system == null) {
        return [];
    }
    var keys = Object.keys(system);
    var result = [];
    for (var _i = 0, keys_1 = keys; _i < keys_1.length; _i++) {
        var key = keys_1[_i];
        if (key.startsWith("_")) {
            continue;
        }
        var value = system[key];
        if (typeof value === "function") {
            continue;
        }
        result.push(key);
    }
    return result;
}
function snapshotSystem(system, opts) {
    var _a, _b, _c, _d, _e, _f, _g;
    if (system == null) {
        throw new Error("snapshotSystem expects a non-null system object");
    }
    var sysName = (_b = (_a = opts === null || opts === void 0 ? void 0 : opts.systemName) !== null && _a !== void 0 ? _a : (system.constructor && system.constructor.name)) !== null && _b !== void 0 ? _b : "";
    var compartment = system._compartment;
    if (!compartment) {
        throw new Error("snapshotSystem expects a V3 TypeScript system with a '_compartment' field");
    }
    var state = String((_c = compartment.state) !== null && _c !== void 0 ? _c : "");
    var rawStateArgs = (_d = compartment.stateArgs) !== null && _d !== void 0 ? _d : {};
    var stateArgs = cloneValue(rawStateArgs);
    var domainState;
    if (opts === null || opts === void 0 ? void 0 : opts.encodeDomain) {
        domainState = __assign({}, opts.encodeDomain(system));
    }
    else {
        var domainKeys = (opts === null || opts === void 0 ? void 0 : opts.domainKeys)
            ? Array.from(opts.domainKeys)
            : defaultDomainKeys(system);
        domainState = {};
        for (var _i = 0, domainKeys_1 = domainKeys; _i < domainKeys_1.length; _i++) {
            var key = domainKeys_1[_i];
            if (Object.prototype.hasOwnProperty.call(system, key)) {
                domainState[key] = system[key];
            }
        }
    }
    var stackSnapshots = [];
    var stack = (_e = system._stack) !== null && _e !== void 0 ? _e : [];
    if (Array.isArray(stack)) {
        for (var _h = 0, stack_1 = stack; _h < stack_1.length; _h++) {
            var comp = stack_1[_h];
            if (!comp)
                continue;
            var s = String((_f = comp.state) !== null && _f !== void 0 ? _f : "");
            var rawArgs = (_g = comp.stateArgs) !== null && _g !== void 0 ? _g : {};
            stackSnapshots.push({ state: s, stateArgs: cloneValue(rawArgs) });
        }
    }
    return {
        schemaVersion: 1,
        systemName: String(sysName),
        state: state,
        stateArgs: stateArgs,
        domainState: domainState,
        stack: stackSnapshots,
    };
}
function restoreSystem(snapshot, systemFactory, opts) {
    var _a, _b;
    var sys = systemFactory();
    var comp = new frame_runtime_ts_1.FrameCompartment(snapshot.state, undefined, undefined, snapshot.stateArgs);
    sys._compartment = comp;
    var stack = [];
    for (var _i = 0, _c = (_a = snapshot.stack) !== null && _a !== void 0 ? _a : []; _i < _c.length; _i++) {
        var frame = _c[_i];
        var c = new frame_runtime_ts_1.FrameCompartment(frame.state, undefined, undefined, frame.stateArgs);
        stack.push(c);
    }
    sys._stack = stack;
    var domain = (_b = snapshot.domainState) !== null && _b !== void 0 ? _b : {};
    var keys = (opts === null || opts === void 0 ? void 0 : opts.domainKeys)
        ? Array.from(opts.domainKeys)
        : Object.keys(domain);
    for (var _d = 0, keys_2 = keys; _d < keys_2.length; _d++) {
        var key = keys_2[_d];
        if (Object.prototype.hasOwnProperty.call(domain, key)) {
            sys[key] = domain[key];
        }
    }
    if (opts === null || opts === void 0 ? void 0 : opts.decodeDomain) {
        opts.decodeDomain(snapshot, sys);
    }
    return sys;
}
function snapshotToJson(snapshot, indent) {
    var spacing = indent !== undefined ? indent : 0;
    return JSON.stringify(snapshot, null, spacing);
}
function snapshotFromJson(text) {
    var _a, _b;
    var raw = JSON.parse(text);
    if (typeof raw !== "object" || raw === null) {
        throw new Error("snapshotFromJson expected a JSON object at the top level");
    }
    var data = raw;
    var schemaVersion = typeof data.schemaVersion === "number" ? data.schemaVersion : 1;
    var systemName = data.systemName !== undefined ? String(data.systemName) : "";
    var state = data.state !== undefined ? String(data.state) : "";
    var stateArgs = (_a = data.stateArgs) !== null && _a !== void 0 ? _a : {};
    var domainState = (_b = data.domainState) !== null && _b !== void 0 ? _b : {};
    var stackRaw = Array.isArray(data.stack) ? data.stack : [];
    var stack = stackRaw.map(function (frame) {
        var _a;
        return ({
            state: frame && frame.state !== undefined ? String(frame.state) : "",
            stateArgs: (_a = frame === null || frame === void 0 ? void 0 : frame.stateArgs) !== null && _a !== void 0 ? _a : {},
        });
    });
    return {
        schemaVersion: schemaVersion,
        systemName: systemName,
        state: state,
        stateArgs: stateArgs,
        domainState: domainState,
        stack: stack,
    };
}
function compareSnapshots(a, b) {
    var differences = [];
    if (a.schemaVersion !== b.schemaVersion) {
        differences.push("schemaVersion: ".concat(a.schemaVersion, " != ").concat(b.schemaVersion));
    }
    if (a.systemName !== b.systemName) {
        differences.push("systemName: ".concat(a.systemName, " != ").concat(b.systemName));
    }
    if (a.state !== b.state) {
        differences.push("state: ".concat(a.state, " != ").concat(b.state));
    }
    if (JSON.stringify(a.stateArgs) !== JSON.stringify(b.stateArgs)) {
        differences.push("stateArgs differ: ".concat(JSON.stringify(a.stateArgs), " != ").concat(JSON.stringify(b.stateArgs)));
    }
    if (JSON.stringify(a.domainState) !== JSON.stringify(b.domainState)) {
        differences.push("domainState differ: ".concat(JSON.stringify(a.domainState), " != ").concat(JSON.stringify(b.domainState)));
    }
    if (JSON.stringify(a.stack) !== JSON.stringify(b.stack)) {
        differences.push("stack differ: ".concat(JSON.stringify(a.stack), " != ").concat(JSON.stringify(b.stack)));
    }
    return { equal: differences.length === 0, differences: differences };
}
