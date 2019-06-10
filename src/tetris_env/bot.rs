
extern crate na;

use na::*;

use super::shape::*;
use rand::*;

pub struct TetrisBot {

}

impl TetrisBot {

	pub fn new() -> Self {
		Self {}
	}

	/// Calculates the desired position of the piece
	/// Returns a number in the range [1..8). Eight being excluded.
	pub fn ask(&self, grid: &MatrixMN<u8, U20, U10>, current: &(Vec2<i32>, Shape)) -> usize {
		let mut rng = rand::thread_rng();
		let pos = rng.gen_range(1_usize, 9_usize - current.1.w());
		pos
	}
}
