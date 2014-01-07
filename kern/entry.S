#include <inc/mmu.h>

    .p2align 4
.text
.global start32
    .code32
start32:
#    movl    $(PTE_P+PTE_W), %eax
#    movl    $entry_pt_1, %ecx
#pt_loop_1:
#    movl    %eax, (%ecx)
#    addl    $0x1000, %eax
#    addl    $0x8, %ecx
#    cmp     $0x200000, %eax
#    jb      pt_loop_1
#pt_loop_2:
#    movl    %eax, (%ecx)
#    addl    $0x1000, %eax
#    addl    $0x8, %ecx
#    cmp     $0x400000, %eax
#    jb      pt_loop_2

    # Enable extended properties
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
    movq    $start32, %rsp
    movq    $0, %fs:0x70
    call    init

    # Spin if init returns (it shouldn't).
spin:
    jmp     spin

.global __morestack
__morestack:
    ret

.data

# Long mode bootstrap GDT
.p2align 4
gdt64:
    .quad 0x0000000000000000    # null entry
    .quad 0x0020980000000000    # code segment
    .quad 0x0000900000000000    # data segment
gdt64_end:
gdt64_desc:
    .word gdt64_end - gdt64 - 1
    .long gdt64

# The bootstrap memory map, which maps
#   [0, 4M) -> [0, 4M)
.p2align PG4K_SHIFT
.global entry_pml4
entry_pml4:
    .quad   entry_pdp+PTE_W+PTE_P
    .space  PG4K_SIZE-8
.global entry_pdp
entry_pdp:
    .quad   entry_pd+PTE_W+PTE_P
    .space  PG4K_SIZE-8
.global entry_pd
entry_pd:
    .quad   0x0+PTE_P+PTE_W+PDE_PS_ON
    .quad   0x200000+PTE_P+PTE_W+PDE_PS_ON
    #.quad   entry_pt_1+PTE_P+PTE_W
    #.quad   entry_pt_2+PTE_P+PTE_W
    .space  PG4K_SIZE-16
#entry_pt_1:
#    .space  PG4K_SIZE
#entry_pt_2:
#    .space  PG4K_SIZE
