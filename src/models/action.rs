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

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct ActionComposite {
    pub name: String,
    pub actions: Vec<ActionComponent>,
}

impl ActionComposite {
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

impl Action for ActionComposite {
    fn print_ln(&self) -> () {
        println!("{}", self.name);
        for action in self.actions.iter() {
            match action {
                ActionComponent::Leaf(leaf) => leaf.print_ln(),
                ActionComponent::Component(component) => component.print_ln(),
            }
        }
    }

    fn execute(&self) -> () {
        for action in self.actions.iter() {
            match action {
                ActionComponent::Leaf(leaf) => leaf.execute(),
                ActionComponent::Component(component) => component.execute(),
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum ActionComponent {
    Leaf(ActionLeaf),
    Component(ActionComposite),
}
