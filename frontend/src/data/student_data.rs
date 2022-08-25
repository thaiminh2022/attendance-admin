use serde::{self, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StudentData {
    pub name: String,
    pub class: String,
    pub class_order: i8,
    pub date: String,
}

impl StudentData {
    fn _new() -> Self {
        Self {
            name: "".to_string(),
            class: "".to_string(),
            class_order: -1,
            date: "".to_string(),
        }
    }
}
