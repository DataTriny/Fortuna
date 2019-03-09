#[derive(Deserialize)]
pub struct Player {
	pub current_room: usize
}

impl Player {
	pub fn new(starting_room: usize) -> Player {
		Player {
			current_room: starting_room
		}
	}
}