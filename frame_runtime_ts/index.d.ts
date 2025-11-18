export interface FrameEventParameters {
  [key: string]: any;
}

export class FrameEvent {
  constructor(message: string, parameters: FrameEventParameters | null);
  message: string;
  parameters: FrameEventParameters | null;
}

export class FrameCompartment {
  constructor(
    state: string,
    enterArgs?: any,
    exitArgs?: any,
    stateArgs?: any,
    stateVars?: any,
    enterArgsCollection?: any,
    exitArgsCollection?: any,
    forwardEvent?: FrameEvent | null
  );

  state: string;
  enterArgs?: any;
  exitArgs?: any;
  stateArgs?: any;
  stateVars?: any;
  enterArgsCollection?: any;
  exitArgsCollection?: any;
  forwardEvent?: FrameEvent | null;
}

