build:
	@cargo build

run *args: build
	./target/debug/alex {{args}}
