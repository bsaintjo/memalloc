use std::ptr;
use libc;
use libc::{c_void, ssize_t, pthread_mutex_t};

static mut GLOBAL_MALLOC_LOCK: *mut pthread_mutex_t = ptr::null_mut();
static mut HEAD: *mut Header = ptr::null_mut();
static mut TAIL: *mut Header = ptr::null_mut();

struct Header {
    size: ssize_t,
    is_free: bool,
    next: *mut Header,
}

impl Header {
    fn get_free_block(size: ssize_t) -> Header {
        unimplemented!()
    }
}

pub unsafe fn malloc(size: ssize_t) -> *mut c_void {
    if size == 0 {
        return ptr::null_mut();
    }
    libc::pthread_mutex_lock(GLOBAL_MALLOC_LOCK);
    unimplemented!()
}
