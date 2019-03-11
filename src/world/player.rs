use super::objects::Object;

#[derive(Deserialize)]
pub struct Player {
	pub current_place: usize,
	pub inventory: Vec<Object>
}