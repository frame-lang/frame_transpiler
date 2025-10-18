// Emitted from framec_v0.83.3


import { FrameEvent, FrameCompartment } from '../../typescript/runtime/frame_runtime';

export class SimpleTypeScript {
    private _compartment: FrameCompartment;
    private _nextCompartment: FrameCompartment | null = null;
    private returnStack: any[] = [];

    constructor() {
        this._compartment = new FrameCompartment('__simpletypescript_state_Start');
        this._nextCompartment = null;
        this.returnStack = [null];
        this._frame_kernel(new FrameEvent("$>", null));
    }

    // Event handlers
    private _handle_start_enter(__e: FrameEvent, compartment: FrameCompartment): void {
        console.log("Test");
        return;
    }

    // State dispatchers
    private __simpletypescript_state_Start(__e: FrameEvent, compartment: FrameCompartment): void {
        switch(__e.message) {
            case "$>":
                this._handle_start_enter(__e, compartment);
                break;
        }
    }

    // Operations
    private _operation_getValue(): any {
        this.returnStack[this.returnStack.length - 1] = "hello";
        return;
    }

    // Frame runtime
    private _frame_kernel(__e: FrameEvent): void {
        this._frame_router(__e);
        while (this._nextCompartment !== null) {
            const nextCompartment = this._nextCompartment;
            this._nextCompartment = null;
            this._frame_router(new FrameEvent("<$", this._compartment.exitArgs));
            this._compartment = nextCompartment;
            if (nextCompartment.forwardEvent === null) {
                this._frame_router(new FrameEvent("$>", this._compartment.enterArgs));
            } else {
                this._frame_router(nextCompartment.forwardEvent);
                nextCompartment.forwardEvent = null;
            }
        }
    }

    private _frame_router(__e: FrameEvent, compartment?: FrameCompartment): void {
        const targetCompartment = compartment || this._compartment;
        switch(targetCompartment.state) {
            case '__simpletypescript_state_Start':
                this.__simpletypescript_state_Start(__e, targetCompartment);
                break;
        }
    }

    private _frame_transition(nextCompartment: FrameCompartment): void {
        this._nextCompartment = nextCompartment;
    }
}

