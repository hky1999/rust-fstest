use super::config;
use super::statistician::Statistician;
use super::FILE_NAME;

#[cfg(feature = "unishyper-alloc")]
use unishyper::*;

#[cfg(feature = "unishyper-alloc")]
use alloc::vec;

cfg_if::cfg_if! {
if #[cfg(any(feature = "std", feature = "unishyper-std"))] {
use std::io::{Read, Write};
use std::time::Instant;
use std::fs;
use std::fs::OpenOptions;
}
}

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        use clap::Parser;
        use std::os::unix::fs::OpenOptionsExt;
        extern crate libc;
    }
}

#[cfg(not(feature = "unishyper-alloc"))]
fn file_test(round: u32, bytes: usize) -> (u128, u128, u128, u128, u128) {
    if config::Config::parse().verbose() {
        println!("file test round {}", round);
    }

    /* create phase */
    let start = Instant::now();
    let f = fs::File::create(FILE_NAME)
        .unwrap_or_else(|err| panic!("failed to create file {}, err {}", FILE_NAME, err));
    let create_time = Instant::now().duration_since(start).as_nanos();

    drop(f);

    #[cfg(target_os = "linux")]
    let mut f = OpenOptions::new()
        .write(true)
        .custom_flags(libc::O_SYNC)
        .open(FILE_NAME)
        .expect("Can't open for write");

    #[cfg(target_os = "shyper")]
    let mut f = OpenOptions::new()
        .write(true)
        .open(FILE_NAME)
        .expect("Can't open for write");

    /* write phase */
    let write_buf = vec![0xf as u8; bytes];
    let start = Instant::now();
    f.write(write_buf.as_slice())
        .unwrap_or_else(|err| panic!("failed to write to file {}, err {}", FILE_NAME, err));
    let write_time = Instant::now().duration_since(start).as_nanos();

    drop(f);

    /* open phase */
    let start = Instant::now();

    #[cfg(target_os = "linux")]
    let mut f = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_SYNC)
        .open(FILE_NAME)
        .expect("Can't open for write");

    #[cfg(target_os = "shyper")]
    let mut f = OpenOptions::new()
        .read(true)
        .open(FILE_NAME)
        .expect("Can't open for write");

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

#[cfg(feature = "unishyper-alloc")]
fn file_test(round: u32, bytes: usize) -> (u128, u128, u128, u128, u128) {
    if config::Config::parse().verbose() {
        println!("file test round {}", round);
    }

    use unishyper::fs::Path;

    let path = Path::new(FILE_NAME);

    /* create phase */
    let start = current_ns();
    let f = fs::File::create(&path)
        .unwrap_or_else(|err| panic!("failed to create file {}, err {}", FILE_NAME, err));
    let create_time = current_ns() - start;

    /* write phase */
    let write_buf = vec![0xf as u8; bytes];
    let start = current_ns();
    f.write(write_buf.as_slice())
        .unwrap_or_else(|err| panic!("failed to write to file {}, err {}", FILE_NAME, err));
    let write_time = current_ns() - start;

    drop(f);

    /* open phase */
    let start = current_ns();
    let f = fs::File::open(&path)
        .unwrap_or_else(|err| panic!("failed to open file {}, err {}", FILE_NAME, err));
    let open_time = current_ns() - start;

    /* read phase */
    let mut read_buf = vec![0; bytes];
    let start = current_ns();
    f.read(&mut read_buf)
        .unwrap_or_else(|err| panic!("failed to read from file {}, err {}", FILE_NAME, err));
    let read_time = current_ns() - start;

    assert_eq!(write_buf, read_buf, "file write/read failed");

    /* remove phase */
    let start = current_ns();
    fs::unlink(&path)
        .unwrap_or_else(|err| panic!("failed to remove file {}, err {}", FILE_NAME, err));
    let remove_time = current_ns() - start;

    (
        create_time as u128,
        open_time as u128,
        write_time as u128,
        read_time as u128,
        remove_time as u128,
    )
}

pub fn files_latency_test(rounds: u32, bytes: usize) {
    let mut create_time_statistics = Statistician::default();
    let mut open_time_statistics = Statistician::default();
    let mut write_time_statistics = Statistician::default();
    let mut read_time_statistics = Statistician::default();
    let mut remove_time_statistics = Statistician::default();

    let progress_tracking_percentage = rounds / 10;

    for i in 0..rounds / 10 {
        let _ = file_test(i, bytes as usize);
    }
    println!("warnup finished, start testing latency...");

    for i in 0..rounds {
        let (create_time, open_time, write_time, read_time, remove_time) =
            file_test(i, bytes as usize);
        create_time_statistics.update(create_time as f64);
        open_time_statistics.update(open_time as f64);
        write_time_statistics.update(write_time as f64);
        read_time_statistics.update(read_time as f64);
        remove_time_statistics.update(remove_time as f64);
        if config::Config::parse().verbose() {
            println!(
                "round {} create_time {}ns,open_time {}ns,write_time {}ns, read_time {}ns, remove_time {}ns.",
                i,  create_time, open_time, write_time, read_time, remove_time
            );
        }
        if i % progress_tracking_percentage == 0 {
            println!("{}0% completed", i / progress_tracking_percentage);
        }
    }

    if config::Config::parse().verbose() {
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
