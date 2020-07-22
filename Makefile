
BUILD_DIR=target
TARGET=twackup

.PHONY: all, release, ios, clean

ios:
	@RUSTFLAGS='-C link-arg=-s' cargo build --release --target aarch64-apple-ios
	@ldid -S ${BUILD_DIR}/aarch64-apple-ios/release/${TARGET}

release:
	@RUSTFLAGS='-C link-arg=-s' cargo build --release

all:
	@cargo build --debug

clean:
	@rm -rf ${BUILD_DIR}
