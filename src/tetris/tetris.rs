
extern crate ggez;
extern crate na;
extern crate rand;

use ggez::*;
use ggez::graphics::*;
use ggez::conf::*;
use ggez::event::*;
use na::*;
use rand::*;

use super::shape::*;

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
		let mut rng = rand::thread_rng();
		let index = rng.gen_range(1u8, 8u8);
		Shape::from_index(&index).unwrap()
	}

}

pub struct TetrisGame {
	config: TetrisDisplayConfig,
	bot: Option<TetrisBot>,
	grid: MatrixMN<u8, U20, U10>,
	collector: PieceCollector,
	current_piece: (Vec2<usize>, Shape),
}

impl TetrisGame {

	pub fn new(ctx: &mut Context) -> GameResult<TetrisGame> {

		// Initialize grid
		let mut grid: MatrixMN<u8, U20, U10> = zero();
		grid.fill_row(0, 8u8);
		grid.fill_row(19, 8u8);
		grid.fill_column(0, 8u8);
		grid.fill_column(9, 8u8);

		// Set collector with seed
		let collector = PieceCollector::new(0);
		let current_piece = ([1, 1].into(), collector.get_next());

		// Calculate values
		let block_size = ctx.conf.window_mode.height / 20.0;
		let x = ctx.conf.window_mode.width / 2.0 - block_size * 5.0;
		let y = ctx.conf.window_mode.height / 2.0 - block_size * 10.0;
		let w = ctx.conf.window_mode.width;
		let h = ctx.conf.window_mode.height;
		let background_color = (28.0 / 255.0, 28.0 / 255.0, 30.0 / 255.0, 1.0).into();

		// Build state
		Ok(TetrisGame {
			config: TetrisDisplayConfig { x, y, w, h, block_size, background_color },
			bot: None,
			grid,
			collector,
			current_piece
		})
	}

	// Commands

	fn down(&mut self) {

		if self.can_down() {

			self.current_piece.0.y += 1;

		}
		else {

			// Place in the board
			self.place_current_piece();

			// Generate new piece
			let new_shape = self.collector.get_next();
			self.current_piece = ([1, 1].into(), new_shape);
		}
	}

	fn right(&mut self) {
		if self.can_right() {
			self.current_piece.0.x += 1;
		}
	}

	fn left(&mut self) {
		if self.can_left() {
			self.current_piece.0.x -= 1;
		}
	}

	// Conditions

	fn can_down(&self) -> bool {
		let x = self.current_piece.0.x as usize;
		let y = self.current_piece.0.y as usize;
		for (i, row) in self.current_piece.1.value().row_iter().enumerate() {
			for (j, cell) in row.iter().enumerate() {
				if *cell != 0 && self.grid[(y + i + 1, x + j)] != 0 {
					return false
				}
			}
		}
		true
	}

	fn can_left(&self) -> bool {
		let x = self.current_piece.0.x as usize;
		let y = self.current_piece.0.y as usize;
		for (i, row) in self.current_piece.1.value().row_iter().enumerate() {
			for (j, cell) in row.iter().enumerate() {
				if *cell != 0 && self.grid[(y + i, x + j - 1)] != 0 {
					return false
				}
			}
		}
		true
	}

	fn can_right(&self) -> bool {
		let x = self.current_piece.0.x as usize;
		let y = self.current_piece.0.y as usize;
		for (i, row) in self.current_piece.1.value().row_iter().enumerate() {
			for (j, cell) in row.iter().enumerate() {
				if *cell != 0 && self.grid[(y + i, x + j + 1)] != 0 {
					return false
				}
			}
		}
		true
	}

	// Behavior

	fn place_current_piece(&mut self) {
		let x = self.current_piece.0.x as usize;
		let y = self.current_piece.0.y as usize;

		for (i, row) in self.current_piece.1.value().row_iter().enumerate() {
			for (j, cell) in row.iter().enumerate() {
				if *cell != 0 {
					self.grid[(y + i, x + j)] = *cell;
					if y + i == 1 {

					}
				}
			}
		}
	}

	// Helpers

	fn pt_from_wnd_to_world(&self, point: Vec2) -> Vec2 {
		let x = (point.x - self.config.x) / self.config.block_size;
		let y = (point.y - self.config.y) / self.config.block_size;
		Vec2::new(x, y)
	}

	fn pt_from_world_to_wnd(&self, point: Vec2) -> Vec2 {
		let x = self.config.x + point.x * self.config.block_size;
		let y = self.config.y + point.y * self.config.block_size;
		Vec2::new(x, y)
	}
}

impl EventHandler for TetrisGame {

	fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

		const FPS: u32 = 2;

		while timer::check_update_time(ctx, FPS) {
			self.down();
		}

		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {

		clear(ctx, self.config.background_color);

		let mut builder = MeshBuilder::new();

		// Draw board
		for (i, row) in self.grid.row_iter().enumerate() {
			for (j, cell) in row.iter().enumerate() {

				// Get position
				let pos = self.pt_from_world_to_wnd(Vec2::new(j as f32, i as f32));
				let sz = self.config.block_size;

				// Get color
				let color: Color;
				let shape = Shape::from_index(cell);
				if shape.is_some() { color = shape.unwrap().color(); }
				else if *cell == 8u8 { color = Color::new(33.0 / 255.0, 33.0 / 255.0, 35.0 / 255.0, 1.0); }
				else { color = Color::new(0.0, 0.0, 0.0, 0.0); }

				builder.circle(
					DrawMode::fill(),
					Point2::new(pos.x + sz / 2.0, pos.y + sz / 2.0),
					2.0,
					0.01,
					Color::new(37.0 / 255.0, 37.0 / 255.0, 39.0 / 255.0, 0.6)
				);

				builder.rectangle(
					DrawMode::fill(),
					Rect::new(pos.x, pos.y, sz, sz),
					color
				);
			}
		}

		// Draw current piece
		let position = &self.current_piece.0;
		let sz = self.config.block_size;
		let shape = &self.current_piece.1;
		let color = shape.color();

		for (i, row) in shape.value().row_iter().enumerate() {
			for (j, cell) in row.iter().enumerate() {

				let pos = self.pt_from_world_to_wnd(Vec2::new(position.x as f32 + j as f32, position.y as f32 + i as f32));

				// Draw cell
				builder.rectangle(
					DrawMode::fill(),
					Rect::new(pos.x, pos.y, sz, sz),
					if *cell != 0u8 { color } else { Color::new(0.0, 0.0, 0.0, 0.0) }
				);
			}
		}

		let mesh: Mesh = builder.build(ctx)
			.expect("Could not build the mesh");

		mesh.draw(ctx, DrawParam::default())
			.expect("Could not draw the mesh");

		graphics::present(ctx)
			.expect("Could not present the scene");

		Ok(())
	}

	fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {

		match keycode {
			KeyCode::Left => self.left(),
			KeyCode::Right => self.right(),
			KeyCode::Down => self.down(),
			_ => ()
		}
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

		let mut tetris = TetrisGame::new(ctx)
			.expect("Could not create a game");

		let points: Vec<Vec2> = vec![
			[0.0, 0.0].into(),
			[1.0, 0.0].into(),
			[0.0, 5.0].into(),
			[19.0, 3.0].into(),
			[-5.0, -7.0].into(),
		];

		let wnd_points: Vec<Vec2> = points.iter().map(|p| tetris.pt_from_world_to_wnd(*p)).collect();
		let transformed: Vec<Vec2> = wnd_points.iter().map(|p| tetris.pt_from_wnd_to_world(*p)).collect();

		assert_eq!(points, transformed);
	}
}
