use crate::Named;
use serde::Deserialize;
use super::Direction;

#[derive(Deserialize)]
pub struct Object {
	aliases: Vec<String>,
	pub carryable: bool,
	description: String,
	pub kind: ObjectKind
}

impl Named for Object {
	fn get_aliases(&self) -> &[String] { &self.aliases }
	
	fn get_description(&self) -> &str { &self.description }
}

#[derive(Deserialize)]
pub enum ObjectKind {
	Container { capacity: u32, content: Vec<Object> },
	Exit(Exit),
	Weapon(Weapon)
}

#[derive(Deserialize)]
pub struct Exit {
	pub destination: usize,
	pub direction: Direction
}

#[derive(Deserialize)]
pub struct Weapon {
	pub damages: u32,
	pub durability: u32
}