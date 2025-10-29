use crate::event_system::{FrameCompartment, FrameEvent};
use crate::frame_kernel::{FrameKernel, FrameKernelResult};
use libc::c_char;
use std::ffi::CStr;
use std::ptr;

fn cstr_to_string(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
}

#[no_mangle]
pub extern "C" fn frame_runtime_event_new(message: *const c_char) -> *mut FrameEvent {
    Box::into_raw(Box::new(FrameEvent::new(cstr_to_string(message))))
}

#[no_mangle]
pub extern "C" fn frame_runtime_event_free(event: *mut FrameEvent) {
    if event.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(event));
    }
}

#[no_mangle]
pub extern "C" fn frame_runtime_compartment_new(state: *const c_char) -> *mut FrameCompartment {
    Box::into_raw(Box::new(FrameCompartment::new(cstr_to_string(state))))
}

#[no_mangle]
pub extern "C" fn frame_runtime_compartment_free(compartment: *mut FrameCompartment) {
    if compartment.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(compartment));
    }
}

#[no_mangle]
pub extern "C" fn frame_runtime_kernel_new(compartment: *mut FrameCompartment) -> *mut FrameKernel {
    FrameKernel::new(compartment)
        .map(|kernel| Box::into_raw(Box::new(kernel)))
        .unwrap_or(ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn frame_runtime_kernel_free(kernel: *mut FrameKernel) {
    if kernel.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(kernel));
    }
}

#[no_mangle]
pub extern "C" fn frame_runtime_kernel_dispatch(
    kernel: *mut FrameKernel,
    event: *mut FrameEvent,
) -> i32 {
    if kernel.is_null() || event.is_null() {
        return -1;
    }
    let kernel_ref = unsafe { &mut *kernel };
    let event_ref = unsafe { &*event };
    match kernel_ref.dispatch(event_ref) {
        FrameKernelResult::Continue => 0,
        FrameKernelResult::Halt => 1,
    }
}

#[no_mangle]
pub extern "C" fn frame_runtime_kernel_set_state(kernel: *mut FrameKernel, state: *const c_char) {
    if kernel.is_null() {
        return;
    }
    let state_string = cstr_to_string(state);
    unsafe {
        (*kernel).set_state(state_string);
    }
}

#[no_mangle]
pub extern "C" fn frame_runtime_kernel_next_event(kernel: *mut FrameKernel) -> *mut FrameEvent {
    if kernel.is_null() {
        return ptr::null_mut();
    }
    unsafe {
        (*kernel)
            .next_event()
            .map(|event| Box::into_raw(Box::new(event)))
            .unwrap_or(ptr::null_mut())
    }
}

#[no_mangle]
pub extern "C" fn frame_runtime_event_is_message(
    event: *const FrameEvent,
    message: *const c_char,
) -> bool {
    if event.is_null() || message.is_null() {
        return false;
    }
    let event_ref = unsafe { &*event };
    let message_str = cstr_to_string(message);
    event_ref.message() == message_str
}

#[no_mangle]
pub extern "C" fn frame_runtime_compartment_set_enter_event(
    compartment: *mut FrameCompartment,
    event: *mut FrameEvent,
) {
    if compartment.is_null() {
        return;
    }

    let event_opt = if event.is_null() {
        None
    } else {
        Some(*unsafe { Box::from_raw(event) })
    };

    unsafe {
        (*compartment).set_enter_event(event_opt);
    }
}

#[no_mangle]
pub extern "C" fn frame_runtime_compartment_take_enter_event(
    compartment: *mut FrameCompartment,
) -> *mut FrameEvent {
    if compartment.is_null() {
        return ptr::null_mut();
    }
    unsafe {
        (*compartment)
            .take_enter_event()
            .map(|event| Box::into_raw(Box::new(event)))
            .unwrap_or(ptr::null_mut())
    }
}

#[no_mangle]
pub extern "C" fn frame_runtime_compartment_set_exit_event(
    compartment: *mut FrameCompartment,
    event: *mut FrameEvent,
) {
    if compartment.is_null() {
        return;
    }

    let event_opt = if event.is_null() {
        None
    } else {
        Some(*unsafe { Box::from_raw(event) })
    };

    unsafe {
        (*compartment).set_exit_event(event_opt);
    }
}

#[no_mangle]
pub extern "C" fn frame_runtime_compartment_take_exit_event(
    compartment: *mut FrameCompartment,
) -> *mut FrameEvent {
    if compartment.is_null() {
        return ptr::null_mut();
    }
    unsafe {
        (*compartment)
            .take_exit_event()
            .map(|event| Box::into_raw(Box::new(event)))
            .unwrap_or(ptr::null_mut())
    }
}

#[no_mangle]
pub extern "C" fn frame_runtime_compartment_set_forward_event(
    compartment: *mut FrameCompartment,
    event: *mut FrameEvent,
) {
    if compartment.is_null() {
        return;
    }

    let event_opt = if event.is_null() {
        None
    } else {
        Some(*unsafe { Box::from_raw(event) })
    };

    unsafe {
        (*compartment).set_forward_event(event_opt);
    }
}

#[no_mangle]
pub extern "C" fn frame_runtime_kernel_push_compartment(
    kernel: *mut FrameKernel,
    compartment: *mut FrameCompartment,
) -> *mut FrameCompartment {
    if kernel.is_null() || compartment.is_null() {
        return ptr::null_mut();
    }
    unsafe { (*kernel).push_compartment(Box::from_raw(compartment)) }
}

#[no_mangle]
pub extern "C" fn frame_runtime_compartment_get_parent(
    compartment: *mut FrameCompartment,
) -> *mut FrameCompartment {
    if compartment.is_null() {
        return ptr::null_mut();
    }
    unsafe { (*compartment).parent_ptr().unwrap_or(ptr::null_mut()) }
}
