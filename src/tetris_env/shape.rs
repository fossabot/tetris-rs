
extern crate ggez;
extern crate na;

use ggez::graphics::*;
use na::*;

pub enum Shape { I, J, L, O, S, T, Z }

impl Shape {

	pub fn value(&self) -> DMatrix<u8> {
		match *self {
			Shape::I => DMatrix::from_row_slice(4, 4, &[ 0,0,0,0, 1,1,1,1, 0,0,0,0, 0,0,0,0 ]),
			Shape::J => DMatrix::from_row_slice(3, 3, &[ 2,0,0,   2,2,2,   0,0,0            ]),
			Shape::L => DMatrix::from_row_slice(3, 3, &[ 0,0,3,   3,3,3,   0,0,0            ]),
			Shape::O => DMatrix::from_row_slice(2, 2, &[ 4,4,     4,4                       ]),
			Shape::S => DMatrix::from_row_slice(3, 3, &[ 0,5,5,   5,5,0,   0,0,0            ]),
			Shape::T => DMatrix::from_row_slice(3, 3, &[ 0,6,0,   6,6,6,   0,0,0            ]),
			Shape::Z => DMatrix::from_row_slice(3, 3, &[ 7,7,0,   0,7,7,   0,0,0            ])
		}
	}

	pub fn color(&self) -> Color {
		match *self {
			Shape::I => Color::from_rgb(249, 35, 56),
			Shape::J => Color::from_rgb(201, 115, 255),
			Shape::L => Color::from_rgb(28, 118, 188),
			Shape::O => Color::from_rgb(254, 227, 86),
			Shape::S => Color::from_rgb(83, 213, 4),
			Shape::T => Color::from_rgb(54, 224, 255),
			Shape::Z => Color::from_rgb(248, 147, 29)
		}
	}

	pub fn x(&self) -> u8 {
		for (i, col) in self.value().column_iter().enumerate() {
			for cell in col.iter() {
				if *cell > 0u8 {
					return i as u8
				}
			}
		}
		0u8
	}

	pub fn y(&self) -> u8 {
		for (i, row) in self.value().row_iter().enumerate() {
			for cell in row.iter() {
				if *cell > 0u8 {
					return i as u8
				}
			}
		}
		0u8
	}

	pub fn w(&self) -> u8 {
		let mut width = self.value().ncols() as u8;
		for col in self.value().column_iter() {
			let mut count = 0;
			for cell in col.iter() {
				count += if *cell != 0 { 1 } else { 0 };
			}
			if count == 0 {
				width -= 1;
			}
		}
		width
	}

	pub fn h(&self) -> u8 {
		let mut height = self.value().ncols() as u8;
		for row in self.value().row_iter() {
			let mut count = 0;
			for cell in row.iter() {
				count += if *cell != 0 { 1 } else { 0 };
			}
			if count == 0 {
				height -= 1;
			}
		}
		height
	}

	pub fn rotate(shape: &mut DMatrix<u8>) {

		let n = shape.nrows() - 1;

		for i in 0..=(n / 2) {
			for j in i..(n - i) {
				let item1 = *shape.index((i, j));
				let item2 = *shape.index((j, n - i));
				let item3 = *shape.index((n - i, n - j));
				let item4 = *shape.index((n - j, i));
				*shape.index_mut((j, n - i)) = item1;
				*shape.index_mut((n - i, n - j)) = item2;
				*shape.index_mut((n - j, i)) = item3;
				*shape.index_mut((i, j)) = item4;
			}
		}
	}

	pub fn from_index(index: u8) -> Option<Shape> {
		match index {
			1 => Some(Shape::I),
			2 => Some(Shape::J),
			3 => Some(Shape::L),
			4 => Some(Shape::O),
			5 => Some(Shape::S),
			6 => Some(Shape::T),
			7 => Some(Shape::Z),
			_ => None
		}
	}
}

#[cfg(test)]
mod test {

	use super::*;

	#[test]
	fn rotate_test() {
		let mut t = Shape::T.value();
		for _ in 0..4 {
			Shape::rotate(&mut t);
		}
		assert_eq!(t, Shape::T.value());
	}

	#[test]
	fn shape_x_test() {
		assert_eq!(0, Shape::I.x());
		assert_eq!(0, Shape::J.x());
		assert_eq!(0, Shape::L.x());
		assert_eq!(0, Shape::S.x());
	}

	#[test]
	fn shape_y_test() {
		assert_eq!(1, Shape::I.y());
		assert_eq!(0, Shape::J.y());
		assert_eq!(0, Shape::L.y());
		assert_eq!(0, Shape::S.y());
	}

	#[test]
	fn shape_w_test() {
		assert_eq!(4, Shape::I.w());
		assert_eq!(3, Shape::J.w());
		assert_eq!(3, Shape::L.w());
		assert_eq!(3, Shape::S.w());
	}

	#[test]
	fn shape_h_test() {
		assert_eq!(1, Shape::I.h());
		assert_eq!(2, Shape::J.h());
		assert_eq!(2, Shape::L.h());
		assert_eq!(2, Shape::S.h());
	}
}
