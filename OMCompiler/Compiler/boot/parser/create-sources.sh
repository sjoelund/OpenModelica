#!/bin/bash
RUST=$PWD
/usr/bin/omc listMSL.mos
mkdir -p ~/OpenModelica/OMCompiler/Compiler/boot/parser
cd ~/OpenModelica/OMCompiler/Compiler/boot/parser
/usr/bin/omc $RUST/listMetaModelicaFiles.mos
cp compilerSources.txt $RUST
