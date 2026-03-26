#![cfg_attr(target_os = "psp", no_std)]
#![cfg_attr(target_os = "psp", no_main)]

use foam::lprintln;
use foam_proc_macro::{cfg_retro, foam_main};

#[cfg_retro]
use alloc::vec;

#[foam_main]
fn main() {
    lprintln!("hi");
    lprintln!("penis");

    let people = vec!["yo", "what's", "up"];
    for i in people {
        lprintln!("It's {}", i)
    }
}
