# Patch and recompile libcore to remove float support

RUSTLIB_TMP = $(TMP)/rustlib
LIBS = libcore liballoc librustc_unicode libcollections
RUNTIME = $(patsubst %, $(RUSTLIB)/%.rlib, $(LIBS))

$(patsubst %, $(RUSTLIB_TMP)/%, $(LIBS)): $(RUSTLIB_TMP)/%:
	$(MKDIR) $(@D)
	git clone https://github.com/phil-opp/nightly-$(@F) $@

$(patsubst %, $(RUSTLIB_TMP)/%/$(notdir $(TARGET_FILE)), $(LIBS)): $(RUSTLIB_TMP)/%/$(notdir $(TARGET_FILE)): $(RUSTLIB_TMP)/% $(TARGET_FILE)
	cp $(TARGET_FILE) $@

$(patsubst %, $(RUSTLIB)/%.rlib, $(filter-out libcore, $(LIBS))): $(RUSTLIB)/%.rlib: $(RUSTLIB_TMP)/% $(RUSTLIB_TMP)/%/$(notdir $(TARGET_FILE))
	$(MKDIR) $(@D)
	cargo rustc --manifest-path $</Cargo.toml --target $(TARGET_TRIPLE) --release -- -Z no-landing-pads
	cp $</target/$(TARGET_TRIPLE)/release/$(@F) $@

$(RUSTLIB)/libcore.rlib: $(RUSTLIB_TMP)/libcore $(RUSTLIB_TMP)/libcore/$(notdir $(TARGET_FILE))
	$(MKDIR) $(@D)
	cargo rustc --manifest-path $</Cargo.toml --target $(TARGET_TRIPLE) --release --features disable_float -- -Z no-landing-pads
	cp $</target/$(TARGET_TRIPLE)/release/$(@F) $@

$(RUSTLIB)/liballoc.rlib:         $(RUSTLIB)/libcore.rlib
$(RUSTLIB)/librustc_unicode.rlib: $(RUSTLIB)/libcore.rlib
$(RUSTLIB)/libcollections.rlib:   $(RUSTLIB)/liballoc.rlib $(RUSTLIB)/librustc_unicode.rlib

.PHONY: runtime
runtime: $(patsubst %, $(RUSTLIB)/%.rlib, $(LIBS))

.PHONY: clean-runtime
clean-runtime:
	rm -rf $(patsubst %, $(RUSTLIB)/%.rlib, $(LIBS))
