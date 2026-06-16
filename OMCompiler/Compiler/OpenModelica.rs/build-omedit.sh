#!/bin/bash
#
# Build OMEdit against the Rust omc port (libOpenModelicaCompiler.so) in-process.
#
# Builds the cdylib, installs the Rust-targeting OMC interface + the embedding
# header + the .so into the OpenModelica build tree, then builds OMEdit with the
# OMC_RUST_ABI compile-time switch enabled. See omedit-rust-embedding notes.
#
# Run from the OpenModelica.rs checkout:  ./build-omedit.sh
# Override the build tree with: OMBUILDDIR=/path/to/build ./build-omedit.sh
#
set -euo pipefail

RUST="$(cd "$(dirname "$0")" && pwd)"
OMBUILDDIR="${OMBUILDDIR:-/projects/OpenModelica/build}"
OMEDIT="${OMEDIT:-/projects/OpenModelica/OMEdit}"
OMPLOT="${OMPLOT:-/projects/OpenModelica/OMPlot/OMPlot/OMPlotGUI}"
HOST_SHORT="${HOST_SHORT:-$(gcc -dumpmachine)}"
JOBS="${JOBS:-12}"
export OMEDIT_RUST_OMC=1  # enables the rust_omc / OMC_RUST_ABI switch in OMEdit + OMPlot

INC="$OMBUILDDIR/include/omc/scripting-API"
LIB="$OMBUILDDIR/lib/$HOST_SHORT/omc"

echo ">>> 1/5 building libOpenModelicaCompiler.so (release)"
cd "$RUST"
cargo build -j"$JOBS" --release -p libopenmodelica_compiler

echo ">>> 2/5 installing the Rust OMC interface + embedding header into $INC"
mkdir -p "$INC" "$LIB"
# Back up the C-omc versions once, so the stock build can be restored.
for f in "$LIB/libOpenModelicaCompiler.so" \
         "$INC/OpenModelicaScriptingAPIQt.cpp" "$INC/OpenModelicaScriptingAPIQt.h"; do
  [ -f "$f" ] && [ ! -f "$f.cbak" ] && cp -a "$f" "$f.cbak" && echo "    backed up $(basename "$f") -> $(basename "$f").cbak"
done
cp "$RUST/openmodelica_scripting_qt/qt/OpenModelicaScriptingAPIQt.cpp" "$INC/"
cp "$RUST/openmodelica_scripting_qt/qt/OpenModelicaScriptingAPIQt.h"   "$INC/"
cp "$RUST/openmodelica_scripting_qt/qt/OpenModelicaScriptingAPIQtABI.h" "$INC/"
cp "$RUST/libopenmodelica_compiler/include/omc_rust_embedding.h"        "$INC/"
cp "$RUST/target/release/libOpenModelicaCompiler.so" "$LIB/libOpenModelicaCompiler.so"

echo ">>> 3/5 building libOMPlot.so against the Rust readers (no C runtime)"
# Force a clean relink: a LIBS-only change does not re-trigger the .so link rule.
cd "$OMPLOT"
rm -f Makefile Makefile.lib ../bin/libOMPlot.so* ../bin/OMPlot
make -f Makefile.unix -j"$JOBS"                # builds lib (+ standalone exe) and installs to build/
cp -a ../bin/libOMPlot* "$LIB/" 2>/dev/null || true
cp -p ../bin/OMPlot "$OMBUILDDIR/bin/" 2>/dev/null || true

echo ">>> 4/5 qmake + building OMEdit (OMC_RUST_ABI switch)"
cd "$OMEDIT"
qmake6 -r
rm -f bin/OMEdit OMEditGUI/OMEdit                # force relink (LIBS changed)
make -j"$JOBS"

echo ">>> 5/5 installing OMEdit into $OMBUILDDIR/bin"
# The binary's rpath ($ORIGIN/../lib) resolves the omc libraries only from the
# build tree's bin/, so install it there (overwriting the stock C build, backed
# up once). Running the OMEdit/bin/ copy directly would need LD_LIBRARY_PATH.
mkdir -p "$OMBUILDDIR/bin"
[ -f "$OMBUILDDIR/bin/OMEdit" ] && [ ! -f "$OMBUILDDIR/bin/OMEdit.cbak" ] && \
  cp -a "$OMBUILDDIR/bin/OMEdit" "$OMBUILDDIR/bin/OMEdit.cbak" && echo "    backed up build/bin/OMEdit -> OMEdit.cbak"
cp -a "$OMEDIT/bin/OMEdit" "$OMBUILDDIR/bin/OMEdit"

cat <<EOF

OMEdit installed: $OMBUILDDIR/bin/OMEdit  (run it from here so its rpath finds the omc libs)

  OPENMODELICAHOME=$OMBUILDDIR $OMBUILDDIR/bin/OMEdit

(add QT_QPA_PLATFORM=offscreen for a headless smoke test).
To restore the stock C build, copy the *.cbak files back over the installed ones
(the .so, the scripting-API headers, and build/bin/OMEdit).
EOF
