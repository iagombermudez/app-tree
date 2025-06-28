use std::process::Command;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ActionJSON {
    pub name: String,
    pub command: Option<String>,
    pub actions: Option<Vec<ActionJSON>>,
}

pub trait Action {
    fn print_ln(&self) -> ();
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
    fn print_ln(&self) -> () {
        println!("{} --- {}", self.name, self.command);
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
    pub fn add(&mut self, action: ActionComponent) {
        self.actions.push(action);
    }
}

impl Action for ActionBranch {
    fn print_ln(&self) -> () {
        println!("{}", self.name);
        for action in self.actions.iter() {
            match action {
                ActionComponent::Leaf(leaf) => leaf.print_ln(),
                ActionComponent::Branch(branch) => branch.print_ln(),
            }
        }
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
