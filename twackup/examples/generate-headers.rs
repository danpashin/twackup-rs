use std::env::args;
use std::path::PathBuf;

fn main() -> ::std::io::Result<()> {
    let target_dir = args().nth(1).expect("target dir is required");
    ::twackup::ffi::generate_headers(PathBuf::from(target_dir))
}
