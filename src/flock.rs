/*
 * This part was taken right from Cargo
 * https://github.com/rust-lang/cargo/blob/master/src/cargo/util/flock.rs
 *
 */

use std::fs::File;
use std::io::{Error, Result};
use std::os::unix::io::AsRawFd;

pub fn lock_exclusive(file: &File) -> Result<()> {
    flock(file, libc::LOCK_EX)
}

pub fn unlock(file: &File) -> Result<()> {
    flock(file, libc::LOCK_UN)
}

fn flock(file: &File, flag: libc::c_int) -> Result<()> {
    let ret = unsafe { libc::flock(file.as_raw_fd(), flag) };
    if ret < 0 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}
