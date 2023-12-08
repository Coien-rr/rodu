use rodu::task;
use std::env;

fn main() {
    let task = task::Task::new(env::args().skip(1).collect());
    if !task.is_meta() {
        task.do_task();
    }
}
