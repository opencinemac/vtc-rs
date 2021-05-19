.PHONY: format
format:
	-cargo fmt

.PHONY: lint
lint:
	-cargo clippy
	-rustfmt ./src/*.rs --check

.PHONY: test
test:
	-cargo test --color=always --no-fail-fast

.PHONY: doc
doc:
	-cargo doc --lib --open
