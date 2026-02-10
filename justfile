build:
	@cargo build

run *args: build
	@./target/debug/alex {{args}}

gen:
	@flatc -o ./src/flatbuffers_gen/ ./data/request_packet.fbs ./data/response_packet.fbs --rust --filename-suffix _

fmt:
	@alejandra .
	@cargo fmt
