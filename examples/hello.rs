#![cfg_attr(target_os = "psp", no_std)]
#![cfg_attr(target_os = "psp", no_main)]

use foam::{App, cfg_retro, foam_main, lprintln};

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
    let app = App::new() else {
        panic!("diddy foid");
    };
}
