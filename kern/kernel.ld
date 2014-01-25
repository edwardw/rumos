OUTPUT_FORMAT("elf64-x86-64", "elf64-x86-64", "elf64-x86-64")
OUTPUT_ARCH(i386:x86-64)
ENTRY(start32)

SECTIONS
{
	/*
	 * We need the elf file header to relocate the code which are not 1:1 mapped.
	 * That's where mbh_patch.py comes in to trick grub2 loading the whole file.
	 * If the code starts at 0x100000, then the elf header will be loaded into
	 * memory of 0x1000 bytes before that. But grub2 seems to use that part of the
	 * memory and the file header will be trashed if being loaded there. So offset
	 * 0x1000.
	 */
	. = 0x101000;

	.boot : {
		obj/kern/boot_grub.o(*)
	}

	. = 0xFFFFFFFFFB800000;

	/*
	 * Unless with '-n' or '-N' option, x86_64-ld seems to align the program
	 * header section at 2M page boundary regardless the alignment directive
	 * without warning. It may not be desirable.
	 */
	.text ALIGN(0x1000) : AT(0x200000) {
		*(.text .text.* .gnu.linkonce.t.*)
	}

	PROVIDE(etext = .);

	.rodata : {
		*(.rodata .rodata.* .gnu.linkonce.r.*)
	}

	.data ALIGN(0x1000) : {
		*(.data .data.* .got .got.*)
	}

	PROVIDE(edata = .);

	.bss : {
		*(.bss)
	}

	PROVIDE(end = .);

	/DISCARD/ : {
		*(.eh_frame)
	}
}
