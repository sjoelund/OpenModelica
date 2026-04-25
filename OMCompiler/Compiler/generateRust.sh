#!/bin/bash -e

for MOFILE in */*.mo; do
  if grep -q "$MOFILE" boot/LoadCompilerSources.mos; then
    echo $MOFILE
  fi
  # grep -qE "^(public|protected)? *function" $f || echo $f; done | grep -Ev "Stubs|Template|susan_codegen"`; do grep -q `echo $f | sed s/.mo// | cut -d/ -f2` boot/Makefile.depends && echo $f;
done | grep -Ev "Stubs|Template|susan_codegen" > interfaceModules.txt

set +x

for MOFILE in `cat interfaceModules.txt`; do
  BASEFILE=`echo $MOFILE | sed s/[.]mo// | cut -d/ -f2`
  NUM=`grep "[)]$BASEFILE.stamp:" boot/Makefile.depends | grep -o interface.mo | wc -l`
  echo "$NUM;$MOFILE"
done > interfaceModules.2.txt

sort -h interfaceModules.2.txt | cut "-d;" -f2 > interfaceModules.txt

for MOFILE in `cat interfaceModules.txt`; do
  MODULE=`echo $MOFILE | sed s/[.]mo// | cut -d/ -f2 | tr A-Z a-z`
  RSFILE="src/$MODULE.rs"
  if test ! -f $RSFILE; then
    sed s/MODULE/$MODULE/ main.rs > src/main.rs
    ./claude.sh -p "Translate $MOFILE into $RSFILE"
  fi
done
