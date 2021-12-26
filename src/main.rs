#![allow(unused)]
use serde::{Deserialize, Serialize};
use serde_json::Result;
use structopt::StructOpt;
use std::env;
use std::io::{BufWriter, Write};
use std::fs::File;

// Controller to open and save the todo list.
#[derive(StructOpt)]
struct Todo_Ctrl {
    #[structopt(parse(from_os_str))]
    data_file: std::path::PathBuf,
    items: Vec<Todo>,
}

// Single task.
#[derive(Serialize, Deserialize)]
struct Todo {
    task: String,
    priority: Priority,
    id: usize,
}
impl FromStr for Todo {
    type Err = ParseStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let todo: Vec<&str> = s.trim_matches()
                                .split(',')
                                .collect();
        let task = todo[0].parse::<string>()?;
    }
}


// Priority of a task, default value is low, option command is -p.
#[derive(Serialize, Deserialize)]
enum Priority {
    Important,
    High,
    Medium,
    Low,
}

// Command reader, don't know if it is really in need.
struct Cli {
    cmd: String,
    options: Vec<String>,
}

struct Cli_Add {
    cmd: String,
    task: String,
    priority: Priority,
    datafilename: String,
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

    let (query, filename) = parse_config(&args);

    match env::home_dir() {
        Some(path) => println!("Your home directory, probably: {}", path.display()),
        None => println!("Impossible to get your home dir!"),
    }

    let cmd = Cli {
        cmd = query.to_string(),
        options.append(filename),
    }

    //let new_task = Todo {
    //    task: cmd.task,
    //    priority: Priority::Low,
    //    id: 1,
    //};

    // Create temp file
    let temp_file = env::temp_dir().join(cmd.data_file);

    // Create file if not exists
    let mut file = File::create(temp_file).unwrap();
    let mut writer = BufWriter::new(file);

//    let encoded = serde_json::to_string(&new_task).unwrap();

 //   println!("{}", encoded);

    println!("Write file ... ");
  //  serde_json::to_writer(&mut writer, &new_task)?;
   // writer.flush()?;
    Ok(())
}

