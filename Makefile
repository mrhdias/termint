# perl -pi -e 's/^  */\t/' Makefile
BINARY_NAME=termint

all: clean build build_small

build:
	cargo build --release
	cp target/release/${BINARY_NAME} ./${BINARY_NAME}

build_small:
	upx ${BINARY_NAME}

run: ./${BINARY_NAME}

build_and_run: build run

clean:
	go clean
	rm -f ${BINARY_NAME}