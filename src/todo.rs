use std::string::ParseError;
use std::error::Error;
use std::str::FromStr;
use std::env;
use serde::{Deserialize, Serialize};
//use serde_json::Result;
use std::fs::File;
use std::path::PathBuf;
use std::io::BufWriter;
use std::io::Write;
use std::io::BufReader;

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub enum Status {
    Open,
    Progress,
    Done,
}
impl FromStr for Status {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let status: &str = &s.to_lowercase();
        match status
        {
            "open" => Ok(Status::Open),
            "progress" => Ok(Status::Progress),
            "done" => Ok(Status::Done),
            _ => Ok(Status::Open),
        }
    }
}

// Priority of a task, default value is low, option command is -p.
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub enum Priority {
    Important,
    High,
    Medium,
    Low,
}
impl FromStr for Priority {
    type Err = ParseError;

    fn from_str(p: &str) -> Result<Self, Self::Err> {
        // TODO accept numbers
        let prio: &str = &p.to_lowercase();
        match prio {
            "important" => Ok(Priority::Important),
            "high" => Ok(Priority::High),
            "medium" => Ok(Priority::Medium),
            "low" => Ok(Priority::Low),
            _ => Ok(Priority::Low), // Err(format!("Could not parse the priority: {}", prio)),
        }
    }
}

// Single task.
#[derive(Debug, Deserialize, Serialize)]
pub struct Todo {
    pub id: usize,
    pub task: String,
    pub priority: Priority,
    pub status: Status,
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
            data_file,
            items: Vec::new(), 
        }
    }

    pub fn add(&mut self, mut todo: Todo) -> std::io::Result<()> {
        todo.id = self.items.len();
        &self.items.push(todo);
        Ok(())
    }

    pub fn save(&self) -> std::io::Result<()> {
        // Create temp file
        let temp_file = env::temp_dir().join(&self.data_file);
        // Create file if not exists
        let mut file = File::create(temp_file).unwrap();
        let mut writer = BufWriter::new(file);
        let encoded = serde_json::to_string(&self.items).unwrap();
        serde_json::to_writer(&mut writer, &self.items)?;
        writer.flush()?;
        Ok(())
    }

    pub fn open(&mut self) -> std::io::Result<()> { //Vec<Todo>, io::Error> {
        let file = File::open(&self.data_file)?;
        let reader = BufReader::new(file);
        self.items = serde_json::from_reader(reader)?; //.expect("Can not find the file.");
        Ok(())
    }

    pub fn list(&self) -> std::io::Result<()> {
        println!("Todo List: {}", &self.items.len());
        for task in &self.items {
            println!("{:#?}", task);
        }
        Ok(())
    }
}

