MAKE_BIN=/usr/bin/make
RUST_DIR=./rust/nbody/
C_DIR=./c/

.DEFAULT_GOAL := run_all

build_all:
	$(MAKE_BIN) -C $(C_DIR) build
	$(MAKE_BIN) -C $(RUST_DIR) build

run_all: build_all
	@echo
	@echo vvvvvv C results vvvvvv
	$(MAKE_BIN) -C $(C_DIR) run
	@echo ^^^^^^ C results ^^^^^^
	@echo
	@echo vvvvvv Rust results vvvvvv
	$(MAKE_BIN) -C $(RUST_DIR) run
	@echo ^^^^^^ Rust results ^^^^^^
