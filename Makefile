#!/usr/bin/make -f

cfiles := $(shell find $(CURDIR) -type f -name '*.c')
llfiles := $(shell find $(CURDIR) -type f -name '*.ll')
emits := $(patsubst $(CURDIR)/%,emit/%,$(cfiles))
execs := $(patsubst $(CURDIR)/%,exec/%,$(llfiles))
runs := $(patsubst $(CURDIR)/%,run/%,$(cfiles))

.PHONY: default
default:

.PHONY: $(emits)
$(emits): emit/%:
	clang -O0 -S -emit-llvm $* -o $(basename $*).ll

.PHONY: $(execs)
$(execs): exec/%:
	lli $*

.PHONY: $(runs)
$(runs): run/%:
	cargo run -- $* >$(basename $*).ll
	lli $(basename $*).ll
