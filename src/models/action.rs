use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    pub name: String,
    pub command: String,
}
