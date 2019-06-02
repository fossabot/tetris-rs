
use super::tetris::*;
use super::world::*;

pub struct GameOverScene {
	pub world: World
}

impl GameOverScene {
	pub fn new(world: World) -> GameOverScene {
		GameOverScene {
			world
		}
	}
}

impl From<TetrisScene> for GameOverScene {
	fn from(value: TetrisScene) -> GameOverScene {
		GameOverScene::new(value.world)
	}
}
