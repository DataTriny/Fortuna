use crate::Named;
use std::fmt;
use super::{
	Direction,
	objects::{Object, ObjectKind}
};

#[derive(Deserialize)]
pub struct Place {
	description: String,
	pub objects: Vec<Object>
}

impl Place {
	pub fn get_exit(&self, dir: Direction) -> Option<&Object> {
		self.objects.iter().filter(|o| {
			match o.kind {
				ObjectKind::Exit(ref exit) if exit.direction == dir => true,
				_ => false
			}
		}).next()
	}
	
	pub fn get_exits(&self) -> impl Iterator<Item = &Object> {
		self.objects.iter().filter(|o| {
			match o.kind {
				ObjectKind::Exit(_) => true,
				_ => false
			}
		})
	}
	
	pub fn get_objects_except_exits(&self) -> impl Iterator<Item = &Object> {
		self.objects.iter().filter(|o| {
			match o.kind {
				ObjectKind::Exit(_) => false,
				_ => true
			}
		})
	}
}

impl fmt::Display for Place {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.description)?;
		for e in self.get_exits() {
			write!(f, "\n{}", e.get_description())?;
		}
		for o in self.get_objects_except_exits() {
			write!(f, "\n{}", o.get_description())?;
		}
		Ok(())
	}
}