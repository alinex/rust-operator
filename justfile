test:
  RUST_BACKTRACE=1 cargo test -- --nocapture

testlib PATTERN:
	RUST_BACKTRACE=1 cargo test --lib {{PATTERN}} -- --nocapture

debug TEST:
	cargo test --test {{TEST}} --features debug

build:
	cargo build

check:
	cargo check

@bench: nightly
	cargo bench && just remove-nightly

nightly:
	rustup override add nightly

remove-nightly:
	rustup override remove

@lint: nightly
	cargo build --features lints && just remove-nightly

showdoc:
  cargo doc --open


name = `sed -En 's/name[[:space:]]*=[[:space:]]*"([^"]+)"/v\1/p' Cargo.toml`
version = `sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/\1/p' Cargo.toml`

release:
	cargo build --release
	strip target/release/operator

publish:
	@mkdir -p dist
	cargo build --release --target x86_64-unknown-linux-gnu
	@cp target/x86_64-unknown-linux-gnu/release/operator dist/{{name}}-{{version}}-x86_64-unknown-linux-gnu

clean:
	cargo clean
	find . -type f -name "*.orig" -exec rm {} \;
	find . -type f -name "*.bk" -exec rm {} \;
	find . -type f -name ".*~" -exec rm {} \;
