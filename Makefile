OBJDIR := obj
TOP := .

TARGET := x86_64-pc-netbsd
CC := $(TARGET)-gcc
LD := $(TARGET)-ld
AR := $(TARGET)-ar
OBJCOPY := $(TARGET)-objcopy
RUSTC := rustc

CFLAGS := $(CFLAGS) -O1 -I$(TOP)

RUSTFLAGS := -O --target x86_64-linux-gnu --save-temps
RUSTFLAGS += -L $(OBJDIR)/arch -L $(OBJDIR)/rust-core -L $(OBJDIR)/rust-std

LDFLAGS :=

# Lists that the */Makefrag makefile fragments will add to
OBJDIRS :=

# List of library names
LIB_MORESTACK := libmorestack.a
RLIB_CORE := libcore-2e829c2f-0.0.rlib
RLIB_STD := libstd-64b22b00-0.0.rlib
RLIB_ARCH := libarch-5a75d89e-0.0.rlib
LIB_KERN64 := libkern64-cfc1451f-0.0.a

# All must be the first target
all:

# Eliminate default suffix rules
.SUFFIXES:

# Delete target files if there is an error (or make is interrupted)
.DELETE_ON_ERROR:

include boot/Makefrag
include kern/Makefrag
include arch/Makefrag

# try to generate a unique GDB port
GDBPORT	:= $(shell expr `id -u` % 5000 + 25000)

QEMU ?= qemu-system-x86_64
QEMUOPTS := -hda $(OBJDIR)/kern/kernel.img -serial mon:stdio -gdb tcp::$(GDBPORT)
CPUS ?= 1
QEMUOPTS += -m 512 -smp $(CPUS)
IMAGES := $(OBJDIR)/kern/kernel.img

.gdbinit: .gdbinit.tmpl
	sed "s/localhost:1234/localhost:$(GDBPORT)/" < $^ > $@

pre-qemu: .gdbinit

qemu: $(IMAGES) pre-qemu
	$(QEMU) $(QEMUOPTS)

qemu-gdb: $(IMAGES) pre-qemu
	@echo "***"
	@echo "*** Now run 'gdb'." 1>&2
	@echo "***"
	$(QEMU) $(QEMUOPTS) -S

clean:
	rm -rf $(OBJDIR)

$(OBJDIR)/rust-core/$(RLIB_CORE): $(wildcard rust-core/core/*.rs)
	@mkdir -p $(@D)
	$(RUSTC) $(RUSTFLAGS) --out-dir $(@D) rust-core/core/lib.rs

$(OBJDIR)/rust-std/$(RLIB_STD): $(wildcard rust-std/*.rs)
	@mkdir -p $(@D)
	$(RUSTC) $(RUSTFLAGS) --out-dir $(@D) rust-std/mod.rs

$(OBJDIR)/.deps:
	@mkdir -p $(@D)
	@touch $@

-include $(OBJDIR)/.deps

.PHONY: all
