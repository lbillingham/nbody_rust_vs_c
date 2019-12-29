ifndef NUM_STEPS
	NUM_STEPS=50000000
endif
SRC_FILE=nbody.gcc-8.c
SRC_PATH=./$(SRC_FILE)
BIN_FILE=nbody.clang-8.bench.run
OPT_LVL=3
TARGET_CPU=native
CGN_UNITS=1

clean:
	-rm $(BIN_FILE)

build: clean
	clang -O$(OPT_LVL) \
		-fomit-frame-pointer \
		-funroll-loops \
		-march=$(TARGET_CPU) \
		-lm \
    	$(SRC_PATH) -o $(BIN_FILE)

run: build
	./$(BIN_FILE) $(NUM_STEPS)
