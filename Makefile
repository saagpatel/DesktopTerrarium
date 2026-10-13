.PHONY: build test lint clean check audit run

build:
	cargo build --locked --release

check:
	@command -v cargo >/dev/null 2>&1 && command -v rustfmt >/dev/null 2>&1 && command -v clippy-driver >/dev/null 2>&1 || { echo "Rust setup incomplete: install the pinned rust-toolchain.toml with rustfmt and clippy components." >&2; exit 1; }
	./scripts/check_local_artifacts.sh
	cargo fmt --all -- --check
	cargo clippy --locked --all-targets --no-deps -- -D warnings
	cargo test --locked
	cargo build --locked

audit:
	cargo audit

test:
	cargo test --locked

lint:
	cargo clippy --locked -- -D warnings

run:
	cargo run --locked

clean:
	cargo clean
