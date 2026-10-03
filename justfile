default:
    @just all

all: fmt test clippy audit coverage 

fmt:
    cargo fmt --all -- --check

test:
	cargo test --all-features

clippy:
	cargo clippy -- -D warnings

audit:
	cargo deny check advisories

coverage:
	cargo tarpaulin
