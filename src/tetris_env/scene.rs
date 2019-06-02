
mod tetris_env;

use ggez::Context;
use tetris_env::world::World;
use tetris_env::menu::MenuScene;
use tetris_env::tetris::TetrisScene;
use std::time::Duration;

pub struct Scene<S> {
	world: World,
	scene: S
}

impl Scene<MenuScene> {
	pub fn new(ctx: Context) -> Self {
		Scene {
			world: World {},
			scene: MenuScene::new(ctx)
		}
	}

	pub fn run(mut self) {
		leg::info("Running...", None, None);
		Scene::<TetrisScene>::from(self);
		leg::info("Initiating Tetris", None, None);
	}
}

impl Scene<TetrisScene> {
	pub fn run(&self) {
		leg::info("Running Tetris", None, None);
		std::thread::sleep(Duration::from_secs(20));
	}
}

impl From<Scene<MenuScene>> for Scene<TetrisScene> {
	fn from(value: Scene<MenuScene>) -> Scene<TetrisScene> {
		Scene {
			world: value.world,
			scene: value.scene.into()
		}
	}
}
