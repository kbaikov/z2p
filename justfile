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

zizmor:
	uvx zizmor --gh-token $(gh auth token) --quiet --fix=all .github

podman-build:
	podman build --tag z2p --file Dockerfile .

podman-run:
	podman run -p 8000:8000 z2p

