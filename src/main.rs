#![allow(unused)]
use serde::{Deserialize, Serialize};
use serde_json::Result;
use structopt::StructOpt;
use std::env;
use std::str::FromStr;
use std::string::ParseError;
use std::io::{BufWriter, Write};
use std::fs::File;

// Priority of a task, default value is low, option command is -p.
enum Priority {
    Important,
    High,
    Medium,
    Low,
}

// Single task.
struct Todo {
    task: String,
    priority: Priority,
    id: usize,
}

// Controller to open and save the todo list.
struct Todo_Ctrl {
    data_file: std::path::PathBuf,
    items: Vec<Todo>,
}

// Command reader, don't know if it is really in need.

#[derive(StructOpt)]
struct Cli {
    #[structopt(short = "a", long = "add")]
    cmd: String,
    options: Vec<String>,
}

#[derive(StructOpt)]
struct Cli_Add {
    task: String,
    priority: Priority,
    datafilename: String,
}
impl FromStr for Cli_Add {
    type Err = ParseError;
    
    pub fn from_str(s: &str) -> Result<Self,Self::Err> {
        let cmd: Vec<&str> = s.split(' ').collect();

        match &s {
            Low => Ok(Priority::Low);
            Medium => Ok(Priority::Medium);
            High => Ok(Priority::High);
            Important => Ok(Priority::Important);
        }
    }
}

// Function to handle the income args.
fn parse_config(args: &[String]) -> (&str, &str) {
    
    match &args[1] {
        Add => println!("CMD: Add"),
        List => println!("CMD: List"),
        Edit => println!("CMD: Edit"),
    }
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let cmd = args[1];
    if args[1] == "All" {
        let add = Cli_Add::from_args();
    }

    let (query, filename) = parse_config(&args);

    match env::home_dir() {
        Some(path) => println!("Your home directory, probably: {}", path.display()),
        None => println!("Impossible to get your home dir!"),
    }

    let cmd = Cli {
        cmd : query.to_string(),
        options: Vec::new(),
    };

    //let new_task = Todo {
    //    task: cmd.task,
    //    priority: Priority::Low,
    //    id: 1,
    //};

    // Create temp file
    let temp_file = env::temp_dir().join(filename);

    // Create file if not exists
    let mut file = File::create(temp_file).unwrap();
    let mut writer = BufWriter::new(file);

 // let encoded = serde_json::to_string(&new_task).unwrap();

 //   println!("{}", encoded);

    println!("Write file ... ");
  //  serde_json::to_writer(&mut writer, &new_task)?;
   // writer.flush()?;
    Ok(())
}

