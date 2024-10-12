#include <cstddef>

extern "C" void *cxl_mi_malloc(size_t size);
extern "C" void cxl_mi_free(void *ptr);
