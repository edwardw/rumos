OBJDIR := obj
TOP := .

CC := x86_64-pc-netbsd-gcc
RUSTC := rustc
LD := x86_64-pc-netbsd-ld
OBJCOPY := x86_64-pc-netbsd-objcopy

CFLAGS := $(CFLAGS) -O1 -I$(TOP)

RUSTFLAGS := -O --target x86_64-pc-linux

LDFLAGS := -m elf_x86_64

# Lists that the */Makefrag makefile fragments will add to
OBJDIRS :=

# All must be the first target
all:

# Eliminate default suffix rules
.SUFFIXES:

# Delete target files if there is an error (or make is interrupted)
.DELETE_ON_ERROR:

include boot/Makefrag
include kern/Makefrag

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

$(OBJDIR)/.deps:
	@mkdir -p $(@D)
	@touch $@

-include $(OBJDIR)/.deps

.PHONY: all
