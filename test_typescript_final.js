"use strict";
// Emitted from framec_v0.85.6
Object.defineProperty(exports, "__esModule", { value: true });
exports.FileIOTest = void 0;
/// <reference lib="es2015.promise" />
class FrameEvent {
    constructor(message, parameters) {
        this.message = message;
        this.parameters = parameters;
    }
}
class FrameCompartment {
    constructor(state, enterArgs, exitArgs, stateArgs, stateVars, enterArgsCollection, exitArgsCollection, forwardEvent) {
        this.state = state;
        this.enterArgs = enterArgs;
        this.exitArgs = exitArgs;
        this.stateArgs = stateArgs;
        this.stateVars = stateVars;
        this.enterArgsCollection = enterArgsCollection;
        this.exitArgsCollection = exitArgsCollection;
        this.forwardEvent = forwardEvent;
        this.forwardEvent = forwardEvent || null;
        this.stateArgs = stateArgs || {};
        this.stateVars = stateVars || {};
    }
}
const fs = require("fs");
class FileIOTest {
    constructor() {
        this._nextCompartment = null;
        this.returnStack = [];
        this._nextCompartment = null;
        this.returnStack = [null];
        this._frame_kernel(new FrameEvent("$>", null));
    }
    // Actions
    _action_testFileOperations() {
        console.log("=== File I/O Test ===");
        fs.writeFileSync("test_input.txt", "Frame File I/O Test Data");
        let exists = fs.existsSync("test_input.txt");
        console.log(`File exists: ${exists}`);
        if (exists) {
            let content = fs.readFileSync("test_input.txt", "utf8").trim();
            console.log(`File content: ${content}`);
        }
        else {
            console.log("File content: [file not found]");
        }
        fs.writeFileSync("test_output.txt", "Hello from Frame TypeScript!");
        let write_exists = fs.existsSync("test_output.txt");
        console.log(`Write successful: ${write_exists}`);
        console.log("=== File I/O Test Complete ===");
        return true; // Default success return for Frame action
    }
    // Missing method stubs (would be implemented in runtime environment)
    handlePythonStdout(data) {
        // Implementation provided by runtime environment
        console.log('[Python stdout]:', data);
    }
    handlePythonStderr(data) {
        // Implementation provided by runtime environment
        console.error('[Python stderr]:', data);
    }
    handlePythonExit(exitCode) {
        // Implementation provided by runtime environment
        console.log('[Python exit]:', exitCode);
    }
    handlePythonError(error) {
        // Implementation provided by runtime environment
        console.error('[Python error]:', error);
    }
    handleRuntimeConnection(socket) {
        // Implementation provided by runtime environment
        console.log('[Runtime connection]:', socket);
    }
    // Frame runtime
    _frame_kernel(__e) {
        this._frame_router(__e);
        while (this._nextCompartment !== null) {
            const nextCompartment = this._nextCompartment;
            this._nextCompartment = null;
            this._frame_router(new FrameEvent("<$", this._compartment.exitArgs));
            this._compartment = nextCompartment;
            if (nextCompartment.forwardEvent === null) {
                this._frame_router(new FrameEvent("$>", this._compartment.enterArgs));
            }
            else {
                this._frame_router(nextCompartment.forwardEvent);
                nextCompartment.forwardEvent = null;
            }
        }
    }
    _frame_router(__e, compartment) {
        const targetCompartment = compartment || this._compartment;
    }
    _frame_transition(nextCompartment) {
        this._nextCompartment = nextCompartment;
    }
}
exports.FileIOTest = FileIOTest;
