#![allow(unused)]
use structopt::StructOpt;
use std::str::FromStr;

use crate::todo;
use crate::status;
use crate::priority;

// Command reader, don't know if it is really in need.
#[derive(StructOpt, Debug)]
#[structopt(name = "shit_to_do")]
pub struct Cli {
    /// Set a priority 1 - 4, 1 is really important.
    #[structopt(short, long, default_value = "Low")]
    pub priority: String,
    /// Add an new task
    #[structopt(short, long)]
    pub add: Vec<String>,
    /// List all tasks
    #[structopt(short, long)]
    pub list: bool,
    /// Set status, -s [progress, done].
    #[structopt(short, long, default_value = "")]
    pub status: String,
    /// Choose a task.
    #[structopt(short, long, default_value = "")]
    pub task_num: String,

}
impl Cli {
    pub fn run(&self, mut todo_ctrl: todo::TodoCtrl) -> std::io::Result<()> {
        if !self.add.is_empty() {
            for todo in self.add.iter() {
                let mut status = status::Status::Open;
                let new_task = todo::Todo {
                    id: 0,
                    task: todo.to_string(),
                    priority: priority::Priority::from_str(&self.priority).unwrap(),
                    status: status::Status::Open,
                };
                todo_ctrl.add(new_task);
            }
            todo_ctrl.save();
        } 

        if self.list {
            todo_ctrl.list();
        }

        if self.status != "Open" && self.task_num != "".to_string() {
            println!("Status change ...");
            for mut task in todo_ctrl.items.iter_mut() {
                if task.id == self.task_num.parse::<usize>().unwrap() {
                    //println!("[DEBUG]Found the id.");
                    task.status = status::Status::from_str(&self.status).unwrap();
                }
            }
            todo_ctrl.save();
        }
        Ok(())
    }
}
