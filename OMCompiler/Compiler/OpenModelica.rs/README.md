# How to run

Setup:
```bash
apt install rustup
rustup default nightly # For cranelift - faster backend
rustup component add rust-analyzer clippy rustfmt
rustup component add rustc-codegen-cranelift-preview --toolchain nightly
./create-sources.sh # One-time only; register the MetaModelica source files
```

```
cargo run --release -p mmtorust -j10 # Translates MetaModelica sources to Rust
```

```
cargo build -p openmodelica -j10 # Using cranelift
```
