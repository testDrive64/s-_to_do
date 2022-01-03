use std::env;
use std::str::FromStr;
use std::fs::File;
use std::io::Write;
use std::io::BufWriter;
use std::io::BufReader;
use std::string::ParseError;
use serde::{Deserialize, Serialize};
use enum_utils::FromStr;

use crate::todo;

#[enumeration(case_insensitive)]
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, FromStr)]
pub enum Status {
    Open,
    Progress,
    Done,
}

