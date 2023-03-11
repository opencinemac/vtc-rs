.PHONY: format
format:
	-cargo fmt

.PHONY: lint
lint:
	-cargo clippy -- --deny warnings
	-rustfmt ./src/*.rs --check
	-find . -type f | grep -e \.rs$ | grep -v /target | xargs misspell -error

.PHONY: test
test:
	-cargo test --color=always --no-fail-fast

.PHONY: doc
doc:
	-cargo doc --lib --open

# Installs command line tools for development
.PHONY: install-tools
install-tools:
	-go install github.com/client9/misspell/cmd/misspell@latest