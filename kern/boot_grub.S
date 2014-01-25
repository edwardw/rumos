#include <inc/mmu.h>
#define MB_HEADER_MAGIC     0x1BADB002
#define MB_HEADER_FLAGS     0x00010003
#define MB_HEADER_CHECKSUM  -(MB_HEADER_MAGIC + MB_HEADER_FLAGS)

.global start32
start32:
    .code32
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
    .long   0x11111111  # header_addr, patched by mbh_patch.py
    .long   0x22222222  # load_addr, patched by mbh_patch.py
    .long   0x0         # load_end_addr, 0 = entire file
    .long   0x0         # bss_end_addr
    .long   0x33333333  # entry_addr, patched by mbh_patch.py
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

longmode:
    .code64

    /*
     * Relocate the section that is PT_LOAD and not 1:1 mapped
     *
     * Structures and offsets
     * Multiboot header
     *      magic           0x0
     *      flags           0x4
     *      checksum        0x8
     *      header_addr     0xC
     *      load_addr       0x10
     *
     * Elf64
     *      e_magic         0x0
     *      e_elf           0x4
     *      e_type          0x10
     *      e_machine       0x12
     *      e_version       0x14
     *      e_entry         0x18
     *      e_phoff         0x20
     *      e_shoff         0x28
     *      e_flags         0x30
     *      e_ehsize        0x34
     *      e_phentsize     0x36
     *      e_phnum         0x38
     *
     * Program Header 64
     *      p_type          0x0
     *      p_flags         0x4
     *      p_offset        0x8
     *      p_va            0x10
     *      p_pa            0x18
     *      p_filesz        0x20
     */
    xorq    %rax, %rax
    movq    %rax, %rdx
    movq    %rax, %rbp
    movl    $mb_header, %edx
    movl    0x10(%edx), %eax    # offset 0x10 = load_addr, also where elf64 header begins
    movq    0x20(%eax), %rbx    # offset 0x20 = e_phoff
    addl    %eax, %ebx          # load_addr + e_phoff = the first program header
    movw    0x36(%eax), %bp     # offset 0x36 = e_phentsize
    movw    0x38(%eax), %dx     # offset 0x38 = e_phnum
relocate_sec:
    cmpl    $0x1, (%ebx)        # p_type == PT_LOAD?
    jne     next_proghdr        # skip if not
    movq    0x10(%ebx), %rsi    # p_va
    movq    0x18(%ebx), %rdi    # p_pa
    cmpq    %rsi, %rdi          # p_va == p_pa?
    je      next_proghdr        # skip if yes
    movq    0x20(%ebx), %rcx    # the count
    movq    0x8(%ebx), %rsi     # p_offset
    addq    %rax, %rsi          # load_addr + p_offset makes it the address of loaded section
    /*
     * XXX - here we assume no overlapping between copying-from and copying-to memory area,
     * so the copy can be done from the beginning.
     */
    rep movsb                   # do the copy: %rsi -> %rdi, %rcx bytes
next_proghdr:
    addl    %ebp, %ebx
    subw    $0x1, %dx
    jnz     relocate_sec

    movq    $bootstacktop, %rsp
    xorq    %rax, %rax
    movq    %rax, %rbp
    movq    %rax, %fs:0x70
    /*
     * The x86-64 ABI is different than i386's
     * According to http://www.x86-64.org/documentation/abi.pdf, the integer
     * parameters are passed in registers if available, not on stack, in the
     * sequence of:
     *       %rdi, %rsi, %rdx, %rcx, %r8 and %r9
     */
    xorq    %rdi, %rdi
    movl    $mb_info, %ebx
    movl    (%ebx), %edi    # argv0 for the init
    call    init

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
    .quad gdt64
mb_info:
    .long 0x0

# The bootstrap memory map, which maps
#   [0, 4M) -> [0, 4M)
#   [KERN_TEXT, KERN_TEXT+4M) -> [2M, 6M)
    .p2align PG4K_SHIFT
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
