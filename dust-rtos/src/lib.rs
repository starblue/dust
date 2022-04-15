#![feature(naked_functions)]
#![feature(const_in_array_repeat_expressions)]
#![no_std]

use core::num::Wrapping;
use core::ptr::read_volatile;
use core::ptr::write_volatile;

use dust_cortex_m::scb::SCB_CCR_STKALIGN;
use dust_cortex_m::SCB;
use dust_register::Write;

const STACK_FRAME_PC_OFFSET: isize = 14;
const STACK_FRAME_XPSR_OFFSET: isize = 15;

const XPSR_THUMB: u32 = 1 << 24;

type OsTime = Wrapping<i32>;

#[derive(PartialEq)]
enum TaskState {
    Runnable,
    SleepingUntil(OsTime),
}

struct IntervalTimer {
    interval: OsTime,
    last_wake_time: OsTime,
}

struct Task {
    state: TaskState,
    psp: u32,
    stack_bottom: u32,
    stack_top: u32,
    interval_timer: Option<IntervalTimer>,
}
impl Task {
    pub fn new(f: fn(), stack: &mut [u64]) -> Task {
        let stack_bottom = (&stack[0] as *const u64) as u32;
        let stack_top = stack_bottom + (stack.len() * 8) as u32;
        let psp = stack_top - 16 * 4;
        unsafe {
            // put an initial stack frame on the stack
            let fp = psp as *mut u32;
            write_volatile(fp.offset(STACK_FRAME_PC_OFFSET), f as u32);
            write_volatile(fp.offset(STACK_FRAME_XPSR_OFFSET), XPSR_THUMB);
        }
        let state = TaskState::Runnable;
        let interval_timer = None;
        Task {
            state,
            psp,
            stack_bottom,
            stack_top,
            interval_timer,
        }
    }
}

struct Scheduler<'a> {
    current_task_index: usize,
    tasks: &'a mut [Task],
    time: OsTime,
    next_wake_time: Option<OsTime>,
}
impl<'a> Scheduler<'a> {
    fn new(tasks: &mut [Task]) -> Scheduler {
        Scheduler {
            current_task_index: 0,
            tasks,
            time: Wrapping(0),
            next_wake_time: None,
        }
    }
    fn set_task(&mut self, i: usize, task: Task) {
        if i < self.tasks.len() {
            self.tasks[i] = Some(task);
        }
    }
    fn set_interval(&mut self, interval: OsTime) {
        let last_wake_time = self.get_time();
        if let Some(current_task) = &mut self.tasks[self.current_task_index] {
            current_task.interval_timer = Some(IntervalTimer {
                interval,
                last_wake_time,
            });
        }
    }
    fn wait_for_next_interval(&mut self) {
        if let Some(current_task) = &mut self.tasks[self.current_task_index] {
            if let Some(it) = &mut current_task.interval_timer {
                let next_wake_time = it.last_wake_time + it.interval;
                it.last_wake_time = next_wake_time;
                current_task.state = TaskState::SleepingUntil(next_wake_time);
                self.trigger_scheduler();
            }
        }
    }
    fn sleep(&mut self, d: OsTime) {
        if let Some(current_task) = &mut self.tasks[self.current_task_index] {
            current_task.state = TaskState::SleepingUntil(self.time + d);
            self.trigger_scheduler();
        }
    }
    fn context_switch(&mut self, current_psp: u32) -> u32 {
        if let Some(task) = &mut self.tasks[self.current_task_index] {
            task.psp = current_psp;
        } else {
            // the current task vanished (should never happen)
        }
        loop {
            let time = self.get_time();
            self.next_wake_time = None;
            for i in 0..self.tasks.len() {
                // Walk through tasks in priority order.
                // Save earliest wakeup time of higher priority tasks
                // until runnable task found.
                // Lower priority tasks should be disregarded.
                if let Some(task) = &mut self.tasks[i] {
                    match task.state {
                        TaskState::Runnable => {
                            self.current_task_index = i;
                            return task.psp;
                        }
                        TaskState::SleepingUntil(wake_time) => {
                            if wake_time - time <= Wrapping(0) {
                                // wake up
                                // TODO check for late wake up?
                                task.state = TaskState::Runnable;
                                self.current_task_index = i;
                                return task.psp;
                            } else {
                                if let Some(next_wake_time) = self.next_wake_time {
                                    if wake_time - next_wake_time < Wrapping(0) {
                                        // earlier than existing wake time
                                        self.next_wake_time = Some(wake_time);
                                    }
                                } else {
                                    // no wake time yet
                                    self.next_wake_time = Some(wake_time);
                                }
                            }
                        }
                    }
                }
            }
            // no runnable task
            // TODO sleep?
        }
    }
    fn tick(&mut self) {
        let time = self.get_time();
        unsafe {
            write_volatile(&mut self.time, time + Wrapping(1));
        }
        if Some(time) == self.next_wake_time {
            self.trigger_scheduler();
        }
    }
    fn trigger_scheduler(&mut self) {
        let scb = unsafe { &mut *SCB };
        scb.set_pending_pendsv();
    }
    fn get_time(&self) -> OsTime {
        unsafe { read_volatile(&self.time) }
    }
}
static mut SCHEDULER: Scheduler = Scheduler::new();
/// SVCall handler.
///
/// This implementation uses an assembly wrapper to save lr, msp and psp
/// and then jumps to svcall_handler_rust to do the main work.
#[no_mangle]
#[naked]
pub unsafe extern "C" fn svcall_handler() {
    asm!(
        "mov r0, lr
         mrs r1, msp
         mrs r2, psp
         b svcall_handler_rust",
        options(noreturn),
    )
}

#[repr(C)]
struct StackFrame {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r12: u32,
    lr: u32,
    pc: u32,
    xpsr: u32,
}

#[no_mangle]
unsafe extern "C" fn svcall_handler_rust(lr: u32, msp: u32, psp: u32) {
    let sp = if (lr & (1 << 2)) == 0 { msp } else { psp };
    let stack_frame = &*(sp as *const StackFrame);
    // TODO implement svc handler
}

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
    asm!(
        "mrs     r0, psp
         subs    r0, #32
         stmia   r0!, {{r4-r7}}
         mov     r4, r8
         mov     r5, r9
         mov     r6, r10
         mov     r7, r11
         stmia   r0!, {{r4-r7}}
         subs    r0, #32
         bl      dust_switch_context
         adds    r0, #16
         ldmia   r0!, {{r4-r7}}
         mov     r8, r4
         mov     r9, r5
         mov     r10, r6
         mov     r11, r7
         msr     psp, r0
         subs    r0, #32
         ldmia   r0!, {{r4-r7}}
         bx      lr",
        options(noreturn)
    );
}

pub fn init() {
    unsafe {
        // set stack alignment on exception entry to 8 bytes
        let scb = &mut *SCB;
        scb.ccr.modify(|w| w | SCB_CCR_STKALIGN);

        // set exception priorities

        // PENDSV has lowest priority
        let pendsv_prio = 0xc0;
        // SVC and SYSTICK have second lowest priority
        let svcall_prio = 0x80;
        let systick_prio = 0x80;
        // set SHPR2
        scb.shpr[1].modify(|w| (w & 0x00ffffff) | (svcall_prio << 24));
        // set SHPR3
        scb.shpr[2].modify(|w| (w & 0x0000ffff) | (pendsv_prio << 16) | (systick_prio << 24));
    }
}
