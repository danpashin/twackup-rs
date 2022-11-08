CARGO_FLAGS=--release
TARGET=twackup
BUILD_DIR=build
RUSTFLAGS=
OUTPUT_DIR=packages

IOS_ARCHS=aarch64-apple-ios
NATIVE_ARCH=$(shell basename $(shell dirname $(shell rustc --print target-libdir)))
ALL_ARCHS=$(shell rustc --print target-list)

CONFIGURATION=debug
ifneq (,$(findstring --release,$(CARGO_FLAGS)))
	CONFIGURATION=release
	RUSTFLAGS+=-C link-arg=-s
endif

IOS_BINARIES=$(addsuffix /$(CONFIGURATION)/${TARGET},$(addprefix ${BUILD_DIR}/,${IOS_ARCHS}))
NATIVE_BINARY=$(addsuffix /$(CONFIGURATION)/${TARGET},$(addprefix ${BUILD_DIR}/,${NATIVE_ARCH}))

PKG_METADATA:='$(shell cargo metadata --format-version 1 | jq -r '.packages[] | select(.name == "$(TARGET)")')'
DEB_IDENTIFIER:=$(shell echo $(PKG_METADATA) | jq -r '.metadata.deb_pkg.identifier')
DEB_VERSION:=$(shell echo $(PKG_METADATA) | jq -r '.version')
DEB_NAME:=$(shell echo $(PKG_METADATA) | jq -r '.metadata.deb_pkg.name')
DEB_DESCRITPTION:=$(shell echo $(PKG_METADATA) | jq -r '.description')
DEB_AUTHOR:=$(shell echo $(PKG_METADATA) | jq -r '.authors[0]')
DEB_CATEGORY:=$(shell echo $(PKG_METADATA) | jq -r '.metadata.deb_pkg.section')
DEB_PRIORITY:=$(shell echo $(PKG_METADATA) | jq -r '.metadata.deb_pkg.priority')
DEB_ARCHITECTURE:=$(shell echo $(PKG_METADATA) | jq -r '.metadata.deb_pkg.architecture')
DEB_DEPENDS:=$(shell echo $(PKG_METADATA) | jq -r '.metadata.deb_pkg.depends')
DEB_CHANGELOG:=$(shell echo $(PKG_METADATA) | jq -r '.metadata.deb_pkg.changelog')
DEB_FILES:=$(shell echo $(PKG_METADATA) | jq -r '.metadata.deb_pkg.assets[] | join("=")')
DEB_HOMEPAGE:=$(shell echo $(PKG_METADATA) | jq -r '.metadata.deb_pkg.homepage')
DEB_LICENSE_TYPE:=$(shell echo $(PKG_METADATA) | jq -r '.metadata.deb_pkg.license_type')


.PHONY: all, native, ios, test, clean
all: native ios

native: $(OUTPUT_DIR) $(NATIVE_ARCH) test
	@cp -a $(NATIVE_BINARY) $(OUTPUT_DIR)/$(TARGET)_$(NATIVE_ARCH)


ios: $(OUTPUT_DIR) $(IOS_ARCHS) test
	@lipo -create $(IOS_BINARIES) -o $(BUILD_DIR)/$(TARGET)-ios
	@ldid -S $(BUILD_DIR)/$(TARGET)-ios
	@fpm -s dir -t deb -f --log error -n "$(DEB_IDENTIFIER)" --category "$(DEB_CATEGORY)" -d "$(DEB_DEPENDS)" \
	-a "$(DEB_ARCHITECTURE)" -m "$(DEB_AUTHOR)" --deb-priority "$(DEB_PRIORITY)" --description "$(DEB_DESCRITPTION)" \
	-v "$(DEB_VERSION)" --license "$(DEB_LICENSE_TYPE)" --vendor "" --deb-changelog "$(DEB_CHANGELOG)" \
	--url "$(DEB_HOMEPAGE)" --deb-field "Name: $(DEB_NAME)" \
	-p $(OUTPUT_DIR)/$(DEB_IDENTIFIER)_$(DEB_VERSION)_$(DEB_ARCHITECTURE).deb  $(DEB_FILES)

clean:
	@rm -rf ${BUILD_DIR}

test:
	@cargo test --target-dir $(BUILD_DIR)

$(ALL_ARCHS):
	@RUSTFLAGS="$(RUSTFLAGS)" cargo build $(CARGO_FLAGS) --target-dir $(BUILD_DIR) --target $@

$(OUTPUT_DIR):
	@mkdir -p $@
