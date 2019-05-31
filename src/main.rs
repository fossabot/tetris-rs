
extern crate ggez;
extern crate na;

use ggez::*;
use ggez::graphics::*;
use ggez::conf::*;
use na::*;

mod shape;

use shape::*;

struct TetrisDisplayConfig {
	x: f32,
	y: f32,
	w: f32,
	h: f32,
	block_size: f32,
	background_color: Color
}

struct TetrisBot {

}

struct PieceCollector {
	seed: u32,
	counter: u64
}

impl PieceCollector {

	fn new(seed: u32) -> PieceCollector {
		PieceCollector {
			seed,
			counter: 0
		}
	}

	fn get_current(&self) -> Shape {
		Shape::O
	}

	fn get_next(&self) -> Shape {
		Shape::O
	}

}

struct Tetris {
	config: TetrisDisplayConfig,
	bot: Option<TetrisBot>,
	grid: MatrixMN<u8, U20, U10>,
	collector: PieceCollector,
	current_piece: (Vec2<usize>, Shape),
}

impl Tetris {

	fn new(ctx: &mut Context) -> GameResult<Tetris> {

		// Initialize grid
		let mut grid: MatrixMN<u8, U20, U10> = zero();
		grid.fill_row(0, 8u8);
		grid.fill_row(19, 8u8);
		grid.fill_column(0, 8u8);
		grid.fill_column(9, 8u8);

		// Set collector with seed
		let collector = PieceCollector::new(0);
		let current_piece = (Vec2::new(1, 1), collector.get_next());

		// Build state
		Ok(Tetris {
			config: TetrisDisplayConfig {
				x: ctx.conf.window_mode.width / 2.0 - ctx.conf.window_mode.height / 20.0 * 5.0,
				y: 0.0,
				w: ctx.conf.window_mode.width,
				h: ctx.conf.window_mode.height,
				block_size: ctx.conf.window_mode.height / 20.0,
				background_color: Color::new(28.0 / 255.0, 28.0 / 255.0, 30.0 / 255.0, 1.0)
			},
			bot: None,
			grid,
			collector,
			current_piece,
		})
	}

	fn down(&mut self) {
		self.current_piece.0.y = ((self.current_piece.0.y - 1 + 1) % 17) + 1;
	}

	fn from_wnd(&self, point: Vec2) -> Vec2 {
		let x = (point.x - self.config.x) / self.config.block_size;
		let y = (point.y - self.config.y) / self.config.block_size;
		Vec2::new(x, y)
	}

	fn to_wnd(&self, point: Vec2) -> Vec2 {
		let x = self.config.x + point.x * self.config.block_size;
		let y = self.config.y + point.y * self.config.block_size;
		Vec2::new(x, y)
	}
}

impl event::EventHandler for Tetris {

	fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

		const FPS: u32 = 2;

		while timer::check_update_time(ctx, FPS) {
			self.down();
		}

		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {

		clear(ctx, self.config.background_color);

		// Draw board
		for (i, row) in self.grid.row_iter().enumerate() {
			for (j, cell) in row.iter().enumerate() {

				// Get position
				let pos = self.to_wnd(Vec2::new(j as f32, i as f32));
				let sz = self.config.block_size;

				// Get color
				let color: Color;
				let shape = Shape::from_index(cell);
				if shape.is_some() {
					color = shape.unwrap().color();
				}
				else if *cell == 8u8 {
					color = Color::new(33.0 / 255.0, 33.0 / 255.0, 35.0 / 255.0, 1.0);
				}
				else {
					color = Color::new(0.0, 0.0, 0.0, 0.0);
				}

				Mesh::new_circle(
					ctx,
					DrawMode::fill(),
					Point2::new(pos.x + sz / 2.0, pos.y + sz / 2.0),
					2.0,
					0.0001,
					Color::new(37.0 / 255.0, 37.0 / 255.0, 39.0 / 255.0, 0.6)
				)
					.unwrap()
					.draw(ctx, DrawParam::default())
					.expect("Could not draw :(");

				// Draw cell
				Mesh::new_rectangle(
					ctx,
					DrawMode::fill(),
					Rect::new(pos.x, pos.y, sz, sz),
					color
				)
					.unwrap()
					.draw(ctx, DrawParam::default())
					.expect("Could not draw :(");
			}
		}

		// Draw current piece
		let position = &self.current_piece.0;
		let sz = self.config.block_size;
		let shape = &self.current_piece.1;
		let color = shape.color();

		for (i, row) in shape.value().row_iter().enumerate() {
			for (j, cell) in row.iter().enumerate() {

				let pos = self.to_wnd(Vec2::new(position.x as f32 + j as f32, position.y as f32 + i as f32));

				// Draw cell
				Mesh::new_rectangle(
					ctx,
					DrawMode::fill(),
					Rect::new(pos.x, pos.y, sz, sz),
					if *cell != 0u8 { color } else { Color::new(0.0, 0.0, 0.0, 0.0) }
				).unwrap().draw(ctx, DrawParam::default())
					.expect("Could not draw :(");
			}
		}

		graphics::present(ctx)
			.expect("Could not present the scene");

		Ok(())
	}
}


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

	let mut tetris = Tetris::new(ctx)
		.expect("Could not create a game");

	match event::run(ctx, event_loop, &mut tetris) {
		Ok(_) => println!("Exited cleanly."),
		Err(e) => println!("Dirty exit with error: {}", e)
	}
}

#[cfg(test)]
mod test {

	use super::*;

	#[test]
	fn wnd_conversions() {

		// Create simple
		let (ctx, event_loop) = &mut ContextBuilder::new("Tetris", "Mr.Robb")
			.build()
			.expect(" ._. Could not create ggez context");

		let mut tetris = Tetris::new(ctx)
			.expect("Could not create a game");

		let points: Vec<Vec2> = vec![
			[0.0, 0.0].into(),
			[1.0, 0.0].into(),
			[0.0, 5.0].into(),
			[19.0, 3.0].into(),
			[-5.0, -7.0].into(),
		];

		let wnd_points: Vec<Vec2> = points.iter().map(|p| tetris.to_wnd(*p)).collect();
		let transformed: Vec<Vec2> = wnd_points.iter().map(|p| tetris.from_wnd(*p)).collect();

		assert_eq!(points, transformed);
	}
}
