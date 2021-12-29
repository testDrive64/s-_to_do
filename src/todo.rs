use std::string::ParseError;
use std::error::Error;
use std::str::FromStr;
use std::env;
use std::env::home_dir;
use serde::{Deserialize, Serialize};
//use serde_json::Result;
use std::fs::File;
use std::path::PathBuf;
use std::io::BufWriter;
use std::io::Write;
use std::io::BufReader;


pub fn run () {
    //
}

// Priority of a task, default value is low, option command is -p.
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum Priority {
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
#[derive(Debug, Deserialize, Serialize)]
pub struct Todo {
    pub task: String,
    pub priority: Priority,
    pub id: usize,
}

// Controller to open and save the todo list.
#[derive(Debug, Deserialize, Serialize)]
pub struct TodoCtrl {
    data_file: std::path::PathBuf,
    pub items: Vec<Todo>,
}

impl TodoCtrl {
    pub fn new(data_file: std::path::PathBuf) -> Self {
        Self {
            data_file: data_file,
            items: Vec::new(), 
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        // Create temp file
        let temp_file = env::temp_dir().join(&self.data_file);
        // Create file if not exists
        let mut file = File::create(temp_file).unwrap();
        let mut writer = BufWriter::new(file);
        let encoded = serde_json::to_string(&self.items).unwrap();
        //println!("{}", encoded);
        //println!("Write file ... ");
        serde_json::to_writer(&mut writer, &self.items)?;
        writer.flush()?;
        Ok(())
    }

    pub fn open(&mut self) -> std::io::Result<()> { //Vec<Todo>, io::Error> {
        let file = File::open(&self.data_file)?;
        let reader = BufReader::new(file);

        //let tasks: Vec<Todo>
        self.items = serde_json::from_reader(reader)?; //.expect("Can not find the file.");
        //self.items = tasks;
        Ok(())
    }
}

