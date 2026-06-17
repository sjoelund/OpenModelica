# Build the Rust (mmtorust) omc port instead of the bootstrapped C omc.
#
# Enabled with -DOM_OMC_ENABLE_RUST=ON. The chain, all native Rust (no bomc/omc,
# no system omc, no shell scripts):
#
#   1. cargo build the build *tools* (mmtorust, susan) -- always --release, since
#      they run during the build and release is dramatically faster.
#   2. `mmtorust susan` transpiles the Susan-subset crates; cargo builds `susan`.
#   3. `susan` compiles every *.tpl -> *.mo (the omc_add_template_target rules,
#      which use ${OMC_EXE}; in Rust mode ${OMC_EXE} is the susan binary).
#   4. `mmtorust` (full) transpiles all of compilerSources.txt -> crate .rs.
#   5. cargo builds the omc artifacts (openmodelica, libopenmodelica_compiler)
#      with a selectable profile (RUST_OMC_PROFILE, default "debug").
#
# Files are generated in their canonical locations (Compiler/Template/*.mo,
# the OpenModelica.rs crate src/ dirs); nothing is copied via scripts.

find_program(CARGO_EXECUTABLE cargo REQUIRED)

set(RUST_OMC_DIR ${CMAKE_CURRENT_SOURCE_DIR}/OpenModelica.rs
    CACHE PATH "Path to the Rust omc source tree (the mmtorust workspace).")

# CI builds: one switch that flips the defaults to a clean, reproducible build —
# the release profile and cargo incremental compilation OFF (incremental
# artifacts are pure overhead for a from-scratch CI build and bloat the cache).
# It only changes the *defaults* of RUST_OMC_PROFILE / RUST_OMC_INCREMENTAL, so
# either can still be overridden explicitly on the command line.
option(RUST_OMC_CI "CI build of the Rust omc: default to the release profile with cargo incremental compilation disabled." OFF)
if(RUST_OMC_CI)
  set(_rust_omc_profile_default "release")
  set(_rust_omc_incremental_default OFF)
else()
  set(_rust_omc_profile_default "debug")
  set(_rust_omc_incremental_default ON)
endif()

# The omc artifacts (the deliverables) honour this profile; default debug.
# The build tools (mmtorust, susan) are always release regardless.
set(RUST_OMC_PROFILE "${_rust_omc_profile_default}"
    CACHE STRING "Cargo profile for the Rust omc artifacts: debug or release.")
if(RUST_OMC_PROFILE STREQUAL "release")
  set(RUST_OMC_PROFILE_FLAG "--release")
  set(RUST_OMC_TARGET_SUBDIR "release")
else()
  set(RUST_OMC_PROFILE_FLAG "")
  set(RUST_OMC_TARGET_SUBDIR "debug")
endif()

# Cargo incremental compilation: ON for fast local iteration, OFF for CI (set by
# RUST_OMC_CI). Honoured by every cargo invocation via CARGO_ENV below.
option(RUST_OMC_INCREMENTAL "Use cargo incremental compilation for the Rust omc build (OFF for CI)." ${_rust_omc_incremental_default})

# cargo target/ lives in the build tree, not the source crate tree.
set(RUST_OMC_TARGET_DIR ${CMAKE_CURRENT_BINARY_DIR}/rust-target
    CACHE PATH "Directory for cargo's target/ output of the Rust omc build.")
set(RUST_TARGET_DIR ${RUST_OMC_TARGET_DIR})
# Env prefix for every cargo invocation. When incremental compilation is off we
# export CARGO_INCREMENTAL=0, which covers all profiles (including the
# always-debug `cargo test` build) without editing per-profile Cargo.toml keys.
set(CARGO_ENV ${CMAKE_COMMAND} -E env)
if(NOT RUST_OMC_INCREMENTAL)
  list(APPEND CARGO_ENV CARGO_INCREMENTAL=0)
endif()
# Always via ${CARGO_BUILD} so target/ is never the in-source default.
set(CARGO_BUILD ${CARGO_ENV} ${CARGO_EXECUTABLE} build --target-dir ${RUST_TARGET_DIR})
set(SUSAN_BIN   ${RUST_TARGET_DIR}/release/susan)
set(MMTORUST_BIN ${RUST_TARGET_DIR}/release/mmtorust)

# ---------------------------------------------------------------------------
# ctest: run the workspace's cargo tests. The top-level CMakeLists already calls
# include(CTest)/enable_testing(), so this registers a CTest test and CI can do
# `cmake --build . && ctest`. Same target-dir, profile and incremental settings
# as the build; --workspace covers every crate's tests. The test does not run
# codegen itself, so the omc targets must be built first (CTest has no build
# dependency on them) — the standard build-then-ctest order.
# ---------------------------------------------------------------------------
add_test(NAME rust_cargo_test
  COMMAND ${CARGO_ENV} ${CARGO_EXECUTABLE} test --target-dir ${RUST_TARGET_DIR} ${RUST_OMC_PROFILE_FLAG} --workspace
  WORKING_DIRECTORY ${RUST_OMC_DIR})

# ---------------------------------------------------------------------------
# wasm-jit runtime artifact (CI hand-off). The wasm-jit simCodeTarget embeds a
# precompiled linear-memory runtime (the openmodelica_codegen_wasm_jit_runtime
# crate, a standalone wasm32 cdylib) into the compiler via include_bytes!.
# Normally that crate's build.rs builds it on demand during *any* omc build
# (native or wasm); it never runs the wasm-opt binary on it.
#
# Two knobs let a multi-stage CI build it once, optimise it, and reuse it:
#   * rust_wasm_runtime (target): build the runtime crate for wasm32 and, if
#     binaryen is present, `wasm-opt -Oz` it, writing RUST_OMC_WASM_RUNTIME_OUT.
#     Stage 1 (a normal native Rust build) runs this and archives the output.
#   * RUST_OMC_WASM_RUNTIME (cache path): a prebuilt runtime.wasm to embed
#     instead of rebuilding. Stage 2 (the web build) sets it to stage 1's
#     artifact; it is forwarded as OMC_WASM_RUNTIME to the cargo build, which the
#     build.rs honours (skipping the rebuild). Empty = build it normally.
# wasm-opt is found here (optional) and reused by the web target below.
# ---------------------------------------------------------------------------
find_program(WASM_OPT_EXECUTABLE wasm-opt
             HINTS $ENV{CARGO_HOME}/bin $ENV{HOME}/.cargo/bin)
# wasm-opt feature flags, shared by every wasm-opt invocation below. rustc/LLVM
# emit wasm32-unknown-unknown with these post-MVP features on, but the release
# `strip` drops the target_features custom section binaryen would auto-detect
# from — so it defaults to MVP and rejects the bulk-memory/sign-ext/etc. ops.
# Enable exactly the set rustc reports (`rustc --print cfg --target
# wasm32-unknown-unknown`); blindly enabling all features could let wasm-opt emit
# instructions the JIT/browser consumers don't support.
set(WASM_OPT_FEATURES
    --enable-bulk-memory --enable-multivalue --enable-mutable-globals
    --enable-nontrapping-float-to-int --enable-reference-types --enable-sign-ext)
set(RUST_OMC_WASM_RUNTIME "" CACHE FILEPATH
    "Prebuilt wasm-jit runtime.wasm to embed (empty = build it). In CI stage 2, point at the rust_wasm_runtime artifact from stage 1.")
set(RUST_OMC_WASM_RUNTIME_OUT ${RUST_OMC_TARGET_DIR}/runtime.wasm CACHE PATH
    "Output path of the rust_wasm_runtime target (the built + wasm-opt'd wasm-jit runtime).")

set(_wasm_jit_runtime_dir ${RUST_OMC_DIR}/openmodelica_codegen_wasm_jit_runtime)
set(_wasm_jit_runtime_target_dir ${RUST_OMC_TARGET_DIR}/wasm-jit-runtime)
set(_wasm_jit_runtime_wasm
    ${_wasm_jit_runtime_target_dir}/wasm32-unknown-unknown/release/openmodelica_codegen_wasm_jit_runtime.wasm)
if(WASM_OPT_EXECUTABLE)
  set(_wasm_jit_runtime_opt COMMAND ${WASM_OPT_EXECUTABLE} -Oz ${WASM_OPT_FEATURES}
      ${RUST_OMC_WASM_RUNTIME_OUT} -o ${RUST_OMC_WASM_RUNTIME_OUT})
else()
  set(_wasm_jit_runtime_opt "")
endif()
# Standalone [workspace] crate, so build it directly (no codegen dependency); its
# own target-dir keeps it from contending with the main build's lock.
add_custom_target(rust_wasm_runtime
  WORKING_DIRECTORY ${_wasm_jit_runtime_dir}
  JOB_SERVER_AWARE TRUE
  COMMAND ${CARGO_ENV} ${CARGO_EXECUTABLE} build --release
          --target wasm32-unknown-unknown --target-dir ${_wasm_jit_runtime_target_dir}
  COMMAND ${CMAKE_COMMAND} -E copy ${_wasm_jit_runtime_wasm} ${RUST_OMC_WASM_RUNTIME_OUT}
  ${_wasm_jit_runtime_opt}
  COMMENT "Rust: building + optimising the wasm-jit runtime -> ${RUST_OMC_WASM_RUNTIME_OUT}"
  VERBATIM)

# ---------------------------------------------------------------------------
# Step 1+2: build mmtorust (release), transpile the Susan subset, build susan.
# A stamp file marks completion; cargo itself handles incremental rebuilds, so
# the command always runs but is a fast no-op when nothing changed.
# ---------------------------------------------------------------------------
set(SUSAN_STAMP ${CMAKE_CURRENT_BINARY_DIR}/rust_susan.stamp)
add_custom_command(
  OUTPUT ${SUSAN_STAMP}
  WORKING_DIRECTORY ${RUST_OMC_DIR}
  # Hand make's -jN jobserver tokens to cargo (needs CMake >= 3.28).
  JOB_SERVER_AWARE TRUE
  # Build tools always in release.
  COMMAND ${CARGO_BUILD} --release -p mmtorust
  COMMAND ${MMTORUST_BIN} susan
  COMMAND ${CARGO_BUILD} --release -p openmodelica_susan --bin susan
  COMMAND ${CMAKE_COMMAND} -E touch ${SUSAN_STAMP}
  COMMENT "Rust: building mmtorust + Susan template compiler (release)"
  VERBATIM)
add_custom_target(rust_susan DEPENDS ${SUSAN_STAMP})

# In Rust mode the template rules (omc_add_template_target) invoke ${OMC_EXE} on
# each *.tpl; point it at susan and make each *.mo rule depend on rust_susan via
# TPL_EXTRA_DEPENDS (consumed by the macro).
set(OMC_EXE ${SUSAN_BIN})
set(TPL_EXTRA_DEPENDS ${SUSAN_STAMP})

# ---------------------------------------------------------------------------
# Step 4: full transpile. Depends on every template-generated *.mo
# (TPL_OUTPUT_MO_FILES, populated by template_compilation.cmake) plus the
# scripting-API .mo (generated below by the standalone scripting_api_gen tool).
# ---------------------------------------------------------------------------
function(omc_rust_setup_codegen)
  # Use the canonical CMake source list (meta_modelica_source_list.cmake), the
  # same set the C build compiles, instead of a separate hardcoded
  # compilerSources.txt — so the Rust build can never drift from it (e.g. the
  # wasm-jit files added in #15847 are picked up automatically). We materialise
  # it to a file and pass `mmtorust --sources`. Absolute paths are fine; mmtorust
  # writes its output relative to its working directory (the crate tree).
  set(RUST_SOURCES_FILE ${CMAKE_CURRENT_BINARY_DIR}/rust_compilerSources.txt)
  set(_rust_src_content "# Generated by rust_omc.cmake from meta_modelica_source_list.cmake.\n# Do not edit by hand — the canonical list is the CMake one.\n")
  foreach(_f ${OMC_MM_ALWAYS_SOURCES} ${OMC_MM_BACKEND_SOURCES})
    string(APPEND _rust_src_content "${_f}\n")
  endforeach()
  # copy_if_different so the mtime (which rust_codegen DEPENDS on) only moves on
  # a real change — a plain file(WRITE) would rewrite it every reconfigure.
  file(WRITE ${RUST_SOURCES_FILE}.tmp "${_rust_src_content}")
  execute_process(COMMAND ${CMAKE_COMMAND} -E copy_if_different
                  ${RUST_SOURCES_FILE}.tmp ${RUST_SOURCES_FILE})

  # -------------------------------------------------------------------------
  # Generate Script/OpenModelicaScriptingAPI.mo (the typed thin wrappers around
  # the interactive API) WITHOUT a built omc, breaking the bootstrap cycle: omc
  # links libOpenModelicaCompiler.so, whose openmodelica_scripting_qt crate is
  # mmtorust-generated *from this .mo*; in the C build the .mo came from running
  # omc itself (OpenModelica.Scripting.generateScriptingAPI), which the Rust port
  # cannot do before omc exists. The standalone `scripting_api_gen` tool depends
  # only on the hand-written parser crate (openmodelica_ast, not generated), so it
  # builds and runs with no prior codegen. It parses the OpenModelica.Scripting
  # package out of FrontEnd/ModelicaBuiltin.mo and emits the .mo directly (no Tpl,
  # no Lookup). The Qt .cpp/.h are emitted later by mmtorust (emit_scripting_api_qt).
  #
  # DEPENDS on ModelicaBuiltin.mo so the API is regenerated whenever the builtin
  # OpenModelica.Scripting package changes, and on the generator's own source.
  set(SCRIPTING_API_MO ${CMAKE_CURRENT_SOURCE_DIR}/Script/OpenModelicaScriptingAPI.mo)
  set(MODELICA_BUILTIN_MO ${CMAKE_CURRENT_SOURCE_DIR}/FrontEnd/ModelicaBuiltin.mo)
  add_custom_command(
    OUTPUT ${SCRIPTING_API_MO}
    WORKING_DIRECTORY ${RUST_OMC_DIR}
    JOB_SERVER_AWARE TRUE
    COMMAND ${CARGO_BUILD} --release -p openmodelica_scripting_api_gen
    COMMAND ${RUST_TARGET_DIR}/release/scripting_api_gen ${MODELICA_BUILTIN_MO} ${SCRIPTING_API_MO}
    DEPENDS ${MODELICA_BUILTIN_MO}
            ${RUST_OMC_DIR}/openmodelica_scripting_api_gen/src/main.rs
    COMMENT "Rust: generating OpenModelicaScriptingAPI.mo from ModelicaBuiltin.mo (no omc)"
    VERBATIM)
  add_custom_target(rust_scripting_api DEPENDS ${SCRIPTING_API_MO})

  # mmtorust emits OMEdit's C++ Qt scripting-API here (build tree); OMEditLIB reads it.
  set(OMC_SCRIPTING_API_QT_DIR ${CMAKE_CURRENT_BINARY_DIR}/scripting-api-qt
      CACHE INTERNAL "Generated OpenModelicaScriptingAPIQt C++ sources (build tree)")

  set(CODEGEN_STAMP ${CMAKE_CURRENT_BINARY_DIR}/rust_codegen.stamp)
  add_custom_command(
    OUTPUT ${CODEGEN_STAMP}
    WORKING_DIRECTORY ${RUST_OMC_DIR}
    JOB_SERVER_AWARE TRUE
    COMMAND ${CARGO_BUILD} --release -p mmtorust
    # Strip unused `import X;` from the Susan-generated *.mo before transpiling:
    # mmtorust lowers every import to a `use crate::X`, so an unused import
    # becomes a `use` of a crate the target does not depend on (e.g.
    # `openmodelica_backend::SimCodeUtil` in openmodelica_codegen_xml). The C
    # build runs the same boot/find-unused-import.sh. It exits non-zero when it
    # removes something, so `; true` keeps the build going.
    COMMAND bash -c "\"$0\" \"$@\" ; true" ${CMAKE_CURRENT_SOURCE_DIR}/boot/find-unused-import.sh ${TPL_OUTPUT_MO_FILES}
    COMMAND ${CMAKE_COMMAND} -E env OMC_SCRIPTING_API_QT_OUT=${OMC_SCRIPTING_API_QT_DIR}
            ${MMTORUST_BIN} --sources ${RUST_SOURCES_FILE}
    COMMAND ${CMAKE_COMMAND} -E touch ${CODEGEN_STAMP}
    DEPENDS ${TPL_OUTPUT_MO_FILES} ${SUSAN_STAMP} ${RUST_SOURCES_FILE}
            ${CMAKE_CURRENT_SOURCE_DIR}/Script/OpenModelicaScriptingAPI.mo
    COMMENT "Rust: transpiling all MetaModelica sources (mmtorust --sources <cmake list>)"
    VERBATIM)
  add_custom_target(rust_codegen DEPENDS ${CODEGEN_STAMP})

  # The native omc artifacts (and their install rules) are pointless for the
  # wasm/web target — it ships a single .wasm bundle, not the cdylib + launcher —
  # so in wasm mode they are not defined at all, leaving `make all` to build only
  # the wasm bundle (omc_rust_setup_wasm). The codegen above is still needed: the
  # wasm crate is built from the same generated .rs.
  if(NOT OM_OMC_WASM)
  # -------------------------------------------------------------------------
  # Step 5: build the omc artifacts with the selected profile. Both are part of
  # `all` (ALL) so a plain `make` produces them and `make install` can stage
  # them — exactly like the C build's omc/OpenModelicaCompiler targets, which the
  # rust branch skips. rust_libopenmodelica builds the
  # target/<profile>/libOpenModelicaCompiler.so that gets installed; rust_omc
  # builds the thin launcher, which links that same .so as an external prebuilt
  # library (its build.rs finds it in the profile dir), so it must be built
  # after — hence rust_omc's DEPENDS on rust_libopenmodelica.
  # -------------------------------------------------------------------------
  add_custom_target(rust_libopenmodelica ALL
    WORKING_DIRECTORY ${RUST_OMC_DIR}
    JOB_SERVER_AWARE TRUE
    COMMAND ${CARGO_BUILD} ${RUST_OMC_PROFILE_FLAG} -p libopenmodelica_compiler
    DEPENDS rust_codegen
    COMMENT "Rust: building libOpenModelicaCompiler (${RUST_OMC_PROFILE})"
    VERBATIM)

  add_custom_target(rust_omc ALL
    WORKING_DIRECTORY ${RUST_OMC_DIR}
    JOB_SERVER_AWARE TRUE
    COMMAND ${CARGO_BUILD} ${RUST_OMC_PROFILE_FLAG} -p openmodelica
    DEPENDS rust_codegen rust_libopenmodelica
    COMMENT "Rust: building omc (cargo build -p openmodelica, ${RUST_OMC_PROFILE})"
    VERBATIM)

  # -------------------------------------------------------------------------
  # Install into the standard layout, mirroring the C build's install rules
  # (OMCompiler/Compiler/CMakeLists.txt, skipped in rust mode): the omc launcher
  # → bin/, the cdylib → ${CMAKE_INSTALL_LIBDIR} (lib/<triple>/omc, next to the
  # simulation-runtime libs installed by OMCompiler/SimulationRuntime under the
  # same `omc` component), and the *Builtin.mo files → lib/omc/. The launcher's
  # rpath ($ORIGIN/../lib/<triple>/omc) then resolves both the cdylib and the
  # dlopened runtime libs. Build the targets first: `make && make install`.
  # -------------------------------------------------------------------------
  set(RUST_OMC_ARTIFACT_DIR ${RUST_TARGET_DIR}/${RUST_OMC_TARGET_SUBDIR})
  install(PROGRAMS ${RUST_OMC_ARTIFACT_DIR}/openmodelica
          DESTINATION ${CMAKE_INSTALL_BINDIR} RENAME omc COMPONENT omc)
  install(PROGRAMS ${RUST_OMC_ARTIFACT_DIR}/libOpenModelicaCompiler.so
          DESTINATION ${CMAKE_INSTALL_LIBDIR} COMPONENT omc)
  install(FILES
            ${CMAKE_CURRENT_SOURCE_DIR}/FrontEnd/AnnotationsBuiltin_1_x.mo
            ${CMAKE_CURRENT_SOURCE_DIR}/FrontEnd/AnnotationsBuiltin_2_x.mo
            ${CMAKE_CURRENT_SOURCE_DIR}/FrontEnd/AnnotationsBuiltin_3_x.mo
            ${CMAKE_CURRENT_SOURCE_DIR}/NFFrontEnd/NFModelicaBuiltin.mo
            ${CMAKE_CURRENT_SOURCE_DIR}/FrontEnd/ModelicaBuiltin.mo
            ${CMAKE_CURRENT_SOURCE_DIR}/FrontEnd/MetaModelicaBuiltin.mo
            ${CMAKE_CURRENT_SOURCE_DIR}/FrontEnd/PDEModelicaBuiltin.mo
          DESTINATION lib/omc COMPONENT omc)
  install(DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/scripts
          DESTINATION ${CMAKE_INSTALL_DATAROOTDIR}/omc/ COMPONENT omc)
  endif() # NOT OM_OMC_WASM

  # NOTE: OpenModelicaScriptingAPI.mo is now produced by the standalone
  # scripting_api_gen tool above (rust_scripting_api target / SCRIPTING_API_MO),
  # *before* codegen, so it no longer needs a built omc and there is no bootstrap
  # cycle. The previous omc-based regeneration target has been removed.
endfunction()

# Provides, for the native CMake build of the Qt GUI clients in Rust mode, the
# OpenModelicaCompiler target they link. The OpenModelicaScriptingAPIQt sources
# OMEdit compiles are generated into OMC_SCRIPTING_API_QT_DIR by rust_codegen.
# Called whenever OM_ENABLE_GUI_CLIENTS is ON.
function(omc_rust_setup_omedit)
  set(RUST_OMC_ARTIFACT_DIR ${RUST_TARGET_DIR}/${RUST_OMC_TARGET_SUBDIR})

  # The OpenModelicaCompiler target the Qt GUI clients link: the cargo cdylib,
  # IMPORTED GLOBAL. IMPORTED_NO_SONAME (the cdylib has none) records the basename
  # in DT_NEEDED, resolved via the client's $ORIGIN/../lib rpath. OMC_RUST_ABI
  # selects the in-process Rust path; the include dirs provide omc_rust_embedding.h
  # and the util/ header it pulls in (SimulationRuntime/c).
  get_filename_component(_simrt_c_inc ${CMAKE_CURRENT_SOURCE_DIR}/../SimulationRuntime/c ABSOLUTE)
  add_library(OpenModelicaCompiler SHARED IMPORTED GLOBAL)
  set_target_properties(OpenModelicaCompiler PROPERTIES
    IMPORTED_LOCATION ${RUST_OMC_ARTIFACT_DIR}/libOpenModelicaCompiler.so
    IMPORTED_NO_SONAME TRUE
    INTERFACE_INCLUDE_DIRECTORIES "${RUST_OMC_DIR}/libopenmodelica_compiler/include;${_simrt_c_inc}"
    INTERFACE_COMPILE_DEFINITIONS OMC_RUST_ABI)
  # 3rd-party deps OMEdit inherited transitively from the C OpenModelicaCompiler
  # (fmilib via backendruntime, libzmq via runtime); the cdylib doesn't carry
  # them, so propagate the targets here.
  foreach(_dep fmilib omc::3rd::libzmq)
    if(TARGET ${_dep})
      set_property(TARGET OpenModelicaCompiler APPEND PROPERTY INTERFACE_LINK_LIBRARIES ${_dep})
    endif()
  endforeach()
endfunction()

# ---------------------------------------------------------------------------
# Web / wasm target. The omc compiler built for wasm32-unknown-unknown plus the
# wasm-bindgen JS bindings — the browser/Node deliverable. CMake drives cargo +
# wasm-bindgen + wasm-opt directly (a first-class target with a proper dependency
# on the codegen); it does NOT shell out to wasm/build.sh.
#
# The bundle is assembled in the build tree (${CMAKE_CURRENT_BINARY_DIR}/web):
# pkg-<host>/ from wasm-bindgen plus the host's launcher (index.html for the
# browser, omc-cli.js for Node). `make install` stages that directory under
# <prefix>/<datarootdir>/omc/web (component `web`), so it can be served from a
# clean location, e.g. `python3 -m http.server -d <prefix>/share/omc/web`.
#
# Selected with -DOM_OMC_WASM=ON (top-level), which also prunes every native
# client/library the wasm bundle does not use, so `make all` builds only this.
# Called from Compiler/CMakeLists.txt in place of the native artifacts/omedit.
# ---------------------------------------------------------------------------
function(omc_rust_setup_wasm)
  # RUST_OMC_WASM_MODE = <host>-<profile>: host selects the wasm-bindgen target
  # (nodejs / web), profile the cargo profile.
  set(RUST_OMC_WASM_MODE "web-release"
      CACHE STRING "wasm build mode: node-debug, node-release, web-debug or web-release.")
  set_property(CACHE RUST_OMC_WASM_MODE PROPERTY STRINGS
               node-debug node-release web-debug web-release)
  if(RUST_OMC_WASM_MODE STREQUAL "node-debug")
    set(_host nodejs)
    set(_profile debug)
  elseif(RUST_OMC_WASM_MODE STREQUAL "node-release")
    set(_host nodejs)
    set(_profile release)
  elseif(RUST_OMC_WASM_MODE STREQUAL "web-debug")
    set(_host web)
    set(_profile debug)
  elseif(RUST_OMC_WASM_MODE STREQUAL "web-release")
    set(_host web)
    set(_profile release)
  else()
    message(FATAL_ERROR "RUST_OMC_WASM_MODE must be one of "
                        "node-debug|node-release|web-debug|web-release, got "
                        "'${RUST_OMC_WASM_MODE}'.")
  endif()

  # wasm-bindgen-cli is mandatory for this target; the wasm32 rustup target must
  # also be installed. REQUIRED → a clear configure error instead of a cryptic
  # mid-build failure. (WASM_OPT_EXECUTABLE is found at file scope and reused
  # here; it is optional, only shrinking the release bundle.)
  find_program(WASM_BINDGEN_EXECUTABLE wasm-bindgen REQUIRED
               HINTS $ENV{CARGO_HOME}/bin $ENV{HOME}/.cargo/bin)

  set(_wasm_target wasm32-unknown-unknown)
  set(_wasm_name OpenModelicaCompiler)
  # wasmtime has no wasm backend, so the wasm-jit engine must be wasmer (`js`);
  # the cdylib is built with no default features (drops the native-only deps).
  set(_wasm_common --target ${_wasm_target} -p libopenmodelica_compiler
                   --no-default-features --features engine-wasmer)

  if(_profile STREQUAL "release")
    set(_cargo_profile_flag --release)
    set(_cargo_backend "")
  else()
    set(_cargo_profile_flag "")
    # The workspace dev profile uses the cranelift *rustc* backend (fast native
    # builds); it cannot target wasm32, so force the LLVM backend for codegen.
    set(_cargo_backend --config profile.dev.codegen-backend=\"llvm\")
  endif()

  set(_wasm_artifact ${RUST_TARGET_DIR}/${_wasm_target}/${_profile}/${_wasm_name}.wasm)
  # Assemble the runnable bundle in the build tree (never the source tree).
  set(_web_dir ${CMAKE_CURRENT_BINARY_DIR}/web)
  set(_wasm_pkgdir ${_web_dir}/pkg-${_host})
  # The launcher that loads pkg-<host>/: index.html for the browser, omc-cli.js
  # for Node. (The other host's launcher would reference an absent pkg dir.)
  if(_host STREQUAL "web")
    set(_web_launcher ${RUST_OMC_DIR}/wasm/index.html)
  else()
    set(_web_launcher ${RUST_OMC_DIR}/wasm/omc-cli.js)
  endif()

  # Release size optimisation, only if binaryen is available.
  set(_wasm_opt_cmd "")
  if(_profile STREQUAL "release" AND WASM_OPT_EXECUTABLE)
    set(_wasm_opt_cmd COMMAND ${WASM_OPT_EXECUTABLE} -Oz ${WASM_OPT_FEATURES}
        ${_wasm_pkgdir}/${_wasm_name}_bg.wasm -o ${_wasm_pkgdir}/${_wasm_name}_bg.wasm)
  endif()

  # Cargo invocation. If a prebuilt runtime.wasm was supplied (CI stage 2),
  # forward it as OMC_WASM_RUNTIME so the wasm-jit build.rs embeds it instead of
  # rebuilding it. Built from CARGO_ENV (incremental setting) like CARGO_BUILD.
  set(_wasm_cargo ${CARGO_ENV})
  if(RUST_OMC_WASM_RUNTIME)
    list(APPEND _wasm_cargo OMC_WASM_RUNTIME=${RUST_OMC_WASM_RUNTIME})
  endif()
  list(APPEND _wasm_cargo ${CARGO_EXECUTABLE} build --target-dir ${RUST_TARGET_DIR})

  # Depend on the full transpile (same stamp rust_codegen uses): the wasm crate
  # is built from the generated .rs.
  set(CODEGEN_STAMP ${CMAKE_CURRENT_BINARY_DIR}/rust_codegen.stamp)
  set(WASM_STAMP ${CMAKE_CURRENT_BINARY_DIR}/rust_wasm.stamp)
  add_custom_command(
    OUTPUT ${WASM_STAMP}
    WORKING_DIRECTORY ${RUST_OMC_DIR}
    JOB_SERVER_AWARE TRUE
    COMMAND ${_wasm_cargo} ${_cargo_profile_flag} ${_wasm_common} ${_cargo_backend}
    COMMAND ${CMAKE_COMMAND} -E rm -rf ${_web_dir}
    COMMAND ${WASM_BINDGEN_EXECUTABLE} ${_wasm_artifact}
            --out-dir ${_wasm_pkgdir} --target ${_host}
    ${_wasm_opt_cmd}
    COMMAND ${CMAKE_COMMAND} -E copy ${_web_launcher} ${_web_dir}/
    COMMAND ${CMAKE_COMMAND} -E touch ${WASM_STAMP}
    DEPENDS ${CODEGEN_STAMP} ${_web_launcher}
    COMMENT "Rust: building wasm/web bundle (${RUST_OMC_WASM_MODE}) -> ${_web_dir}"
    VERBATIM)
  add_custom_target(rust_wasm ALL DEPENDS ${WASM_STAMP})

  # make install: stage the assembled bundle (pkg-<host>/ + launcher) in a clean,
  # runnable location. The trailing slash installs the directory's *contents*.
  install(DIRECTORY ${_web_dir}/
          DESTINATION ${CMAKE_INSTALL_DATAROOTDIR}/omc/web
          COMPONENT web)
endfunction()
