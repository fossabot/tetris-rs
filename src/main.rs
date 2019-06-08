
extern crate ggez;
extern crate na;
extern crate rand;

use ggez::conf::*;

mod tetris_env;

use tetris_env::tetris::*;
use tetris_env::menu::*;
use tetris_env::scene::*;
use tetris_env::world::*;
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
	fn new(world: World) -> Self {
		Game {
			state: GameState::Menu(Scene::new(world))
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

	let args: Vec<String> = std::env::args().collect();

	let nrows: usize;
	let ncols: usize;

	if args.len() == 3 {
		nrows = args[1].parse().unwrap_or(5_usize);
		ncols = args[2].parse().unwrap_or(7_usize);
	}
	else {
		nrows = 5_usize;
		ncols = 7_usize;
	}

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

	let config = Conf {
		window_mode,
		window_setup,
		backend: Backend::default(),
		modules: ModuleConf::default()
	};

	let seed: [u8; 16] = rand::random();

	let world = World {
		nrows,
		ncols,
		config,
		seed
	};

	Game::new(world).run()
}
