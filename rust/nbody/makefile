ifndef NUM_STEPS
	NUM_STEPS=50000000
endif

# via `rustup show`
TARGET_CPU=x86_64-apple-darwin

BIN_NAME=nbody
BINS_DIR=./target
RELEASE_BIN=$(BINS_DIR)/$(TARGET_CPU)/release/$(BIN_NAME)
DEBUG_BIN=$(BINS_DIR)/$(TARGET_CPU)/debug/$(BIN_NAME)

.DEFAULT_GOAL := build_release_then_run

clean:
	rm -r $(BINS_DIR) | true

build_debug: clean
	cargo build

build_release: clean
	cargo build --release

build_release_then_run: build_release run

build_debug_then_run_debug: build_debug run_debug

run:
	$(RELEASE_BIN) $(NUM_STEPS)

run_debug:
	$(DEBUG_BIN) $(NUM_STEPS)

measure_speed: build_release
	hyperfine "$(RELEASE_BIN) $(NUM_STEPS)"
