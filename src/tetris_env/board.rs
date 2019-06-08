
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
use super::bot::*;

struct PieceCollector {
	rng: rngs::SmallRng,
}

impl PieceCollector {

	fn new(seed: [u8; 16]) -> Self {
		Self {
			rng: rngs::SmallRng::from_seed(seed),
		}
	}

	fn get_next(&mut self) -> Shape {
		let index = self.rng.gen_range(1_u8, 8_u8);
		Shape::from_index(index).unwrap()
	}
}

pub struct TetrisBoard {
	pub grid: MatrixMN<u8, U20, U10>,
	collector: PieceCollector,
	pub current: (Vec2<i32>, Shape),
	pub score: u64,
	pub game_over: bool
}

impl TetrisBoard {

	pub fn new(view: Rect, seed: [u8; 16]) -> Self {

		// Initialize grid
		let mut grid: MatrixMN<u8, U20, U10> = zero();
		grid.fill_row(0, 8_u8);
		grid.fill_row(19, 8_u8);
		grid.fill_column(0, 8_u8);
		grid.fill_column(9, 8_u8);

		// Set collector with seed
		let mut collector = PieceCollector::new(seed);
		let current_piece = ([1, 1].into(), collector.get_next());

		// Build state
		Self {
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
}
