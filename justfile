dev:
    trunk serve --port 8000 --open

build:
    trunk build --release

fmt:
    cargo fmt
    leptosfmt .
    just --fmt --unstable
