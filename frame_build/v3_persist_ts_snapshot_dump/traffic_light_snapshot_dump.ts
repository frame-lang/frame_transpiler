import { FrameEvent, FrameCompartment } from 'frame_runtime_ts'

export class TrafficLight {
  public domain: any =  null;
  public _compartment: FrameCompartment = new FrameCompartment('__TrafficLight_state_Red');
  private _stack: FrameCompartment[] = [];
  private _systemReturnStack: any[] = [];
  constructor(...sysParams: any[]) {
    const startCount = 1;
    const enterCount = 0;
    const startArgs = sysParams.slice(0, startCount);
    const enterArgs = sysParams.slice(startCount, startCount + enterCount);
    const domainArgs = sysParams.slice(startCount + enterCount);
    const stateArgs: any = {};
    if (startArgs.length > 0) stateArgs['color'] = startArgs[0];
    if (domainArgs.length > 0) (this as any).domain = domainArgs[0];
    this._compartment = new FrameCompartment('__TrafficLight_state_Red', enterArgs, undefined, stateArgs);
    const enterEvent = new FrameEvent("$enter", enterArgs);
    this._frame_router(enterEvent, this._compartment);
  }
  _frame_transition(n: FrameCompartment){ this._compartment = n; const enterEvent = new FrameEvent("$enter", n.enterArgs); this._frame_router(enterEvent, n); }
  _frame_stack_push(){ this._stack.push(this._compartment); }
  _frame_stack_pop(){ const prev = this._stack.pop(); if (prev) this._frame_transition(prev); }
  public tick(): any {
    const __initial = undefined;
    this._systemReturnStack.push(__initial);
    try {
      const __e = new FrameEvent("tick", null);
      this._frame_router(__e, this._compartment);
      return this._systemReturnStack[this._systemReturnStack.length - 1];
    } finally {
      this._systemReturnStack.pop();
    }
  }
  private _event_tick(__e: FrameEvent, compartment: FrameCompartment): void {
    const c = compartment || this._compartment;
    switch (c.state) {
      case '__TrafficLight_state_Red':
        
        const __frameNextCompartment_Green = new FrameCompartment("__TrafficLight_state_Green");
        __frameNextCompartment_Green.stateArgs = ["green"];
        this._frame_transition(__frameNextCompartment_Green);
        return;
        
        break;
      case '__TrafficLight_state_Green':
        
        const __frameNextCompartment_Yellow = new FrameCompartment("__TrafficLight_state_Yellow");
        __frameNextCompartment_Yellow.stateArgs = ["yellow"];
        this._frame_transition(__frameNextCompartment_Yellow);
        return;
        
        break;
      case '__TrafficLight_state_Yellow':
        
        const __frameNextCompartment_Red = new FrameCompartment("__TrafficLight_state_Red");
        __frameNextCompartment_Red.stateArgs = ["red"];
        this._frame_transition(__frameNextCompartment_Red);
        return;
        
        break;
    }
  }
  _frame_router(__e: FrameEvent, c?: FrameCompartment, ...args: any[]): any {
    const _c = c || this._compartment;
    switch (__e.message) {
      case "tick": return this._event_tick(__e, _c);
      default: return;
    }
  }
}
