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
	
3. * For host platform run `make native` or `cargo build --release`
   * For iOS you have to install additional utilities:

      * [fpm](https://github.com/jordansissel/fpm)
      * [jq](https://stedolan.github.io/jq/)
      * [ldid](https://github.com/xerub/ldid)
      * lipo
      * **aarch64-apple-ios** and **armv7-apple-ios** targets for Rust
     
     Then run `make ios`
     
## Known issues

* Doesn't have arm64e (Apple A12 and higher) support with PAC as Rust doesn't support it. Although, plain arm64 runs fine.

## License

Twackup is licensed under GNU General Public License v3.0

See [COPYING](COPYING) for the full text.
