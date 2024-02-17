set positional-arguments
set dotenv-load

build:
    @just dev

dev:
    cargo build --profile dev

release:
    cargo build --release

help:
    @just --list

system-info:
    @echo "Running on {{arch()}}/{{os_family()}}/{{os()}}"

fmt:
    cargo fmt --all -- --check

check: fmt
    cargo clippy --fix

clean:
    cargo clean

test:
    cargo test

coverage:
    cargo tarpaulin

install: system-info check
    cargo install --path .

run *ARGS:
    cargo run {{ARGS}}
