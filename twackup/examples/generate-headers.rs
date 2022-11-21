use std::env::args;
use std::path::PathBuf;
use twackup::ffi::generate_headers;

fn main() -> ::std::io::Result<()> {
    let output_dir = args().nth(1).expect("target dir is required");
    let output_dir = PathBuf::from(output_dir);
    generate_headers(output_dir.as_ref())
}
