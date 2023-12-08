use core::panic;

use super::opts;
use super::utils;

#[derive(Debug)]
pub struct Task {
    option: opts::Opts,

    task: Option<String>,
}

impl Task {
    pub fn new(args: Vec<String>) -> Task {
        Task {
            option: opts::Opts::new(&args[0]),
            task: {
                if args.len() > 1 {
                    Some(format!(" {}\n", &args[1..].join(" ")))
                } else {
                    None
                }
            },
        }
    }

    pub fn is_meta(&self) -> bool {
        self.option.is_meta()
    }

    pub fn do_task(&self) {
        match self.option {
            opts::Opts::Add => self.add_task(),
            opts::Opts::Done => self.done_task(),
            opts::Opts::List => self.list_task(),
            _ => panic!("option Error"),
        }
    }

    fn list_task(&self) {
        if !utils::todo_exist() {
            println!("Sorry! You haven't configured todo yet.");
            println!("Please Enter todo to init firstly!")
        } else {
            utils::todo_list();
        }
    }

    fn done_task(&self) {
        if !utils::todo_exist() {
            println!("Sorry! You haven't configured todo yet.");
            println!("Please Enter todo to init firstly!")
        } else if let Some(task) = &self.task {
            utils::todo_done(task);
        }
    }

    fn add_task(&self) {
        if !utils::todo_exist() {
            utils::todo_init()
        }
        if let Some(task) = &self.task {
            utils::todo_add(task)
        }
    }
}
