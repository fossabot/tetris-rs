
use ggez::conf::Conf;

pub struct World {
	pub nrows: usize,
	pub ncols: usize,
	pub has_player: bool,
	pub config: Conf,
	pub seed: [u8; 16]
}
