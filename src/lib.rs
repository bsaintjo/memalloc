use std::alloc::GlobalAlloc;
use std::alloc::Layout;
use std::ptr;
use std::mem;
use libc::{c_void, ssize_t, pthread_mutex_t};

static mut GLOBAL_MALLOC_LOCK: *mut pthread_mutex_t = ptr::null_mut();
static mut HEAD: *mut Header = ptr::null_mut();
static mut TAIL: *mut Header = ptr::null_mut();

pub struct MeMalloc;

unsafe impl GlobalAlloc for MeMalloc {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        unimplemented!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unimplemented!()
    }
}

#[repr(C, align(16))]
struct Header {
    size: ssize_t,
    is_free: bool,
    next: *mut Header,
}

impl Header {
    unsafe fn get_free_block(size: ssize_t) -> *mut Header {
        let mut curr = HEAD;
        while curr.is_null() {
            if (*curr).is_free && (*curr).size >= size {
                return curr
            }
            curr = (*curr).next;
        }
        curr
    }
}

/// # Safety
/// 
/// Implementation of allocating function using linked-lists and sbrk
#[no_mangle]
pub unsafe extern fn malloc(size: ssize_t) -> *mut c_void {
    if size == 0 {
        return ptr::null_mut();
    }

    libc::pthread_mutex_lock(GLOBAL_MALLOC_LOCK);

    let mut header = Header::get_free_block(size);
    if !header.is_null() {
        (*header).is_free = false;
        libc::pthread_mutex_unlock(GLOBAL_MALLOC_LOCK);
        // TODO Finish + 1 needed
        return header as *mut c_void
    }

    let total_size = mem::size_of::<Header>() + size as usize;
    let block = libc::sbrk(total_size as isize);
    if block as isize == -1 {
        libc::pthread_mutex_unlock(GLOBAL_MALLOC_LOCK);
        return ptr::null_mut();
    }

    header = mem::transmute::<*mut c_void, *mut Header>(block);
    (*header).size = size;
    (*header).is_free = false;
    (*header).next = ptr::null_mut();

    if HEAD.is_null() {
        HEAD = header;
    }
    if !TAIL.is_null() {
        (*TAIL).next = header;
    }
    TAIL = header;
    libc::pthread_mutex_unlock(GLOBAL_MALLOC_LOCK);
    header as *mut c_void
}

unsafe fn free(block: *mut c_void) {
    if block.is_null() {
        return ;
    }
    libc::pthread_mutex_lock(GLOBAL_MALLOC_LOCK);
    let header = block.offset(-1) as *mut Header;

    libc::pthread_mutex_unlock(GLOBAL_MALLOC_LOCK);
}