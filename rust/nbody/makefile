ifndef NUM_STEPS
	NUM_STEPS=50000000
endif
SRC_FILE=main.rs
SRC_PATH=./src/$(SRC_FILE)
BIN_FILE=nbody-rust-1.run
OPT_LVL=3
TARGET_CPU=native
CGN_UNITS=1

clean:
	-rm $(BIN_FILE)

build: clean
	rustc -C opt-level=$(OPT_LVL) \
		-C target-cpu=$(TARGET_CPU) \
		-C codegen-units=$(CGN_UNITS) \
    	$(SRC_PATH) -o $(BIN_FILE)

run: build
	./$(BIN_FILE) $(NUM_STEPS)
