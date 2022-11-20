## Description

This is Twackup - a simple iOS CLI tool for re-packaging installed packages to DEB's or backing up and restoring packages or repos.

Twackup is the fastest tool at this moment. It uses Rust lightweight threads and doesn't rely on iOS IO. It also doesn't run any processes as many other utilities do.

## Building from source

1. Make sure you have Rust installed. If not,  I would recommend installing it via [rustup](https://rustup.rs). 
2. Clone the source with git:
	
	```sh
	git clone https://github.com/danpashin/twackup-rs.git
	cd twackup-rs
	```
	
3. * For host platform just run `cargo build --release`
   * For iOS, you have to install some utilities:

      * [cargo-make](https://github.com/sagiegurari/cargo-make)
      * [fpm](https://github.com/jordansissel/fpm)
      * [ldid](https://github.com/xerub/ldid)
      * lipo (on macOS it is already installed with XCLT or Xcode)
      * **aarch64-apple-ios** target for Rust
     
     Then run `cargo build-ios`

## MSRV
**1.62.1** if you are targeting to build library only

**1.64.0** for CLI
     
## License

Twackup is licensed under GNU General Public License v3.0

See [COPYING](COPYING) for the full text.
