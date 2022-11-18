#include "twackup.h"
#include <iostream>

int main(int argc, char *argv[]) {

  auto dpkg = tw_init("/var/lib/dpkg", false);

  auto packages = tw_get_packages(dpkg, true, TW_PACKAGES_SORT_UNSORTED);

  std::cout << "packages_ptr = "<< packages.ptr << ", count = " << packages.len << std::endl;

  for (int i = 0; i < packages.len; i++) {
    auto package = packages.ptr[i];
    std::cout << i + 1 << ". " << std::string((char *)package.identifier.ptr, package.identifier.len) << "; ";

    auto section = tw_package_section_description(&package);
    std::cout << "section = " << std::string((char *)section.ptr, section.len) << "; ";

    auto arch = tw_package_get_field(&package, TW_PACKAGE_FIELD_ARCHITECTURE);
    std::cout << "arch = " << std::string((char *)arch.ptr, arch.len) << "; ";

    std::cout << std::endl;
  }

  tw_free(dpkg);

  return 0;
}
