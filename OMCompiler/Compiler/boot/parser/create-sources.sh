#!/bin/bash
RUST=$PWD
/usr/bin/omc listMSL.mos
mkdir -p ~/OpenModelica/OMCompiler/Compiler/boot/parser
cd ~/OpenModelica/OMCompiler/Compiler/boot/parser
omc -g=MetaModelica $RUST/listMetaModelicaFiles.mos
cp compilerSources.txt $RUST
