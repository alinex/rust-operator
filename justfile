debug TEST:
	cargo test --test {{TEST}} --features debug

tests:
  cargo test
#  cargo test --features "yaml unstable"

test TEST:
	cargo test --test {{TEST}}

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
  
clean:
	cargo clean
	find . -type f -name "*.orig" -exec rm {} \;
	find . -type f -name "*.bk" -exec rm {} \;
	find . -type f -name ".*~" -exec rm {} \;
