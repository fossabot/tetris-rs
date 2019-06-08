
use ggez::conf::Conf;

pub struct World {
	pub config: Conf,
	pub seed: [u8; 16],
	pub nrows: usize,
	pub ncols: usize
}
