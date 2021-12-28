#![allow(unused)]
use structopt::StructOpt;
use std::string::ParseError;
use std::str::FromStr;
use std::path::PathBuf;
use std::env;
use std::env::home_dir;
//use serde::{Deserialize, Serialize};
//use serde_json::Result;

// Priority of a task, default value is low, option command is -p.
#[derive(Debug, Copy, Clone)]
enum Priority {
    Important,
    High,
    Medium,
    Low,
}
impl FromStr for Priority {
    type Err = ParseError;

    fn from_str(prio: &str) -> Result<Self, Self::Err> {
        match prio {
            "Important" => Ok(Priority::Important),
            "High" => Ok(Priority::High),
            "Medium" => Ok(Priority::Medium),
            "Low" => Ok(Priority::Low),
            _ => Ok(Priority::Low), // Err(format!("Could not parse the priority: {}", prio)),
        }
    }
}

// Single task.
#[derive(Debug)]
struct Todo {
    task: String,
    priority: Priority,
    id: usize,
}

// Controller to open and save the todo list.
#[derive(Debug)]
struct TodoCtrl {
    data_file: std::path::PathBuf,
    items: Vec<Todo>,
}
impl TodoCtrl {
    pub fn new(data_file: std::path::PathBuf) -> Self {
        Self {
            data_file: data_file,
            items : Vec::new(),
        }
    }
}

// Command reader, don't know if it is really in need.
#[derive(StructOpt, Debug)]
#[structopt(name = "shit_to_do")]
struct Cli {
    cmd: String,
    /// Set a priority 1 - 4, 1 is really important.
    #[structopt(short, long, default_value = "Low")]
    priority: Priority,
    /// Add an new task
    #[structopt(short, long)]
    add: Vec<String>,
    /// List all tasks
    #[structopt(short, long, default_value = "")]
    list: String,
    /// Set status, -s [progress, done].
    #[structopt(short, long, default_value = "")]
    status: String,

}

fn main() -> std::io::Result<()> {
    let args = Cli::from_args();

    println!("{:#?}", args);
    let mut datapath = PathBuf::new();
    match env::home_dir() {
        Some(path) => {
            println!("Your home directory, probably: {}", path.display());
            datapath = path;
        },
        None => println!("Impossible to get your home dir!"),
    }

    if datapath.capacity() == 0 {
        datapath.push("~/.tridos.json");
    }
    let mut todo_ctrl = TodoCtrl::new(datapath);

    if !args.add.is_empty() {
        for todo in args.add.iter() {
            let new_task = Todo {
                id: 1,
                task: todo.to_string(),
                priority: args.priority,
            }; 
            todo_ctrl.items.push(new_task);
        }
    }
    
    println!("{:#?}", todo_ctrl);

      // Create temp file
//    let temp_file = env::temp_dir().join(filename);

    // Create file if not exists
//    let mut file = File::create(temp_file).unwrap();
//    let mut writer = BufWriter::new(file);

 // let encoded = serde_json::to_string(&new_task).unwrap();

 //   println!("{}", encoded);

//    println!("Write file ... ");
  //  serde_json::to_writer(&mut writer, &new_task)?;
   // writer.flush()?;
    Ok(())
}

