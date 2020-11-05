use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Request {
    pub A: bool, 
    pub B: bool,
    pub C: bool,
    pub D: f64, 
    pub E: i32, 
    pub F: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LogicalPath {
    M,
    P,
    T
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct Response {
    pub H: LogicalPath,
    pub K: f64,
}

impl Response {
    pub fn new(logical_path: LogicalPath, k: f64) -> Self {
        Self {
            H: logical_path,
            K: k
        }
    }
}