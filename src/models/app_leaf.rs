use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppLeaf {
    pub command: String,
    pub app_name: String,
    pub executable: String,
}
