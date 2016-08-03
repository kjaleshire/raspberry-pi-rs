.globl _start
_start:
    // maybe replace with branches, or mov?
    ldr pc, reset_handler
    ldr pc, undefined_handler
    ldr pc, hvc_handler
    ldr pc, prefetch_handler
    ldr pc, data_handler
    ldr pc, hyp_handler
    ldr pc, irq_handler
    ldr pc, fiq_handler

reset_handler:      .word reset
undefined_handler:  .word hang
hvc_handler:        .word hang
prefetch_handler:   .word hang
data_handler:       .word hang
hyp_handler:        .word hang
irq_handler:        .word irq
fiq_handler:        .word hang

reset:
    mov   r0, #0x8000
    mcr   p15, 4, r0, c12, c0, 0
    mov   sp, #0x8000
    bl    rust_main
hang:
    b     hang

.globl enable_irq
enable_irq:
    mrs   r0, cpsr
    bic   r0, r0, #0x80
    msr   cpsr_c, r0
    bx    lr

irq:
    push  {r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, lr}
    bl    rust_irq_handler
    pop   {r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, lr}
    eret
