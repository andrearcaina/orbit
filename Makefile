.PHONY: tidy fmt fmt-check vet check validate build build-release run clean install

url ?= example.com

tidy:
	cargo fetch

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

vet:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test

check:
	cargo check

validate: tidy fmt fmt-check vet check test build

build:
	cargo build

build-release:
	cargo build --release

run:
	cargo run -- $(url)

clean:
	cargo clean
