#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use hartex_rust::task::*;
use hartex_rust::util::TaskMask;
use hartex_rust::primitive::*;
use hartex_rust::spawn;

const task1: u32 = 1;
const task2: u32 = 2;

#[entry]
fn main() -> ! {
    let peripherals = init_peripherals();

    // These are task parameters, they are passed to the task when called
    let task1_param = "Hello from task 1 !";
    let task2_param = "Hello from task 2 !";

    static mut stack1: [u32; 300] = [0; 300];
    static mut stack2: [u32; 300] = [0; 300];

    /*
    The task definition here is different :
    arg 1 : task name, this will be used to address the task across the code
    arg 2 : priority of the task
    arg 3 : task stack
    arg 4 : this corresponds to by what name will the task body refer the task argument
    arg 5 : the task argument
    arg 6 : task body
    */
    spawn!(task1, stack1, param, task1_param, {
        hprintln!("{}", param);
    });
    spawn!(task2, stack2, param, task2_param, {
        hprintln!("{}", param);
    });

    init();
    release(TaskMask::generate([task1, task2]));
    start_kernel()
}