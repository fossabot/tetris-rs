
use super::super::model::world::World;
use super::menu::Menu;
use super::game::Game;
use super::gameover::GameOver;
use super::scene::Scene;

pub enum Window {
	Menu(Menu),
	Game(Game),
	GameOver(GameOver)
}

impl Window {

	pub fn new(world: World) -> Self {
		Window::Menu(Menu::new(world))
	}

	pub fn run(mut self) {
		loop {
			self = match self {
				Window::Menu(m) => m.run().into(),
				Window::Game(g) => g.run().into(),
				Window::GameOver(o) => { o.run(); break; },
			}
		}
	}
}
