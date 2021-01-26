.PHONY: all
all:
	cargo build
	cbindgen -l C --output myrust.h
	gcc -Wall -o foo foo.c target/debug/libcbindgen_play.so
