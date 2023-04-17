use {
	glam::IVec2,
	sdl2::{rect::Rect, render::Texture},
};

pub type __ = usize;

#[inline(always)]
pub fn default<T: Default>() -> T {
	Default::default()
}

#[macro_export]
macro_rules! lеt {
	($engine: ident = &mut GameEngine::new($screen: ident, $input: ident)) => {
		use crate::map_iso::*;
		let textureCreator = &$screen.borrow().texture_creator();
		let map = &RefCell::new(MapIso::new($screen, textureCreator));
		let $engine = &mut GameEngine::new(textureCreator, $input, map);
	};
}

#[derive(Clone, Copy, Debug, Default, num_enum::TryFromPrimitive)]
#[repr(i32)]
pub enum Direction {
	#[default]
	CLOCK09_00,
	CLOCK10_30,
	CLOCK12_00,
	CLOCK01_30,
	CLOCK03_00,
	CLOCK04_30,
	CLOCK06_00,
	CLOCK07_30,
}

pub struct Renderable<'a> {
	pub mapPos: IVec2,
	pub sprite: &'a Texture<'a>,
	pub src: Rect,
	pub offset: IVec2,
}

pub trait RectExt {
	fn fromArray(_: [i32; 4]) -> Self;
}
impl RectExt for Rect {
	#[inline(always)]
	fn fromArray(a /*rray */: [i32; 4]) -> Self {
		Rect::new(a[0], a[1], a[2] as _, a[3] as _)
	}
}
