use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppLeaf {
    pub app_name: String,
    pub executable: String,
}
