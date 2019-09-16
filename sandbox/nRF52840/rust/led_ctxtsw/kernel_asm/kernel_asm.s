# LLD requires that the section flags are explicitly set here
.section .KernelAsm, "ax"
.global PendSV

# .type and .thumb_func are both required; otherwise its Thumb bit does not
# get set and an invalid vector table is generated
.type PendSV,%function
.thumb_func

PendSV:
    push    {r4, r5, r6, r7}
    mov     r4, r8
    mov     r5, r9
    mov     r6, r10
    mov     r7, r11
    push    {r4, r5, r6, r7}

    mov     r4, lr

    mov     r0, sp
    bl      task_switch
    mov     sp, r0

    mov     lr, r4
    
    pop     {r4, r5, r6, r7}
    mov     r8, r4
    mov     r9, r5
    mov     r10, r6
    mov     r11, r7
    pop     {r4, r5, r6, r7}

    bx      lr
