#[derive(Debug, PartialEq)]
pub enum Config {
    Include { path: String },
    IncludeOne { paths: Vec<String> },
}