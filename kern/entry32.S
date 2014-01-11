#include <inc/mmu.h>

.global start32
    .code32
start32:
    movl    %cr4, %eax
    orl     $(CR4_PAE_ON+CR4_PGE_ON), %eax
    movl    %eax, %cr4

    # CR3 points to PML4
    movl    $entry_pml4, %eax
    movl    %eax, %cr3

    # Enable long mode and SYSCALL/SYSRET
    movl    $0xC0000080, %ecx
    rdmsr
    orl     $0x00000101, %eax
    wrmsr

    # Enable paging then long mode
    movl    %cr0, %eax
    orl     $CR0_PG_ON, %eax
    movl    %eax, %cr0

    lgdt    gdt64_desc

    ljmp    $PROT_MODE_CSEG, $start64

    .code64
start64:
    # Set up the stack and call into rust
    movq    $bootstacktop, %rsp
    movq    $0, %rbp
    movq    $0, %fs:0x70
    # The x86-64 ABI is different than i386's
    # According to http://www.x86-64.org/documentation/abi.pdf, the integer
    # parameters are passed in registers if available, not on stack, in the
    # sequence of:
    #       %rdi, %rsi, %rdx, %rcx, %r8 and %r9
    movq    $65, %rdi       # argv0, stage2 kernel starts at sector 65, must match kern/Makefrag
    movq    $0x10000, %rsi  # argv1, the scratch space
    call    bootmain
    # Convert to the physical address
    sub     $(KERN_TEXT-0xFFFFFFFF00000000-STAGE2_TEXT), %eax
    call    *%rax

    # Spin if returns (it shouldn't).
spin:
    jmp     spin

.global __morestack
__morestack:
    ret

    .p2align 4
.data
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


bootstack:
    .space  PG4K_SIZE
bootstacktop:
