#!/usr/bin/make -f

cfiles := $(shell find $(CURDIR) -type f -name '*.c')
llfiles := $(shell find $(CURDIR) -type f -name '*.ll')
emits := $(patsubst $(CURDIR)/%,emit/%,$(cfiles))
execs := $(patsubst $(CURDIR)/%,exec/%,$(llfiles))

.PHONY: default
default:

$(emits): emit/%:
	clang -O0 -S -emit-llvm $* -o $(basename $*).ll

$(execs): exec/%:
	lli $*
