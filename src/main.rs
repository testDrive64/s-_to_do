#![allow(unused)]
use structopt::StructOpt;
use std::string::ParseError;
use std::str::FromStr;
use std::path::PathBuf;
use std::env;
use std::env::home_dir;
use serde::{Deserialize, Serialize};
//use serde_json::Result;
use std::fs::File;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;

mod todo;

// Command reader, don't know if it is really in need.
#[derive(StructOpt, Debug)]
#[structopt(name = "shit_to_do")]
struct Cli {
    //cmd: String,
    /// Set a priority 1 - 4, 1 is really important.
    #[structopt(short, long, default_value = "Low")]
    priority: todo::Priority,
    /// Add an new task
    #[structopt(short, long)]
    add: Vec<String>,
    /// List all tasks
    #[structopt(short, long)]
    list: bool,
    /// Set status, -s [progress, done].
    #[structopt(short, long, default_value = "")]
    status: todo::Status,
    /// Choose a task.
    #[structopt(short, long, default_value = "")]
    task_num: String,

}
impl Cli {
    pub fn run(&self) -> std::io::Result<()> {
        Ok(())
    }
}


fn main() -> std::io::Result<()> {
    let args = Cli::from_args();

    //println!("{:#?}", args);
    let mut datapath = PathBuf::new();
    match env::home_dir() {
        Some(mut path) => {
//            println!("Your home directory, probably: {}", path.display());
            path.push(".tridos.json");
            datapath = path
        },
        None => println!("Impossible to get your home dir!"),
    }
    println!("{:#?}", datapath);

    if datapath.as_os_str().is_empty() {
        println!("Found empty datapath string.");
        datapath.push("~/.tridos.json");
    }
    let mut todo_ctrl = todo::TodoCtrl::new(datapath);
    let res = todo_ctrl.open();

    println!("{:#?}", res);

    if !args.add.is_empty() {
        for todo in args.add.iter() {
            let new_task = todo::Todo {
                id: 0,
                task: todo.to_string(),
                priority: args.priority,
                status: args.status,
            }; 
            todo_ctrl.add(new_task);
        }
        todo_ctrl.save();
    } 

    if args.list {
        todo_ctrl.list();
    }

    if args.status != todo::Status::Open && args.task_num != "".to_string() {
//        let mut task = todo_ctrl.items.iter().filter(|t| t.id == args.task_num.parse::<usize>().unwrap());
//        task.status = args.status;
        println!("Status change ...");
        for mut task in todo_ctrl.items.iter_mut() {
            if task.id == args.task_num.parse::<usize>().unwrap() {
                println!("Found the id.");
                task.status = args.status;
            }
        }
        todo_ctrl.save();
    }

    Ok(())
}

