use core::ffi::*;
use core::mem::offset_of;

pub const MI_MALLOC_VERSION: u32 = 316;
pub const __BOOL_TRUE_FALSE_ARE_DEFINED: u32 = 1;
pub const TRUE: u32 = 1;
pub const FALSE: u32 = 0;
pub const MI_SMALL_WSIZE_MAX: u32 = 128;
#[allow(non_camel_case_types)]
pub type wchar_t = c_int;
#[allow(non_camel_case_types)]
pub type max_align_t = f64;
unsafe extern "C" {
    pub fn mi_malloc(size: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_calloc(count: usize, size: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_realloc(p: *mut c_void, newsize: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_expand(p: *mut std::os::raw::c_void, newsize: usize) -> *mut std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn mi_free(p: *mut std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn mi_strdup(s: *const std::os::raw::c_char) -> *mut c_char;
}
unsafe extern "C" {
    pub fn mi_strndup(s: *const std::os::raw::c_char, n: usize) -> *mut c_char;
}
unsafe extern "C" {
    pub fn mi_realpath(fname: *const c_char, resolved_name: *mut c_char) -> *mut c_char;
}
unsafe extern "C" {
    pub fn mi_malloc_small(size: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_zalloc_small(size: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_zalloc(size: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_mallocn(count: usize, size: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_reallocn(p: *mut c_void, count: usize, size: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_reallocf(p: *mut c_void, newsize: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_usable_size(p: *const c_void) -> usize;
}
unsafe extern "C" {
    pub fn mi_good_size(size: usize) -> usize;
}
#[allow(non_camel_case_types)]
pub type mi_deferred_free_fun =
Option<unsafe extern "C" fn(force: bool, heartbeat: c_ulonglong, arg: *mut c_void)>;
unsafe extern "C" {
    pub fn mi_register_deferred_free(deferred_free: mi_deferred_free_fun, arg: *mut c_void);
}
#[allow(non_camel_case_types)]
pub type mi_output_fun = Option<unsafe extern "C" fn(msg: *const c_char, arg: *mut c_void)>;
unsafe extern "C" {
    pub fn mi_register_output(out: mi_output_fun, arg: *mut c_void);
}
#[allow(non_camel_case_types)]
pub type mi_error_fun = Option<unsafe extern "C" fn(err: c_int, arg: *mut c_void)>;
unsafe extern "C" {
    pub fn mi_register_error(fun: mi_error_fun, arg: *mut c_void);
}
unsafe extern "C" {
    pub fn mi_collect(force: bool);
}
unsafe extern "C" {
    pub fn mi_version() -> c_int;
}
unsafe extern "C" {
    pub fn mi_stats_reset();
}
unsafe extern "C" {
    pub fn mi_stats_merge();
}
unsafe extern "C" {
    pub fn mi_stats_print(out: *mut c_void);
}
unsafe extern "C" {
    pub fn mi_stats_print_out(out: mi_output_fun, arg: *mut c_void);
}
unsafe extern "C" {
    pub fn mi_thread_stats_print_out(out: mi_output_fun, arg: *mut c_void);
}
unsafe extern "C" {
    pub fn mi_options_print();
}
unsafe extern "C" {
    pub fn mi_process_info(
        elapsed_msecs: *mut usize,
        user_msecs: *mut usize,
        system_msecs: *mut usize,
        current_rss: *mut usize,
        peak_rss: *mut usize,
        current_commit: *mut usize,
        peak_commit: *mut usize,
        page_faults: *mut usize,
    );
}
unsafe extern "C" {
    pub fn mi_process_init();
}
unsafe extern "C" {
    pub fn mi_process_done();
}
unsafe extern "C" {
    pub fn mi_thread_init();
}
unsafe extern "C" {
    pub fn mi_thread_done();
}
unsafe extern "C" {
    pub fn mi_malloc_aligned(size: usize, alignment: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_malloc_aligned_at(size: usize, alignment: usize, offset: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_zalloc_aligned(size: usize, alignment: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_zalloc_aligned_at(size: usize, alignment: usize, offset: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_calloc_aligned(count: usize, size: usize, alignment: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_calloc_aligned_at(
        count: usize,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_realloc_aligned(p: *mut c_void, newsize: usize, alignment: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_realloc_aligned_at(
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct mi_heap_s {
    _unused: [u8; 0],
}
#[allow(non_camel_case_types)]
pub type mi_heap_t = mi_heap_s;
unsafe extern "C" {
    pub fn mi_heap_new() -> *mut mi_heap_t;
}
unsafe extern "C" {
    pub fn mi_heap_delete(heap: *mut mi_heap_t);
}
unsafe extern "C" {
    pub fn mi_heap_destroy(heap: *mut mi_heap_t);
}
unsafe extern "C" {
    pub fn mi_heap_set_default(heap: *mut mi_heap_t) -> *mut mi_heap_t;
}
unsafe extern "C" {
    pub fn mi_heap_get_default() -> *mut mi_heap_t;
}
unsafe extern "C" {
    pub fn mi_heap_get_backing() -> *mut mi_heap_t;
}
unsafe extern "C" {
    pub fn mi_heap_collect(heap: *mut mi_heap_t, force: bool);
}
unsafe extern "C" {
    pub fn mi_heap_malloc(heap: *mut mi_heap_t, size: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_heap_zalloc(heap: *mut mi_heap_t, size: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_heap_calloc(heap: *mut mi_heap_t, count: usize, size: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_heap_mallocn(heap: *mut mi_heap_t, count: usize, size: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_heap_malloc_small(heap: *mut mi_heap_t, size: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_heap_realloc(heap: *mut mi_heap_t, p: *mut c_void, newsize: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_heap_reallocn(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        count: usize,
        size: usize,
    ) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_heap_reallocf(heap: *mut mi_heap_t, p: *mut c_void, newsize: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_heap_strdup(heap: *mut mi_heap_t, s: *const c_char) -> *mut c_char;
}
unsafe extern "C" {
    pub fn mi_heap_strndup(heap: *mut mi_heap_t, s: *const c_char, n: usize) -> *mut c_char;
}
unsafe extern "C" {
    pub fn mi_heap_realpath(
        heap: *mut mi_heap_t,
        fname: *const c_char,
        resolved_name: *mut c_char,
    ) -> *mut c_char;
}
unsafe extern "C" {
    pub fn mi_heap_malloc_aligned(
        heap: *mut mi_heap_t,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_heap_malloc_aligned_at(
        heap: *mut mi_heap_t,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_heap_zalloc_aligned(
        heap: *mut mi_heap_t,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_heap_zalloc_aligned_at(
        heap: *mut mi_heap_t,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_heap_calloc_aligned(
        heap: *mut mi_heap_t,
        count: usize,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_heap_calloc_aligned_at(
        heap: *mut mi_heap_t,
        count: usize,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_heap_realloc_aligned(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
    ) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_heap_realloc_aligned_at(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_rezalloc(p: *mut c_void, newsize: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_recalloc(p: *mut c_void, newcount: usize, size: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_rezalloc_aligned(p: *mut c_void, newsize: usize, alignment: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_rezalloc_aligned_at(
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_recalloc_aligned(
        p: *mut c_void,
        newcount: usize,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_recalloc_aligned_at(
        p: *mut c_void,
        newcount: usize,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_heap_rezalloc(heap: *mut mi_heap_t, p: *mut c_void, newsize: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_heap_recalloc(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newcount: usize,
        size: usize,
    ) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_heap_rezalloc_aligned(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
    ) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_heap_rezalloc_aligned_at(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_heap_recalloc_aligned(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newcount: usize,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_heap_recalloc_aligned_at(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newcount: usize,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_heap_contains_block(heap: *mut mi_heap_t, p: *const c_void) -> bool;
}
unsafe extern "C" {
    pub fn mi_heap_check_owned(heap: *mut mi_heap_t, p: *const c_void) -> bool;
}
unsafe extern "C" {
    pub fn mi_check_owned(p: *const c_void) -> bool;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct mi_heap_area_s {
    pub blocks: *mut c_void,
    pub reserved: usize,
    pub committed: usize,
    pub used: usize,
    pub block_size: usize,
    pub full_block_size: usize,
    pub heap_tag: c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of mi_heap_area_s"][size_of::<mi_heap_area_s>() - 56usize];
    ["Alignment of mi_heap_area_s"][align_of::<mi_heap_area_s>() - 8usize];
    ["Offset of field: mi_heap_area_s::blocks"][offset_of!(mi_heap_area_s, blocks) - 0usize];
    ["Offset of field: mi_heap_area_s::reserved"][offset_of!(mi_heap_area_s, reserved) - 8usize];
    ["Offset of field: mi_heap_area_s::committed"][offset_of!(mi_heap_area_s, committed) - 16usize];
    ["Offset of field: mi_heap_area_s::used"][offset_of!(mi_heap_area_s, used) - 24usize];
    ["Offset of field: mi_heap_area_s::block_size"]
        [offset_of!(mi_heap_area_s, block_size) - 32usize];
    ["Offset of field: mi_heap_area_s::full_block_size"]
        [offset_of!(mi_heap_area_s, full_block_size) - 40usize];
    ["Offset of field: mi_heap_area_s::heap_tag"][offset_of!(mi_heap_area_s, heap_tag) - 48usize];
};
#[allow(non_camel_case_types)]
pub type mi_heap_area_t = mi_heap_area_s;
#[allow(non_camel_case_types)]
pub type mi_block_visit_fun = Option<
    unsafe extern "C" fn(
        heap: *const mi_heap_t,
        area: *const mi_heap_area_t,
        block: *mut c_void,
        block_size: usize,
        arg: *mut c_void,
    ) -> bool,
>;
unsafe extern "C" {
    pub fn mi_heap_visit_blocks(
        heap: *const mi_heap_t,
        visit_blocks: bool,
        visitor: mi_block_visit_fun,
        arg: *mut c_void,
    ) -> bool;
}
unsafe extern "C" {
    pub fn mi_is_in_heap_region(p: *const c_void) -> bool;
}
unsafe extern "C" {
    pub fn mi_is_redirected() -> bool;
}
unsafe extern "C" {
    pub fn mi_reserve_huge_os_pages_interleave(
        pages: usize,
        numa_nodes: usize,
        timeout_msecs: usize,
    ) -> c_int;
}
unsafe extern "C" {
    pub fn mi_reserve_huge_os_pages_at(
        pages: usize,
        numa_node: c_int,
        timeout_msecs: usize,
    ) -> c_int;
}
unsafe extern "C" {
    pub fn mi_reserve_os_memory(size: usize, commit: bool, allow_large: bool) -> c_int;
}
unsafe extern "C" {
    pub fn mi_manage_os_memory(
        start: *mut c_void,
        size: usize,
        is_committed: bool,
        is_pinned: bool,
        is_zero: bool,
        numa_node: c_int,
    ) -> bool;
}
unsafe extern "C" {
    pub fn mi_debug_show_arenas();
}
unsafe extern "C" {
    pub fn mi_arenas_print();
}
unsafe extern "C" {
    pub fn mi_arena_min_alignment() -> usize;
}
#[allow(non_camel_case_types)]
pub type mi_arena_id_t = *mut c_void;
unsafe extern "C" {
    pub fn mi_arena_area(arena_id: mi_arena_id_t, size: *mut usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_reserve_huge_os_pages_at_ex(
        pages: usize,
        numa_node: c_int,
        timeout_msecs: usize,
        exclusive: bool,
        arena_id: *mut mi_arena_id_t,
    ) -> c_int;
}
unsafe extern "C" {
    pub fn mi_reserve_os_memory_ex(
        size: usize,
        commit: bool,
        allow_large: bool,
        exclusive: bool,
        arena_id: *mut mi_arena_id_t,
    ) -> c_int;
}
unsafe extern "C" {
    pub fn mi_manage_os_memory_ex(
        start: *mut c_void,
        size: usize,
        is_committed: bool,
        is_pinned: bool,
        is_zero: bool,
        numa_node: c_int,
        exclusive: bool,
        arena_id: *mut mi_arena_id_t,
    ) -> bool;
}
unsafe extern "C" {
    pub fn mi_heap_new_in_arena(arena_id: mi_arena_id_t) -> *mut mi_heap_t;
}
#[allow(non_camel_case_types)]
pub type mi_subproc_id_t = *mut c_void;
unsafe extern "C" {
    pub fn mi_subproc_main() -> mi_subproc_id_t;
}
unsafe extern "C" {
    pub fn mi_subproc_new() -> mi_subproc_id_t;
}
unsafe extern "C" {
    pub fn mi_subproc_delete(subproc: mi_subproc_id_t);
}
unsafe extern "C" {
    pub fn mi_subproc_add_current_thread(subproc: mi_subproc_id_t);
}
unsafe extern "C" {
    pub fn mi_abandoned_visit_blocks(
        subproc_id: mi_subproc_id_t,
        heap_tag: c_int,
        visit_blocks: bool,
        visitor: mi_block_visit_fun,
        arg: *mut c_void,
    ) -> bool;
}
unsafe extern "C" {
    pub fn mi_heap_set_numa_affinity(heap: *mut mi_heap_t, numa_node: c_int);
}
unsafe extern "C" {
    pub fn mi_heap_guarded_set_sample_rate(heap: *mut mi_heap_t, sample_rate: usize, seed: usize);
}
unsafe extern "C" {
    pub fn mi_heap_guarded_set_size_bound(heap: *mut mi_heap_t, min: usize, max: usize);
}
unsafe extern "C" {
    pub fn mi_thread_set_in_threadpool();
}
unsafe extern "C" {
    pub fn mi_heap_new_ex(
        heap_tag: c_int,
        allow_destroy: bool,
        arena_id: mi_arena_id_t,
    ) -> *mut mi_heap_t;
}
unsafe extern "C" {
    #[deprecated]
    pub fn mi_reserve_huge_os_pages(
        pages: usize,
        max_secs: f64,
        pages_reserved: *mut usize,
    ) -> c_int;
}
unsafe extern "C" {
    #[deprecated]
    pub fn mi_collect_reduce(target_thread_owned: usize);
}
#[allow(non_camel_case_types)]
pub type mi_commit_fun_t = Option<
    unsafe extern "C" fn(
        commit: bool,
        start: *mut c_void,
        size: usize,
        is_zero: *mut bool,
        user_arg: *mut c_void,
    ) -> bool,
>;
unsafe extern "C" {
    pub fn mi_manage_memory(
        start: *mut c_void,
        size: usize,
        is_committed: bool,
        is_pinned: bool,
        is_zero: bool,
        numa_node: c_int,
        exclusive: bool,
        commit_fun: mi_commit_fun_t,
        commit_fun_arg: *mut c_void,
        arena_id: *mut mi_arena_id_t,
    ) -> bool;
}
unsafe extern "C" {
    pub fn mi_arena_unload(
        arena_id: mi_arena_id_t,
        base: *mut *mut c_void,
        accessed_size: *mut usize,
        size: *mut usize,
    ) -> bool;
}
unsafe extern "C" {
    pub fn mi_arena_reload(
        start: *mut c_void,
        size: usize,
        commit_fun: mi_commit_fun_t,
        commit_fun_arg: *mut c_void,
        arena_id: *mut mi_arena_id_t,
    ) -> bool;
}
unsafe extern "C" {
    pub fn mi_heap_reload(heap: *mut mi_heap_t, arena: mi_arena_id_t) -> bool;
}
unsafe extern "C" {
    pub fn mi_heap_unload(heap: *mut mi_heap_t);
}
unsafe extern "C" {
    pub fn mi_arena_contains(arena_id: mi_arena_id_t, p: *const c_void) -> bool;
}
pub const MI_OPTION_E_MI_OPTION_SHOW_ERRORS: mi_option_e = 0;
pub const MI_OPTION_E_MI_OPTION_SHOW_STATS: mi_option_e = 1;
pub const MI_OPTION_E_MI_OPTION_VERBOSE: mi_option_e = 2;
pub const MI_OPTION_E_MI_OPTION_EAGER_COMMIT: mi_option_e = 3;
pub const MI_OPTION_E_MI_OPTION_ARENA_EAGER_COMMIT: mi_option_e = 4;
pub const MI_OPTION_E_MI_OPTION_PURGE_DECOMMITS: mi_option_e = 5;
pub const MI_OPTION_E_MI_OPTION_ALLOW_LARGE_OS_PAGES: mi_option_e = 6;
pub const MI_OPTION_E_MI_OPTION_RESERVE_HUGE_OS_PAGES: mi_option_e = 7;
pub const MI_OPTION_E_MI_OPTION_RESERVE_HUGE_OS_PAGES_AT: mi_option_e = 8;
pub const MI_OPTION_E_MI_OPTION_RESERVE_OS_MEMORY: mi_option_e = 9;
pub const MI_OPTION_E_MI_OPTION_DEPRECATED_SEGMENT_CACHE: mi_option_e = 10;
pub const MI_OPTION_E_MI_OPTION_DEPRECATED_PAGE_RESET: mi_option_e = 11;
pub const MI_OPTION_E_MI_OPTION_ABANDONED_PAGE_PURGE: mi_option_e = 12;
pub const MI_OPTION_E_MI_OPTION_DEPRECATED_SEGMENT_RESET: mi_option_e = 13;
pub const MI_OPTION_E_MI_OPTION_EAGER_COMMIT_DELAY: mi_option_e = 14;
pub const MI_OPTION_E_MI_OPTION_PURGE_DELAY: mi_option_e = 15;
pub const MI_OPTION_E_MI_OPTION_USE_NUMA_NODES: mi_option_e = 16;
pub const MI_OPTION_E_MI_OPTION_DISALLOW_OS_ALLOC: mi_option_e = 17;
pub const MI_OPTION_E_MI_OPTION_OS_TAG: mi_option_e = 18;
pub const MI_OPTION_E_MI_OPTION_MAX_ERRORS: mi_option_e = 19;
pub const MI_OPTION_E_MI_OPTION_MAX_WARNINGS: mi_option_e = 20;
pub const MI_OPTION_E_MI_OPTION_DEPRECATED_MAX_SEGMENT_RECLAIM: mi_option_e = 21;
pub const MI_OPTION_E_MI_OPTION_DESTROY_ON_EXIT: mi_option_e = 22;
pub const MI_OPTION_E_MI_OPTION_ARENA_RESERVE: mi_option_e = 23;
pub const MI_OPTION_E_MI_OPTION_ARENA_PURGE_MULT: mi_option_e = 24;
pub const MI_OPTION_E_MI_OPTION_DEPRECATED_PURGE_EXTEND_DELAY: mi_option_e = 25;
pub const MI_OPTION_E_MI_OPTION_DISALLOW_ARENA_ALLOC: mi_option_e = 26;
pub const MI_OPTION_E_MI_OPTION_RETRY_ON_OOM: mi_option_e = 27;
pub const MI_OPTION_E_MI_OPTION_VISIT_ABANDONED: mi_option_e = 28;
pub const MI_OPTION_E_MI_OPTION_GUARDED_MIN: mi_option_e = 29;
pub const MI_OPTION_E_MI_OPTION_GUARDED_MAX: mi_option_e = 30;
pub const MI_OPTION_E_MI_OPTION_GUARDED_PRECISE: mi_option_e = 31;
pub const MI_OPTION_E_MI_OPTION_GUARDED_SAMPLE_RATE: mi_option_e = 32;
pub const MI_OPTION_E_MI_OPTION_GUARDED_SAMPLE_SEED: mi_option_e = 33;
pub const MI_OPTION_E_MI_OPTION_GENERIC_COLLECT: mi_option_e = 34;
pub const MI_OPTION_E_MI_OPTION_PAGE_RECLAIM_ON_FREE: mi_option_e = 35;
pub const MI_OPTION_E_MI_OPTION_PAGE_FULL_RETAIN: mi_option_e = 36;
pub const MI_OPTION_E_MI_OPTION_PAGE_MAX_CANDIDATES: mi_option_e = 37;
pub const MI_OPTION_E_MI_OPTION_MAX_VABITS: mi_option_e = 38;
pub const MI_OPTION_E_MI_OPTION_PAGEMAP_COMMIT: mi_option_e = 39;
pub const MI_OPTION_E_MI_OPTION_PAGE_COMMIT_ON_DEMAND: mi_option_e = 40;
pub const MI_OPTION_E_MI_OPTION_PAGE_MAX_RECLAIM: mi_option_e = 41;
pub const MI_OPTION_E_MI_OPTION_PAGE_CROSS_THREAD_MAX_RECLAIM: mi_option_e = 42;
pub const MI_OPTION_E_MI_OPTION_LAST: mi_option_e = 43;
pub const MI_OPTION_E_MI_OPTION_LARGE_OS_PAGES: mi_option_e = 6;
pub const MI_OPTION_E_MI_OPTION_EAGER_REGION_COMMIT: mi_option_e = 4;
pub const MI_OPTION_E_MI_OPTION_RESET_DECOMMITS: mi_option_e = 5;
pub const MI_OPTION_E_MI_OPTION_RESET_DELAY: mi_option_e = 15;
pub const MI_OPTION_E_MI_OPTION_ABANDONED_PAGE_RESET: mi_option_e = 12;
pub const MI_OPTION_E_MI_OPTION_LIMIT_OS_ALLOC: mi_option_e = 17;
#[allow(non_camel_case_types)]
pub type mi_option_e = c_uint;
pub use self::mi_option_e as mi_option_t;
unsafe extern "C" {
    pub fn mi_option_is_enabled(option: mi_option_t) -> bool;
}
unsafe extern "C" {
    pub fn mi_option_enable(option: mi_option_t);
}
unsafe extern "C" {
    pub fn mi_option_disable(option: mi_option_t);
}
unsafe extern "C" {
    pub fn mi_option_set_enabled(option: mi_option_t, enable: bool);
}
unsafe extern "C" {
    pub fn mi_option_set_enabled_default(option: mi_option_t, enable: bool);
}
unsafe extern "C" {
    pub fn mi_option_get(option: mi_option_t) -> c_long;
}
unsafe extern "C" {
    pub fn mi_option_get_clamp(option: mi_option_t, min: c_long, max: c_long) -> c_long;
}
unsafe extern "C" {
    pub fn mi_option_get_size(option: mi_option_t) -> usize;
}
unsafe extern "C" {
    pub fn mi_option_set(option: mi_option_t, value: c_long);
}
unsafe extern "C" {
    pub fn mi_option_set_default(option: mi_option_t, value: c_long);
}
unsafe extern "C" {
    pub fn mi_cfree(p: *mut c_void);
}
unsafe extern "C" {
    pub fn mi__expand(p: *mut c_void, newsize: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_malloc_size(p: *const c_void) -> usize;
}
unsafe extern "C" {
    pub fn mi_malloc_good_size(size: usize) -> usize;
}
unsafe extern "C" {
    pub fn mi_malloc_usable_size(p: *const c_void) -> usize;
}
unsafe extern "C" {
    pub fn mi_posix_memalign(p: *mut *mut c_void, alignment: usize, size: usize) -> c_int;
}
unsafe extern "C" {
    pub fn mi_memalign(alignment: usize, size: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_valloc(size: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_pvalloc(size: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_aligned_alloc(alignment: usize, size: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_reallocarray(p: *mut c_void, count: usize, size: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_reallocarr(p: *mut c_void, count: usize, size: usize) -> c_int;
}
unsafe extern "C" {
    pub fn mi_aligned_recalloc(
        p: *mut c_void,
        newcount: usize,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_aligned_offset_recalloc(
        p: *mut c_void,
        newcount: usize,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_wcsdup(s: *const c_ushort) -> *mut c_ushort;
}
unsafe extern "C" {
    pub fn mi_mbsdup(s: *const c_uchar) -> *mut c_uchar;
}
unsafe extern "C" {
    pub fn mi_dupenv_s(buf: *mut *mut c_char, size: *mut usize, name: *const c_char) -> c_int;
}
unsafe extern "C" {
    pub fn mi_wdupenv_s(buf: *mut *mut c_ushort, size: *mut usize, name: *const c_ushort) -> c_int;
}
unsafe extern "C" {
    pub fn mi_free_size(p: *mut c_void, size: usize);
}
unsafe extern "C" {
    pub fn mi_free_size_aligned(p: *mut c_void, size: usize, alignment: usize);
}
unsafe extern "C" {
    pub fn mi_free_aligned(p: *mut c_void, alignment: usize);
}
unsafe extern "C" {
    pub fn mi_new(size: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_new_aligned(size: usize, alignment: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_new_nothrow(size: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_new_aligned_nothrow(size: usize, alignment: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_new_n(count: usize, size: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_new_realloc(p: *mut c_void, newsize: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_new_reallocn(p: *mut c_void, newcount: usize, size: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_heap_alloc_new(heap: *mut mi_heap_t, size: usize) -> *mut c_void;
}
unsafe extern "C" {
    pub fn mi_heap_alloc_new_n(heap: *mut mi_heap_t, count: usize, size: usize) -> *mut c_void;
}
