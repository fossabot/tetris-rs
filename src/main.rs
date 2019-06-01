
extern crate ggez;
extern crate na;
extern crate rand;

use ggez::*;
use ggez::graphics::*;
use ggez::conf::*;
use ggez::event::*;
use na::*;

mod tetris;
use tetris::shape::*;
use tetris::tetris::*;

fn main() {

	let conf = Conf {
		window_mode: WindowMode {
			width: 1600.0,
			height: 1200.0,
			borderless: false,
			fullscreen_type: FullscreenType::Windowed,
			min_width: 0.0,
			max_width: 0.0,
			min_height: 0.0,
			max_height: 0.0,
			hidpi: false,
			maximized: false,
			resizable: true
		},
		window_setup: WindowSetup {
			title: "Tetris".to_owned(),
			icon: "".to_owned(),
			samples: NumSamples::Eight,
			vsync: true,
			transparent: false,
			srgb: true
		},
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
