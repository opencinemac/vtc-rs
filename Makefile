.PHONY: format
format:
	-cargo fmt

.PHONY: lint
lint:
	-cargo clippy

.PHONY: test
test:
	-cargo test --color=always --no-fail-fast

.PHONY: doc
doc:
	-cargo doc --lib
	open ./target/doc/vtc/index.html