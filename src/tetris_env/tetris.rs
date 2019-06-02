
extern crate ggez;
extern crate na;
extern crate rand;
extern crate leg;

use ggez::*;
use ggez::graphics::*;
use ggez::event::*;
use na::*;
use rand::*;

use super::shape::*;
use super::menu::*;
use super::world::*;

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
		Shape::O(0)
	}

	fn get_next(&self) -> Shape {
		let mut rng = rand::thread_rng();
		let index = rng.gen_range(1u8, 8u8);
		Shape::from_index(index).unwrap()
	}

}

pub struct TetrisScene {
	pub world: World,
	config: TetrisDisplayConfig,
	bot: Option<TetrisBot>,
	grid: MatrixMN<u8, U20, U10>,
	collector: PieceCollector,
	current_piece: (Vec2<i32>, Shape),
	score: u64,
	game_over: bool
}

impl TetrisScene {

	pub fn new(world: World) -> Self {

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
		let block_size = world.config.window_mode.height / 20.0;
		let x = world.config.window_mode.width / 2.0 - block_size * 5.0;
		let y = world.config.window_mode.height / 2.0 - block_size * 10.0;
		let w = world.config.window_mode.width;
		let h = world.config.window_mode.height;
		let background_color = (28.0 / 255.0, 28.0 / 255.0, 30.0 / 255.0, 1.0).into();

		// Build state
		TetrisScene {
			world,
			config: TetrisDisplayConfig { x, y, w, h, block_size, background_color },
			bot: None,
			grid,
			collector,
			current_piece,
			score: 0,
			game_over: false
		}
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

	fn rotate(&mut self) {
		if self.can_rotate() {
			self.current_piece.1 = self.current_piece.1.rotate_clockwise();
			let mut i = 0;
			while i < 4 && self.overlapping(0, &self.current_piece) {
				if !self.overlapping(-1, &self.current_piece) {
					self.current_piece.0.x -= 1;
				}
				else if !self.overlapping(1, &self.current_piece) {
					self.current_piece.0.x += 1;
				}
				else if (self.current_piece.1 == Shape::I(0) || self.current_piece.1 == Shape::I(2)) && !self.overlapping(-2, &self.current_piece) {
					self.current_piece.0.x -= 2;
				}
				else if (self.current_piece.1 == Shape::I(0) || self.current_piece.1 == Shape::I(2)) && !self.overlapping(2, &self.current_piece) {
					self.current_piece.0.x += 2;
				}
				i += 1;
			}
		}
	}

	// Conditions

	fn can_down(&self) -> bool {
		let x = self.current_piece.0.x as i32;
		let y = self.current_piece.0.y as i32;
		for (i, row) in self.current_piece.1.value().row_iter().enumerate() {
			for (j, cell) in row.iter().enumerate() {
				if *cell != 0 && self.grid[((y + i as i32 + 1) as usize, (x + j as i32) as usize)] != 0 {
					return false
				}
			}
		}
		true
	}

	fn can_left(&self) -> bool {
		!self.overlapping(-1, &self.current_piece)
	}

	fn can_right(&self) -> bool {
		!self.overlapping(1, &self.current_piece)
	}

	fn can_rotate(&self) -> bool {
		let mut rotated = (self.current_piece.0, self.current_piece.1.rotate_clockwise());
		let mut i = 0;
		while i < 4 && self.overlapping(0, &rotated) {
			if !self.overlapping(-1, &rotated) {
				rotated.0.x -= 1;
			}
			else if !self.overlapping(1, &rotated) {
				rotated.0.x += 1;
			}
			else if (rotated.1 == Shape::I(0) || rotated.1 == Shape::I(2)) && !self.overlapping(-2, &rotated) {
				rotated.0.x -= 2;
			}
			else if (rotated.1 == Shape::I(0) || rotated.1 == Shape::I(2)) && !self.overlapping(2, &rotated) {
				rotated.0.x += 2;
			}
			else {
				return false
			}
			i += 1;
		}
		true
	}

	// Behavior

	fn place_current_piece(&mut self) {

		let x = self.current_piece.0.x as i32;
		let y = self.current_piece.0.y as i32;

		for (i, row) in self.current_piece.1.value().row_iter().enumerate() {
			for (j, cell) in row.iter().enumerate() {
				if *cell != 0 {
					let ii = y + i as i32;
					let jj = x + j as i32;
					self.grid[(ii as usize, jj as usize)] = *cell;
					if ii == 1 {
						leg::wait("Game over", None, None);
						self.game_over = true;
					}
				}
			}
		}

		self.remove_full_lines();
	}

	fn remove_full_lines(&mut self) {

		for i in 1..self.grid.nrows() - 1 {
			let mut complete = true;
			for cell in self.grid.row(i).iter() {
				if *cell == 0 {
					complete = false;
				}
			}
			if complete {

				self.score += 50;
				leg::success("Line completed", "🍻".into(), None);
				leg::success(format!("Score: {}", self.score).as_str(), "🎉".into(), None);

				for ii in (2usize..=i).rev() {
					self.grid.swap_rows(ii, ii - 1);
				}
				self.grid.fill_row(1, 0);
				self.grid[(1, 0)] = 8u8;
				self.grid[(1, 9)] = 8u8;
			}
		}
	}

	pub fn run(&mut self) {

		let (ctx, event_loop) = &mut ContextBuilder::new("Tetris", "Mr.Robb")
			.conf(self.world.config.clone())
			.with_conf_file(true)
			.build()
			.expect(" ._. Could not create ggez context");

		event::run(ctx, event_loop, self)
			.expect("Dirty exit.");
	}

	// Helpers

	fn overlapping(&self, offset_x: i8, piece: &(Vec2<i32>, Shape)) -> bool {
		let x = piece.0.x as i8;
		let y = piece.0.y as i8;
		for i in 0..piece.1.value().nrows() {
			for j in 0..piece.1.value().ncols() {
				let ii = y + i as i8;
				let jj = x + j as i8 + offset_x;
				if piece.1.value()[(i, j)] != 0 && (jj < 0 || jj >= 9 || self.grid[(ii as usize, jj as usize)] != 0) {
					return true
				}
			}
		}
		false
	}

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

impl EventHandler for TetrisScene {

	fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

		const FPS: u32 = 2;

		while timer::check_update_time(ctx, FPS) {

			self.down();

		}

		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {

		clear(ctx, self.config.background_color);

		// Check if you lost <3
		if self.game_over {
			quit(ctx);
		}

		let mut builder = MeshBuilder::new();

		// Draw board
		for (i, row) in self.grid.row_iter().enumerate() {
			for (j, cell) in row.iter().enumerate() {

				// Get position
				let pos = self.pt_from_world_to_wnd(Vec2::new(j as f32, i as f32));
				let sz = self.config.block_size;

				// Get color
				let color: Color;
				let shape = Shape::from_index(*cell);
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

	fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {

		match keycode {
			KeyCode::Left => self.left(),
			KeyCode::Right => self.right(),
			KeyCode::Down => self.down(),
			KeyCode::Up => self.rotate(),
			_ => ()
		}
	}
}

impl From<MenuScene> for TetrisScene {
	fn from(value: MenuScene) -> TetrisScene {
		TetrisScene::new(value.world)
	}
}
