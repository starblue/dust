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
         stmdb   r0!, {{r4-r11}}".
        out("r0") prev_psp,
    );
    let next_psp = dust_switch_context(prev_psp);
    asm!(
        "ldmia   r0!, {{r4-r11}}
         msr     psp, r0
         bx      lr",
        in("r0") next_psp,
        options(noreturn)
    );
}
