# Rust-capable build image for the mmtorust omc port and its wasm/web bundle.
#
# Extends the C/C++ build-deps image (Qt, simulation-runtime deps) with the Rust
# toolchain the native C build does not need. Built by the Jenkins `dockerfile`
# agent (docker build) — see .CI/cmake/Jenkinsfile.cmake.rust.wasm.
#
# The workspace requires *nightly* cargo: Cargo.toml enables the unstable
# `codegen-backend` cargo-feature and profile.dev uses the cranelift rustc
# backend, so nightly + the rustc-codegen-cranelift component are installed. The
# wasm32 target + a matching wasm-bindgen-cli + binaryen (wasm-opt) are for the
# web bundle (OM_OMC_WASM).
FROM ghcr.io/openmodelica/build-deps:v1.22.3

# Pinned nightly so CI is reproducible (the repo has no rust-toolchain.toml; if
# one is added, prefer it as the single source of truth and drop this ARG).
# cranelift must publish for this date. Bump together with the dev image.
ARG RUST_NIGHTLY=nightly-2026-05-31
# wasm-bindgen-cli must match the pinned wasm-bindgen (see wasm/build.sh).
ARG WASM_BINDGEN_VERSION=0.2.100

# World-writable caches (the building uid is unknown), as in ../Dockerfile.
RUN mkdir -p /cache/runtest/ /cache/omlibrary/ && chmod ugo+rwx /cache/runtest/ /cache/omlibrary/

# System-wide Rust install so any uid can build and cargo can write its caches.
ENV RUSTUP_HOME=/opt/rust/rustup \
    CARGO_HOME=/opt/rust/cargo \
    PATH=/opt/rust/cargo/bin:$PATH
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
      sh -s -- -y --profile minimal --default-toolchain "${RUST_NIGHTLY}" \
               --target wasm32-unknown-unknown && \
    rustup component add rustc-codegen-cranelift-preview && \
    cargo install wasm-bindgen-cli --version "${WASM_BINDGEN_VERSION}" && \
    chmod -R ugo+rwx /opt/rust

# binaryen → wasm-opt for the release web bundle and the wasm-jit runtime.
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends binaryen && \
    rm -rf /var/lib/apt/lists/*
