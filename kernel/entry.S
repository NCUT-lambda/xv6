        # qemu -kernel loads the kernel at 0x80000000
        # and causes each hart (i.e. CPU) to jump there.
        # kernel.ld causes the following code to
        # be placed at 0x80000000.
.section .text
.global _start
_start:
        la sp, boot_stack_top
        call main

        .section .bss.stack
        .globl boot_stack_lower_bound
boot_stack_lower_bound:
        .space 4096 * 16
        .globl boot_stack_top

boot_stack_top:

spin:
        j spin
