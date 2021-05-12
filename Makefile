.PHONY: format
format:
	-cargo fmt

.PHONY: lint
lint:
	-cargo clippy

.PHONY: test
test:
	-cargo test --color=always --package vtc --lib framerate_test --no-fail-fast