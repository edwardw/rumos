#!/usr/bin/env python
import sys, mmap
from ctypes import *

class MbHdr(Structure):
    _fields_ = [("magic",       c_uint),
                ("flags",       c_uint),
                ("checksum",    c_uint),
                ("header_addr", c_uint),
                ("load_addr",   c_uint),
                ("load_end_addr",   c_uint),
                ("bss_end_addr",    c_uint),
                ("entry_addr",  c_uint)]

class Elf64(Structure):
    _fields_ = [("e_magic",     c_uint),
                ("e_elf",       c_byte * 12),
                ("e_type",      c_ushort),
                ("e_machine",   c_ushort),
                ("e_version",   c_uint),
                ("e_entry",     c_ulonglong),
                ("e_phoff",     c_ulonglong),
                ("e_shoff",     c_ulonglong),
                ("e_flags",     c_uint),
                ("e_ehsize",    c_ushort),
                ("e_phentsize", c_ushort),
                ("e_phnum",     c_ushort),
                ("e_shentsize", c_ushort),
                ("e_shnum",     c_ushort),
                ("shstrndx",    c_ushort)]

class Proghdr64(Structure):
    _fields_ = [("p_type",      c_uint),
                ("p_flags",     c_uint),
                ("p_offset",    c_ulonglong),
                ("p_va",        c_ulonglong),
                ("p_pa",        c_ulonglong),
                ("p_filesz",    c_ulonglong),
                ("p_memsz",     c_ulonglong),
                ("p_align",     c_ulonglong)]

if (len(sys.argv) != 2):
    print "Usage: mbh_patch.py <file_name>"
    exit(1)
else:
    with open(sys.argv[1], 'a+b') as f:
        # Only map the first 8K since multiboot header must be contained
        # in the first 8K of the file.
        mm = mmap.mmap(f.fileno(), 8192)
        elfhdr = Elf64.from_buffer(mm)

        # We expect an elf64 file
        if (elfhdr.e_magic != 0x464c457f):
            print "%s: not an elf file" % sys.argv[1]
            exit(1)
        if (elfhdr.e_elf[0] != 2):
            print "%s: not an elf64 file" % sys.argv[1]
            exit(1)

        # Locate the multiboot header
        for mbh_start in xrange(0, 8192-sizeof(MbHdr)+4, 4):
            mbh = MbHdr.from_buffer(mm, mbh_start)
            if (mbh.magic == 0x1badb002):
                break
        if (mbh_start >= 8192-sizeof(MbHdr)):
            print "%s: can't find multiboot header" % sys.argv[1]
            exit(1)

        # Vecrify some elf64 basics
        if (elfhdr.e_type != 2):
            print "%s: expect type to be ET_EXEC, got %d" % (sys.argv[1], elfhdr.e_type)
            exit(1)
        if (elfhdr.e_phnum == 0 or elfhdr.e_phoff == 0):
            print "%s: no program header" % sys.argv[1]
            exit(1)

        # Now patch the elf64 file
        for n in xrange(elfhdr.e_phnum):
            ph = Proghdr64.from_buffer(mm, elfhdr.e_phoff + n * elfhdr.e_phentsize)
            # 1 = PT_LOAD
            if (ph.p_type != 1 or ph.p_memsz == 0 or ph.p_va != ph.p_pa):
                continue
            if (mbh_start < ph.p_offset or mbh_start >= (ph.p_offset + ph.p_filesz)):
                print "%s: 1:1 mapped PT_LOAD section doesn't come first" % sys.argv[1]
                exit(1)
            if (elfhdr.e_entry != ph.p_pa):
                print "%s: entry point != physical addr" % sys.argv[1]
                exit(1)
            mbh.load_addr = ph.p_pa - ph.p_offset
            mbh.entry_addr = ph.p_pa
            mbh.header_addr = mbh.load_addr + mbh_start
            print "multiboot header patched."
            break

        mm.close()
