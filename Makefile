#!/usr/bin/make -f

cfiles := $(shell find $(CURDIR) -type f -name '*.c')
emits := $(patsubst $(CURDIR)/%,emit/%,$(cfiles))

.PHONY: default
default:

$(emits): emit/%:
	clang -O0 -S -emit-llvm $* -o $(basename $*).ll
