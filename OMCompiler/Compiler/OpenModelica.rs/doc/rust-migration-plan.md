# Moving omc from MetaModelica to Rust: migration plan

Draft, 2026-11-18. Measured on master `d7a542f7f0` (2026-11-18), with a fresh
`mmtorust` transpile of that commit. How each number
was measured is in the [appendix](#appendix-how-the-numbers-were-measured).

Diagrams (Graphviz; `.dot` sources next to the `.svg`):

| Diagram | Shows |
| --- | --- |
| [current-build.svg](rust-migration-plan/current-build.svg) | the two ways omc is built today (bootstrapped C, transpiled Rust) |
| [current-structure.svg](rust-migration-plan/current-structure.svg) | the crates of the Rust omc today, what is generated and what is hand-written |
| [target-build.svg](rust-migration-plan/target-build.svg) | how omc is built after the move |
| [target-structure.svg](rust-migration-plan/target-structure.svg) | the crates at the end of this plan, and what goes later with the old backend |
| [old-frontend-deps.svg](rust-migration-plan/old-frontend-deps.svg) | which parts of `FrontEnd/` the new frontend, the backends and the scripting API still use |
| [phases.svg](rust-migration-plan/phases.svg) | the order of the work |

---

## 1. Summary

- **Order.** (0) Before the branch: make the Rust omc good enough to be the only
  omc, and do every systematic clean-up *in the generator* (mmtorust, Susan),
  not in the generated code. (1) Right after the branch: the Rust omc becomes the
  only omc build; FMI 1.0, the old frontend entry points and `-g=MetaModelica`
  are deleted while the sources are still `.mo`. (2) The flip: the generated Rust
  is committed, formatted and moved, the `.mo` files are deleted, in one
  reproducible PR during a short announced `.mo` freeze. (3) Lint burn-down and
  removal of the leftovers. (4) Redesign.
- **Where the code goes.** One cargo workspace with its root at the top of the
  repository. The compiler crates move from `OMCompiler/Compiler/OpenModelica.rs/<crate>`
  to `OMCompiler/Compiler/<crate>`; the simulation runtime crates stay in
  `OMCompiler/SimulationRuntime/rust/<crate>`. Crate names do not change in the
  move. Susan templates stay `.tpl` files and are compiled by `build.rs`.
- **How it is compiled.** Plain cargo. `cargo check`, `cargo test`, `cargo clippy`
  and rust-analyzer work on a clean checkout. CMake stays the top-level driver: it
  builds the C/C++ runtimes and 3rdParty, calls cargo with the right features and
  environment, and installs. mmtorust, the source mirror (`rust_src_sync`), bomc
  and the `.mo -> C` path are gone.
- **Formatting.** Already solved: mmtorust and Susan run rustfmt on what they
  write. Running rustfmt again over the 1.34M generated lines changes 21 lines
  in 6 files and takes about one minute. The hand-written Rust is the part that
  is not formatted: 226 of 543 files (+28k/-8k lines).
- **Linting.** Without the generated `allow`s, clippy reports 332,491 warnings
  (99.7% in generated code; 260,569 in code that becomes source, the rest in
  Susan output that stays generated). About 137,000 are machine-fixable, but
  `cargo clippy --fix` cannot apply them: its fixes break the `match_deref!`
  macro input and cargo rolls the crate back. ~112,000 are by design (names,
  `Arc`, type complexity) and stay allowed; ~11,400 need a person. Clippy takes
  about 10 minutes for the workspace (1.7x a `cargo check`). So: fix the
  high-volume lints **in mmtorust before the flip**, commit the flip with a
  workspace allow-list so that CI can run `cargo clippy -D warnings` from day
  one, then remove one lint from the allow-list per PR in the weeks after the
  flip ([section 6](#6-formatting-and-linting-measured)).
- **Speed.** Not a blocker any more. On OMLT (2026-11-18, same commit for
  both) the Rust omc translates in 0.89x the time of the C omc (backend 0.87x),
  with a slower tail: a third of the larger models, and Buildings at 1.10x.
- **What remains of the old frontend.** The new frontend needs 21 packages of
  `FrontEnd/`, all of them AST and data packages (Absyn, SCode, DAE, Values,
  AbsynUtil, SCodeUtil, ...) plus `FBuiltin.getInitialFunctions`. The old
  backend, SimCode and the templates need the whole DAE toolbox (Expression,
  ComponentReference, Types, DAEUtil, Ceval, ...). Only the instantiation engine
  (79k lines `.mo`, 119k lines Rust) can go now; the DAE toolbox goes when SimCode
  no longer uses DAE ([section 8](#8-what-remains-of-the-old-frontend)).

## 2. Where we are today

See [current-build.svg](rust-migration-plan/current-build.svg) and
[current-structure.svg](rust-migration-plan/current-structure.svg).

| What | Size |
| --- | --- |
| MetaModelica compiler sources (`OMCompiler/Compiler/**/*.mo`, without generated) | ~460 packages, ~620k lines |
| Susan templates (`Template/*.tpl`) | 32 files, 61k lines (generated `Template/*.mo`: 376k lines) |
| Generated Rust, mmtorust | 391 files, 1,044,721 lines |
| Generated Rust, Susan (templates) | 32 files, 280,238 lines |
| Generated Rust, other (`lib.rs`, OMEdit C ABI) | 38 files, 13,950 lines |
| Hand-written Rust, compiler workspace (`OpenModelica.rs`) | 380 files, 227k lines |
| Hand-written Rust, simulation runtime workspace | 163 files, 101k lines |
| `matchcontinue` left in the `.mo` sources | 2,192 (862 old frontend, 706 old backend, 243 Script) |
| Commits in the last 6 months touching `.mo`/`.tpl` / Rust | 1,026 / 770 (of 2,116) |
| Authors in the last 6 months touching `.mo`/`.tpl` / Rust | 21 / 18; 5 touched only `.mo` |
| Open PRs (2026-11-18) | 78; 47 touch compiler `.mo`, 22 touch `.tpl`, 30 of those were updated in the last 30 days |

Points that matter for the plan:

1. **Two compilers from one source.** The default build bootstraps: `bomc` is
   compiled from a C tarball and compiles all `.mo` (with the *old* frontend) to
   C. The Rust build transpiles the same `.mo` with mmtorust. CI runs the C, C+Rust
   testsuites with the C omc and the wasm-jit testsuite with the Rust omc.
2. **The generated Rust exists only in the build tree.** mmtorust writes it into
   a per-build mirror (`<build>/OMCompiler/Compiler/rust-src/`), which
   `rust_src_sync` fills from a hand-maintained file list
   (`.cmake/rust_src_files.txt`). A clean checkout does not load in cargo or
   rust-analyzer; `cargo test --workspace` does not work in the source tree.
3. **Editing one `.mo` file re-runs the whole transpile.** The `rust_codegen`
   step takes 7 minutes wall time from a changed source (mmtorust analyses plus
   code generation about 3.5 minutes, the rest is building mmtorust and Susan and
   compiling templates), before cargo starts.
4. **Generated code silences all warnings.** Every generated file starts with
   `#![allow(warnings)]` and a second allow list for naming and dead code.
5. **Integers are 32 bits.** MetaModelica `Integer` is `i32` in the Rust omc
   (63 bits in the C omc). A manual list of tests fails only because of this.
6. **Speed is no longer a blocker.** OMLT, latest runs (2026-11-18): `master`
   (C omc) and `wasm-jit` (Rust omc), both built from the same commit
   (`v1.28.0-dev.1974+gf37507374b`). Sums over the models that both lanes got
   through each phase:

   | Phase | Models | C omc | Rust omc | Ratio | Median per model (C ≥ 0.5 s) |
   | --- | ---: | ---: | ---: | ---: | ---: |
   | frontend | 20,527 | 4,586 s | 4,417 s | 0.96 | 0.70 |
   | backend | 20,401 | 7,304 s | 6,342 s | 0.87 | 0.83 |
   | SimCode | 20,353 | 1,765 s | 1,449 s | 0.82 | 0.74 |
   | **translation** (all three) | 20,353 | 13,534 s | 12,105 s | **0.89** | 0.88 (≥ 2 s) |

   The Rust omc is still slower for a third of the models with at least 2 s of
   translation (p90 1.23x). By library: Buildings 1.10x (backend 0.96x, so the
   rest is frontend and SimCode), IDEAS 1.02x, ClaRa 1.00x; MSL 0.79x,
   ScalableTestSuite 0.79x, ThermoSysPro 0.66x. Peak memory (`maxrss`): median
   0.89x. Templates and compile cannot be compared (that lane generates wasm,
   not C); simulation on wasm-jit takes 1.10x the C runtime's time. The ~1.9x
   backend gap of September 2026 is gone.
7. **Mechanisms that only exist because the source is `.mo`:** 9 `*.rust.mo`
   overrides, 2 `*.handwritten.rs` partial replacements, the
   `HANDWRITTEN_TOP_PACKAGES` list, `external_c_calls.rs` (823 lines of fallibility
   registrations), `WASM_GATED_TOP_MODULES` stubs, the `Stubs/` packages,
   `openmodelica_ast` keeping generated `Absyn.rs` in git because mmtorust needs
   it, and the two cargo workspaces that path-depend on each other.

## 3. Where we want to end up

See [target-build.svg](rust-migration-plan/target-build.svg) and
[target-structure.svg](rust-migration-plan/target-structure.svg).

### 3.1 Directory layout

```text
OpenModelica/
├── Cargo.toml              workspace root: members = OMCompiler/Compiler/*,
├── Cargo.lock              OMCompiler/SimulationRuntime/rust/*
├── rust-toolchain.toml     moved up from OpenModelica.rs/
├── rustfmt.toml            one copy (today: one per workspace, kept in sync by format.sh)
├── clippy.toml
├── .taplo.toml
└── OMCompiler/
    ├── Compiler/
    │   ├── CMakeLists.txt              calls cargo; no mirror, no transpile
    │   ├── metamodelica/               core runtime types (List, Array, Ref, Result)
    │   ├── metamodelica_derive/
    │   ├── openmodelica/               omc launcher
    │   ├── libopenmodelica_compiler/   cdylib loaded by OMEdit
    │   ├── openmodelica_util/ ...      one directory per crate, crate names unchanged
    │   ├── openmodelica_nf_frontend/src/NFInst.rs, ...
    │   ├── openmodelica_codegen_c/build.rs     compiles ../Template/CodegenC.tpl
    │   ├── openmodelica_susan/ openmodelica_susan_index/ openmodelica_scripting_api_gen/
    │   ├── Template/*.tpl, *TV.mo      templates stay where they are
    │   ├── builtin/*.mo                ModelicaBuiltin.mo, NFModelicaBuiltin.mo,
    │   │                               AnnotationsBuiltin_*.mo (data, include_str!)
    │   └── web/                        was OpenModelica.rs/wasm
    └── SimulationRuntime/
        ├── c/  cpp/  ...
        └── rust/<crate>/               unchanged
```

Why this shape:

- **One workspace.** The second workspace in `SimulationRuntime/rust` exists only
  because the compiler workspace cannot load before mmtorust has run. Without
  generated code that reason is gone, and one `Cargo.lock` and one target
  directory are simpler for everyone.
- **Root at the top of the repository.** VS Code / rust-analyzer find it without
  `linkedProjects`, and `rust-toolchain.toml` then applies to every cargo command
  in the repository.
- **`OMCompiler/Compiler/<crate>`, not `OpenModelica.rs/<crate>`.** The compiler
  stays where people look for it. The `OpenModelica.rs` level only marked the
  experimental port.
- **Crate names unchanged in the move.** `openmodelica_frontend` → `frontend_legacy`,
  `nf_frontend` → `frontend` and similar renames are useful, but they are a separate
  decision and a separate PR; a rename of a crate touches every `use` line.
- **Templates stay in `Template/`.** Susan resolves `import` between templates by
  file name in one directory; moving each `.tpl` next to its crate buys nothing.

### 3.2 How it is compiled

- `cargo build -p openmodelica -p libopenmodelica_compiler` builds omc. Features
  that need C libraries or the wasm toolchain (SUNDIALS, Ipopt, wasm-jit blobs, Qt
  ABI) become non-default features, so that `cargo check --workspace` works
  without CMake. CMake turns them on, as it does today with
  `--no-default-features --features ...`.
- `build.rs` generates the two things that remain generated:
  - the template crates: Susan (a build-dependency) compiles `Template/X.tpl`
    into `$OUT_DIR/X.rs`; `susan-index` reads the signatures it needs from the
    Rust sources of the crates the template calls, as it does today. Set
    `[profile.*.build-override] opt-level = 3` so Susan is fast in debug builds.
  - `openmodelica_scripting_qt`: `scripting_api_gen` reads the
    `OpenModelica.Scripting` package of `ModelicaBuiltin.mo` and writes the typed
    Rust API and the OMEdit C++ (`OpenModelicaScriptingAPIQt`). The Qt emitter
    moves from mmtorust (`emit_scripting_api_qt`) into `scripting_api_gen`;
    `Script/OpenModelicaScriptingAPI.mo` is no longer needed.
- CMake: builds 3rdParty and the C/C++ runtimes, runs cargo with
  `--target-dir <build>/rust-target`, the features and the `OMC_*` environment it
  passes today, installs, and builds the Qt clients. `rust_src_sync`,
  `rust_codegen`, `rust_susan` and `template_compilation.cmake` go away.
  Profiles, cross builds (`RUST_OMC_TARGET`), the prebuilt wasm hand-over and
  the web build stay as they are.
- A changed `.rs` file rebuilds only its crate and the crates above it. Today the
  same change in a `.mo` file costs the 7-minute transpile first.

### 3.3 Developer workflow, before and after

| Task | Today | After the flip |
| --- | --- | --- |
| Fix a frontend bug | edit `.mo`; transpile; cargo | edit `.rs`; cargo |
| Type-check | `OpenModelicaCompiler` target (bomc) or full Rust build | `cargo check -p <crate>` |
| Unit test | only in the build copy | `cargo test -p <crate>` in the source tree |
| Go to definition | lands in the build copy (edits are lost) | lands in the source |
| Add a builtin / API function | `.mo` in 2 places + `external_c_calls.rs` for externals | `ModelicaBuiltin.mo` (declaration) + Rust implementation |
| Format | `format.sh` on changed files | `cargo fmt` (all files are formatted) |

## 4. Decisions to make before the branch

These are cheap now and expensive after the flip, because before the flip a
generator change applies the decision to all 1.3M lines at once.

| # | Decision | Recommendation | Why before the flip |
| --- | --- | --- | --- |
| D1 | Directory layout and workspace root | [3.1](#31-directory-layout) | the flip moves every file anyway |
| D2 | Identifier names (`camelCase` functions, `UPPER` records, `CamelCase` modules) | Keep the MetaModelica names, with `non_snake_case` etc. allowed. If the project wants Rust names, mmtorust must emit them (it knows every definition and reference, and Susan uses the same index); do not rename by hand after the flip | a rename after the flip touches about a million identifiers and every open branch |
| D3 | Module file names (`NFInst.rs`) | Keep | as D2 |
| D4 | Integer width | Switch MetaModelica `Integer` to `i64` in mmtorust before the Rust omc becomes the only omc (the web/wasm32 build can keep `i32` through the same switch, or also use `i64`). Decide with a measurement of memory and speed | behaviour change for users of the C omc (63-bit), and it is one type mapping in the generator but a hand edit of thousands of signatures afterwards |
| D5 | Templates | Keep `.tpl` + Susan, compiled in `build.rs`, exempt from lints and rustfmt (like any `$OUT_DIR` code). Rewrite generators in Rust later, one at a time (the wasm-jit generator shows how) | committing the 280k lines of Susan output would make every `.tpl` change a double diff |
| D6 | Lint policy | [section 6.4](#64-when-to-lint) | the flip commit must pass the CI gate |
| D7 | Traceability | mmtorust writes `// Ported from OMCompiler/Compiler/<dir>/<File>.mo at <sha>` as the first line of each file | `git blame` cannot follow a file across languages |
| D8 | Toolchain | Keep the pinned nightly for now (cranelift dev backend, `-Z build-std` for the FMI3 adapters). Make sure the compiler crates themselves build on stable, so that a distribution can package omc with a stable rustc | generated code must not start to use nightly-only features (for example `deref_patterns` as a replacement for `match_deref!`) without a decision |
| D9 | `matchcontinue` | Nothing more to do before the flip: `mmtorust --fix` has already converted what it can prove (its report finds 1 more). The other 2,192 become Rust closures that discard errors; 862 go with the old frontend, the rest are rewritten in the redesign | after the flip the conversion is manual, so finish the provable part first |
| D10 | mmtorust after the flip | Tag it (`mmtorust-final`) and keep it runnable for porting `.mo` PRs and fixes from `maintenance/v1.28` ([5.5](#55-porting-open-mo-prs-and-release-branch-fixes)); delete it from master in phase 3 | it is needed at the flip commit exactly |

## 5. The plan

See [phases.svg](rust-migration-plan/phases.svg).

### 5.1 Phase 0: now, before the branch (master is still MetaModelica)

Nothing in this phase changes `.mo` files, so it does not disturb anyone.

1. **Make the decisions in [section 4](#4-decisions-to-make-before-the-branch).**
2. **Generator work in mmtorust and Susan**, driven by the lint data in
   [section 6](#6-formatting-and-linting-measured):
   - emit code that does not trigger the high-volume lints (see the table there);
   - remove the per-file `#![allow(warnings)]` and replace it with the
     workspace `[lints]` table (D6);
   - the D4 Integer switch, the D7 header line, D2 if wanted;
   - optional: merge the 2 `*.handwritten.rs` files into their generated files
     (`flip.sh` commits them as they are; the 9 `*.rust.mo` overrides need
     nothing, the transpile has applied them).
3. **Rust omc at parity, as the release gate for the switch.**
   - CI: the C and C+Rust testsuite lanes run with the Rust omc (today they use
     the C omc); the list of tests that fail only because of 32-bit Integers is
     empty (D4).
   - Windows (MSYS2/UCRT64 *or* the MSVC cross build, decide which one ships),
     macOS (arm64 and x86_64), Linux packages, the Windows installer.
   - OMEdit and the other Qt clients link the Rust `libOpenModelicaCompiler`
     (`OMC_RUST_ABI`), including the OMEdit testsuite lane.
   - `libOMCDLL` (`omcCAPI`) and OMCompiler.jl work against the Rust library, or
     are declared unsupported.
   - Speed: met overall (translation 0.89x of the C omc on OMLT). Keep the
     master vs wasm-jit comparison as a gate, and look at the slower third,
     starting with Buildings (1.10x).
4. **Tools for the flip** (in `OMCompiler/Compiler/OpenModelica.rs/tools/`; run
   them on a copy of master many times before the real flip):
   - `flip.sh` (written): transpiles the freeze commit with the normal CMake
     build and makes the commits of [5.3](#53-phase-2-the-flip), each of which
     builds. Still to add: the move to the layout of
     [3.1](#31-directory-layout) and `build.rs` for the templates and the
     scripting API;
   - `check-flip.sh` (written): compares the committed Rust with a fresh
     transpile of the freeze commit, after rustfmt;
   - `port-mo-pr.sh` (to write): turn a `.mo` diff into a Rust diff (see 5.5).
5. **Fix the 38 deny-level clippy findings** in hand-written code
   ([6.2](#62-what-clippy-reports)); they are bugs today.
6. **Tell people early.** The date of the freeze, which PRs should be merged
   before it, the onboarding document
   (`OMCompiler/Compiler/OpenModelica.rs/ONBOARDING.md`) and a short
   "MetaModelica to Rust" cheat sheet. Offer pairing sessions for the 5 people
   who wrote only `.mo` in the last 6 months.

### 5.2 Phase 1: the first two weeks after the branch

The branch `maintenance/v1.28` keeps the C omc and the MetaModelica sources.

1. **The Rust omc becomes the only omc.** `OM_OMC_ENABLE_RUST` is the default and
   then the only mode: remove `boot/` (bomc and the bootstrap tarball), the
   `.mo -> C` rules in `OMCompiler/Compiler/CMakeLists.txt`, `Compiler/runtime/*.c`
   (the C implementations of omc's `external "C"` functions, already replaced by
   hand-written Rust), `FrontEndCpp/` (C++ readers of MetaModelica values),
   `Parser/` and its ANTLR rules (the Rust omc has its own parser; check the
   OMEdit debugger, which also uses ANTLR, before removing `3rdParty/antlr`),
   `Stubs/` and the bootstrapping tests. The `.mo` files are still the source;
   mmtorust still transpiles them in every build.
2. **Delete FMI 1.0.** `CodegenFMU1.tpl` and the 1.0 branches in
   `CodegenFMU.tpl` (208 references), `CodegenFMUCommon.tpl`, `CodegenFMUCpp.tpl`,
   `SimCodeMain.mo`, `CevalScriptBackend.mo`, `Util/FMI.mo`; the export headers
   `SimulationRuntime/fmi/export/fmi1/` and `fmu1_*`; FMI 1.0 import
   (`SimulationRuntime/c/fmi/FMI1*.c`, `openmodelica_fmi/src/parse/v1.rs`,
   `FMIExt.rs`); the OMEdit options; 90 tests. Small and independent; do it
   first so that the old-frontend work does not wait for it. Tracked in
   #6143 ("Drop support for FMI 1.0", open).
3. **Delete the old frontend and MetaModelica language support, where it is
   only a deletion.** The old frontend is deprecated already: #17177 proposed
   removing it after 1.28.0, and #17179 (merged 2026-11-02) added the
   deprecation warning; tests are being moved off `-d=-newInst` in small PRs. Remove the `-d=newInst` switch and the code paths it
   guards, `-g=MetaModelica`, the D group of
   [section 8](#8-what-remains-of-the-old-frontend) where nothing outside it
   calls in, `MetaModelicaBuiltin.mo`, `testsuite/metamodelica` (223 files) and
   the tests that switch the old frontend on (301 tests use `-d=-newInst` or
   similar; 157 use `-g=MetaModelica`). This is possible now, and not before,
   because the C omc no longer has to compile itself with the old frontend.
   mmtorust's unused-function analysis finds what becomes dead. Callers that
   need a real replacement (the scripting API's `Inst`/`Lookup`/`Static` calls,
   `Ceval` in the old backend) stay until phase 4; do not redesign in `.mo`.

Doing these deletions while the source is still `.mo` has three benefits: the
reviewers know the language, the flip converts about 120k fewer lines, and
fixes can still move between `maintenance/v1.28` and master as `.mo` patches.
Timebox it: if it is not finished in two weeks, flip anyway and delete the rest
in Rust.

### 5.3 Phase 2: the flip

One PR, prepared by `flip.sh`, merged **with rebase, not squash** (the
repository normally squash-merges; here the move commit must stay separate so
that git detects the renames, and the format commit must stay separate so that
it can go into `.git-blame-ignore-revs`).

```sh
OMCompiler/Compiler/OpenModelica.rs/tools/flip.sh -B build-flip
OMCompiler/Compiler/OpenModelica.rs/tools/check-flip.sh -B build-flip <freeze commit>
```

1. Announce the date two weeks before. From T-2 days, no `.mo`/`.tpl` PRs are
   merged (freeze). Ask the authors of open `.mo` PRs to get them merged before,
   or to port them after with `port-mo-pr.sh`.
2. At T: run `flip.sh` on master. The commits it makes today:
   1. add the Rust that mmtorust generates at the freeze commit, and the Qt
      scripting API files for OMEdit (`openmodelica_scripting_qt/qt/`). The
      build still transpiles and writes the same files;
   2. `rustfmt` and `taplo` over all Rust and TOML files (pure formatting, into
      `.git-blame-ignore-revs`);
   3. build from the committed Rust (`tools/flip/build.patch`): no mmtorust,
      no template `.mo` rules, no `Autoconf.mo`, no `OpenModelicaScriptingAPI.mo`.
      susan is built from the committed Rust and compiles the templates as
      before. `RuntimeSources.rs` is configured from `RuntimeSources.rs.cmake`,
      which `flip.sh` translates from `RuntimeSources.mo.cmake`;
   4. delete the `.mo` sources, except the builtin files (data that omc reads)
      and the template interfaces `Template/*TV.mo` (susan reads them).

   To add to `flip.sh` before the real flip:
   5. `git mv` the crates to `OMCompiler/Compiler/<crate>`, merge the two
      workspaces, update paths in `Cargo.toml`, CMake, CI, docs (no content
      change), and remove `.cmake/rust_src_files.txt` and `rust_src_sync`;
   6. `build.rs` for the template crates and the scripting API. Until then a
      new scripting API function needs the Rust and C++ wrappers by hand,
      because `scripting_api_gen` writes `.mo` and mmtorust is gone;
   7. CI: `cargo fmt --check` and `cargo clippy --workspace -- -D warnings`
      with the workspace allow-list.

   Tested on master `7ca911d3c5` (2026-11-18): `flip.sh` committed 428
   generated Rust files (about 1.0M lines), formatted 262 files, changed 11
   build files and removed 468 files (640k lines). `check-flip.sh` found the
   430 committed files equal to the transpile. In a release build of the
   flipped tree, susan wrote the same 32 template files byte for byte, the
   configured `RuntimeSources.rs` has the same 32 constants, and the installed
   omc simulated with wasm-jit, exported a source-code FMU and failed the same
   130 of 1,125 testsuite tests (wasm-jit) as the omc built from the `.mo`
   sources.
3. Verify: `check-flip.sh` (committed Rust equals a fresh transpile of the freeze
   commit), the full CI on all lanes and platforms, and a comparison of the
   testsuite outputs before and after (they must be identical: the code is the
   same).
4. Merge, tag `rust-flip`, lift the freeze. Rollback is one revert PR.

### 5.4 Phase 3: the first six weeks after the flip

1. **Lint burn-down** ([6.4](#64-when-to-lint)): one lint per PR, merged fast,
   done by one or two people so that the PRs do not conflict with each other.
2. **Remove the leftovers** ([section 7](#7-what-to-remove-and-when)): mmtorust
   and its fallibility registry, the `match_deref!` and `loop_unwrap`
   dependencies where a generator change did not already remove them, the
   `WASM_GATED_TOP_MODULES` stubs (turn them into ordinary `cfg`), stale
   `.gitignore` entries, `OMCompiler.jl` if nobody uses it.
3. **Port the open `.mo` PRs** with `port-mo-pr.sh`.
4. **Update the documentation**: AGENTS.md, CONTRIBUTING.md (the commit hooks
   for `.mo`), README.cmake.md, the OpenModelica.rs README and ONBOARDING (it
   describes `.mo` as the source of truth), the user's guide chapters about
   MetaModelica and FMI 1.0.

### 5.5 Porting open `.mo` PRs and release-branch fixes

mmtorust is deterministic, so a `.mo` change can be turned into a Rust change:

```sh
# port-mo-pr.sh <last-commit-before-flip> <patch>, in a separate clone
git checkout <last-commit-before-flip>
cmake --build build --target rust_codegen  # copy the generated files -> tree A (base)
git apply <patch>
cmake --build build --target rust_codegen  # copy the generated files -> tree B (base + patch)
diff -ruN A B > rust.patch                 # the PR, in Rust
git -C <master clone> apply -3 rust.patch  # on today's master
```

`flip.sh` and `check-flip.sh` already select the generated files
(`tools/flip/common.sh`).

This works well while the touched Rust code has not changed much since the
flip, so port open PRs in the first weeks. The same script forward-ports fixes
from `maintenance/v1.28` to master. Backports from master to the release branch
are manual (Rust to MetaModelica); fix a release bug on the release branch first
and forward-port it.

### 5.6 Phase 4: months after the flip

Redesign, see [section 9](#9-what-to-redesign-after-the-flip).

## 6. Formatting and linting (measured)

### 6.1 rustfmt: already done for generated code

- mmtorust and Susan run rustfmt with the repository's `rustfmt.toml` on
  everything they write. Running rustfmt again over the 461 generated files
  (1.34M lines) changes **21 lines in 6 files** (rustfmt moves a few trailing
  comments on a second pass). One rustfmt process takes 35-60 s for all of them.
- **30% of the generated code is never formatted**: 406k of the 1.34M lines are
  inside `match_deref! { ... }` blocks, and rustfmt does not format the body of
  a macro call with braces. They keep mmtorust's own layout, and `cargo fmt` will
  not format what people later write inside them either. mmtorust already emits
  a plain `match &*x` where it can; the remaining cases need deref patterns,
  which exist only on nightly (`deref_patterns`, see D8). Reduce them in the
  generator where possible; accept the rest, and remove them as code is
  rewritten.
- **The hand-written Rust is the part that is not formatted**: 226 of 543
  files fail `format.sh --check` (the policy formats a file the first time it
  is changed). Formatting them is +28,443/-8,381 lines. Do it in the format
  commit of the flip (it is in `.git-blame-ignore-revs`), then check all files
  with `cargo fmt --check` in CI instead of only the changed files.

So rustfmt at the flip costs nothing to plan: one command, about a minute.

### 6.2 What clippy reports

All runs on the full compiler graph (generated + hand-written + the simulation
runtime crates it uses), dev profile, 24 cores:

| Run | Warnings | Wall time |
| --- | ---: | ---: |
| `cargo check`, from scratch (dependencies + workspace members) | ~120 (hand-written) | 8 min 50 s (peak 3.3 GB for one rustc) |
| `cargo check`, workspace members only, generated `allow`s removed | 198,612 (rustc lints) | 6 min 9 s |
| `cargo clippy`, workspace members only, `allow`s as today | 1,218 | 10 min 11 s |
| `cargo clippy`, workspace members only, generated `allow`s removed | **332,491** | 10 min 13 s |

- **99.7% of the warnings are in generated code**, 49% (161,893) have a
  machine-applicable fix.
- 71,922 are in the crates that contain only Susan output (`codegen_c`,
  `codegen_cfunctions`, `codegen_cpp*`, `codegen_fmu*`, `codegen_xml`,
  `codegen`). With D5 these stay generated and are not linted. **That leaves
  260,569 warnings in code that becomes source at the flip.**
- 41,047 of those are in `openmodelica_frontend`, the old frontend engine. Delete
  it before the flip (phase 1) and they never have to be fixed.
- The number of warnings does not change the run time. Clippy costs about 1.7x
  a `cargo check` (+4 minutes on this machine); it cannot reuse the artifacts of
  a `cargo build`, so in CI it is a separate job of about 10 minutes that can run
  in parallel with the testsuite.
- `cargo clippy --fix` on one crate: see [6.3](#63-clippy---fix-on-one-crate).

The top lints (120 different lints in total; the top 8 are 81% and the top 20
are 94% of all warnings):

| Lint | Count | Machine-applicable | Cause in the generated code | Action |
| --- | ---: | :---: | --- | --- |
| `non_snake_case` | 115,721 | no | MetaModelica names (`getOption`, `inExp`) | allow (D2) |
| `unused_mut` | 36,848 | yes | every parameter is `mut inX` | generator: `mut` only if assigned |
| `clippy::arc_with_non_send_sync` | 31,993 | no | `Arc::new(...)` for function values and records that hold `Rc`-based arrays | allow; redesign item 4 (`Rc` or `Send` types) |
| `unused_parens` | 24,741 | yes | `out = (match ... )` | generator |
| `clippy::clone_on_copy` | 20,779 | yes | `.clone()` on `i32`/`bool`/`f64` | generator (the type is known) |
| `clippy::explicit_auto_deref` | 20,310 | yes | `&**x` where `&x` is enough | generator |
| `clippy::needless_late_init` | 17,448 | yes | `let mut out: T; ... out = ...` (output variables) | generator |
| `clippy::needless_return` | 6,938 | yes | `return` at the end of a block | generator |
| `unreachable_patterns` | 5,253 | no | arms after a catch-all; 3,943 in Susan output | generator; allow for the rest at first |
| `clippy::unneeded_struct_pattern` | 4,251 | yes | `X { .. }` for a unit variant | generator |
| `unused_variables` | 3,952 | mostly | unused pattern bindings, outputs | generator |
| `clippy::type_complexity` | 3,920 | no | MetaModelica tuple and function types | allow |
| `clippy::unused_unit` | 3,611 | yes | `-> ()` | generator |
| `dead_code` | 3,583 | no | functions nothing calls | delete (mmtorust already lists unused functions) |
| `clippy::borrow_deref_ref` | 3,144 | yes | `&*&x` | generator |
| `clippy::redundant_field_names` | 2,869 | yes | `x: x` | generator |
| `unused_assignments` | 2,497 | no | defaults that are always overwritten | by hand |
| `clippy::match_single_binding` | 2,174 | partly | one-arm `match` from pattern assignments | generator |
| `unused_imports` | 1,986 | yes | `use` of the whole import list | generator |
| `clippy::let_and_return`, `double_parens`, `unnecessary_cast`, `deref_addrof`, `needless_borrows_for_generic_args`, `assign_op_pattern`, `redundant_closure_call`, ... | ~10,000 | yes | | generator |
| `non_camel_case_types`, `non_upper_case_globals`, `clippy::too_many_arguments`, `clippy::upper_case_acronyms` | 3,255 | no | MetaModelica names and signatures | allow |

Split of the 260,569 warnings in code that becomes source:

| Kind | Warnings | What happens |
| --- | ---: | --- |
| machine-applicable | ~137,000 | generator fixes before the flip (`cargo clippy --fix` cannot do it, [6.3](#63-clippy---fix-on-one-crate)) |
| allowed by design (naming, `Arc`, type complexity, many arguments) | ~112,000 | `[workspace.lints]` allow; revisit with the redesign |
| the rest, by hand | **~11,400** | burn-down after the flip: `dead_code` 3,583, `unused_assignments` 1,884, `unreachable_patterns` 1,310, `unreachable_code` 1,010, `match_single_binding` 815, `type_alias_bounds` 699, `match_like_matches_macro` 612, `unused_variables` 520, `derived_hash_with_manual_eq` 322, ... |

**Findings that are real bugs, in hand-written code, independent of the
migration.** Clippy's deny-by-default correctness lints stop the build today:
`clippy::uninit_vec` in `metamodelica/src/Dangerous.rs:84` (`set_len` after
`with_capacity` exposes uninitialized memory) and 37
`clippy::not_unsafe_ptr_arg_deref` (public safe functions that dereference raw
pointers, most in `openmodelica_solvers/src/sundials.rs`). Fix these now.

### 6.3 `clippy --fix` on one crate

Test: `cargo clippy --fix --no-deps -p openmodelica_frontend_base` on the copy
without the allows (69k lines, 19,456 warnings, 11,105 suggested fixes).

- **It does not work on the generated code.** cargo applies the fixes, finds
  that the crate no longer compiles, and rolls back *all* fixes for the crate
  ("errors present after applying fixes"). The fixes that break it are inside
  `match_deref! { ... }`: `match_single_binding` removes the `match` keyword the
  macro needs (`match_deref! { Ok(false) }`), and with that lint switched off,
  another fix rewrites the macro's `Deref @ pattern` bindings
  (`E0416: identifier Deref is bound more than once`). Clippy sees the expanded
  code and suggests edits in the macro input, which has a different grammar.
- Each attempt took 2-3 minutes for this one crate (plus 2.7 minutes to re-check
  the crates that depend on it).

Consequence: the machine-applicable fixes must come from the generator (or, after
the flip, from a `--fix` with a hand-picked set of lints that do not touch macro
input, tested crate by crate). This is the main reason why
[6.4](#64-when-to-lint) puts the lint work into mmtorust before the flip.

### 6.4 When to lint

1. **Now (phase 0), in the generator.** Fix the 38 deny-level findings. Then
   change mmtorust so that it does not emit the machine-fixable patterns above
   (`mut` parameters, parentheses around `match`, late initialization of outputs,
   `.clone()` on `Copy` types, `&**`, `return` at the end, `X { .. }` for unit
   variants, `-> ()`). Each is a small mmtorust PR that CI tests as usual, and it
   conflicts with nobody, because nobody edits generated code. The generated
   code that the flip commits has then been tested in master for weeks. The 8
   largest machine-fixable lints are ~110,000 of the ~137,000 outside the Susan
   crates; the generator is the only practical way to fix them
   ([6.3](#63-clippy---fix-on-one-crate)).
2. **At the flip.** The per-file `#![allow(warnings)]` is replaced by a
   `[workspace.lints]` table that allows exactly the lints that are left
   (whatever the generator work did not remove, plus the by-design ones), and
   CI runs
   `cargo clippy --workspace --all-targets -- -D warnings` from the first day.
   New code is then clean, and the baseline can only shrink. Susan output keeps
   `#![allow(warnings)]` in the `include!` wrapper.
3. **Weeks 1-6 after the flip: burn down the ~11,400.** One lint per PR, across
   the workspace, removing one line from the allow table. `dead_code` first (it
   is deletion), then `unreachable_*`, `unused_assignments`,
   `match_like_matches_macro`, `type_alias_bounds`. One or two people, PRs
   merged within a day, so that other PRs need at most a trivial rebase.
4. **Keep allowed until the redesign:** naming (D2), `arc_with_non_send_sync`,
   `type_complexity`, `too_many_arguments`. They describe the MetaModelica
   design, and a mechanical fix would only hide that.

Not recommended: committing the generated code with `#![allow(warnings)]` and
"linting later". Without a gate, new code adds warnings, and the later
clean-up is a long series of large diffs against code that people are
editing.

## 7. What to remove and when

| What | Size | Remove in | Blocked by |
| --- | --- | --- | --- |
| C build of omc: `boot/` (bomc + tarball), `.mo -> C` CMake rules, `OpenModelicaBootstrappingHeader.h`, bootstrapping tests | 6 files + tarball | phase 1 | Rust omc at parity on all platforms (5.1) |
| `Compiler/runtime/*.c` (omc's `external "C"` in C) | 80 files, 27k lines | phase 1 | as above; hand-written Rust replaces it already |
| `FrontEndCpp/` | 114 files, 19k lines | phase 1 | as above |
| `OMCompiler/Parser/` (ANTLR grammar), `3rdParty/antlr` | 18 files + 3rdParty | phase 1 (Parser), later (antlr) | OMEdit's debugger also uses ANTLR |
| `Compiler/Stubs/`, `Compiler/Unused/` | 26 files | phase 1 | nothing |
| 3rdParty that only the C omc links | audit | phase 1 | ColPack (`ColPackBicoloring_omc.cpp`; the Rust omc has no star-bicoloring yet, see `OpenModelica.rs/TODO.md`); the omc side of simdjson, ryu, libzmq (OMEdit still uses them). oneTBB (ParModelica auto) and METIS (Ipopt/MUMPS) stay |
| FMI 1.0 export and import | `CodegenFMU1.tpl`, ~230 references, ~10 runtime files, 90 tests | phase 1 | nothing |
| `-g=MetaModelica`, `MetaModelicaBuiltin.mo`, `testsuite/metamodelica` | 223 test files + 157 tests elsewhere | phase 1 | the C omc no longer bootstraps |
| Old frontend instantiation engine (group D) | 79k lines `.mo` / 119k lines Rust | phase 1 (entry points + dead code), rest in phase 4 | scripting API and SimCodeUtil callers (section 8) |
| `.mo` sources, `Template/*.mo`, `*.rust.mo`, `*.handwritten.rs`, `rust_src_sync` + mirror, `rust_src_files.txt`, `OpenModelicaScriptingAPI.mo` | ~620k lines `.mo` | phase 2 (the flip) | |
| mmtorust (49k lines), `external_c_calls.rs` | | phase 3 | the last `.mo` PR is ported |
| MetaModelica grammar in the Rust parser (`openmodelica_ast`): `match`, `uniontype`, `list<>`, ... | | phase 3, after mmtorust | mmtorust parses the `.mo` sources with this parser. Susan reads `*TV.mo` with its own parser, so the TV files can stay |
| MetaModelica-only code generation in `CodegenCFunctions.tpl` (match, uniontypes, lists) and the matching parts of `SimulationRuntime/c/meta` | | phase 3-4 | nothing generates MetaModelica code any more; check what the Modelica function code still uses (strings, boxing, `threadData` and `MMC_TRY`) |
| Boehm GC in the C simulation runtime | | phase 4, optional | the C runtime's own allocation (it is not needed by omc itself after phase 1) |
| Old backend (`BackEnd/`: 316k lines Rust incl. `backend_*`), `NFConvertDAE` | | phase 4 | the new backend is the default and covers the testsuite |
| DAE toolbox (group K2: Expression, ComponentReference, Types, DAEUtil, ...) and `DAE` itself | 65k lines `.mo` / 98k lines Rust | after the old backend | SimCode and the templates use DAE expressions; NSimCode converts to the old SimCode |
| `metamodelica` runtime crate | | gradually | every generated module uses `List`, `Array`, `Ref`; shrink it as code is redesigned |

## 8. What remains of the old frontend

See [old-frontend-deps.svg](rust-migration-plan/old-frontend-deps.svg). The
analysis uses package imports (`import X;`, `import A = X;`) of the 460 packages
in the CMake source list, and call counts in the code (comments removed).

`FrontEnd/` + `FFrontEnd/` are 95 packages and 172k lines of `.mo` (not counting the builtin declaration files). They fall
into three groups.

**K1: shared AST and data. Stays.** 29k lines `.mo`.
Absyn, AbsynUtil, AbsynToSCode, SCode, SCodeUtil, SCodeDump, Dump, DAE,
DAEDumpTypes, ClassInf, ElementSource, Values, ValuesDump, ValuesMake,
ValuesUtil, Parser, ParserExt, ClassLoader, AvlTreePathFunction, MMath,
InstBasics, BackendInterface, ComponentReferenceBasics, ExpressionBasics.
These are what the new frontend uses. Of the 84 NF packages, 50 import Absyn,
44 DAE, 31 SCode, 29 AbsynUtil, 26 SCodeUtil, 24 ElementSource, 13 Dump.

The new frontend's other imports from `FrontEnd/`:

| NF package | Uses | What to do |
| --- | --- | --- |
| NFInstanceAPI | `FBuiltin.getInitialFunctions` (the parsed builtin program), `Parser`, `AbsynToSCode` | move `getInitialFunctions` into a small `builtin` module next to the builtin `.mo` files; it is not old-frontend logic |
| NFFunction | `InstBasics.getFunctionRestrictionPurity`, `commentIsInlineFunc` | keep (K1), or move into NF |
| NFCeval, NFCall, NFInst, NFLookup | `BackendInterface.evalScriptingFunction`, `lookupScriptVariable` (call-backs into the scripting API) | keep; after the flip this can become a trait object |
| NFConvertDAE | DAE, `AvlTreePathFunction`, `ComponentReferenceBasics` | the bridge to the old backend; goes with it |
| NFRestriction | `ClassInf.State` for the conversion to DAE | as above |
| NFExpression | `Values`, `ValuesDump`, `ValuesMake` | keep (`Values` is the scripting value type) |

At the crate level the separation already exists: `openmodelica_nf_frontend`
does not depend on `openmodelica_frontend` (the old engine).

**K2: the old DAE toolbox. Stays while the old backend and the DAE-based
SimCode live.** 65k lines `.mo`, 98k lines Rust.
Expression, ExpressionSimplify, ExpressionDump, ComponentReference, DAEUtil,
DAEDump, Types, TypesDump, Inline, Algorithm, CheckModel, ClassInfUtil,
StateMachineFlatten, PrefixUtil, Builtin, UnitAbsyn*, Graphviz, Patternm (one
helper), Ceval, FCore, FGraph, FBuiltin. Users: the old backend (DAE in 42 of 53
packages, Expression in 37, Ceval in 8, Inline in 8), SimCode, the templates
(Types 15, Expression 15, Values 11), and the new backend through NSimCode,
which converts its result to the old `SimCode` with DAE expressions.

Two items in K2 are special:

- `Ceval` is old-frontend constant evaluation. The old backend calls
  `Ceval.ceval`/`Ceval.evalExp` (20 call sites in `BackEnd/`, 5 in `SimCode/`, 14
  in `Script/`). Replace it with an evaluator on DAE expressions (a reduced
  `Ceval` without environment lookup) or with NF evaluation before the
  conversion to DAE.
- `FCore.Cache` and `FGraph.Graph` are passed through hundreds of signatures as
  `FCore.emptyCache()` (438 uses) and `FGraph.empty()` (403 uses), mostly in
  `Script/` (707 and 427 references). They carry nothing for NF-based code; remove
  them from the signatures in phase 4 (a mechanical change that is much easier
  in Rust than in `.mo`).

**D: the old instantiation engine. Removed.** 79k lines `.mo`, 119k lines Rust.
Inst and 13 Inst* packages, Lookup, Static, Mod, ConnectUtil, ConnectionGraph,
InnerOuter, CevalFunction, OperatorOverloading, MetaUtil, SCodeInstUtil,
DumpGraphviz, the NFSCode*/NFEnv*/NFInst* packages (the 2012 prototype, not NF),
and `FFrontEnd/` except FCore, FGraph, FBuiltin.

Callers of group D outside the old frontend, i.e. the work needed before D is
gone completely:

| Caller | Calls into D | Replacement |
| --- | --- | --- |
| `Script/` (CevalScript, CevalScriptBackend, Interactive, ...) | `Inst` (~50 calls: `instantiateClass`, `makeEnvFromProgram`, `instExp`, `lookupRootClass`), `Lookup` (~38), `Static` (12: `elabExp`, `elabGraphicsExp`, `elabCodeExp`), `Mod`, `InnerOuter`, `InstHashTable`, `NFSCodeFlatten` | NF + NFApi for every API function; `elabGraphicsExp` for annotations through `NFApi`; this is the biggest item |
| `SimCode/SimCodeUtil` | `Static.crefVectorize`, `Mod.getUnelabedSubMod` (11), `PrefixUtil` (1) | small rewrites on DAE |
| `BackEnd/` | `Inst.instExp`-like helpers (1), `ConnectUtil.topLevelInput` (3), `InnerOuter` constants | small rewrites |
| `Util/` | `Static` (1), `InstStateMachineUtil` (1) | small rewrites |

## 9. What to redesign after the flip

Ordered by value for the effort. None of this should be mixed into the flip.

1. **Scripting API on NF only.** `CevalScript*`/`Interactive` is the largest
   user of the old frontend and threads `FCore.Cache`/`FGraph.Graph` through
   everything. Move each API function to NF/NFApi, then drop the dummy
   parameters. This unblocks removing group D.
2. **Global state into a context.** `Flags`, `Global` roots
   (`thread_local` in `Globals.rs`), the error buffer and the caches are globals.
   A compile context struct lets OMEdit, the web build and parallel compilation
   run several models in one process safely.
3. **Errors.** `Result<T, &'static str>` with MetaModelica `fail()` semantics,
   and ~2,000 `matchcontinue` turned into closures that discard errors. Introduce
   a real error type where failures are expected, and plain `Option`/`match` where
   `matchcontinue` was only control flow. Also the fallibility analysis
   classified 24,453 of 31,635 functions as fallible; many of them only because a
   callee could fail in theory.
4. **Data structures.** `metamodelica::List` (cons lists), `Array<T>` as
   `Rc<RefCell<Vec<T>>>`, the generated `HashTable*`/`BaseHashTable`/`AvlTree*`
   packages, `Arc` everywhere and the cycle collector. Replace by `Vec`, slices,
   `HashMap`/`BTreeMap` and owned data where it matters. This is also where
   more speed comes from: in a 2026-09 profile of the backend, the allocator
   and drop glue were about 35% of all instructions.
5. **Old backend retirement.** Make the new backend the default, remove
   `BackEnd/`, `NFConvertDAE` and the DAE-based code paths, then move SimCode to
   NF/NB types so that the DAE toolbox (K2) can go.
6. **Code generators.** Replace Susan templates by Rust code one generator at a
   time (C, C++, FMU, XML), starting with the ones that change least. Susan goes
   when the last template is gone.
7. **Crate structure.** Today crates follow `__OpenModelica_Interface`
   annotations. `openmodelica_backend` (277k lines) is one compilation unit and
   the bottleneck of a build; split it by subsystem. Rename crates (D2 territory)
   at the same time.
8. **Naming.** If D2 kept MetaModelica names, rename modules and functions
   gradually, crate by crate, together with the redesign of that crate, never as
   a repository-wide sweep.

## 10. How the plan keeps disruption small

- **Everything systematic happens in the generator**, before the flip, where one
  reviewed change fixes a million lines and nobody's branch conflicts.
- **The flip is reproducible**: anyone can run `check-flip.sh` and get the same
  Rust from the freeze commit. Review the tools, not the 1.3M lines.
- **One short freeze** (2-3 days) instead of a long period with two sources of
  truth.
- **Open PRs have a path**: merge before the freeze, or port with
  `port-mo-pr.sh`. 47 open PRs touch `.mo`; 30 of them were active in the last
  30 days. Contact those authors with the date.
- **History stays usable**: rebase-merge for the flip PR, the move in its own
  commit (rename detection), the formatting commit and the mechanical lint
  commits in `.git-blame-ignore-revs`, and the `// Ported from <file>.mo at <sha>` header to
  find the history before the flip.
- **The release branch is the fallback**: `maintenance/v1.28` keeps the C omc;
  users who need FMI 1.0, the old frontend or MetaModelica stay on it.
- **CI is green from day one** with the allow-list, and the allow-list only
  shrinks.
- **Deletions before the flip are timeboxed**, so the flip date does not move.

## 11. Risks

| Risk | Mitigation |
| --- | --- |
| Slower translation for some models (overall 0.89x, but a third of the models with ≥ 2 s are slower, p90 1.23x; Buildings 1.10x) | keep the OMLT master vs wasm-jit comparison as a gate until the C omc is gone; the release branch keeps the C omc; redesign item 4 |
| Behaviour changes from `i32` Integers | D4 before the switch |
| Windows/macOS packaging not ready | gate in 5.1; the MSVC and macOS cross builds exist, MSYS2 needs checking |
| OMEdit or other users of `libOpenModelicaCompiler`/`libOMCDLL` break | gate in 5.1; OMEdit already runs against the Rust library (`OMC_RUST_ABI`) |
| Contributors who only know MetaModelica stop contributing | onboarding, pairing, keep the generated code close to the `.mo` it came from (D2/D3), review help in the first months |
| A long tail of `.mo` PRs that nobody ports | port in the first weeks; after that the author re-implements |
| Nightly toolchain (cranelift, build-std) | D8 |
| Lint burn-down PRs conflict with feature PRs | one lint per PR, merged within a day, by one or two people, in a short window |

## Appendix: how the numbers were measured

Machine: 24 cores, 39 GB RAM (WSL2), toolchain `nightly-2026-05-31`
(clippy 0.1.98). master at `d7a542f7f0`.

- **Generated code.** `cmake --build <build> --target rust_codegen`
  after a source change: 7:00 wall, 1.5 GB peak. mmtorust's own report: analyses
  ~90 s (fallibility 21 s, visibility 51 s, PartialEq/Default 16 s), code
  generation 121 s wall (457 s CPU, 426 files; `CevalScriptBackend.rs` alone 88 s).
  Generated files are the `.rs` files in the build copy that are not tracked in
  git and are declared in their crate's `lib.rs` (461 files; 5 stale leftovers
  excluded). mmtorust files start with `// Auto-generated from MetaModelica
  source`, Susan files with `// Generated by Susan`.
- **Package imports.** A script over the 460 files of
  `rust_compilerSources.txt` (the CMake source list) that removes comments and
  collects `import X;`/`import A = X;`. Call counts are `grep` over the code
  without comments; NF/NB use aliases such as `import Ceval = NFCeval;`, so their
  counts were taken from the import graph, not from `Pkg.` prefixes.
- **rustfmt.** `rustfmt --edition 2024 --config max_width=120,style_edition=2024`
  over the 461 generated files in a copy of the build tree, then `cmp` with the
  original. Hand-written files: `OpenModelica.rs/format.sh --check --all`, then
  rustfmt on a copy of the 226 reported files and `git diff --no-index --shortstat`.
- **Lines inside `match_deref!`.** Outermost `match_deref! { ... }` blocks in the
  generated files, brace-matched.
- **cargo check / clippy.** A copy of `<build>/OMCompiler/Compiler/rust-src` in a
  scratch directory, with the environment of the CMake `rust_libopenmodelica` rule
  (blobs taken from the build with `OMC_WASM_PREBUILT_IN`, no sccache) and
  `--no-default-features --features codegen_c,codegen_fmu_c,openmodelica_codegen_wasm_jit/sundials,mimalloc -p libopenmodelica_compiler`
  (the compiler and everything it depends on, including the simulation runtime
  crates; not the GUI, web and tool crates). Dev profile. `--message-format=json`
  piped into a counter (per lint, per crate, generated or not, machine-applicable
  or not). Clippy's deny-level lints were capped with `-- --cap-lints=warn` so
  that the whole graph is linted.
- **Compile speed.** OMLT database (`omdb`), tables `master` (run 1794966338)
  and `wasm-jit` (run 1794977495), both `v1.28.0-dev.1974+gf37507374b`, joined on
  library and model. A phase counts for a model when `finalphase` in both lanes
  is at least that phase (1 frontend, 2 backend, 3 SimCode). Per-model medians
  only over models where the C omc needs at least 0.5 s (2 s for translation).
- **Repository activity.** `git log --since=2026-05-18` on `OMCompiler/Compiler`
  `*.mo`/`*.tpl` and on the two Rust trees; `gh pr list --state open`.

- **`clippy --fix`.** `cargo clippy --fix --allow-no-vcs --no-deps -p
  openmodelica_frontend_base -- --cap-lints=warn` on the same copy, once as is
  and once with `-A clippy::match_single_binding`; both rolled back; then
  `cargo check` of the graph and `git diff --no-index` against a snapshot.

Raw data and scripts are in [rust-migration-plan/data/](rust-migration-plan/data/):

| File | Contents |
| --- | --- |
| `clippy-allows-removed.json` | the 332,491 warnings by lint, by crate, by crate and lint; generated and machine-applicable counts |
| `clippy-allows-as-today.json` | the same with today's `allow`s (1,218 warnings) |
| `check-allows-removed.json` | `cargo check` (rustc lints only) without the `allow`s |
| `mo-package-imports.json` | per `.mo` package: directory, lines, imports, `__OpenModelica_Interface` |
| `crate-deps.json`, `crate-lines.json` | path dependencies between the crates; generated and hand-written lines per crate |
| `scripts/` | `imports.py`, `count.py`, `strip_allows.py`, `run_clippy.sh`, `env.sh` (paths are for this machine) |
