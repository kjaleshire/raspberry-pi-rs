.globl _start
_start:
    mov   sp, #0x8000
    bl    rust_main
hang:
    b     hang
