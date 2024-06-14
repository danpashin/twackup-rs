#include "twackup.h"
#include <iostream>
#include <cassert>

static void started_processing(void *context, TwPackage_t package) {
  std::cout
  << "started_processing(\""
  << std::string((char *)package.identifier.ptr, package.identifier.len)
  << "\")"
  << std::endl;

  tw_package_release(package.inner);
}

static void finished_processing(void *context, TwPackage_t package, slice_raw_uint8_t deb_path) {
  std::cout
  << "finished_processing(\""
  << std::string((char *)package.identifier.ptr, package.identifier.len)
  << "\")"
  << std::endl;

  tw_package_release(package.inner);
}

static void finished_all(void *context) {
  std::cout << "finished_all()" <<  std::endl;
}


int main(int argc, char *argv[]) {

  if (argc != 2) {
    std::cout << "Usage: <" << argv[0] << "> path_to_dpkg_dir" << std::endl;
  }

  auto dpkg = tw_init(argv[1], false);

  TwPackage_t *packagesPtr = NULL;
  size_t count = tw_get_packages(dpkg, false, TW_PACKAGES_SORT_UNSORTED, &packagesPtr);

  assert(count > 0);
  std::cout << "packages_ptr = "<< packagesPtr << ", count = " << count << std::endl;

  std::vector<TwPackage_t> packages(packagesPtr,packagesPtr+count);

  for (auto package: packages) {
    std::cout << ". " << std::string((char *)package.identifier.ptr, package.identifier.len) << "; ";

    auto section = tw_package_section_str(package.inner);
    std::cout << "section = " << std::string((char *)section.ptr, section.len) << "; ";

    auto arch = tw_package_field_str(package.inner, TW_PACKAGE_FIELD_ARCHITECTURE);
    std::cout << "arch = " << std::string((char *)arch.ptr, arch.len) << "; ";

    auto deps = tw_package_dependencies(package.inner);
    std::cout << std::endl;
    for (int j = 0; j < deps.len; j++) {
      std::cout << "dep: " << std::string((char *)deps.ptr[j].ptr, deps.ptr[j].len) << std::endl;
    }
    std::cout << std::endl;

    std::cout << std::endl;
  }

  auto rawPkgsPointers = new TwPackageRef_t[5];
  auto rawPkgsPointersPtr = rawPkgsPointers;
  for (auto package: packages) {
    *rawPkgsPointersPtr++ = package.inner;
  }

  slice_ref_TwPackageRef_t rebuild_packages;
  rebuild_packages.ptr = rawPkgsPointers;
  rebuild_packages.len = count;

  TwProgressFunctions_t functions;
  functions.started_processing = started_processing;
  functions.finished_processing = finished_processing;
  functions.finished_all = finished_all;

  slice_boxed_TwPackagesRebuildResult_t results;

  TwBuildParameters_t parameters{};
  parameters.packages = rebuild_packages;
  parameters.functions = functions;
  parameters.out_dir = "/tmp";
  parameters.results = &results;

  auto result_code = tw_rebuild_packages(dpkg, parameters);
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
