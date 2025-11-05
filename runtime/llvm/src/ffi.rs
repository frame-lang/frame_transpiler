use crate::event_system::{FrameCompartment, FrameEvent, StateValue};
use crate::frame_kernel::{FrameKernel, FrameKernelResult};
use libc::c_char;
use std::ffi::{CStr, CString};
use std::ptr;

fn cstr_to_string(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_event_new(message: *const c_char) -> *mut FrameEvent {
    Box::into_raw(Box::new(FrameEvent::new(cstr_to_string(message))))
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_event_free(event: *mut FrameEvent) {
    if event.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(event));
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_event_push_param_i32(event: *mut FrameEvent, value: i32) {
    if event.is_null() {
        return;
    }
    unsafe {
        (*event).push_param(StateValue::I32(value));
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_event_push_param_double(event: *mut FrameEvent, value: f64) {
    if event.is_null() {
        return;
    }
    unsafe {
        (*event).push_param(StateValue::F64(value));
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_event_push_param_bool(event: *mut FrameEvent, value: bool) {
    if event.is_null() {
        return;
    }
    unsafe {
        (*event).push_param(StateValue::Bool(value));
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_event_push_param_cstring(
    event: *mut FrameEvent,
    value: *const c_char,
) {
    if event.is_null() {
        return;
    }
    let value_str = cstr_to_string(value);
    let c_string =
        CString::new(value_str).unwrap_or_else(|_| CString::new("".to_string()).unwrap());
    unsafe {
        (*event).push_param(StateValue::CString(c_string));
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_event_get_param_i32(
    event: *const FrameEvent,
    index: i32,
) -> i32 {
    if event.is_null() || index < 0 {
        return 0;
    }
    let event_ref = unsafe { &*event };
    event_ref
        .param(index as usize)
        .and_then(|value| value.as_i32())
        .unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_event_get_param_double(
    event: *const FrameEvent,
    index: i32,
) -> f64 {
    if event.is_null() || index < 0 {
        return 0.0;
    }
    let event_ref = unsafe { &*event };
    event_ref
        .param(index as usize)
        .and_then(|value| value.as_f64())
        .unwrap_or(0.0)
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_event_get_param_bool(
    event: *const FrameEvent,
    index: i32,
) -> bool {
    if event.is_null() || index < 0 {
        return false;
    }
    let event_ref = unsafe { &*event };
    event_ref
        .param(index as usize)
        .and_then(|value| value.as_bool())
        .unwrap_or(false)
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_event_get_param_cstring(
    event: *const FrameEvent,
    index: i32,
) -> *const c_char {
    if event.is_null() || index < 0 {
        return ptr::null();
    }
    let event_ref = unsafe { &*event };
    event_ref
        .param(index as usize)
        .and_then(|value| value.as_c_str_ptr())
        .unwrap_or(ptr::null())
}

#[no_mangle]
pub extern "C" fn frame_runtime_compartment_new(state: *const c_char) -> *mut FrameCompartment {
    Box::into_raw(Box::new(FrameCompartment::new(cstr_to_string(state))))
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_compartment_free(compartment: *mut FrameCompartment) {
    if compartment.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(compartment));
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_kernel_new(
    compartment: *mut FrameCompartment,
) -> *mut FrameKernel {
    FrameKernel::new(compartment)
        .map(|kernel| Box::into_raw(Box::new(kernel)))
        .unwrap_or(ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_kernel_free(kernel: *mut FrameKernel) {
    if kernel.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(kernel));
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_kernel_dispatch(
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
pub unsafe extern "C" fn frame_runtime_kernel_set_state(
    kernel: *mut FrameKernel,
    state: *const c_char,
) {
    if kernel.is_null() {
        return;
    }
    let state_string = cstr_to_string(state);
    unsafe {
        (*kernel).set_state(state_string);
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_kernel_next_event(
    kernel: *mut FrameKernel,
) -> *mut FrameEvent {
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
pub unsafe extern "C" fn frame_runtime_event_is_message(
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
pub unsafe extern "C" fn frame_runtime_compartment_set_enter_event(
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
pub unsafe extern "C" fn frame_runtime_compartment_take_enter_event(
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
pub unsafe extern "C" fn frame_runtime_compartment_set_exit_event(
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
pub unsafe extern "C" fn frame_runtime_kernel_push_compartment(
    kernel: *mut FrameKernel,
    compartment: *mut FrameCompartment,
) -> *mut FrameCompartment {
    if kernel.is_null() || compartment.is_null() {
        return ptr::null_mut();
    }
    unsafe { (*kernel).push_compartment(Box::from_raw(compartment)) }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_kernel_pop_compartment(
    kernel: *mut FrameKernel,
) -> *mut FrameCompartment {
    if kernel.is_null() {
        return ptr::null_mut();
    }
    unsafe { (*kernel).pop_compartment().unwrap_or(ptr::null_mut()) }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_compartment_get_parent(
    compartment: *mut FrameCompartment,
) -> *mut FrameCompartment {
    if compartment.is_null() {
        return ptr::null_mut();
    }
    unsafe { (*compartment).parent_ptr().unwrap_or(ptr::null_mut()) }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_compartment_state_arg_set_i32(
    compartment: *mut FrameCompartment,
    key: *const c_char,
    value: i32,
) {
    if compartment.is_null() || key.is_null() {
        return;
    }
    let key = cstr_to_string(key);
    unsafe {
        (*compartment).set_state_arg(key, StateValue::I32(value));
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_compartment_state_arg_set_double(
    compartment: *mut FrameCompartment,
    key: *const c_char,
    value: f64,
) {
    if compartment.is_null() || key.is_null() {
        return;
    }
    let key = cstr_to_string(key);
    unsafe {
        (*compartment).set_state_arg(key, StateValue::F64(value));
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_compartment_state_arg_set_bool(
    compartment: *mut FrameCompartment,
    key: *const c_char,
    value: bool,
) {
    if compartment.is_null() || key.is_null() {
        return;
    }
    let key = cstr_to_string(key);
    unsafe {
        (*compartment).set_state_arg(key, StateValue::Bool(value));
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_compartment_state_arg_set_cstring(
    compartment: *mut FrameCompartment,
    key: *const c_char,
    value: *const c_char,
) {
    if compartment.is_null() || key.is_null() || value.is_null() {
        return;
    }
    let key = cstr_to_string(key);
    let value_str = cstr_to_string(value);
    let c_string =
        CString::new(value_str).unwrap_or_else(|_| CString::new("".to_string()).unwrap());
    unsafe {
        (*compartment).set_state_arg(key, StateValue::CString(c_string));
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_compartment_state_arg_get_i32(
    compartment: *mut FrameCompartment,
    key: *const c_char,
) -> i32 {
    if compartment.is_null() || key.is_null() {
        return 0;
    }
    let key = cstr_to_string(key);
    unsafe {
        (*compartment)
            .state_arg(&key)
            .and_then(|value| value.as_i32())
            .unwrap_or(0)
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_compartment_state_arg_get_double(
    compartment: *mut FrameCompartment,
    key: *const c_char,
) -> f64 {
    if compartment.is_null() || key.is_null() {
        return 0.0;
    }
    let key = cstr_to_string(key);
    unsafe {
        (*compartment)
            .state_arg(&key)
            .and_then(|value| value.as_f64())
            .unwrap_or(0.0)
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_compartment_state_arg_get_bool(
    compartment: *mut FrameCompartment,
    key: *const c_char,
) -> bool {
    if compartment.is_null() || key.is_null() {
        return false;
    }
    let key = cstr_to_string(key);
    unsafe {
        (*compartment)
            .state_arg(&key)
            .and_then(|value| value.as_bool())
            .unwrap_or(false)
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_compartment_state_arg_get_cstring(
    compartment: *mut FrameCompartment,
    key: *const c_char,
) -> *const c_char {
    if compartment.is_null() || key.is_null() {
        return ptr::null();
    }
    let key = cstr_to_string(key);
    unsafe {
        (*compartment)
            .state_arg(&key)
            .and_then(|value| value.as_c_str_ptr())
            .unwrap_or(ptr::null())
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_compartment_enter_arg_set_i32(
    compartment: *mut FrameCompartment,
    key: *const c_char,
    value: i32,
) {
    if compartment.is_null() || key.is_null() {
        return;
    }
    let key = cstr_to_string(key);
    unsafe {
        (*compartment).set_enter_arg(key, StateValue::I32(value));
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_compartment_enter_arg_set_double(
    compartment: *mut FrameCompartment,
    key: *const c_char,
    value: f64,
) {
    if compartment.is_null() || key.is_null() {
        return;
    }
    let key = cstr_to_string(key);
    unsafe {
        (*compartment).set_enter_arg(key, StateValue::F64(value));
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_compartment_enter_arg_set_bool(
    compartment: *mut FrameCompartment,
    key: *const c_char,
    value: bool,
) {
    if compartment.is_null() || key.is_null() {
        return;
    }
    let key = cstr_to_string(key);
    unsafe {
        (*compartment).set_enter_arg(key, StateValue::Bool(value));
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_compartment_enter_arg_set_cstring(
    compartment: *mut FrameCompartment,
    key: *const c_char,
    value: *const c_char,
) {
    if compartment.is_null() || key.is_null() || value.is_null() {
        return;
    }
    let key = cstr_to_string(key);
    let value_str = cstr_to_string(value);
    let c_string =
        CString::new(value_str).unwrap_or_else(|_| CString::new("".to_string()).unwrap());
    unsafe {
        (*compartment).set_enter_arg(key, StateValue::CString(c_string));
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_compartment_enter_arg_get_i32(
    compartment: *mut FrameCompartment,
    key: *const c_char,
) -> i32 {
    if compartment.is_null() || key.is_null() {
        return 0;
    }
    let key = cstr_to_string(key);
    unsafe {
        (*compartment)
            .enter_arg(&key)
            .and_then(|value| value.as_i32())
            .unwrap_or(0)
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_compartment_enter_arg_get_double(
    compartment: *mut FrameCompartment,
    key: *const c_char,
) -> f64 {
    if compartment.is_null() || key.is_null() {
        return 0.0;
    }
    let key = cstr_to_string(key);
    unsafe {
        (*compartment)
            .enter_arg(&key)
            .and_then(|value| value.as_f64())
            .unwrap_or(0.0)
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_compartment_enter_arg_get_bool(
    compartment: *mut FrameCompartment,
    key: *const c_char,
) -> bool {
    if compartment.is_null() || key.is_null() {
        return false;
    }
    let key = cstr_to_string(key);
    unsafe {
        (*compartment)
            .enter_arg(&key)
            .and_then(|value| value.as_bool())
            .unwrap_or(false)
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_compartment_enter_arg_get_cstring(
    compartment: *mut FrameCompartment,
    key: *const c_char,
) -> *const c_char {
    if compartment.is_null() || key.is_null() {
        return ptr::null();
    }
    let key = cstr_to_string(key);
    unsafe {
        (*compartment)
            .enter_arg(&key)
            .and_then(|value| value.as_c_str_ptr())
            .unwrap_or(ptr::null())
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_compartment_enter_args_clear(
    compartment: *mut FrameCompartment,
) {
    if compartment.is_null() {
        return;
    }
    unsafe {
        (*compartment).clear_enter_args();
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_print_line(message: *const c_char) {
    if message.is_null() {
        return;
    }
    unsafe {
        libc::puts(message);
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_print_int(value: i32) {
    unsafe {
        libc::printf(
            b"%d
 "
            .as_ptr() as *const i8,
            value,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_print_double(value: f64) {
    unsafe {
        libc::printf(
            b"%f
 "
            .as_ptr() as *const i8,
            value,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_print_bool(value: bool) {
    let text_ptr = if value {
        b"true ".as_ptr() as *const i8
    } else {
        b"false ".as_ptr() as *const i8
    };
    unsafe {
        libc::puts(text_ptr);
    }
}
#[no_mangle]
pub unsafe extern "C" fn frame_runtime_kernel_state_stack_push(
    kernel: *mut FrameKernel,
    state_index: i32,
) {
    if kernel.is_null() {
        return;
    }
    unsafe {
        (*kernel).push_state_snapshot(state_index);
    }
}

#[no_mangle]
pub unsafe extern "C" fn frame_runtime_kernel_state_stack_pop(
    kernel: *mut FrameKernel,
    state_out: *mut i32,
) -> *mut FrameCompartment {
    if kernel.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        match (*kernel).restore_state_snapshot() {
            Some((compartment, state_index)) => {
                if !state_out.is_null() {
                    *state_out = state_index;
                }
                compartment
            }
            None => ptr::null_mut(),
        }
    }
}
