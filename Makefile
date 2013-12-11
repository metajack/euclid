RUSTC ?= rustc

SOURCES = $(find . -name '*.rs')

all: librust-geom.dummy

librust-geom.dummy: lib.rs $(SOURCES)
	@$(RUSTC) --lib $<
	@touch $@

clean:
	@rm -f *.so *.dylib *.dll
