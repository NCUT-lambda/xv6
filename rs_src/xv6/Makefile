CARGO_CMD := cargo fmt && cargo clippy -- -D warnings && cargo build

.DEFAULT_GOAL := all

xv6-common:
	cd xv6-common && $(CARGO_CMD)

xv6-kernel:
	cd xv6-kernel && $(CARGO_CMD)

.PHONY: all xv6-common xv6-kernel clean fmt
all: xv6-common xv6-kernel

clean:
	(cd xv6-common && cargo clean)
	(cd xv6-kernel && cargo clean)

fmt:
	(cd xv6-common && cargo fmt)
	(cd xv6-kernel && cargo fmt)
