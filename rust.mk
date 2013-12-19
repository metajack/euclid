define RUST_CRATE

_rust_crate_dir = $(dir $(1))
_rust_crate_lib = $$(_rust_crate_dir)lib.rs
_rust_crate_test = $$(_rust_crate_dir)test.rs

_rust_crate_name = $$(shell $(RUSTC) --crate-name $$(_rust_crate_lib))
_rust_crate_dylib = $$(shell $(RUSTC) --crate-file-name --lib $$(_rust_crate_lib))

.PHONY : $$(_rust_crate_name)
$$(_rust_crate_name) : $$(_rust_crate_dylib)

$$(_rust_crate_dylib) : $$(_rust_crate_lib)
	$$(RUSTC) $$(RUSTFLAGS) --dep-info --lib $$<

-include $$(patsubst %.rs,%.d,$$(_rust_crate_lib))

ifneq ($$(wildcard $$(_rust_crate_test)),"")

.PHONY : check-$$(_rust_crate_name)
check-$$(_rust_crate_name): $$(_rust_crate_name)-test
	./$$(_rust_crate_name)-test

$$(_rust_crate_name)-test : $$(_rust_crate_test)
	$$(RUSTC) $$(RUSTFLAGS) --dep-info --test $$< -o $$@

-include $$(patsubst %.rs,%.d,$$(_rust_crate_test))

endif

endef
