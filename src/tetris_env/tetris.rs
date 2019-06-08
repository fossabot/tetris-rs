
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
use super::board::*;
use super::bot::*;

struct TetrisDisplayConfig {
	x: f32,
	y: f32,
	w: f32,
	h: f32,
	block_size: f32,
}

pub struct TetrisGame {
	config: TetrisDisplayConfig,
	board: TetrisBoard,
	bot: Option<TetrisBot>,
}

impl TetrisGame {

	pub fn new(view: Rect, seed: [u8; 16]) -> Self {

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
			board: TetrisBoard::new(view, seed),
		}
	}

	pub fn update(&mut self, _ctx: &mut Context) {
		self.board.down()
	}

	pub fn draw(&self, ctx: &mut Context) {

		// Check if you lost <3
		if self.board.game_over {
			quit(ctx);
		}

		let mut builder = MeshBuilder::new();

		// Draw board
		for (i, row) in self.board.grid.row_iter().enumerate() {
			for (j, cell) in row.iter().enumerate() {

				// Get position
				let pos = self.pt_from_world_to_wnd(Vec2::new(j as f32, i as f32));
				let sz = self.config.block_size;

				// Get color
				let color: Color;
				let shape = Shape::from_index(*cell);

				if shape.is_some() { color = shape.unwrap().color(); }
				else if *cell == 8_u8 { color = Color::new(33.0 / 255.0, 33.0 / 255.0, 35.0 / 255.0, 1.0); }
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
		let position = &self.board.current.0;
		let sz = self.config.block_size;
		let shape = &self.board.current.1;
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
			KeyCode::Left => self.board.left(),
			KeyCode::Right => self.board.right(),
			KeyCode::Down => self.board.down(),
			KeyCode::Up => self.board.rotate(),
			_ => ()
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
				games.push(TetrisGame::new(rect, world.seed));
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
