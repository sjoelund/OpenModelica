# The environment CMake gives cargo in a Rust omc build. Set REPO (the checkout),
# BUILD (its build directory, -DOM_OMC_ENABLE_RUST=ON) and WORK (run_clippy.sh).
export RUSTC_WRAPPER=
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS=-Clink-arg=-fuse-ld=mold
export OMC_RT_LDFLAGS_GENERATED_CODE=" -lOpenModelicaRuntimeC -llapack -lblas -lm -lpthread -rdynamic"
export OMC_RT_LDFLAGS_GENERATED_CODE_SIM=" -lSimulationRuntimeC -lOpenModelicaRuntimeC -lzlib -llapack -lblas -lm -ldl -lpthread -rdynamic  -lomc_result "
export OMC_RT_LDFLAGS_GENERATED_CODE_SIM_RUST=" -lSimulationRuntimeRust -lOpenModelicaRuntimeC -lzlib -llapack -lblas -lm -ldl -lpthread -rdynamic "
export OMC_RT_LDFLAGS_GENERATED_CODE_SOURCE_FMU=" -llapack -lblas -lm -lpthread -rdynamic "
export OMC_HDF5_LDFLAGS=-l:libhdf5_serial.so.310
export OMC_FMILIB_LDFLAGS="/usr/lib/x86_64-linux-gnu/libexpat.so.1 /usr/lib/x86_64-linux-gnu/libminizip.so.1"
export OMC_SIMRT_INCLUDE_DIRS="$REPO/OMCompiler/SimulationRuntime/c|$REPO/OMCompiler/3rdParty/gc/include"
export OMC_EXTERNAL_C_SOURCES=$REPO/OMCompiler/SimulationRuntime/ModelicaExternalC/C-Sources
export OMC_REVISION_FILE=$BUILD/OMCompiler/Compiler/omc-revision.txt
export OMC_WASI_PIC_SYSROOT=$BUILD/rust-wasi-pic-sysroot
export OMC_WASI_P1_ADAPTER=$BUILD/downloads/wasi_snapshot_preview1.reactor.wasm
export OMC_FMU_LOADERS_OUT=$WORK/fmu-loaders
export OMC_SUNDIALS_WASM_DIR=$BUILD/rust-sundials-wasm
export OMC_SUNDIALS_NATIVE_DIR=$BUILD/rust-sundials-native
export OMC_SUNDIALS_NATIVE_INDEX_SIZE=64
export OMC_IPOPT_NATIVE_DIR=$BUILD/rust-ipopt-native
export OMC_WASM_BLOB_OUT=$WORK/wasm-blobs
export OMC_WASM_PREBUILT_IN=$BUILD/OMCompiler/Compiler/wasm-blobs
export OMC_WASI_LIBC_SRC=$BUILD/rust-wasi-libc-src/wasi-libc-wasi-sdk-32
export OMC_WASM_OPT=/bin/true
export OMC_WASM_OPT_FEATURES="--enable-bulk-memory --enable-multivalue --enable-mutable-globals --enable-nontrapping-float-to-int --enable-reference-types --enable-sign-ext --enable-simd"
FEAT="--no-default-features --features codegen_c,codegen_fmu_c,openmodelica_codegen_wasm_jit/sundials,mimalloc -p libopenmodelica_compiler"
