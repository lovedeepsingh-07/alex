build:
	@cargo build

run *args: build
	@./target/debug/alex {{args}}

gen:
	@mkdir -p ./src/generated
	@flatc -o ./src/generated ./data/request_packet.fbs --rust --filename-suffix _
	@flatc -o ./src/generated ./data/response_packet.fbs --rust --filename-suffix _

fmt:
	@alejandra .
	@cargo fmt
