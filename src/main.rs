#![feature(core_intrinsics)]
#![feature(format_args_nl)]
#![feature(stmt_expr_attributes)]
// #![cfg_attr(feature = "unishyper-alloc", format_args_nl)]
#![cfg_attr(feature = "unishyper-alloc", no_std)]
#![cfg_attr(feature = "unishyper-alloc", no_main)]

#[cfg(target_os = "shyper")]
use unishyper as _;

#[cfg(feature = "unishyper-alloc")]
use unishyper::*;

#[cfg(feature = "unishyper-alloc")]
extern crate alloc;

cfg_if::cfg_if! {
if #[cfg(target_os = "linux")] {
use clap::Parser;
pub mod config;
} else {
#[path = "config_nostd.rs"]
pub mod config;
}
}

pub mod statistician;

mod latency;
mod bandwidth;

pub const FILE_NAME: &str = "test_file";

#[cfg_attr(feature = "unishyper-alloc", no_mangle)]
fn main() {
    let args = config::Config::parse();

    let (rounds, bytes) = args.rounds_and_bytes();

    println!(
        "Rust-based simple fs benchmark perform {} rounds, each read/write {} bytes",
        rounds, bytes
    );

    latency::files_latency_test(rounds, bytes as usize);
    bandwidth::files_bandwidth_test(rounds, bytes as usize);

    #[cfg(feature = "unishyper-alloc")]
    exit()
}
