#!/usr/bin/env bash
# Replace the MetaModelica sources of omc by the Rust that mmtorust generates
# from them (doc/rust-migration-plan.md, section 5.3). Run it at the top of a
# clean checkout of the freeze commit, on the branch that gets the commits:
#
#   OMCompiler/Compiler/OpenModelica.rs/tools/flip.sh [-B DIR] [-j N] [-- CMAKE_OPTION...]
#
#   -B DIR  build directory for the transpile, default build-flip. It must not
#           exist, unless it holds a transpile of this commit by an earlier run.
#   -j N    parallel jobs for the transpile.
#
# Commits, each one builds:
#   1. the Rust that mmtorust generates, and the Qt scripting API files;
#   2. rustfmt and taplo over all Rust and TOML files (layout only);
#   3. the build uses the committed Rust and no longer runs mmtorust;
#   4. the MetaModelica sources are deleted.
# Check the result with tools/check-flip.sh.
set -euo pipefail

build=build-flip
jobs=$(nproc)
while [ $# -gt 0 ]; do
  case "$1" in
    -B) build=$2; shift ;;
    -B*) build=${1#-B} ;;
    -j) jobs=$2; shift ;;
    -j*) jobs=${1#-j} ;;
    --) shift; break ;;
    *) sed -n '2,/^set -e/p' "$0" | sed '$d; s/^# \{0,1\}//' >&2; exit 2 ;;
  esac
  shift
done

build=$(realpath -m "$build")
cd "$(git rev-parse --show-toplevel)"
tools=OMCompiler/Compiler/OpenModelica.rs/tools
# shellcheck source=flip/common.sh
. $tools/flip/common.sh

git diff --quiet --ignore-submodules HEAD -- || die "the working tree has changes"
if [ -d OMCompiler/Compiler/boot ]; then
  echo "warning: OMCompiler/Compiler/boot exists: phase 1 is not done, and after" \
       "this only a build with -DOM_OMC_ENABLE_RUST=ON works" >&2
fi
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

commit() {
  git commit -q -F - && git log --oneline -1
}

# Paths in file $1 (relative to the repository root): stage them, also when
# a .gitignore rule matches.
stage() {
  git add -f --pathspec-from-file="$1"
}

echo "== Transpile into $build"
if [ ! -e "$build/CMakeCache.txt" ]; then
  [ -e "$build" ] && die "$build exists but is not a build directory"
  cmake -S . -B "$build" -DOM_OMC_ENABLE_RUST=ON -DOM_ENABLE_GUI_CLIENTS=OFF \
    -DOM_ENABLE_OMSIMULATOR=OFF -DOM_OMC_ENABLE_CPP_RUNTIME=OFF "$@"
fi
cmake --build "$build" --target rust_codegen -j "$jobs"
mirror=$(mirror_dir "$build")
qt=$build/OMCompiler/Compiler/scripting-api-qt

echo "== 1. Add the generated Rust"
generated_files "$build" HEAD > "$tmp/generated"
[ -s "$tmp/generated" ] || die "no generated files in $mirror"
while read -r f; do
  mkdir -p "$RS/$(dirname "$f")"
  cp "$mirror/$f" "$RS/$f"
  echo "$RS/$f"
done < "$tmp/generated" > "$tmp/stage"
mkdir -p $RS/openmodelica_scripting_qt/qt
for f in OpenModelicaScriptingAPIQt.cpp OpenModelicaScriptingAPIQt.h; do
  cp "$qt/$f" $RS/openmodelica_scripting_qt/qt/
  echo "$RS/openmodelica_scripting_qt/qt/$f" >> "$tmp/stage"
done
grep -vxF -f "$tmp/generated" $RS/.gitignore > "$tmp/gitignore" || true
cp "$tmp/gitignore" $RS/.gitignore
{
  grep '^#' $MANIFEST
  grep -v '^#' $MANIFEST | cat - "$tmp/generated" | sort -u
} > "$tmp/manifest"
cp "$tmp/manifest" $MANIFEST
printf '%s\n' $RS/.gitignore $MANIFEST >> "$tmp/stage"
stage "$tmp/stage"
commit <<EOF
[Rust] Add the generated Rust sources

Add the $(wc -l < "$tmp/generated") Rust files that mmtorust generates from the MetaModelica
sources at $(git rev-parse --short HEAD), and the Qt scripting API files for OMEdit. They
are copied without changes from a transpile of that commit. The build
still runs mmtorust, which writes the same files.
EOF

echo "== 2. Format all Rust and TOML files"
$RS/format.sh --all
git diff --name-only --ignore-submodules > "$tmp/stage"
if [ -s "$tmp/stage" ]; then
  stage "$tmp/stage"
  commit <<EOF
[Rust] Format all Rust and TOML files

Run format.sh --all. This commit only changes the layout of
$(wc -l < "$tmp/stage") files. Add it to .git-blame-ignore-revs.
EOF
fi

echo "== 3. Build from the committed Rust"
git apply --index --3way $tools/flip/build.patch
python3 $tools/flip/runtime_sources_rs.py OMCompiler/SimulationRuntime/c/RuntimeSources.mo.cmake \
  OMCompiler/SimulationRuntime/c/RuntimeSources.rs.cmake
git add OMCompiler/SimulationRuntime/c/RuntimeSources.rs.cmake
git rm -q OMCompiler/SimulationRuntime/c/RuntimeSources.mo.cmake
commit <<'EOF'
[Rust] Build omc from the committed Rust

The build no longer runs mmtorust. It builds susan from the committed
Rust and uses it to compile the templates, as before. RuntimeSources.rs
comes from RuntimeSources.rs.cmake, which CMake configures as it did
RuntimeSources.mo. The Qt scripting API files are read from
openmodelica_scripting_qt/qt.
EOF

echo "== 4. Remove the MetaModelica sources"
# The transpiled sources, and every other compiler .mo except the builtin files
# (data that omc reads) and the Susan template interfaces.
keep='OMCompiler/Compiler/((FrontEnd/(AnnotationsBuiltin_[0-9]+_x|MetaModelicaBuiltin|ModelicaBuiltin|PDEModelicaBuiltin)|NFFrontEnd/NFModelicaBuiltin|Template/[A-Za-z0-9]+TV)\.mo|scripts/.*)$'
{
  grep -v '^#' "$build/OMCompiler/Compiler/rust_compilerSources.txt" |
    while read -r f; do realpath -m --relative-to=. "$f"; done | grep -v '^\.\./'
  git ls-files 'OMCompiler/Compiler/*.mo' 'OMCompiler/Compiler/*.mo.in' 'OMCompiler/Compiler/susan_codegen/*' |
    grep -v "^$RS/" | grep -vE "^$keep"
  echo $RS/susanSources.txt
} | sort -u | xargs -r -d '\n' git ls-files -- > "$tmp/remove"
if grep -E "^$keep" "$tmp/remove"; then die "a transpiled source is also a builtin file"; fi
git rm -q --pathspec-from-file="$tmp/remove"
sed -n "s|^$RS/||p" "$tmp/remove" > "$tmp/remove-rs"
grep -vxF -f "$tmp/remove-rs" $MANIFEST > "$tmp/manifest" || true
cp "$tmp/manifest" $MANIFEST
git add $MANIFEST
commit <<EOF
[Rust] Remove the MetaModelica sources

Remove the MetaModelica sources of the compiler ($(wc -l < "$tmp/remove") files). The Rust
files that were generated from them are the source now. The builtin .mo
files and the template interfaces (Template/*TV.mo) stay: omc and susan
read them.
EOF
