"use strict";
/**
 * Frame Runtime for TypeScript
 * Shared runtime classes and utilities for all Frame-generated TypeScript code.
 * Emitted from framec_v0.83.0
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.FrameCompartment = exports.FrameEvent = void 0;
var FrameEvent = /** @class */ (function () {
    function FrameEvent(message, parameters) {
        this.message = message;
        this.parameters = parameters;
    }
    return FrameEvent;
}());
exports.FrameEvent = FrameEvent;
var FrameCompartment = /** @class */ (function () {
    function FrameCompartment(state, forwardEvent, exitArgs, enterArgs, parent, stateVars, stateArgs) {
        if (forwardEvent === void 0) { forwardEvent = null; }
        if (exitArgs === void 0) { exitArgs = null; }
        if (enterArgs === void 0) { enterArgs = null; }
        if (parent === void 0) { parent = null; }
        if (stateVars === void 0) { stateVars = {}; }
        if (stateArgs === void 0) { stateArgs = {}; }
        this.state = state;
        this.forwardEvent = forwardEvent;
        this.exitArgs = exitArgs;
        this.enterArgs = enterArgs;
        this.parent = parent;
        this.stateVars = stateVars;
        this.stateArgs = stateArgs;
    }
    return FrameCompartment;
}());
exports.FrameCompartment = FrameCompartment;
