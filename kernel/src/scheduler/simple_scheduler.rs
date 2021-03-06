use core::option::Option;

use crate::process_list::{ProcessList, ProcessListItem};
use super::{Scheduler, ExecResult};


pub struct SimpleScheduler<'a> {
    active: ProcessList<'a>,
    waiting: ProcessList<'a>,
}

impl<'a> Scheduler<'a> for SimpleScheduler<'a> {
    fn get_current_proc(&mut self) -> Option<&mut &'a mut ProcessListItem<'a>> {
        self.active.head_mut()
    }
    
    fn pop_current_proc(&mut self) -> Option<&'a mut ProcessListItem<'a>> {
        self.active.pop()
    }

    fn exec_current_proc(&mut self) -> ExecResult
    {
        if self.active.is_empty() {
            ExecResult::Nothing
        } else {
            let process = &mut self.active.head_mut().unwrap();

            process.execute();
            ExecResult::Executed
        }
    }

    fn schedule_next(&mut self) {
        if !self.active.is_empty() {
            let current = self.active.pop().unwrap();
            self.active.push(current);
        }
    }

    fn resume_list(&mut self, process_list: &mut ProcessList<'a>) {
        self.active.join(process_list);
    }

    fn push(&mut self, proc: &'a mut ProcessListItem<'a>) {
        self.active.push(proc);
    }

    fn push_wait(&mut self, proc: &'a mut ProcessListItem<'a>) {
        self.waiting.push(proc);
    }

    fn resume_waiting(&mut self) {
        self.active.join(&mut self.waiting);
    }
}

impl<'a> SimpleScheduler<'a> {
    pub fn create() -> SimpleScheduler<'a> {
        SimpleScheduler {
            active: ProcessList::new(),
            waiting: ProcessList::new(),
        }
    }

}