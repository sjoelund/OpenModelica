S=${WORK:?set WORK to a copy of the build tree, see the appendix}
TAG=$1
. $S/env.sh
cd $S/rust-src/Compiler/OpenModelica.rs
start=$(date +%s)
cargo clippy --target-dir $S/target --message-format=json $FEAT -- --cap-lints=warn 2> $S/${TAG}_stderr.log | python3 $S/count.py $S/gen_live.txt $S/${TAG}.json > $S/${TAG}_count.txt
rc=${PIPESTATUS[0]}
end=$(date +%s)
echo "WALL=$((end-start)) EXIT=$rc" >> $S/${TAG}_count.txt
