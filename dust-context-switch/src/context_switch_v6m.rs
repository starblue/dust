use crate::dust_switch_context;

#[naked]
#[no_mangle]
pub unsafe extern "C" fn pendsv_handler() {
    // Saved process stack layout
    //
    //   64    <- PSP just before exception
    //
    // Offset  Saved by hardware on exception entry
    //   60    xPSR
    //   56    PC
    //   52    LR
    //   48    R12
    //   44    R3
    //   40    R2
    //   36    R1
    //   32    R0 <- PSP on entry to this function
    //
    // Offset  Saved here on context switch
    //   28    R11
    //   24    R10
    //   20    R9
    //   16    R8
    //   12    R7
    //    8    R6
    //    4    R5
    //    0    R4 <- saved PSP when task is not active
    //
    let prev_psp: u32;
    asm!(
        "mrs     r0, psp
         subs    r0, #32
         stmia   r0!, {{r4-r7}}
         mov     r4, r8
         mov     r5, r9
         mov     r6, r10
         mov     r7, r11
         stmia   r0!, {{r4-r7}}
         subs    r0, #32",
        out("r0") prev_psp,
    );
    let next_psp = dust_switch_context(prev_psp);
    asm!(
        "adds    r0, #16
         ldmia   r0!, {{r4-r7}}
         mov     r8, r4
         mov     r9, r5
         mov     r10, r6
         mov     r11, r7
         msr     psp, r0
         subs    r0, #32
         ldmia   r0!, {{r4-r7}}
         bx      lr",
        in("r0") next_psp,
        options(noreturn)
    );
}
