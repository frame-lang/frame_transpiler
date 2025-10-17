"use strict";
/**
 * Frame Runtime for TypeScript
 * Shared runtime classes and utilities for all Frame-generated TypeScript code.
 * Emitted from framec_v0.83.0
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.FrameCompartment = exports.FrameEvent = void 0;
class FrameEvent {
    constructor(message, parameters) {
        this.message = message;
        this.parameters = parameters;
    }
}
exports.FrameEvent = FrameEvent;
class FrameCompartment {
    constructor(state, forwardEvent = null, exitArgs = null, enterArgs = null, parent = null, stateVars = {}, stateArgs = {}) {
        this.state = state;
        this.forwardEvent = forwardEvent;
        this.exitArgs = exitArgs;
        this.enterArgs = enterArgs;
        this.parent = parent;
        this.stateVars = stateVars;
        this.stateArgs = stateArgs;
    }
}
exports.FrameCompartment = FrameCompartment;
