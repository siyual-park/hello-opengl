#![cfg_attr(feature = "cargo-clippy", allow(cast_lossless))]

extern crate gl;

#[cfg(feature = "chapter-1")]
mod _1_getting_started;
#[cfg(feature = "chapter-1")]
use _1_getting_started::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Call with the number of the tutorial, e.g. `1_1_2` for _1_2_hello_window_clear.rs");
        std::process::exit(1);
    }
    let tutorial_id = &args[1];

    match tutorial_id.as_str() {
        #[cfg(feature = "chapter-1")] "1_1_1" => main_1_1_1(),
        _     => println!("Unknown tutorial id")
    }
}