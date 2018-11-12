use super::world::Direction;

pub enum PlayerAction {
	DoNothing,
	Exit,
	Go(Direction)
}