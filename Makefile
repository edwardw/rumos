OBJDIR := obj
TOP := $(PWD)

TARGET := x86_64-pc-netbsd
CC := $(TARGET)-gcc
LD := $(TARGET)-ld
AR := $(TARGET)-ar
OBJCOPY := $(TARGET)-objcopy
RUSTC := rustc

CFLAGS := $(CFLAGS) -O1 -I$(TOP)

RUSTFLAGS := -O --target x86_64-linux-gnu --save-temps -Z no-landing-pads
RUSTFLAGS += -L $(OBJDIR)/arch -L $(OBJDIR)/rust-std -L $(OBJDIR)/rust-extra

LDFLAGS :=

# Lists that the */Makefrag makefile fragments will add to
OBJDIRS :=

# List of library names
LIB_MORESTACK := libmorestack.a
RLIB_STD := libstd-64b22b00-0.0.rlib
RLIB_EXTRA := libextra-a68a2dc1-0.0.rlib
RLIB_ARCH := libarch-5a75d89e-0.0.rlib
LIB_KERN64 := libkern64-cfc1451f-0.0.a

# Recursive wildcard function
# http://blog.jgc.org/2011/07/gnu-make-recursive-wildcard-function.html
rwildcard=$(foreach d,$(wildcard $1*),$(call rwildcard,$d/,$2) \
  $(filter $(subst *,%,$2),$d))

# All must be the first target
all:

# Eliminate default suffix rules
.SUFFIXES:

# Delete target files if there is an error (or make is interrupted)
.DELETE_ON_ERROR:

# try to generate a unique GDB port
GDBPORT	:= $(shell expr `id -u` % 5000 + 25000)

QEMU ?= qemu-system-x86_64
QEMUOPTS := -cdrom $(OBJDIR)/kern/rumos.iso -serial mon:stdio -gdb tcp::$(GDBPORT)
CPUS ?= 1
QEMUOPTS += -m 512 -smp $(CPUS)
ISO := $(OBJDIR)/kern/rumos.iso

include arch/Makefrag
include kern/Makefrag

.gdbinit: .gdbinit.tmpl
	sed "s/localhost:1234/localhost:$(GDBPORT)/" < $^ > $@

pre-qemu: .gdbinit

qemu: $(ISO) pre-qemu
	$(QEMU) $(QEMUOPTS)

qemu-gdb: $(ISO) pre-qemu
	@echo "***"
	@echo "*** Now run 'gdb'." 1>&2
	@echo "***"
	$(QEMU) $(QEMUOPTS) -S

VBOX_DISK := $(OBJDIR)/kern/disk.vdi
vbox: $(ISO)
	@if [[ -f $(VBOX_DISK) ]]; then \
		VBoxManage unregistervm rumos --delete; \
		rm -f $(VBOX_DISK); \
	fi
	@VBoxManage convertfromraw $(ISO) $(VBOX_DISK)
	@VBoxManage createvm --name rumos --register
	@VBoxManage modifyvm rumos --acpi on --ioapic on --cpus 2 --memory 512 --boot1 disk --boot2 none
	@VBoxManage storagectl rumos --name "SATA Controller" --add sata
	@VBoxManage storageattach rumos --storagectl "SATA Controller" --port 0 --type hdd --medium $(VBOX_DISK)
	VBoxManage startvm rumos

clean:
	rm -rf $(OBJDIR)/rust-extra
	rm -rf $(OBJDIR)/boot
	rm -rf $(OBJDIR)/arch
	rm -rf $(OBJDIR)/kern

STD_SRCS := $(call rwildcard,rust-std/core/,*.rs)
$(OBJDIR)/rust-std/$(RLIB_STD): $(STD_SRCS)
	@mkdir -p $(@D)
	$(RUSTC) $(RUSTFLAGS) --cfg libc --out-dir $(@D) rust-std/core/lib.rs

$(OBJDIR)/rust-extra/$(RLIB_EXTRA): $(OBJDIR)/rust-std/$(RLIB_STD) $(call rwildcard,rust-extra/,*.rs)
	@mkdir -p $(@D)
	$(RUSTC) $(RUSTFLAGS) --out-dir $(@D) rust-extra/mod.rs

$(OBJDIR)/.deps:
	@mkdir -p $(@D)
	@touch $@

-include $(OBJDIR)/.deps

.PHONY: all
