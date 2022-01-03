#![allow(unused)]
use std::env;
use std::env::home_dir;
use structopt::StructOpt;
use std::path::PathBuf;

mod todo;
mod status;
mod priority;
mod cli;

fn main() -> std::io::Result<()> {
    let args = cli::Cli::from_args();

    let mut datapath = PathBuf::new();
    match home_dir() {
        Some(mut path) => {
            path.push(".tridos.json");
            datapath = path
        },
        None => println!("Impossible to get your home dir!"),
    }
    //println!("{:#?}", datapath);

    if datapath.as_os_str().is_empty() {
        println!("Found empty datapath string.");
        datapath.push("~/.tridos.json");
    }
    let mut todo_ctrl = todo::TodoCtrl::new(datapath);
    let res = todo_ctrl.open();

    args.run(todo_ctrl);
   
    Ok(())
}

