FROM ubuntu:26.04

RUN apt-get update && apt-get install -qyy curl rustc cargo && rm -rf /var/lib/apt/lists/*
RUN mkdir /app && curl -fsSL https://claude.ai/install.sh | HOME=/app bash
RUN find / -name claude
COPY settings.json /app/.claude/settings.json
RUN chown -R ubuntu:ubuntu /app
