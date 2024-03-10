use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<Argument>,
    pub category: String,
    pub description: String,
}

impl Function {
    pub fn new(
        name: String,
        arguments: Vec<Argument>,
        category: String,
        description: String,
    ) -> Self {
        Self {
            name,
            arguments,
            category,
            description,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Argument {
    pub name: Option<String>,
    pub supported_argument_type: String,
}

impl Argument {
    pub fn new(name: Option<String>, supported_argument_type: String) -> Self {
        Self {
            name,
            supported_argument_type,
        }
    }
}
