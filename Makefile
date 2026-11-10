.PHONY: tidy fmt fmt-check vet check validate build build-release run clean install

name ?= world

tidy:
	cargo fetch

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

vet:
	cargo clippy --all-targets --all-features -- -D warnings

check:
	cargo check

validate: tidy fmt fmt-check vet check build

build:
	cargo build

build-release:
	cargo build --release

run:
	cargo run -- --name $(name)

clean:
	cargo clean
