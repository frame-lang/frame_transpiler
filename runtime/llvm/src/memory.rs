use std::cell::RefCell;
use std::rc::Rc;

/// Boxed value helper – mirrors the handle types exposed by other runtimes.
pub struct FrameBox<T>(Box<T>);

impl<T> FrameBox<T> {
    pub fn new(value: T) -> Self {
        FrameBox(Box::new(value))
    }

    pub fn into_inner(self) -> Box<T> {
        self.0
    }

    pub fn as_ref(&self) -> &T {
        &self.0
    }

    pub fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

/// Reference-counted helper to share state between compartments.
pub type FrameRc<T> = Rc<RefCell<T>>;
