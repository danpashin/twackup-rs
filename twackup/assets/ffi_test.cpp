#include "twackup.h"
#include <iostream>

static void progress_did_increment(uint64_t delta) {
  std::cout << "progress_did_increment(" << delta << ")" <<  std::endl;
}

static void progress_set_message(slice_raw_uint8_t msg) {
  std::cout << "progress_set_message(\"" << std::string((char *)msg.ptr, msg.len) << "\")" <<  std::endl;
}

static void progress_print_message(slice_raw_uint8_t msg) {
  std::cout << "progress_print_message(\"" << std::string((char *)msg.ptr, msg.len) << "\")" <<  std::endl;
}

static void progress_print_warning(slice_raw_uint8_t msg) {
  std::cout << "progress_print_warning(\"" << std::string((char *)msg.ptr, msg.len) << "\")" <<  std::endl;
}

static void progress_print_error(slice_raw_uint8_t msg) {
  std::cout << "progress_print_error(\"" << std::string((char *)msg.ptr, msg.len) << "\")" <<  std::endl;
}


int main(int argc, char *argv[]) {

  if (argc != 2) {
    std::cout << "Usage: <" << argv[0] << "> path_to_dpkg_dir" << std::endl;
  }

  auto dpkg = tw_init(argv[1], false);

  auto packages = tw_get_packages(dpkg, true, TW_PACKAGES_SORT_UNSORTED);

  std::cout << "packages_ptr = "<< packages.ptr << ", count = " << packages.len << std::endl;

  for (int i = 0; i < packages.len; i++) {
    auto package = packages.ptr[i];
    std::cout << i + 1 << ". " << std::string((char *)package.identifier.ptr, package.identifier.len) << "; ";

    auto section = package.get_section_string(package.inner_ptr);
    std::cout << "section = " << std::string((char *)section.ptr, section.len) << "; ";

    auto arch = package.get_field(package.inner_ptr, TW_PACKAGE_FIELD_ARCHITECTURE);
    std::cout << "arch = " << std::string((char *)arch.ptr, arch.len) << "; ";

    auto deps = package.get_dependencies(package.inner_ptr);
    std::cout << std::endl;
    for (int j = 0; j < deps.len; j++) {
      std::cout << "dep: " << std::string((char *)deps.ptr[j].ptr, deps.ptr[j].len) << std::endl;
    }
    std::cout << std::endl;

    std::cout << std::endl;
  }

  slice_ref_TwPackage_t rebuild_packages;
  rebuild_packages.ptr = packages.ptr;
  rebuild_packages.len = packages.len;

  TwProgressFunctions_t functions;
  functions.did_increment = progress_did_increment;
  functions.set_message = progress_set_message;
  functions.print_message = progress_print_message;
  functions.print_warning = progress_print_warning;
  functions.print_error = progress_print_error;

  auto errors = tw_rebuild_packages(dpkg, rebuild_packages, functions, "/tmp");
  for (int i = 0; i < errors.len; i++) {
    auto error = errors.ptr[i];
    std::cout << "dep: " << std::string((char *)error.ptr, error.len) << std::endl;
  }

  free_packages_rebuild_result(errors);

  tw_free(dpkg);

  return 0;
}
