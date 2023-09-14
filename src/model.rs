extern crate rustc_serialize;
use rustc_serialize::json::Json;
use std::fs::File;
use std::io::copy;
use std::io::stdout;
use serde::{Deserialize, Serialize};

pub(crate) struct Model {}
impl Model {
    pub fn new() {}
    pub fn fetch_data<T:Deserialize + Serialize>(filename:&String){
        let data: T = serde_json::from_str(filename).expect("JSON was not well-formatted");
    }
}
