
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
}

struct TetrisBot {

}

struct PieceCollector {
	rng: rngs::SmallRng,
	seed: u8,
	counter: u64
}

impl PieceCollector {

	fn new(seed: u8) -> Self {
		Self {
			rng: rngs::SmallRng::from_seed([seed; 16]),
			seed,
			counter: 0
		}
	}

	fn get_next(&mut self) -> Shape {
		let index = self.rng.gen_range(1_u8, 8_u8);
		Shape::from_index(index).unwrap()
	}
}

pub struct TetrisGame {
	config: TetrisDisplayConfig,
	bot: Option<TetrisBot>,
	grid: MatrixMN<u8, U20, U10>,
	collector: PieceCollector,
	current: (Vec2<i32>, Shape),
	score: u64,
	game_over: bool
}

impl TetrisGame {

	pub fn new(view: Rect) -> Self {

		// Initialize grid
		let mut grid: MatrixMN<u8, U20, U10> = zero();
		grid.fill_row(0, 8_u8);
		grid.fill_row(19, 8_u8);
		grid.fill_column(0, 8_u8);
		grid.fill_column(9, 8_u8);

		// Set collector with seed
		let mut collector = PieceCollector::new(0);
		let current_piece = ([1, 1].into(), collector.get_next());

		// Calculate values
		let block_size = view.h / 20.0;
		let x = view.x + view.w / 2.0 - block_size * 5.0;
		let y = view.y + view.h / 2.0 - block_size * 10.0;
		let w = view.w;
		let h = view.h;

		// Build state
		Self {
			config: TetrisDisplayConfig { x, y, w, h, block_size },
			bot: None,
			grid,
			collector,
			current: current_piece,
			score: 0,
			game_over: false
		}
	}

	// Commands

	pub fn down(&mut self) {

		if self.can_down() {

			self.current.0.y += 1;

		}
		else {

			// Place in the board
			self.place_current_piece();

			// Generate new piece
			let new_shape = self.collector.get_next();
			self.current = ([1, 1].into(), new_shape);
		}
	}

	pub fn right(&mut self) {
		if self.can_right() {
			self.current.0.x += 1;
		}
	}

	pub fn left(&mut self) {
		if self.can_left() {
			self.current.0.x -= 1;
		}
	}

	pub fn rotate(&mut self) {
		if self.can_rotate() {
			self.current.1 = self.current.1.rotate_clockwise();
			let mut i = 0;
			while i < 4 && self.overlapping(0, 0, &self.current) {
				if !self.overlapping(-1, 0, &self.current) {
					self.current.0.x -= 1;
				}
				else if !self.overlapping(1, 0, &self.current) {
					self.current.0.x += 1;
				}
				else if (self.current.1 == Shape::I(0) || self.current.1 == Shape::I(2)) && !self.overlapping(-2, 0, &self.current) {
					self.current.0.x -= 2;
				}
				else if (self.current.1 == Shape::I(0) || self.current.1 == Shape::I(2)) && !self.overlapping(2, 0, &self.current) {
					self.current.0.x += 2;
				}
				i += 1;
			}
		}
	}

	// Conditions

	fn can_down(&self) -> bool {
		!self.overlapping(0, 1, &self.current)
	}

	fn can_left(&self) -> bool {
		!self.overlapping(-1, 0, &self.current)
	}

	fn can_right(&self) -> bool {
		!self.overlapping(1, 0, &self.current)
	}

	fn can_rotate(&self) -> bool {
		let mut rotated = (self.current.0, self.current.1.rotate_clockwise());
		let mut i = 0;
		while i < 4 && self.overlapping(0, 0, &rotated) {
			if !self.overlapping(-1, 0, &rotated) {
				rotated.0.x -= 1;
			}
			else if !self.overlapping(1, 0, &rotated) {
				rotated.0.x += 1;
			}
			else if (rotated.1 == Shape::I(0) || rotated.1 == Shape::I(2)) && !self.overlapping(-2, 0, &rotated) {
				rotated.0.x -= 2;
			}
			else if (rotated.1 == Shape::I(0) || rotated.1 == Shape::I(2)) && !self.overlapping(2, 0, &rotated) {
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

		let global_offset_y = self.current.0.y;
		let global_offset_x = self.current.0.x;
		let x = self.current.1.x();
		let y = self.current.1.y();
		let w = self.current.1.w();
		let h = self.current.1.h();
		let mut slice_grid = self.grid.slice_mut(
			((global_offset_y + y as i32) as usize, (global_offset_x + x as i32) as usize),
			(h, w)
		);

		for (x1, x2) in slice_grid.iter_mut().zip(self.current.1.value().slice((y, x), (h, w)).iter()) {
			if *x2 != 0 {
				*x1 = *x2;
			}
		}

		if global_offset_y + y as i32 == 1 {
			leg::wait("Game over", None, None);
			self.game_over = true;
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
				leg::success("Line completed", "\u{1f37b}".into(), None);
				leg::success(format!("Score: {}", self.score).as_str(), "\u{1f389}".into(), None);

				for ii in (2_usize..=i).rev() {
					self.grid.swap_rows(ii, ii - 1);
				}
				self.grid.fill_row(1, 0);
				self.grid[(1, 0)] = 8_u8;
				self.grid[(1, 9)] = 8_u8;
			}
		}
	}

	pub fn update(&mut self, _ctx: &mut Context) {
		self.down()
	}

	pub fn draw(&self, ctx: &mut Context) {

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
				if shape.is_some() { color = shape.unwrap().color(); } else if *cell == 8_u8 { color = Color::new(33.0 / 255.0, 33.0 / 255.0, 35.0 / 255.0, 1.0); } else { color = Color::new(0.0, 0.0, 0.0, 0.0); }

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
		let position = &self.current.0;
		let sz = self.config.block_size;
		let shape = &self.current.1;
		let color = shape.color();

		for (i, row) in shape.value().row_iter().enumerate() {
			for (j, cell) in row.iter().enumerate() {
				let pos = self.pt_from_world_to_wnd(Vec2::new(position.x as f32 + j as f32, position.y as f32 + i as f32));

				// Draw cell
				builder.rectangle(
					DrawMode::fill(),
					Rect::new(pos.x, pos.y, sz, sz),
					if *cell == 0_u8 { Color::new(0.0, 0.0, 0.0, 0.0) } else { color }
				);
			}
		}

		let mesh: Mesh = builder.build(ctx)
			.expect("Could not build the mesh");

		mesh.draw(ctx, DrawParam::default())
			.expect("Could not draw the mesh");
	}

	pub fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
		match keycode {
			KeyCode::Left => self.left(),
			KeyCode::Right => self.right(),
			KeyCode::Down => self.down(),
			KeyCode::Up => self.rotate(),
			_ => ()
		}
	}

	// Helpers

	fn overlapping(&self, offset_x: i32, offset_y: i32, piece: &(Vec2<i32>, Shape)) -> bool {

		let global_offset_y = piece.0.y;
		let global_offset_x = piece.0.x;
		let x = piece.1.x();
		let y = piece.1.y();
		let w = piece.1.w();
		let h = piece.1.h();

		if  (global_offset_y + y as i32 + offset_y) < 0 ||
			(global_offset_x + x as i32 + offset_x) < 0 ||
			(global_offset_y + y as i32 + offset_y) >= self.grid.nrows() as i32 ||
			(global_offset_x + x as i32 + offset_x) >= self.grid.ncols() as i32 {
			return true;
		}

		let slice_grid = self.grid.slice(
			(
				(global_offset_y + y as i32 + offset_y) as usize,
				(global_offset_x + x as i32 + offset_x) as usize
			),
			(h, w)
		);

		for (x1, x2) in slice_grid.iter().zip(piece.1.value().slice((y, x), (h, w)).iter()) {
			if *x1 != 0 && *x2 != 0 {
				return true;
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

pub struct TetrisScene {
	pub world: World,
	games: Vec<TetrisGame>
}

impl TetrisScene {

	fn new(world: World) -> Self {

		let mut games = vec![];

		let col_offset = world.config.window_mode.width / world.ncols as f32;
		let row_offset = world.config.window_mode.height / world.nrows as f32;

		for i in 0..world.nrows {
			for j in  0..world.ncols {
				let rect = Rect {
					x: col_offset * j as f32,
					y: row_offset * i as f32,
					w: col_offset,
					h: row_offset
				};
				games.push(TetrisGame::new(rect));
			}
		}

		Self {
			world,
			games
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
}

impl EventHandler for TetrisScene {

	fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

		const FPS: u32 = 2;

		while timer::check_update_time(ctx, FPS) {
			for game in self.games.iter_mut() {
				game.update(ctx);
			}
		}

		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {

		let background_color = (28.0 / 255.0, 28.0 / 255.0, 30.0 / 255.0, 1.0).into();
		clear(ctx, background_color);

		for game in self.games.iter_mut() {
			game.draw(ctx);
		}

		graphics::present(ctx)
			.expect("Could not present the scene");

		Ok(())
	}

	fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
		for game in self.games.iter_mut() {
			game.key_down_event(_ctx, keycode, _keymods, _repeat);
		}
	}
}

impl From<MenuScene> for TetrisScene {
	fn from(value: MenuScene) -> TetrisScene {
		TetrisScene::new(value.world)
	}
}
