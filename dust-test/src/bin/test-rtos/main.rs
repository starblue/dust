#![feature(asm)]
#![feature(naked_functions)]
#![no_std]
#![no_main]

use core::num::Wrapping;
use core::ptr::read_volatile;
use core::ptr::write_volatile;

use dust_cortex_m::scb::SCB_CCR_STKALIGN;
use dust_cortex_m::sys_tick;
use dust_cortex_m::SCB;
use dust_cortex_m::SYS_TICK;

// This dependency is needed to pull in the hard-fault handler
#[allow(unused)]
use dust_hardfault::HARD_FAULT_HANDLER;

use dust_lpc8xx::swm::{U0_RXD, U0_TXD};
use dust_lpc8xx::syscon::Syscon;
use dust_lpc8xx::syscon::CLOCK_SWM;
use dust_lpc8xx::syscon::CLOCK_UART0;
#[cfg(any(feature = "lpc802", feature = "lpc804", feature = "lpc84x"))]
use dust_lpc8xx::syscon::RESET_UART0;
#[cfg(any(feature = "lpc81x", feature = "lpc82x", feature = "lpc83x"))]
use dust_lpc8xx::syscon::RESET_USART0;
use dust_lpc8xx::usart::Usart;
use dust_lpc8xx::SWM;
use dust_lpc8xx::SYSCON;
use dust_lpc8xx::USART;

fn delay(n: usize) {
    for _ in 0..n {
        // Make sure the loop is not optimized away
        unsafe { asm!("" :::: "volatile") }
    }
}

#[cfg(any(
    feature = "lpc802m001",
    feature = "lpc804m101",
    feature = "lpc81x",
    feature = "lpc82x",
    feature = "lpc83x"
))]
const UART_PINS: (usize, usize) = (4, 0);
#[cfg(any(feature = "lpc802m011", feature = "lpc804m111"))]
const UART_PINS: (usize, usize) = (9, 8);
#[cfg(feature = "lpc84x")]
const UART_PINS: (usize, usize) = (25, 24);

/// Multiplier setting for fractional baud rate generators
///
/// This allows up to 460800 baud from the default 12 MHz IRC/FRO clock.
/// A divisor of 4 in the UART baud rate generator
/// then gives the standard baud rate of 115200 baud.
/// The factor of 16 results from the default number of samples per bit.
///
/// U_PCLK = main_clock / (1 + MULT / DIV)
/// 12000000 / (1 + 160 / 256)
/// 12000000 / ((256 + 160) / 256)
/// 12000000 / (416 / 256)
/// 12000000 / (13 / 8)
/// 12000000 * 8 / 13
/// 96000000 / 13
///  7384615 ~(0.16%) 7372800 = 4 * 16 * 115200
///
/// TODO Use 13 samples per bit and avoid the FRG?
///
const FRG_MULT: u32 = 160;

/// Divider setting for fractional baud rate generators
///
/// Must be set to 0xff, the only supported value.
const FRG_DIV: u32 = 0xff;

/// Divisor for the baud rate generator
const UART_BRG_DIVISOR: u32 = 4;

#[cfg(any(feature = "lpc81x", feature = "lpc82x", feature = "lpc83x"))]
fn init_uart0_syscon(syscon: &mut Syscon) {
    unsafe {
        syscon.enable_clock(CLOCK_UART0);
        syscon.periph_reset(RESET_USART0);

        // Divide the input clock by 1
        syscon.uartclkdiv.write(1);

        syscon.uartfrgdiv.write(FRG_DIV);
        syscon.uartfrgmult.write(FRG_MULT);
    }
}
#[cfg(any(feature = "lpc802", feature = "lpc804", feature = "lpc84x"))]
fn init_uart0_syscon(syscon: &mut Syscon) {
    unsafe {
        syscon.enable_clock(CLOCK_UART0);
        syscon.periph_reset(RESET_UART0);

        syscon.frg0div.write(FRG_DIV);
        syscon.frg0mult.write(FRG_MULT);

        // Select main clock as input clock for FRG0
        syscon.frg0clksel.write(1);

        // Select FRG0 as clock for USART0
        syscon.uart0clksel.write(2);
    }
}

fn init_uart() -> &'static mut Usart {
    let syscon = unsafe { &mut *SYSCON };
    let usart = unsafe { &mut *USART[0] };

    // Configure SYSCON for USART0
    init_uart0_syscon(syscon);

    // initialize UART
    usart.init(UART_BRG_DIVISOR);
    usart
}

#[no_mangle]
extern "C" fn hard_fault_write_byte(b: u8) {
    let uart = unsafe { &mut *USART[0] };
    uart.tx(b);
}

unsafe fn read(a: u32) -> u32 {
    use volatile_register::RO;

    let p = a as *const RO<u32>;
    let r = &*p;
    r.read()
}

const STACK_FRAME_PC_OFFSET: isize = 14;
const STACK_FRAME_XPSR_OFFSET: isize = 15;

const XPSR_THUMB: u32 = 1 << 24;

const STACK_SIZE0: usize = 256;
const STACK_SIZE1: usize = 256;

// use u64 to force alignment to 8 bytes
static mut STACK0: [u64; STACK_SIZE0 / 8] = [0_u64; STACK_SIZE0 / 8];
static mut STACK1: [u64; STACK_SIZE0 / 8] = [0_u64; STACK_SIZE1 / 8];

type OsTime = Wrapping<i32>;

#[derive(PartialEq)]
enum TaskState {
    Runnable,
    SleepingUntil(OsTime),
}

struct Task {
    state: TaskState,
    psp: u32,
    stack_bottom: u32,
    stack_top: u32,
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
        Task {
            state,
            psp,
            stack_bottom,
            stack_top,
        }
    }
}

const MAX_TASKS: usize = 2;

struct Scheduler {
    curr_task_index: usize,
    tasks: [Option<Task>; MAX_TASKS],
    time: OsTime,
    next_wake_time: Option<OsTime>,
}
impl Scheduler {
    const fn new() -> Scheduler {
        Scheduler {
            curr_task_index: 0,
            tasks: [None, None],
            time: Wrapping(0),
            next_wake_time: None,
        }
    }
    fn set_task(&mut self, i: usize, task: Task) {
        if i < MAX_TASKS {
            self.tasks[i] = Some(task);
        }
    }
    fn context_switch(&mut self, curr_psp: u32) -> u32 {
        if let Some(task) = &mut self.tasks[self.curr_task_index] {
            task.psp = curr_psp;
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
                            self.curr_task_index = i;
                            return task.psp;
                        }
                        TaskState::SleepingUntil(wake_time) => {
                            if wake_time - time <= Wrapping(0) {
                                // wake up
                                // TODO check for late wake up?
                                task.state = TaskState::Runnable;
                                self.curr_task_index = i;
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
            let scb = unsafe { &mut *SCB };
            scb.set_pending_pendsv();
        }
    }
    fn get_time(&self) -> OsTime {
        unsafe { read_volatile(&self.time) }
    }
}
static mut SCHEDULER: Scheduler = Scheduler::new();

#[cfg(feature = "atsamd09")]
const CLOCK_FREQUENCY: u32 = 1_000_000;
#[cfg(feature = "atsaml11")]
const CLOCK_FREQUENCY: u32 = 4_000_000;
#[cfg(feature = "lpc8xx")]
const CLOCK_FREQUENCY: u32 = 12_000_000;
#[cfg(feature = "lpc11xx")]
const CLOCK_FREQUENCY: u32 = 12_000_000;
#[cfg(feature = "lpc13xx")]
const CLOCK_FREQUENCY: u32 = 12_000_000;
#[cfg(feature = "stm32f0x0")]
const CLOCK_FREQUENCY: u32 = 8_000_000;

/// Time since system startup in milliseconds
static mut TIME_MS: u32 = 0;

#[no_mangle]
pub unsafe extern "C" fn systick_handler() {
    TIME_MS += 1;
}

fn get_time_ms() -> u32 {
    unsafe { read_volatile(&TIME_MS) }
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
    let prev_psp: u32;
    asm!("mrs     r0, psp
          subs    r0, #32
          stmia   r0!, {r4-r7}
          mov     r4, r8
          mov     r5, r9
          mov     r6, r10
          mov     r7, r11
          stmia   r0!, {r4-r7}
          subs    r0, #32"
         : "={r0}"(prev_psp)
         :
         :
         : "volatile"
    );
    let next_psp = SCHEDULER.context_switch(prev_psp);
    asm!("adds    r0, #16
          ldmia   r0!, {r4-r7}
          mov     r8, r4
          mov     r9, r5
          mov     r10, r6
          mov     r11, r7
          msr     psp, r0
          subs    r0, #32
          ldmia   r0!, {r4-r7}
          bx      lr"
         :
         : "{r0}"(next_psp)
         :
         : "volatile"
    );
}

fn task0() {}
fn task1() {}

#[no_mangle]
pub fn main() {
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

    // set up systick interrupt
    unsafe {
        let sys_tick = &mut *SYS_TICK;

        // set reload value for one tick per millisecond
        sys_tick.rvr.write(CLOCK_FREQUENCY / 1000);

        // reset timer value
        sys_tick.cvr.write(0);

        // enable timer and timer interrupt
        sys_tick.csr.write(
            sys_tick::CSR_ENABLE  // enable timer
                   | sys_tick::CSR_TICKINT  // enable timer interrupt
                   | sys_tick::CSR_CLKSOURCE, // use core clock
        );

        // interrupt is already enabled after reset
    }

    // Configure switch matrix (SWM)
    let syscon = unsafe { &mut *SYSCON };
    let swm = unsafe { &mut *SWM };
    let (txd_pin, rxd_pin) = UART_PINS;
    unsafe {
        syscon.enable_clock(CLOCK_SWM);
        swm.set_movable_function_pin(U0_RXD, rxd_pin);
        swm.set_movable_function_pin(U0_TXD, txd_pin);
        syscon.disable_clock(CLOCK_SWM);
    }
    let uart = init_uart();

    unsafe {
        SCHEDULER.set_task(0, Task::new(task0, unsafe { &mut STACK0 }));
        SCHEDULER.set_task(1, Task::new(task1, unsafe { &mut STACK1 }));
    }

    let d = 1_000_000;
    loop {
        for c in b"Still alive!\r\n" {
            uart.tx(*c);
        }
        delay(d);
    }
}
