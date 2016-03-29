.globl _start
_start:
    mov   sp, #0x8000
    bl    rust_main
hang:
    b     hang

.globl dummy
dummy:
    // just do some work to waste cycles
    push  {r4, r5, r6, r7, r8, lr}
    mov   r4, #0x0
    mov   r5, #0x1
    mov   r6, #0x2
    mov   r7, #0x3
    mov   r8, #0x4
    mov   lr, #0x5
    pop   {r4, r5, r6, r7, r8, pc}
