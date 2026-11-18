# Shared by flip.sh and check-flip.sh. Source it at the top of the repository.

export LC_ALL=C
RS=OMCompiler/Compiler/OpenModelica.rs
MANIFEST=OMCompiler/Compiler/.cmake/rust_src_files.txt

die() {
  echo "error: $*" >&2
  exit 1
}

# The per-build copy of $RS in build directory $1.
mirror_dir() {
  local work
  work=$(sed -n 's/^RUST_OMC_WORK_DIR:PATH=//p' "$1/CMakeCache.txt")
  echo "${work:-$1/OMCompiler/Compiler}/rust-src/Compiler/OpenModelica.rs"
}

# The .rs files mmtorust wrote into the copy in build directory $1 from
# commit $2, relative to $RS, sorted: everything except the hand-written files
# of the manifest, the templates' Rust (Susan writes it in every build) and
# RuntimeSources.rs (it comes from the CMake configure step).
generated_files() {
  local mirror templates
  mirror=$(mirror_dir "$1")
  templates=$(git ls-tree --name-only "$2" OMCompiler/Compiler/Template/ |
    sed -n 's|.*/\([^/]*\)\.tpl$|\1|p' | paste -sd'|')
  (cd "$mirror" && find . -path ./target -prune -o -type f -name '*.rs' -print) |
    sed 's|^\./||' | sort |
    comm -23 - <(git show "$2:$MANIFEST" | grep -v '^#' | sort) |
    grep -vE "^[^/]+/src/($templates|RuntimeSources)\.rs$"
}
