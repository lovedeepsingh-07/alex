build:
	@cargo build

run *args: build
	@./target/debug/alex {{args}}

lint:
	@cargo clippy -- \
		--allow clippy::needless_return \
		--allow clippy::uninlined_format_args

fmt:
	@alejandra .
	@cargo fmt

test:
	cargo test -- --no-capture
