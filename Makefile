include rust.mk

RUSTC ?= rustc
RUSTFLAGS ?=

.PHONY : all
all : geom

.PHONY : check
check : check-geom

$(eval $(call RUST_CRATE, .))
