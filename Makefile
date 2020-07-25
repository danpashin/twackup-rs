CC=cargo
CARGO_FLAGS=--release
TARGET=twackup
BUILD_DIR=build
RUSTFLAGS="-C link-arg=-s"
ARCHS=aarch64-apple-ios armv7-apple-ios

ifneq (,$(findstring --release,$(CARGO_FLAGS)))
	BINARY_TARGETS=$(addsuffix /release/${TARGET},$(addprefix ${BUILD_DIR}/,${ARCHS}))
else
	BINARY_TARGETS=$(addsuffix /debug/${TARGET},$(addprefix ${BUILD_DIR}/,${ARCHS}))
endif

.PHONY: all, native, clean

all: $(ARCHS)
	@lipo -create $(BINARY_TARGETS) -o $(BUILD_DIR)/$(TARGET)
	@ldid -S $(BUILD_DIR)/$(TARGET)
	@$(CC) deb --no-build --no-strip -o $(BUILD_DIR)

native:
	@RUSTFLAGS=$(RUSTFLAGS) $(CC) build $(CARGO_FLAGS) --target-dir $(BUILD_DIR)

clean:
	@rm -rf ${BUILD_DIR}

$(ARCHS):
	@RUSTFLAGS=$(RUSTFLAGS) $(CC) build $(CARGO_FLAGS) --target-dir $(BUILD_DIR) --target $@
