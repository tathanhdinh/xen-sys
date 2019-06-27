#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// WIP
#![allow(dead_code)]
#![feature(c_variadic)]

use std::{ffi::c_void, ptr::NonNull};

mod gen {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use gen::{
    __va_list_tag, xc_hypercall_buffer, xc_hypercall_buffer_array, xc_interface, xentoollog_level,
};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xentoollog_logger {
    pub vmessage: Option<
        unsafe extern "C" fn(
            logger: &Self,
            level: xentoollog_level,
            errnoval: i32,
            context: Option<NonNull<i8>>,
            format: Option<NonNull<i8>>,
            al: &__va_list_tag, // use ffi::VaList !? (c.f. Dan's Using C's va_list in Rust)
        ),
    >,
    pub progress: Option<
        unsafe extern "C" fn(
            logger: &Self,
            context: Option<NonNull<i8>>,
            doing_what: Option<NonNull<i8>>,
            percent: i32,
            done: u64,
            total: u64,
        ),
    >,
    pub destroy: Option<unsafe extern "C" fn(logger: &Self)>,
}

#[repr(transparent)]
pub struct xc_cpumap(gen::xc_cpumap_t);

#[repr(transparent)]
pub struct xc_nodemap(gen::xc_nodemap_t);

// Re-exports
extern "C" {
    pub fn xc_interface_open(
        logger: &xentoollog_logger,
        dombuild_logger: &xentoollog_logger,
        open_flags: u32,
    ) -> Option<NonNull<xc_interface>>;

    pub fn xc_interface_close(xch: &xc_interface) -> u32;

    pub fn xc__hypercall_buffer_alloc(
        xch: &xc_interface,
        b: &mut xc_hypercall_buffer,
        size: usize,
    ) -> Option<NonNull<c_void>>;

    pub fn xc__hypercall_buffer_free(xch: &xc_interface, b: &xc_hypercall_buffer);

    pub fn xc__hypercall_buffer_free_pages(
        xch: &xc_interface,
        b: &xc_hypercall_buffer,
        nr_pages: i32,
    );

    pub fn xc_hypercall_buffer_array_create(
        xch: &xc_interface,
        n: u32,
    ) -> Option<NonNull<xc_hypercall_buffer_array>>;

    pub fn xc__hypercall_buffer_array_alloc(
        xch: &xc_interface,
        array: &mut xc_hypercall_buffer_array,
        index: u32,
        hbuf: &mut xc_hypercall_buffer,
        size: usize,
    ) -> Option<NonNull<c_void>>;

    pub fn xc__hypercall_buffer_array_get(
        xch: &xc_interface,
        array: &mut xc_hypercall_buffer_array,
        index: u32,
        hbuf: &mut xc_hypercall_buffer,
    ) -> Option<NonNull<c_void>>;

    pub fn xc_hypercall_buffer_array_destroy(
        xc: &mut xc_interface,
        array: &mut xc_hypercall_buffer_array,
    );

    pub fn xc_get_max_cpus(xch: &xc_interface) -> i32;

    pub fn xc_get_online_cpus(xch: &xc_interface) -> i32;

    pub fn xc_get_cpumap_size(xch: &xc_interface) -> i32;

    pub fn xc_cpumap_alloc(xch: &xc_interface) -> xc_cpumap;

    pub fn xc_cpumap_clearcpu(cpu: i32, map: xc_cpumap);

    pub fn xc_cpumap_setcpu(cpu: i32, map: xc_cpumap);

    pub fn xc_cpumap_testcpu(cpu: i32, map: xc_cpumap) -> i32;

    pub fn xc_get_max_nodes(xch: &xc_interface) -> i32;

    pub fn xc_get_nodemap_size(xch: &xc_interface) -> i32;

    pub fn xc_nodemap_alloc(xch: &xc_interface) -> xc_nodemap;
}
