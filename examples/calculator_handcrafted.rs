// Hand-crafted Rust implementation of Calculator Frame system
// This demonstrates the actual Frame runtime semantics that the visitor should generate

use std::collections::HashMap;

// Frame runtime structures following the translation guide
#[derive(Debug, Clone, PartialEq)]
pub struct FrameEvent {
    pub message: String,
    pub parameters: Option<HashMap<String, String>>,
}

impl FrameEvent {
    pub fn new(message: &str, parameters: Option<HashMap<String, String>>) -> Self {
        Self {
            message: message.to_string(),
            parameters,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FrameCompartment {
    pub state: String,
    pub forward_event: Option<FrameEvent>,
    pub exit_args: Option<HashMap<String, String>>,
    pub enter_args: Option<HashMap<String, String>>,
}

impl FrameCompartment {
    pub fn new(state: &str) -> Self {
        Self {
            state: state.to_string(),
            forward_event: None,
            exit_args: None,
            enter_args: None,
        }
    }
    
    pub fn new_with_forward(state: &str, forward_event: Option<FrameEvent>) -> Self {
        Self {
            state: state.to_string(),
            forward_event,
            exit_args: None,
            enter_args: None,
        }
    }
}

// Calculator system implementation with actual Frame semantics
pub struct Calculator {
    // Frame runtime state
    compartment: FrameCompartment,
    next_compartment: Option<FrameCompartment>,
    return_stack: Vec<Option<i32>>,
}

impl Calculator {
    pub fn new() -> Self {
        let mut instance = Self {
            compartment: FrameCompartment::new("Ready"),
            next_compartment: None,
            return_stack: vec![None],
        };
        
        // Send system start event
        let start_event = FrameEvent::new("$>", None);
        instance.kernel(start_event);
        instance
    }
    
    // ==================== Interface Block ==================
    
    pub fn add(&mut self, a: i32, b: i32) -> Option<i32> {
        self.return_stack.push(None);
        let mut params = HashMap::new();
        params.insert("a".to_string(), a.to_string());
        params.insert("b".to_string(), b.to_string());
        let event = FrameEvent::new("add", Some(params));
        self.kernel(event);
        self.return_stack.pop().flatten()
    }
    
    // ==================== State Handlers ==================
    
    fn handle_ready(&mut self, e: &FrameEvent) {
        match e.message.as_str() {
            "add" => {
                if let Some(params) = &e.parameters {
                    if let (Some(a_str), Some(b_str)) = (params.get("a"), params.get("b")) {
                        if let (Ok(a), Ok(b)) = (a_str.parse::<i32>(), b_str.parse::<i32>()) {
                            // Perform the calculation: system.return = a + b
                            let result = a + b;
                            
                            // Set return value on the stack
                            if let Some(last) = self.return_stack.last_mut() {
                                *last = Some(result);
                            }
                            
                            // return statement - no transition needed
                        }
                    }
                }
            }
            "$>" => {
                // Enter event for Ready state - initialize if needed
            }
            "<$" => {
                // Exit event for Ready state - cleanup if needed
            }
            _ => {
                // Unhandled event
            }
        }
    }
    
    // ==================== Frame Runtime Kernel ==================
    
    fn kernel(&mut self, event: FrameEvent) {
        // Send event to current state
        self.router(&event);
        
        // Loop until no transitions occur (implements non-recursive transition loop)
        while let Some(next) = self.next_compartment.take() {
            // Exit current state
            let exit_event = FrameEvent::new("<$", self.compartment.exit_args.clone());
            self.router(&exit_event);
            
            // Change state
            self.compartment = next;
            
            // Handle enter event or forward event
            if let Some(forward_event) = &self.compartment.forward_event.clone() {
                if forward_event.message == "$>" {
                    self.router(forward_event);
                } else {
                    let enter_event = FrameEvent::new("$>", self.compartment.enter_args.clone());
                    self.router(&enter_event);
                    self.router(forward_event);
                }
            } else {
                let enter_event = FrameEvent::new("$>", self.compartment.enter_args.clone());
                self.router(&enter_event);
            }
        }
    }
    
    fn router(&mut self, event: &FrameEvent) {
        match self.compartment.state.as_str() {
            "Ready" => self.handle_ready(event),
            _ => {
                // Unknown state
            }
        }
    }
    
    fn transition(&mut self, next_compartment: FrameCompartment) {
        self.next_compartment = Some(next_compartment);
    }
}

// Main function for testing
fn main() {
    let mut calculator = Calculator::new();
    
    println!("Testing Calculator Frame system:");
    
    // Test the add method
    let result = calculator.add(3, 4);
    match result {
        Some(value) => println!("SUCCESS: add(3, 4) = {}", value),
        None => println!("FAIL: add(3, 4) returned None"),
    }
    
    // Test with different values
    let result2 = calculator.add(10, 20);
    match result2 {
        Some(value) => println!("SUCCESS: add(10, 20) = {}", value),
        None => println!("FAIL: add(10, 20) returned None"),
    }
    
    // Test with negative numbers
    let result3 = calculator.add(-5, 15);
    match result3 {
        Some(value) => println!("SUCCESS: add(-5, 15) = {}", value),
        None => println!("FAIL: add(-5, 15) returned None"),
    }
    
    println!("Calculator test completed");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculator_add() {
        let mut calc = Calculator::new();
        assert_eq!(calc.add(3, 4), Some(7));
        assert_eq!(calc.add(10, 20), Some(30));
        assert_eq!(calc.add(-5, 15), Some(10));
    }
    
    #[test]
    fn test_frame_runtime() {
        // Test that the Frame runtime structures work correctly
        let event = FrameEvent::new("test", None);
        assert_eq!(event.message, "test");
        assert_eq!(event.parameters, None);
        
        let compartment = FrameCompartment::new("TestState");
        assert_eq!(compartment.state, "TestState");
        assert_eq!(compartment.forward_event, None);
    }
}