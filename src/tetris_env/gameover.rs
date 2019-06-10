
use super::tetris::*;
use super::world::*;

pub struct GameOverScene {
	pub world: World
}

impl GameOverScene {
	pub fn new(world: World) -> Self {
		Self {
			world
		}
	}
}

impl From<TetrisScene> for GameOverScene {
	fn from(value: TetrisScene) -> Self {
		Self::new(value.world)
	}
}
