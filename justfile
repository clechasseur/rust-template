set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

default_toolchain := ''

default: test

tidy: clippy fmt

clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

fmt:
    cargo +nightly fmt --all

check toolchain=default_toolchain:
    cargo {{toolchain}} check --workspace --all-features

build toolchain=default_toolchain:
    cargo {{toolchain}} build --workspace --all-features

test toolchain=default_toolchain:
    cargo {{toolchain}} test --workspace --all-features

tarpaulin toolchain=default_toolchain:
    cargo {{toolchain}} tarpaulin --target-dir target-tarpaulin

msrv:
    cargo msrv -- cargo check --workspace --all-targets

doc:
    cargo +nightly doc --workspace --all-features --open
