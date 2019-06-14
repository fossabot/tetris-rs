
use super::super::model::shape;
use super::super::model::board::Board;

pub type Dna = [f64; 4];

pub struct Bot {
	dna: Dna
}

impl Bot {

	pub fn new(dna: Dna) -> Self {
		Self {
			dna
		}
	}

	fn calc_score(&self, board: &Board) -> f64 {
		0.0
	}

	/// Calculates the desired position of the piece
	/// Returns a number in the range [2..9) as the x position and a Rotation.
	pub fn ask(&self, board: &Board) -> (usize, shape::Rotation) {

		let position = 4;
		let rotation= shape::Rotation::Rotate0;

		(position, rotation)
	}
}
