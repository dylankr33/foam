#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

use core::{concat, env, include, prelude::rust_2024::*};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
