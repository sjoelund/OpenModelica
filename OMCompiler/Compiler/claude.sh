#!/bin/bash -ex

docker run -e ANTHROPIC_BASE_URL="http://mountain.sjoelund.se:8080/upstream/Qwen3.6%2035B-A3B%20(TQ)/" \
-e ANTHROPIC_API_KEY='sk-no-key-required' --user ubuntu -e HOME=/app \
 -v /home/martin/dev/OpenModelica-rust/OMCompiler/Compiler:/Compiler -w /Compiler -it claude-rust \
/app/.local/bin/claude --model "Qwen3.6 35B-A3B" --dangerously-skip-permissions "$@"
