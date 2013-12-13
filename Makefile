include rust.mk

RUSTC ?= rustc
RUSTFLAGS ?=

.PHONY : all
all: rust-geom

.PHONY : check
check: check-rust-geom

$(eval $(call RUST_CRATE, .))
