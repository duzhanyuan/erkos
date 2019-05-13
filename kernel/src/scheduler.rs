use crate::process_list::{ProcessList, ProcessListItem};

pub mod simple_scheduler;

pub trait Scheduler<'a> {
    fn get_current_proc(&mut self) -> Option<&mut &'a mut ProcessListItem<'a>>;
    fn pop_current_proc(&mut self) -> Option<&'a mut ProcessListItem<'a>>;
    fn exec_current_proc(&mut self) -> ExecResult;
    fn schedule_next(&mut self);
    fn resume_list(&mut self, process_list: &mut ProcessList<'a>);
    fn push(&mut self, proc: &'a mut ProcessListItem<'a>);
    fn push_wait(&mut self, proc: &'a mut ProcessListItem<'a>);
    fn resume_waiting(&mut self);
}

pub enum ExecResult {
    Nothing,
    Executed,
}
