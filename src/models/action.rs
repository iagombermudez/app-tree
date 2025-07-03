use std::process::Command;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ActionJSON {
    pub name: String,
    pub command: Option<String>,
    pub actions: Option<Vec<ActionJSON>>,
}

pub trait Action {
    fn to_string(&self, num_indents: usize) -> String;
    fn execute(&self) -> ();
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ActionLeaf {
    pub name: String,
    pub command: String,
}

impl ActionLeaf {
    pub fn new(name: String, command: String) -> Self {
        Self { name, command }
    }
}

impl Action for ActionLeaf {
    fn to_string(&self, num_indents: usize) -> String {
        let indent = vec!["  "; num_indents].join("");
        return format!("{}{} --- {}\n", indent, self.name, self.command);
    }

    fn execute(&self) -> () {
        let command_execution_result = Command::new(&self.command).spawn();
        if let Err(error) = command_execution_result {
            println!("There was an error {}", error)
        };
        return ();
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ActionBranch {
    pub name: String,
    pub actions: Vec<ActionComponent>,
}

impl ActionBranch {
    pub fn new(name: String) -> Self {
        Self {
            name,
            actions: vec![],
        }
    }

    pub fn add(&mut self, action: ActionComponent) {
        self.actions.push(action);
    }
}

impl Action for ActionBranch {
    fn to_string(&self, num_indents: usize) -> String {
        let indent = vec!["  "; num_indents].join("");
        let mut action_listing = format!("{}{}\n", indent, self.name);
        for action in self.actions.iter() {
            let child_listing = match action {
                ActionComponent::Leaf(leaf) => &leaf.to_string(num_indents + 1),
                ActionComponent::Branch(branch) => &branch.to_string(num_indents + 1),
            };
            action_listing.push_str(child_listing);
        }
        return action_listing;
    }

    fn execute(&self) -> () {
        for action in self.actions.iter() {
            match action {
                ActionComponent::Leaf(leaf) => leaf.execute(),
                ActionComponent::Branch(branch) => branch.execute(),
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ActionComponent {
    Leaf(ActionLeaf),
    Branch(ActionBranch),
}
