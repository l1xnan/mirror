use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub pypi: Option<SubConfig>,
    pub npm: Option<SubConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubConfig {
    pub name: String,
    pub language: String,
    pub cmd: String,
    pub mirrors: Vec<Mirror>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mirror {
    pub name: String,
    pub label: String,
    pub test: String,
    pub args: Option<String>,
}
