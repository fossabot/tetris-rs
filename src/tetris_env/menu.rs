
use super::world::*;

pub struct MenuScene {
	pub world: World
}

impl MenuScene {
	pub fn new(world: World) -> MenuScene {
		MenuScene {
			world
		}
	}
}
