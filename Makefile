RUSTC ?= rustc

all: librust-geom-851fed20-0.1.dylib

librust-geom-851fed20-0.1.dylib: lib.rs
	@$(RUSTC) --dep-info --lib $<

-include lib.d
