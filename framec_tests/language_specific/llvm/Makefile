FRAMEC := ../../../target/release/framec
SPEC   := basic/test_simple_system.frm
OUT    := simple.ll
BIN    := simple
RUNTIME_DIR := ../../../target/release
RUNTIME_LIB := -lframe_runtime_llvm
RUNTIME_RPATH := -Wl,-rpath,$(RUNTIME_DIR)

.PHONY: build llvm llvm-run clean

build:
	cd ../../.. && cargo build --release

$(OUT): $(SPEC) | build
	$(FRAMEC) -l llvm $(SPEC) > $(OUT)

$(BIN): $(OUT)
	clang $(OUT) -L$(RUNTIME_DIR) $(RUNTIME_LIB) $(RUNTIME_RPATH) -mllvm -opaque-pointers -o $(BIN)

llvm: $(BIN)
	./$(BIN)

llvm-run: llvm

clean:
	rm -f $(OUT) $(BIN)
