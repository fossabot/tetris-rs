
extern crate ggez;
extern crate na;
extern crate rand;

use ggez::conf::*;

mod tetris_env;

use tetris_env::tetris::*;
use tetris_env::menu::*;
use tetris_env::scene::*;
use tetris_env::gameover::GameOverScene;

enum GameState {
	Menu(Scene<MenuScene>),
	Tetris(Scene<TetrisScene>),
	GameOver(Scene<GameOverScene>)
}

impl From<Scene<MenuScene>> for GameState {
	fn from(value: Scene<MenuScene>) -> Self {
		GameState::Menu(value)
	}
}

impl From<Scene<TetrisScene>> for GameState {
	fn from(value: Scene<TetrisScene>) -> Self {
		GameState::Tetris(value)
	}
}

impl From<Scene<GameOverScene>> for GameState {
	fn from(value: Scene<GameOverScene>) -> Self {
		GameState::GameOver(value)
	}
}

struct Game {
	state: GameState
}

impl Game {
	fn new() -> Self {

		let window_mode = WindowMode::default()
			.dimensions(1600.0, 1200.0)
			.hidpi(true)
			.resizable(true);

		let window_setup = WindowSetup::default()
			.title("Tetris")
			.icon("")
			.vsync(true)
			.transparent(false)
			.samples(NumSamples::Zero);

		let conf = Conf {
			window_mode,
			window_setup,
			backend: Backend::default(),
			modules: ModuleConf::default()
		};

		Game {
			state: GameState::Menu(Scene::new(conf))
		}
	}

	fn run(mut self) {
		loop {
			self.state = match self.state {
				GameState::Menu(s) => { s.run().into() },
				GameState::Tetris(s) => { s.run().into() },
				GameState::GameOver(s) => {
					s.run();
					break;
				},
			};
		}
	}
}

fn main() {

	let game = Game::new();
	game.run();
}
