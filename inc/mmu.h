#ifndef RUMOS_INC_MMU_H
#define RUMOS_INC_MMU_H

//
// Intel manual IA32-3A reads:
//  The first descriptor in the GDT is not used by the processor.
//  A segment selector to this 'null descriptor' does not generate
//  an exception when loaded into a data segment register
//  (DS, ES, FS, or GS), but it always generates a general-protection
//  exception (#GP) when an attempt is made to access memory using
//  the descriptor.
//
#define SEG32_NULL  \
    .word 0, 0;     \
    .byte 0, 0, 0, 0

//
// General (32-bit) descriptor format per AMD's 80386 Programmer
// Reference Manual:
//  BASE:
//  LIMIT: can be interpreted as in units of one byte or 4K bytes
//          according to the granularity bit.
//  Granularity bit:
//  TYPE:
//  S: 0 = system descriptor, 1 = application descriptor
//  DPL:
//  Present bit:
//
//  31            23              15               7           0
// +-------------+-+-+-+-+-------+-+-----+-+----+-+-------------+
// |             | | | |A|       | |     | |    | |             |
// | BASE 31..24 |G|X|O|V| LIMIT |P| DPL |S|TYPE|A| BASE 23..16 | 4
// |             | | | |L| 19..16| |     | |    | |             |
// +-------------+-+-+-+-+-------+-+-----+-+----+-+-------------+
// |                             |                              |
// |            BASE 15..0       |           LIMIT15..0         | 0
// |                             |                              |
// +-----------------------------+------------------------------+
//
#define SEG32(type,base,limi)    \
    .word (((limi) >> 12) & 0xFFFF), ((base) & 0xFFFF);  \
    .byte (((base) >> 16) & 0xFF), (0x90 | (type)), \
        (0xC0 | (((limi) >> 28) & 0xF)), (((base) >> 24) & 0xFF)

//
// Intel IA32-3A table 3-1 lists the application descriptor types.
//
#define STA_X       0x8     // Executable segment
#define STA_E       0x4     // Expand down (non-executable segments)
#define STA_C       0x4     // Conforming code segment (executable only)
#define STA_W       0x2     // Writeable
#define STA_R       0x2     // Readable
#define STA_A       0x1     // Accessed

//
// And table 3-2 system descriptor.
//
#define STS_T16A    0x1     // Available 16-bit TSS
#define STS_LDT     0x2     // Local Descriptor Table
#define STS_T16B    0x3     // Busy 16-bit TSS
#define STS_CG16    0x4     // 16-bit Call Gate
#define STS_TG      0x5     // Task Gate
#define STS_IG16    0x6     // 16-bit Interrupt Gate
#define STS_TG16    0x7     // 16-bit Trap Gate
#define STS_T32A    0x9     // Available 32-bit TSS
#define STS_T32B    0xB     // Busy 32-bit TSS
#define STS_CG32    0xC     // 32-bit Call Gate
#define STS_IG32    0xE     // 32-bit Interrupt Gate
#define STS_TG32    0xF     // 32-bit Trap Gate

#define CR0_PE_ON       0x1
#define PROT_MODE_CSEG  0x8
#define PROT_MODE_DSEG  0x10

//
// AMD64 Architecture Programmer's Manual V2 3.1 System Control Registers
//
#define CR0_MP_ON           (1<<1)
#define CR0_EM_OFF          (~(1<<2))
#define CR0_PG_ON           (1<<31)
#define CR4_PAE_ON          (1<<5)
#define CR4_PGE_ON          (1<<7)
#define CR4_OSFXSR_ON       (1<<9)
#define CR4_OSXMMEXCPT_ON   (1<<10)
#define CR3_PML4_PWT        (1<<3)  // Write through
#define CR3_PML4_PCD        (1<<4)  // Cache disable
#define CR3_PML4_SHIFT      12
#define MSR_EFER            0xC0000080  // Extended Feature Enable Register
#define EFER_SCE_ON         0x1         // System Call Extensions
#define EFER_LME_ON         (1<<8)      // Long Mode Enable

#define PML4E_SHIFT     39
#define PDPE_SHIFT      30
#define PDE_SHIFT       21
#define PTE_SHIFT       12
// PML4 index of a linear address
#define PML4X(la)       (((((uintptr_t) la) & 0xFFFFFFFFFFFF) >> PML4E_SHIFT) & 0x1FF)
// PDP index of a linear address
#define PDPX(la)        (((((uintptr_t) la) & 0xFFFFFFFFFFFF) >> PDPE_SHIFT) & 0x1FF)
// PDE index of a linear address
#define PDEX(la)        (((((uintptr_t) la) & 0xFFFFFFFFFFFF) >> PDE_SHIFT) & 0x1FF)
// PTE  infex of a linear address
#define PTEX(la)        (((((uintptr_t) la) & 0xFFFFFFFFFFFF) >> PTE_SHIFT) & 0x1FF)
// Offset in the page
#define PG4K_OFF        (((uintptr_t) la) && 0x3FFFFF)
#define PG2M_OFF        (((uintptr_t) la) && 0x7FFFFFFF)

// Page table entry flags
#define PTE_P           0x001   // Present
#define PTE_W           0x002   // Writeable
#define PTE_U           0x004   // User
#define PTE_PWT         0x008
#define PTE_PCD         0x010
#define PTE_A           0x020   // Accessed
#define PTE_D           0x040   // Dirty
#define PDE_PS_ON       0x080   // on = 2M page, off = 4K page
#define PTE_G           0x100   // Global page

#define PG4K_SIZE       0x1000
#define PG4K_SHIFT      12
#define PG2M_SIZE       0x100000
#define PG2M_SHIFT      20

// Bootloader loads rumos kernel into here initially in 32-bit
// protected mode.
// Must match kern/kernel32.ld.
#define STAGE1_TEXT     0x100000
// The final physical address of the 64-bit kernel.
// Must match kern/kernel64.ld.
#define STAGE2_TEXT     0x200000

// The virtual address of the Rumos kernel, it is a higher half one.
#define KERN_TEXT       0xFFFFFFFFFB800000

#endif // RUMOS_INC_MMU_H
