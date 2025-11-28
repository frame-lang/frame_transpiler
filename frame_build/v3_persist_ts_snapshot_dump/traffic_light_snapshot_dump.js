"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.TrafficLight = void 0;
var frame_runtime_ts_1 = require("frame_runtime_ts");
var TrafficLight = /** @class */ (function () {
    function TrafficLight() {
        var sysParams = [];
        for (var _i = 0; _i < arguments.length; _i++) {
            sysParams[_i] = arguments[_i];
        }
        this.domain = null;
        this._compartment = new frame_runtime_ts_1.FrameCompartment('__TrafficLight_state_Red');
        this._stack = [];
        this._systemReturnStack = [];
        var startCount = 1;
        var enterCount = 0;
        var startArgs = sysParams.slice(0, startCount);
        var enterArgs = sysParams.slice(startCount, startCount + enterCount);
        var domainArgs = sysParams.slice(startCount + enterCount);
        var stateArgs = {};
        if (startArgs.length > 0)
            stateArgs['color'] = startArgs[0];
        if (domainArgs.length > 0)
            this.domain = domainArgs[0];
        this._compartment = new frame_runtime_ts_1.FrameCompartment('__TrafficLight_state_Red', enterArgs, undefined, stateArgs);
        var enterEvent = new frame_runtime_ts_1.FrameEvent("$enter", enterArgs);
        this._frame_router(enterEvent, this._compartment);
    }
    TrafficLight.prototype._frame_transition = function (n) { this._compartment = n; var enterEvent = new frame_runtime_ts_1.FrameEvent("$enter", n.enterArgs); this._frame_router(enterEvent, n); };
    TrafficLight.prototype._frame_stack_push = function () { this._stack.push(this._compartment); };
    TrafficLight.prototype._frame_stack_pop = function () { var prev = this._stack.pop(); if (prev)
        this._frame_transition(prev); };
    TrafficLight.prototype.tick = function () {
        var __initial = undefined;
        this._systemReturnStack.push(__initial);
        try {
            var __e = new frame_runtime_ts_1.FrameEvent("tick", null);
            this._frame_router(__e, this._compartment);
            return this._systemReturnStack[this._systemReturnStack.length - 1];
        }
        finally {
            this._systemReturnStack.pop();
        }
    };
    TrafficLight.prototype._event_tick = function (__e, compartment) {
        var c = compartment || this._compartment;
        switch (c.state) {
            case '__TrafficLight_state_Red':
                var __frameNextCompartment_Green = new frame_runtime_ts_1.FrameCompartment("__TrafficLight_state_Green");
                __frameNextCompartment_Green.stateArgs = ["green"];
                this._frame_transition(__frameNextCompartment_Green);
                return;
                break;
            case '__TrafficLight_state_Green':
                var __frameNextCompartment_Yellow = new frame_runtime_ts_1.FrameCompartment("__TrafficLight_state_Yellow");
                __frameNextCompartment_Yellow.stateArgs = ["yellow"];
                this._frame_transition(__frameNextCompartment_Yellow);
                return;
                break;
            case '__TrafficLight_state_Yellow':
                var __frameNextCompartment_Red = new frame_runtime_ts_1.FrameCompartment("__TrafficLight_state_Red");
                __frameNextCompartment_Red.stateArgs = ["red"];
                this._frame_transition(__frameNextCompartment_Red);
                return;
                break;
        }
    };
    TrafficLight.prototype._frame_router = function (__e, c) {
        var args = [];
        for (var _i = 2; _i < arguments.length; _i++) {
            args[_i - 2] = arguments[_i];
        }
        var _c = c || this._compartment;
        switch (__e.message) {
            case "tick": return this._event_tick(__e, _c);
            default: return;
        }
    };
    return TrafficLight;
}());
exports.TrafficLight = TrafficLight;
