OUTPUT_FORMAT("elf64-x86-64", "elf64-x86-64", "elf64-x86-64")
OUTPUT_ARCH(i386:x86-64)
ENTRY(start32)

SECTIONS
{
	. = 0x100000;

	.text : {
		*(.text .text.* .gnu.linkonce.t.*)
	}

	PROVIDE(etext = .);

	.rodata : {
		*(.rodata .rodata.* .gnu.linkonce.r.*)
	}

	/* Unless with '-n' or '-N' option, x86_64-ld seems to align the program
		header section at 2M page boundary regardless the alignment directive
		without warning. It may not be desirable. */
	. = ALIGN(0x1000);

	.data : {
		*(.data .data.*)
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
