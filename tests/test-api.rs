use libmimalloc3_sys as ffi;
use std::ffi::c_int;
use std::mem::size_of;

#[test]
fn test_heap1() {
    unsafe {
        let heap = ffi::mi_heap_new();
        let p1 = ffi::mi_heap_malloc(heap, size_of::<c_int>()) as *mut c_int;
        let p2 = ffi::mi_heap_malloc(heap, size_of::<c_int>()) as *mut c_int;
        *p2 = 43;
        *p1 = *p2;
        ffi::mi_heap_destroy(heap);
    }
}
