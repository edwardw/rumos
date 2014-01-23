#include <inc/mmu.h>
#define MB_HEADER_MAGIC     0x1BADB002
#define MB_HEADER_FLAGS     0x00010003
#define MB_HEADER_CHECKSUM  -(MB_HEADER_MAGIC + MB_HEADER_FLAGS)

.global start32
    .code32
start32:
    jmp     code_start

    /*
     * The multiboot header
     */
    .align  4
    .global mb_header
mb_header:
    .long   MB_HEADER_MAGIC
    .long   MB_HEADER_FLAGS
    .long   MB_HEADER_CHECKSUM
    .long   mb_header   # header_addr
    .long   0x100000    # load_addr
    .long   0x0         # load_end_addr, 0 = entire file
    .long   0x0         # bss_end_addr
    .long   start32     # entry_addr
    .long   0x0         # video mode - don't care
    .long   0x0         # width
    .long   0x0         # height
    .long   0x0         # depth

code_start:
    movl    %ebx, mb_info    # the multiboot info grub passed in

    movl    %cr4, %eax
    orl     $(CR4_PAE_ON+CR4_PGE_ON), %eax
    movl    %eax, %cr4

    # CR3 points to PML4
    movl    $entry_pml4, %eax
    movl    %eax, %cr3

    # Configure Extended Feature Enable MSR
    movl    $(MSR_EFER), %ecx
    rdmsr
    orl     $(EFER_LME_ON+EFER_SCE_ON), %eax    # set long mode and system call extensions
    wrmsr

    # Now enable SSE instructions
    movl    %cr0, %eax
    andl    $(CR0_EM_OFF), %eax # clear coprocessor emulation CR0.EM
    orl     $(CR0_MP_ON), %eax  # set coprocessor monitoring  CR0.MP
    movl    %eax, %cr0
    movl    %cr4, %eax
    orl     $(CR4_OSFXSR_ON+CR4_OSXMMEXCPT_ON), %eax # set CR4.OSFXSR and CR4.OSXMMEXCPT
    movl    %eax, %cr4

    # Enable paging then long mode
    movl    %cr0, %eax
    orl     $CR0_PG_ON, %eax
    movl    %eax, %cr0

    lgdt    gdt64_desc

    pushl   $PROT_MODE_CSEG
    pushl   $longmode
    lret

    .code64
longmode:
    call    start64

    # Spin if returns (it shouldn't).
spin:
    jmp     spin

.data
    .align 16
# Long mode bootstrap GDT
gdt64:
    .quad 0x0000000000000000    # null entry
    .quad 0x0020980000000000    # code segment
    .quad 0x0000900000000000    # data segment
gdt64_end:
gdt64_desc:
    .word gdt64_end - gdt64 - 1
    .long gdt64

    .p2align PG4K_SHIFT
# The bootstrap memory map, which maps
#   [0, 4M) -> [0, 4M)
#   [KERN_TEXT, KERN_TEXT+4M) -> [2M, 6M)
entry_pml4:
    .quad   entry_pdp+PTE_W+PTE_P
    .space  PG4K_SIZE-16
    .quad   entry_pdp+PTE_W+PTE_P
entry_pdp:
    .quad   entry_pd+PTE_W+PTE_P
    .space  PG4K_SIZE-16
    .quad   entry_pd+PTE_W+PTE_P
entry_pd:
    .quad   0x0+PTE_P+PTE_W+PDE_PS_ON
    .quad   0x200000+PTE_P+PTE_W+PDE_PS_ON
    .space  (476-2)*8
    .quad   0x200000+PTE_P+PTE_W+PDE_PS_ON
    .quad   0x400000+PTE_P+PTE_W+PDE_PS_ON
    .space  34*8
.global mb_info
mb_info:
    .long   0x0
