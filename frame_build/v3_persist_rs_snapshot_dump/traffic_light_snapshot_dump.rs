#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StateId { Red, Yellow, Green }

impl Default for StateId { fn default() -> Self { StateId::Red } }

#[derive(Debug, Clone)] struct FrameEvent{ message: String }
#[derive(Debug, Clone, Default)] struct FrameCompartment{ state: StateId, forward_event: Option<FrameEvent>, exit_args: Option<()>, enter_args: Option<()>, parent_compartment: Option<*const FrameCompartment>, state_args: Option<()>, }
struct TrafficLight {
    compartment: FrameCompartment,
    _stack: Vec<FrameCompartment>,
    domain: &'static str,
}

impl TrafficLight {
    fn new() -> Self {
        Self {
            compartment: FrameCompartment{ state: StateId::default(), ..Default::default() },
            _stack: Vec::new(),
            domain:  "red",
        }
    }
    fn _frame_transition(&mut self, next: &FrameCompartment){
        // Basic transition: update the active state id; other fields remain unchanged for now.
        self.compartment.state = next.state;
    }
    fn _frame_stack_push(&mut self){
        self._stack.push(self.compartment.clone());
    }
    fn _frame_stack_pop(&mut self){
        if let Some(prev) = self._stack.pop() {
            self._frame_transition(&prev);
        }
    }
}

impl TrafficLight {
    fn _frame_router(&mut self, e: Option<FrameEvent>) {
        if let Some(ev) = e {
            match ev.message.as_str() {
                "tick" => self._event_tick(),
                _ => { }
            }
        }
    }
    fn _event_tick(&mut self) {
        match self.compartment.state {
            StateId::Red => {
                
                let next_compartment = FrameCompartment { state: StateId::Green, ..Default::default() };
                self._frame_transition(&next_compartment);
                return;
                
                
            }
            StateId::Green => {
                
                let next_compartment = FrameCompartment { state: StateId::Yellow, ..Default::default() };
                self._frame_transition(&next_compartment);
                return;
                
                
            }
            StateId::Yellow => {
                
                let next_compartment = FrameCompartment { state: StateId::Red, ..Default::default() };
                self._frame_transition(&next_compartment);
                return;
                
                
            }
            _ => { }
        }
    }
}

