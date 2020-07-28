CC=cargo
CARGO_FLAGS=--release
TARGET=twackup
BUILD_DIR=build
RUSTFLAGS=
ARCHS=aarch64-apple-ios armv7-apple-ios

ifneq (,$(findstring --release,$(CARGO_FLAGS)))
	BINARY_TARGETS=$(addsuffix /release/${TARGET},$(addprefix ${BUILD_DIR}/,${ARCHS}))
	RUSTFLAGS+=-C link-arg=-s
else
	BINARY_TARGETS=$(addsuffix /debug/${TARGET},$(addprefix ${BUILD_DIR}/,${ARCHS}))
endif

PKG_METADATA:='$(shell $(CC) metadata --format-version 1 | jq -r '.packages[] | select(.name == "$(TARGET)")')'
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


.PHONY: all, ios, clean

all:
	@RUSTFLAGS="$(RUSTFLAGS)" $(CC) build $(CARGO_FLAGS) --target-dir $(BUILD_DIR)

ios: $(ARCHS)
	@lipo -create $(BINARY_TARGETS) -o $(BUILD_DIR)/$(TARGET)
	@ldid -S $(BUILD_DIR)/$(TARGET)
	@fpm -s dir -t deb -f --log error -n "$(DEB_IDENTIFIER)" --category "$(DEB_CATEGORY)" -d "$(DEB_DEPENDS)" \
	-a "$(DEB_ARCHITECTURE)" -m "$(DEB_AUTHOR)" --deb-priority "$(DEB_PRIORITY)" --description "$(DEB_DESCRITPTION)" \
	-v "$(DEB_VERSION)" --deb-no-default-config-files --vendor "" --deb-changelog "$(DEB_CHANGELOG)" \
	--url "$(DEB_HOMEPAGE)" --deb-field "Name: $(DEB_NAME)" $(DEB_FILES)

clean:
	@rm -rf ${BUILD_DIR}

$(ARCHS):
	@RUSTFLAGS="$(RUSTFLAGS)" $(CC) build $(CARGO_FLAGS) --target-dir $(BUILD_DIR) --target $@
