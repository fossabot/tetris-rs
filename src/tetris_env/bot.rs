
extern crate na;

use na::*;

use super::shape::*;

pub struct TetrisBot {

}

impl TetrisBot {

	pub fn new() -> Self {
		Self {}
	}

	/// Calculates the desired position of the piece
	/// Returns a number in the range [1..8). Eight being excluded.
	fn ask(grid: &MatrixMN<u8, U20, U10>, current: (Vec2<i32>, Shape)) -> u8 {
		0_u8
	}
}
