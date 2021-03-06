//! # Software synchronization bus definition
//!
use core::cell::RefCell;
use crate::system::scheduler::BooleanVector;
use crate::KernelError;
use crate::kernel::tasks::{get_curr_tid, release, schedule};
use crate::utils::arch::critical_section;

#[cfg(feature = "system_logger")]
use {
    crate::system::system_logger::LogEventType,
    crate::kernel::logging,
};

/// Enables task synchronization and communication.
pub struct Semaphore {
    /// It is a boolean vector which represents the tasks notified by the semaphore.
    pub flags: RefCell<BooleanVector>,
    /// It is a boolean vector that corresponds to the tasks that are to be released by the semaphore on being signaled.
    pub tasks: BooleanVector,
}

impl Semaphore {
    /// Initializes a new semaphore instance.
    pub const fn new(tasks: BooleanVector) -> Self {
        Self { flags: RefCell::new(0), tasks }
    }

    /// Signals the semaphore, all tasks specified in semaphore::flags can test for it and all tasks in semaphore::tasks are released
    pub fn signal_and_release(&'static self, tasks_mask: BooleanVector) {
        critical_section(|_| {
            let flags: &mut BooleanVector = &mut self.flags.borrow_mut();
            *flags |= tasks_mask;
            release(self.tasks);
            #[cfg(feature = "system_logger")] {
                if logging::get_semaphore_signal() {
                    logging::report(LogEventType::SemaphoreSignal(*flags, self.tasks));
                }
            }
            schedule();
        })
    }

    /// Checks if the flag was enabled for the currently running task.
    pub fn test_and_reset(&'static self) -> Result<bool, KernelError> {
        critical_section(|_| {
            let curr_tid = get_curr_tid() as u32;
            let curr_tid_mask = 1 << curr_tid;
            let flags: &mut BooleanVector = &mut self.flags.borrow_mut();
            if *flags & curr_tid_mask == curr_tid_mask {
                *flags &= !curr_tid_mask;
                #[cfg(feature = "system_logger")] {
                    if logging::get_semaphore_reset() {
                        logging::report(LogEventType::SemaphoreReset(curr_tid));
                    }
                }
                return Ok(true);
            } else {
                return Ok(false);
            }
    })
}
}

unsafe impl Sync for Semaphore {}