#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod gen {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

// Re-exports
pub use gen::*;
