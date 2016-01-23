//! Memory Manager for CRust-OS

#![no_std]
#![feature(allocator)]
#![feature(lang_items)]
#![allocator]



static mut heap_end : *mut u8 = 0 as *mut u8;

extern {
    static HEAP : *const u8;
}

///! Set `heap_end` to the starting location of the heap
pub fn setup() {
    unsafe {
        *heap_end = *HEAP;
    }
}

#[no_mangle]
///! allocates `size` bytes with allignment `_align`
///! Currently, the pointer can't be freed
pub extern fn __rust_allocate(size: usize, _align: usize) -> *mut u8 {
    unsafe {
        heap_end = ((heap_end as usize) + size + (_align - size % _align)) as *mut u8;
        heap_end
    }
}

#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, _old_size: usize, _align: usize) {

}

#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, _old_size: usize, size: usize,  _align: usize) 
    -> *mut u8 {
        __rust_allocate(size, _align)
}

#[no_mangle]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, old_size: usize, _size: usize, _align: usize)
    -> usize {
        0
}

#[no_mangle]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
        size
}
