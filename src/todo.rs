use std::env;
use std::str::FromStr;
use std::fs::File;
use std::io::Write;
use std::io::BufWriter;
use std::io::BufReader;
use std::string::ParseError;
use serde::{Deserialize, Serialize};

use crate::status;
use crate::priority;

// Single task.
#[derive(Debug, Deserialize, Serialize)]
pub struct Todo {
    pub id: usize,
    pub task: String,
    pub priority: priority::Priority,
    pub status: status::Status,
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
        println!("Task saved successfully!");
        Ok(())
    }

    pub fn open(&mut self) -> std::io::Result<()> { //Vec<Todo>, io::Error> {
        let file = File::open(&self.data_file)?;
        let reader = BufReader::new(file);
        self.items = serde_json::from_reader(reader)?; //.expect("Can not find the file.");
        Ok(())
    }

    pub fn list(&self) -> std::io::Result<()> {
        //println!("Todo List: {}", &self.items.len());
        for task in &self.items {
            println!("{:#?}", task);
        }
        Ok(())
    }
}

