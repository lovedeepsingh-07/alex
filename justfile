build:
	@cargo build

run *args: build
	@./target/debug/alex {{args}}

fmt:
	@alejandra .
	@cargo fmt
