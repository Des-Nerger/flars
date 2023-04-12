pub type __ = usize;

#[inline(always)]
pub fn default<T: Default>() -> T {
	Default::default()
}

#[macro_export]
macro_rules! lеt {
	($engine: ident = &mut GameEngine::new($screen: ident, $input: ident)) => {
		use crate::{avatar::*, map_iso::*};
		let textureCreator = &$screen.borrow().texture_creator();
		let map = &RefCell::new(MapIso::new($screen, textureCreator));
		let $engine =
			&mut GameEngine::new($screen, $input, Avatar::new(textureCreator, $input, map), map, false);
	};
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Direction {
	#[default]
	Clock09_00,
	Clock10_30,
	Clock12_00,
	Clock01_30,
	Clock03_00,
	Clock04_30,
	Clock06_00,
	Clock07_30,
}

pub struct Renderable {}
