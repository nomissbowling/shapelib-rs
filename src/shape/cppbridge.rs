//! cppbridge
//!
//! to control publication included bridge_bindings.rs

#![allow(unused)]
// #![allow(unused_imports)]
// #![allow(unused_attributes)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

include!(concat!("../../include", "/bridge_bindings.rs"));
