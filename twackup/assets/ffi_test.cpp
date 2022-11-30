#include "twackup.h"
#include <iostream>

static void print_message(void *context, slice_raw_uint8_t msg, TwMessageLevel_t level) {
  std::cout
  << "print_message(\""
  << std::string((char *)msg.ptr, msg.len)
  << "\", "
  << level
  << ")"
  << std::endl;
}

static void started_processing(void *context, TwPackage_t const *package) {
  std::cout
  << "started_processing(\""
  << std::string((char *)package->identifier.ptr, package->identifier.len)
  << "\")"
  << std::endl;
}

static void finished_processing(void *context, TwPackage_t const *package) {
  std::cout
  << "finished_processing(\""
  << std::string((char *)package->identifier.ptr, package->identifier.len)
  << "\")"
  << std::endl;
}

static void finished_all(void *context) {
  std::cout << "finished_all()" <<  std::endl;
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
  functions.print_message = print_message;
  functions.started_processing = started_processing;
  functions.finished_processing = finished_processing;
  functions.finished_all = finished_all;

  slice_boxed_TwPackagesRebuildResult_t results;
  auto result_code = tw_rebuild_packages(dpkg, rebuild_packages, functions, "/tmp", &results);
  std::cout << "result code = " << int(result_code) << std::endl;
  for (int i = 0; i < results.len; i++) {
    auto result = results.ptr[i];
    if (result.success) {
      std::cout << "build deb path: " << std::string((char *) result.deb_path.ptr, result.deb_path.len) << std::endl;
    } else {
      std::cout << "build error: " << std::string((char *) result.error.ptr, result.error.len) << std::endl;
    }
  }

  tw_free_rebuild_results(results);

  tw_free(dpkg);

  return 0;
}
