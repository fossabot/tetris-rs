
use super::world::*;
use super::menu::*;
use super::tetris::*;
use super::gameover::*;

// Scene

pub struct Scene<S> {
	scene: S
}

impl Scene<MenuScene> {
	pub fn new(world: World) -> Self {
		Self {
			scene: MenuScene::new(world)
		}
	}
}


// Scene .run()

impl Scene<MenuScene> {
	pub fn run(self) -> Scene<TetrisScene> {

		const VERSION: &str = env!("CARGO_PKG_VERSION");

		leg::success("Opening menu...", None, None);
		leg::head("Tetris", Some("\u{1f579}\u{fe0f}"), Some(VERSION));
		self.into()
	}
}

impl Scene<TetrisScene> {
	pub fn run(mut self) -> Scene<GameOverScene> {
		leg::success("Running Tetris...", None, None);
		self.scene.run();
		self.into()
	}
}

impl Scene<GameOverScene> {
	pub fn run(self) -> Scene<GameOverScene> {
		leg::success("Game over scene", None, None);
		self
	}
}


// Scene .into()

impl From<Scene<MenuScene>> for Scene<TetrisScene> {
	fn from(value: Scene<MenuScene>) -> Scene<TetrisScene> {
		Self {
			scene: value.scene.into()
		}
	}
}

impl From<Scene<TetrisScene>> for Scene<GameOverScene> {
	fn from(value: Scene<TetrisScene>) -> Scene<GameOverScene> {
		Self {
			scene: value.scene.into()
		}
	}
}
