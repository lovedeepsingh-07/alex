BINARY_NAME := "alladin"
BUILD_DIR := "dist"

default:
	just -l

run:
	go run ./examples/base/main.go

build:
	mkdir -p $BUILD_DIR
	go build -o $BUILD_DIR/$BINARY_NAME ./examples/base/main.go

clean:
	rm -rf $BUILD_DIR
