THIS_FILE := $(lastword $(MAKEFILE_LIST))

all:

test-watch:
	cargo watch -x "test --all"

changelog:
	@git-cliff -o CHANGELOG.md

build: build_linux build_win

build_linux:
	@cargo build --release

build_win:
	@cargo build --release --target x86_64-pc-windows-gnu

