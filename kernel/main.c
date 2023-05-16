#include "types.h"
#include "param.h"
#include "memlayout.h"
#include "riscv.h"
#include "defs.h"

volatile static int started = 0;

// start() jumps here in supervisor mode on all CPUs.
void c_main() {
  int this_cpuid = cpuid();
  if (this_cpuid == 0) {
    consoleinit();
    printfinit();
    printf("\n");
    printf("xv6 kernel is booting\n");
    printf("\n");
    kinit();
    printf("[OK] physical page allocator\n");
    kvminit();
    printf("[OK] create kernel page table\n");
    kvminithart();
    printf("[OK] turn on paging\n");
    procinit();
    printf("[OK] process table\n");
    trapinit();
    printf("[OK] trap vectors\n");
    trapinithart();
    printf("[OK] install kernel trap vector\n");
    plicinit();
    printf("[OK] set up interrupt controller\n");
    plicinithart();
    printf("[OK] ask PLIC for device interrupts\n");
    binit();
    printf("[OK] buffer cache\n");
    iinit();
    printf("[OK] inode table\n");
    fileinit();
    printf("[OK] file table\n");
    virtio_disk_init();
    printf("[OK] emulated hard disk\n");
    userinit();
    printf("[OK] first user process\n");
    __sync_synchronize();
    started = 1;
  } else {
    while (started == 0)
      ;
    __sync_synchronize();
    printf("hart %d starting\n", cpuid());
    kvminithart();  // turn on paging
    trapinithart(); // install kernel trap vector
    plicinithart(); // ask PLIC for device interrupts
  }

  scheduler(this_cpuid);
}
