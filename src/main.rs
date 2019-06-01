
extern crate ggez;
extern crate na;
extern crate rand;

use ggez::*;
use ggez::conf::*;

mod tetris_env;
use tetris_env::tetris::*;

fn main() {

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

	let (ctx, event_loop) = &mut ContextBuilder::new("Tetris", "Mr.Robb")
		.conf(conf)
		.with_conf_file(true)
		.build()
		.expect(" ._. Could not create ggez context");

	let mut tetris = TetrisGame::new(ctx)
		.expect("Could not create a game");

	match event::run(ctx, event_loop, &mut tetris) {
		Ok(_) => {
			println!("Exited cleanly.");
		},
		Err(e) => println!("Dirty exit with error: {}", e)
	}
}
