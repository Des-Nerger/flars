use {
	glam::IVec2,
	sdl2::{
		rect::{FPoint, Rect},
		render::Texture,
	},
	serde::Deserialize,
	std::collections::HashMap,
	strum::{EnumCount, FromRepr},
};

pub type __ = usize;

#[inline(always)]
pub fn default<T: Default>() -> T {
	Default::default()
}

#[macro_export]
macro_rules! lеt {
	($engine: ident = &mut GameEngine::new($textureCreator: ident, $input: ident)) => {
		use crate::map_iso::*;
		let map = &RefCell::new(MapIso::new($textureCreator));
		let $engine = &mut GameEngine::new($textureCreator, $input, map);
	};
}

#[macro_export]
macro_rules! unlet {
	($id: ident) => {
		#[allow(unused_variables)]
		let $id = ();
	};
}

#[derive(Clone, Copy, Debug, Default, EnumCount, FromRepr)]
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
	pub image: &'a Texture<'a>,
	pub atlasRegion: &'a AtlasRegion,
}

#[derive(Clone, Copy)]
pub struct AtlasRegion {
	pub src: Rect,
	pub offset: IVec2,
	pub texCoords: [FPoint; 4],
}
impl Default for AtlasRegion {
	fn default() -> Self {
		Self {
			src: Rect::new(default(), default(), default(), default()),
			offset: default(),
			texCoords: [IVec2::default().intо(); 4],
		}
	}
}

#[derive(Deserialize)]
pub struct AtlasDefTOML(pub HashMap<String, Vec<(usize, i32, i32, u32, u32, i32, i32)>>);

pub trait RectExt {
	fn fromArray(_: [i32; 4]) -> Self;
	// fn fromIVec2s(pos: IVec2, dimensions: IVec2) -> Self;
	fn dimensions(&self) -> IVec2;
}
impl RectExt for Rect {
	#[inline(always)]
	fn fromArray(a /*rray */: [i32; 4]) -> Self {
		Self::new(a[0], a[1], a[2] as _, a[3] as _)
	}
	/*
	#[inline(always)]
	fn fromIVec2s(pos: IVec2, dimensions: IVec2) -> Self {
		let ([x, y], [width, height]) = (pos.to_array(), dimensions.to_array());
		Self::new(x, y, width as _, height as _)
	}
	*/
	#[inline(always)]
	fn dimensions(&self) -> IVec2 {
		IVec2::new(self.width() as _, self.height() as _)
	}
}

#[allow(non_camel_case_types)]
pub trait LenConst_Ext {
	const LEN: usize;
}
impl<T, const N: usize> LenConst_Ext for [T; N] {
	const LEN: usize = N;
}

#[macro_export]
macro_rules! applyMacro {
	($ident: ident; $head: tt $(, $tail: tt )* $(,)?) => {
		$ident! $head;
		applyMacro!($ident; $( $tail ),*);
	};
	($ident: ident; ) => {};
}

macro_rules! impl_log2_log2Ceil {
	($dummyStruct: ident, $Self: ty) => {
		pub struct $dummyStruct;
		impl $dummyStruct {
			#[inline(always)]
			pub const fn log2(sеlf: $Self) -> u32 {
				<$Self>::BITS - 1 - sеlf.leading_zeros()
			}
			#[inline(always)]
			pub const fn log2Ceil(sеlf: $Self) -> u32 {
				$dummyStruct::log2(sеlf - 1) + 1
			}
		}
	};
}
applyMacro!(impl_log2_log2Ceil; (uЗ2, u32));

pub trait Intо<T> {
	fn intо(self) -> T;
}

impl Intо<FPoint> for IVec2 {
	fn intо(self) -> FPoint {
		FPoint::new(self.x as _, self.y as _)
	}
}
