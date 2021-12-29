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

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
enum Status {
    New,
    Progress,
    Done,
}
impl FromStr for Status {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let status: &str = &s.to_lowercase();
        match status
        {
            "new" => Ok(Status::New),
            "progress" => Ok(Status::Progress),
            "done" => Ok(Status::Done),
            _ => Ok(Status::New),
        }
    }
}

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
    #[structopt(short, long, default_value = "New")]
    status: Status,

}

fn main() -> std::io::Result<()> {
    let args = Cli::from_args();

    //println!("{:#?}", args);
    let mut datapath = PathBuf::new();
    match env::home_dir() {
        Some(mut path) => {
            //println!("Your home directory, probably: {}", path.display());
            path.push(".tridos.json");
            datapath = path
        },
        None => println!("Impossible to get your home dir!"),
    }

    if datapath.capacity() == 0 {
        datapath.push("~/.tridos.json");
    }
    let mut todo_ctrl = todo::TodoCtrl::new(datapath);
    todo_ctrl.open();

    if !args.add.is_empty() {
        for todo in args.add.iter() {
            let new_task = todo::Todo {
                id: 1,
                task: todo.to_string(),
                priority: args.priority,
            }; 
            todo_ctrl.items.push(new_task);
        }
    } else if args.list {
        todo_ctrl.list();
        //for task in &todo_ctrl.items {
        //    println!("{:#?}", task);
        //}    
    }
    
    //println!("{:#?}", todo_ctrl);
    todo_ctrl.save();
    Ok(())
}

