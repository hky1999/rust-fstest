#![feature(core_intrinsics)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(target_os = "shyper")]
use unishyper as _;

#[cfg(feature = "unishyper-alloc")]
use unishyper::*;


use core::sync::atomic::{AtomicBool, Ordering};

cfg_if::cfg_if! {
if #[cfg(feature = "std")] {
use std::io::{Read, Write};
use std::time::Instant;
use std::fs;
}
}

cfg_if::cfg_if! {
if #[cfg(target_os = "linux")] {
use clap::Parser;
mod config;
} else {
#[path = "config_nostd.rs"]
mod config;
}
}

mod statistician;

use statistician::Statistician;

const FILE_NAME: &str = "test_file";
static VERBOSE: AtomicBool = AtomicBool::new(false);

fn file_test(round: u32, bytes: usize) -> (u128, u128, u128, u128, u128) {
    if VERBOSE.load(Ordering::Relaxed) {
        println!("file test round {}", round);
    }

    /* create phase */
    let start = Instant::now();
    let mut f = fs::File::create(FILE_NAME)
        .unwrap_or_else(|err| panic!("failed to create file {}, err {}", FILE_NAME, err));
    let create_time = Instant::now().duration_since(start).as_nanos();

    /* write phase */
    let write_buf = vec![0xf as u8; bytes];
    let start = Instant::now();
    f.write(write_buf.as_slice())
        .unwrap_or_else(|err| panic!("failed to write to file {}, err {}", FILE_NAME, err));
    let write_time = Instant::now().duration_since(start).as_nanos();

    drop(f);

    /* open phase */
    let start = Instant::now();
    let mut f = fs::File::open(FILE_NAME)
        .unwrap_or_else(|err| panic!("failed to create file {}, err {}", FILE_NAME, err));
    let open_time = Instant::now().duration_since(start).as_nanos();

    /* read phase */
    let mut read_buf = vec![0; bytes];
    let start = Instant::now();
    f.read(&mut read_buf)
        .unwrap_or_else(|err| panic!("failed to read from file {}, err {}", FILE_NAME, err));
    let read_time = Instant::now().duration_since(start).as_nanos();

    assert_eq!(write_buf, read_buf, "file write/read failed");

    /* remove phase */
    let start = Instant::now();
    fs::remove_file(FILE_NAME)
        .unwrap_or_else(|err| panic!("failed to remove file {}, err {}", FILE_NAME, err));
    let remove_time = Instant::now().duration_since(start).as_nanos();

    (create_time, open_time, write_time, read_time, remove_time)
}

fn files_test(rounds: u32, bytes: usize) {
    let mut create_time_statistics = Statistician::default();
    let mut open_time_statistics = Statistician::default();
    let mut write_time_statistics = Statistician::default();
    let mut read_time_statistics = Statistician::default();
    let mut remove_time_statistics = Statistician::default();

    for i in 0..rounds {
        let (create_time, open_time, write_time, read_time, remove_time) =
            file_test(i, bytes as usize);
        create_time_statistics.update(create_time as f64);
        open_time_statistics.update(open_time as f64);
        write_time_statistics.update(write_time as f64);
        read_time_statistics.update(read_time as f64);
        remove_time_statistics.update(remove_time as f64);
        if VERBOSE.load(Ordering::Relaxed) {
            println!(
                "round {} create_time {}ns,open_time {}ns,write_time {}ns, read_time {}ns, remove_time {}ns.",
                i,  create_time, open_time, write_time, read_time, remove_time
            );
        }
    }

    if VERBOSE.load(Ordering::Relaxed) {
        println!("create_time_statistics {}", create_time_statistics);
        println!("open_time_statistics {}", open_time_statistics);
        println!("write_time_statistics {}", write_time_statistics);
        println!("read_time_statistics {}", read_time_statistics);
        println!("remove_time_statistics {}", remove_time_statistics);
    }

    println!("\nSUMMARY: (of {} rounds)\n", rounds);
    println!("   Operation                  Max(ns)        Min(ns)       Mean(ns)        Std Dev");
    println!("   ---------                      ---            ---           ----        -------");
    println!(
        "   File creation     : {:14.3} {:14.3} {:14.3} {:14.3} ",
        create_time_statistics.max(),
        create_time_statistics.min(),
        create_time_statistics.mean(),
        create_time_statistics.sstdev()
    );
    println!(
        "   File open         : {:14.3} {:14.3} {:14.3} {:14.3} ",
        open_time_statistics.max(),
        open_time_statistics.min(),
        open_time_statistics.mean(),
        open_time_statistics.sstdev()
    );
    println!(
        "   File write        : {:14.3} {:14.3} {:14.3} {:14.3} ",
        write_time_statistics.max(),
        write_time_statistics.min(),
        write_time_statistics.mean(),
        write_time_statistics.sstdev()
    );
    println!(
        "   File read         : {:14.3} {:14.3} {:14.3} {:14.3} ",
        read_time_statistics.max(),
        read_time_statistics.min(),
        read_time_statistics.mean(),
        read_time_statistics.sstdev()
    );
    println!(
        "   File removal      : {:14.3} {:14.3} {:14.3} {:14.3} ",
        remove_time_statistics.max(),
        remove_time_statistics.min(),
        remove_time_statistics.mean(),
        remove_time_statistics.sstdev()
    );
}

fn main() {
    let args = config::Config::parse();
    if args.verbose() {
        VERBOSE.swap(true, Ordering::Relaxed);
    }
    let (rounds, bytes) = args.rounds_and_bytes();

    println!(
        "Rust-based simple fs benchmark perform {} rounds, each read/write {} bytes",
        rounds, bytes
    );

    files_test(rounds, bytes as usize)
}
