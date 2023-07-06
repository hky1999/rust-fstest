use super::config;
use super::FILE_NAME;

cfg_if::cfg_if! {
if #[cfg(any(feature = "std", feature = "unishyper-std"))] {
use std::io::Read;
use std::io::Write;
use std::time::Instant;
// use std::fs;
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

#[cfg(feature = "unishyper-alloc")]
use unishyper::*;

#[cfg(feature = "unishyper-alloc")]
use alloc::vec;

#[cfg(not(feature = "unishyper-alloc"))]
pub fn files_bandwidth_test(rounds: u32, bytes: usize) {
    let total_bytes = rounds as u128 * bytes as u128;

    let progress_tracking_percentage = rounds / 10;

    let write_buf = vec![0xf as u8; bytes];

    for i in 0..rounds / 10 {
        #[cfg(target_os = "linux")]
        let mut f = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .custom_flags(libc::O_SYNC)
            .open(FILE_NAME)
            .expect("Can't open");

        #[cfg(target_os = "shyper")]
        let mut f = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(FILE_NAME)
            .expect("Can't open");

        // Write phase.
        let start = Instant::now();
        f.write(write_buf.as_slice())
            .unwrap_or_else(|err| panic!("failed to write to file {}, err {}", FILE_NAME, err));
        let write_time = Instant::now().duration_since(start).as_nanos();
        // total_write_time += write_time;

        drop(f);

        #[cfg(target_os = "linux")]
        let mut f = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .custom_flags(libc::O_SYNC)
            .open(FILE_NAME)
            .expect("Can't open");

        #[cfg(target_os = "shyper")]
        let mut f = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(FILE_NAME)
            .expect("Can't open");

        // Read phase.
        let mut read_buf = vec![0; bytes];
        let start = Instant::now();
        f.read(&mut read_buf)
            .unwrap_or_else(|err| panic!("failed to read from file {}, err {}", FILE_NAME, err));
        let read_time = Instant::now().duration_since(start).as_nanos();
        // total_read_time += read_time;

        if config::Config::parse().verbose() {
            println!(
                "round {} write_time {}ns, read_time {}ns",
                i, write_time, read_time
            );
        }

        assert_eq!(write_buf, read_buf, "file write/read failed");
    }

    println!("warnup finished, start testing bandwidth...");

    let mut total_read_time = 0;
    let mut total_write_time = 0;

    for i in 0..rounds {
        #[cfg(target_os = "linux")]
        let mut f = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .custom_flags(libc::O_SYNC)
            .open(FILE_NAME)
            .expect("Can't open");

        #[cfg(target_os = "shyper")]
        let mut f = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(FILE_NAME)
            .expect("Can't open");

        // Write phase.
        let start = Instant::now();
        f.write(write_buf.as_slice())
            .unwrap_or_else(|err| panic!("failed to write to file {}, err {}", FILE_NAME, err));
        let write_time = Instant::now().duration_since(start).as_nanos();
        total_write_time += write_time;

        drop(f);

        #[cfg(target_os = "linux")]
        let mut f = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .custom_flags(libc::O_SYNC)
            .open(FILE_NAME)
            .expect("Can't open");

        #[cfg(target_os = "shyper")]
        let mut f = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(FILE_NAME)
            .expect("Can't open");

        // Read phase.
        let mut read_buf = vec![0; bytes];
        let start = Instant::now();
        f.read(&mut read_buf)
            .unwrap_or_else(|err| panic!("failed to read from file {}, err {}", FILE_NAME, err));
        let read_time = Instant::now().duration_since(start).as_nanos();
        total_read_time += read_time;

        if config::Config::parse().verbose() {
            println!(
                "round {} write_time {}ns, read_time {}ns",
                i, write_time, read_time
            );
        }

        assert_eq!(write_buf, read_buf, "file write/read failed");
        if i % progress_tracking_percentage == 0 {
            println!("{}0% completed", i / progress_tracking_percentage);
        }
    }

    println!(
        "total {} bytes, total write {} ns, total read {} ns",
        total_bytes, total_write_time, total_read_time
    );
    println!(
        "Available approximated write bandwidth: {} MB/s",
        total_bytes / 1024 * 1000 * 1000 * 1000 / total_write_time
    );
    println!(
        "Available approximated read bandwidth: {} MB/s",
        total_bytes / 1024 * 1000 * 1000 * 1000 / total_read_time
    );
}

#[cfg(feature = "unishyper-alloc")]
pub fn files_bandwidth_test(rounds: u32, bytes: usize) {
    let total_bytes = rounds as u128 * bytes as u128;
    use unishyper::fs::Path;

    let path = Path::new(FILE_NAME);

    let progress_tracking_percentage = rounds / 10;

    let write_buf = vec![0xf as u8; bytes];

    for i in 0..rounds / 10 {
        let f = fs::File::create(&path)
            .unwrap_or_else(|err| panic!("failed to create file {}, err {}", FILE_NAME, err));

        // Write phase.
        let start = current_ns() as u128;
        f.write(write_buf.as_slice())
            .unwrap_or_else(|err| panic!("failed to write to file {}, err {}", FILE_NAME, err));
        let write_time = current_ns() as u128 - start;

        drop(f);

        let f = fs::File::create(&path)
            .unwrap_or_else(|err| panic!("failed to open file {}, err {}", FILE_NAME, err));

        // Read phase.
        let mut read_buf = vec![0; bytes];
        let start = current_ns() as u128;
        f.read(&mut read_buf)
            .unwrap_or_else(|err| panic!("failed to read from file {}, err {}", FILE_NAME, err));
        let read_time = current_ns() as u128 - start;

        if config::Config::parse().verbose() {
            println!(
                "round {} write_time {}ns, read_time {}ns",
                i, write_time, read_time
            );
        }

        assert_eq!(write_buf, read_buf, "file write/read failed");
    }

    println!("warnup finished, start testing bandwidth...");

    let mut total_read_time = 0;
    let mut total_write_time = 0;

    for i in 0..rounds {
        let f = fs::File::create(&path)
            .unwrap_or_else(|err| panic!("failed to open file {}, err {}", FILE_NAME, err));

        // Write phase.
        let start = current_ns() as u128;
        f.write(write_buf.as_slice())
            .unwrap_or_else(|err| panic!("failed to write to file {}, err {}", FILE_NAME, err));
        let write_time = current_ns() as u128 - start;
        total_write_time += write_time;

        drop(f);

        let f = fs::File::open(&path)
            .unwrap_or_else(|err| panic!("failed to open file {}, err {}", FILE_NAME, err));

        // Read phase.
        let mut read_buf = vec![0; bytes];
        let start = current_ns() as u128;
        f.read(&mut read_buf)
            .unwrap_or_else(|err| panic!("failed to read from file {}, err {}", FILE_NAME, err));
        let read_time = current_ns() as u128 - start;
        total_read_time += read_time;

        if config::Config::parse().verbose() {
            println!(
                "round {} write_time {}ns, read_time {}ns",
                i, write_time, read_time
            );
        }

        assert_eq!(write_buf, read_buf, "file write/read failed");
        if i % progress_tracking_percentage == 0 {
            println!("{}0% completed", i / progress_tracking_percentage);
        }
    }

    println!(
        "total {} bytes, total write {} ns, total read {} ns",
        total_bytes, total_write_time, total_read_time
    );
    println!(
        "Available approximated write bandwidth: {} MB/s",
        total_bytes / 1024 * 1000 * 1000 * 1000 / total_write_time
    );
    println!(
        "Available approximated read bandwidth: {} MB/s",
        total_bytes / 1024 * 1000 * 1000 * 1000 / total_read_time
    );
}
