CARGO_CMD := cargo fmt && cargo clippy -- -D warnings && cargo build --release

.DEFAULT_GOAL := all

#BOARD
BOARD := qemu
SBI ?= rustsbi
BOOTLOADER := ../bootloader/$(SBI)-$(BOARD).bin

xv6-user-panic:
	cd xv6-user-panic && $(CARGO_CMD)

xv6-kernel-panic:
	cd xv6-kernel-panic && $(CARGO_CMD)

xv6-ulib:
	cd xv6-ulib && $(CARGO_CMD)

xv6-kernel:
	cd xv6-kernel && $(CARGO_CMD)

.PHONY: all xv6-user-panic xv6-kernel-panic xv6-ulib xv6-kernel clean fmt
all: xv6-user-panic xv6-kernel-panic xv6-ulib xv6-kernel

clean:
	(cd xv6-user-panic && cargo clean)
	(cd xv6-kernel-panic && cargo clean)
	(cd xv6-ulib && cargo clean)
	(cd xv6-kernel && cargo clean)

fmt:
	(cd xv6-user-panic && cargo fmt)
	(cd xv6-kernel-panic && cargo fmt)
	(cd xv6-ulib && cargo fmt)
	(cd xv6-kernel && cargo fmt)
