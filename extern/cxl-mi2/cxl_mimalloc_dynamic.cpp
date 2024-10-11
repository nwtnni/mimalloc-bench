#include "mimalloc.h"
#include <cstdlib>
#include <iostream>
#include <numaif.h>
#include <ostream>
#include <sys/mman.h>

static bool done_process;
static const size_t LENGTH = 1ull << 34;
static mi_arena_id_t arena;
thread_local bool done_thread;

static void init_process();
static void init_thread();

extern "C" void *malloc(size_t size) {
  init_thread();
  return mi_malloc(size);
}

extern "C" void free(void *ptr) {
  init_thread();
  return mi_free(ptr);
}

extern "C" void *realloc(void *ptr, size_t size) {
  init_thread();
  return mi_realloc(ptr, size);
}

extern "C" size_t malloc_usable_size(void *ptr) {
  init_thread();
  return mi_malloc_size(ptr);
}

extern "C" void *memalign(size_t alignment, size_t size) {
  init_thread();
  return mi_memalign(alignment, size);
}

extern "C" int posix_memalign(void **pointer, size_t align, size_t size) {
  init_thread();
  return mi_posix_memalign(pointer, align, size);
}

static void init_thread() {
  if (done_thread) {
    return;
  }

  init_process();

  mi_heap_t *heap = mi_heap_new_in_arena(arena);
  mi_heap_set_default(heap);
  done_thread = true;
}

static void init_process() {
  if (done_process) {
    return;
  }

  void *address = mmap(0, LENGTH, PROT_READ | PROT_WRITE,
                       MAP_ANONYMOUS | MAP_PRIVATE, -1, 0);

  if (address == MAP_FAILED) {
    std::cerr << "mmap" << std::endl;
    std::exit(1);
  }

  if (const char *node = std::getenv("CXL_NUMA_NODE")) {
    unsigned long mask = 1 << std::strtoul(node, nullptr, 10);
    if (mbind(address, LENGTH, MPOL_BIND, &mask, sizeof(mask) * 8, 0) < 0) {
      std::cerr << "mbind" << std::endl;
      std::exit(1);
    }
  }

  mi_manage_os_memory_ex(address, LENGTH, false, false, true, -1, true, &arena);
  done_process = true;
}
