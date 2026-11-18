#!/usr/bin/env bash
# Check that the Rust that flip.sh committed is what mmtorust generates from the
# freeze commit (doc/rust-migration-plan.md, section 5.3):
#
#   OMCompiler/Compiler/OpenModelica.rs/tools/check-flip.sh -B DIR FREEZE [REV]
#
#   -B DIR  build directory with a transpile of FREEZE (cmake --build DIR
#           --target rust_codegen), for example the one flip.sh used.
#   FREEZE  the commit flip.sh ran on.
#   REV     the flipped commit, default HEAD.
#
# Each generated file must be in REV and be equal to the transpiled file after
# rustfmt; the Qt scripting API files must be equal.
set -euo pipefail

build=
while [ $# -gt 0 ]; do
  case "$1" in
    -B) build=$2; shift ;;
    -B*) build=${1#-B} ;;
    -*) build= ; break ;;
    *) break ;;
  esac
  shift
done
if [ -z "$build" ] || [ $# -lt 1 ] || [ $# -gt 2 ]; then
  sed -n '2,/^set -e/p' "$0" | sed '$d; s/^# \{0,1\}//' >&2
  exit 2
fi
freeze=$1
rev=${2:-HEAD}
build=$(realpath -m "$build")

cd "$(git rev-parse --show-toplevel)"
# shellcheck source=flip/common.sh
. OMCompiler/Compiler/OpenModelica.rs/tools/flip/common.sh

mirror=$(mirror_dir "$build")
qt=$build/OMCompiler/Compiler/scripting-api-qt
RUSTUP_TOOLCHAIN=${RUSTUP_TOOLCHAIN:-$(git show "$rev:$RS/rust-toolchain.toml" | sed -n 's/^channel *= *"\(.*\)"/\1/p')}
export RUSTUP_TOOLCHAIN
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
git show "$rev:$RS/rustfmt.toml" > "$tmp/rustfmt.toml"

generated_files "$build" "$freeze" > "$tmp/generated"
[ -s "$tmp/generated" ] || die "no generated files in $mirror"
bad=0
n=0
while read -r f; do
  n=$((n + 1))
  if ! git cat-file -e "$rev:$RS/$f" 2> /dev/null; then
    echo "missing in $rev: $RS/$f"
    bad=$((bad + 1))
    continue
  fi
  (cd "$tmp" && rustfmt --emit stdout) < "$mirror/$f" > "$tmp/expected"
  if ! git show "$rev:$RS/$f" | cmp -s - "$tmp/expected"; then
    echo "differs: $RS/$f"
    git show "$rev:$RS/$f" | diff -u --label "$rev" --label transpile - "$tmp/expected" | head -20 || true
    bad=$((bad + 1))
  fi
done < "$tmp/generated"
for f in OpenModelicaScriptingAPIQt.cpp OpenModelicaScriptingAPIQt.h; do
  n=$((n + 1))
  if ! git show "$rev:$RS/openmodelica_scripting_qt/qt/$f" 2> /dev/null | cmp -s - "$qt/$f"; then
    echo "differs or missing: $RS/openmodelica_scripting_qt/qt/$f"
    bad=$((bad + 1))
  fi
done

if [ $bad != 0 ]; then
  echo "check-flip: $bad of $n files differ from the transpile of $freeze" >&2
  exit 1
fi
echo "check-flip: all $n files equal the transpile of $freeze"
