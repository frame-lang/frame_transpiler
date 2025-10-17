/**
 * Frame Runtime for TypeScript
 * Shared runtime classes and utilities for all Frame-generated TypeScript code.
 * Emitted from framec_v0.83.0
 */

export class FrameEvent {
    constructor(
        public message: string,
        public parameters: any
    ) {}
}

export class FrameCompartment {
    constructor(
        public state: string,
        public forwardEvent: FrameEvent | null = null,
        public exitArgs: any = null,
        public enterArgs: any = null,
        public parent: FrameCompartment | null = null,
        public stateVars: { [key: string]: any } = {},
        public stateArgs: { [key: string]: any } = {}
    ) {}
}