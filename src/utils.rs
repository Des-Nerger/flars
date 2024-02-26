use {
	core::array,
	glam::{IVec2, Vec2},
	glium::Texture2d,
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
	($engine: ident = &mut GameEngine::new($renderer: ident, $input: ident)) => {
		use crate::map_iso::*;
		let map = &RefCell::new(MapIso::new($renderer));
		let $engine = &mut GameEngine::new($renderer, $input, map);
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
	pub image: &'a Texture2d,
	pub atlasRegion: &'a AtlasRegion,
}

#[derive(Clone, Copy)]
pub struct AtlasRegion {
	pub vertexOffsets: [IVec2; 4],
	pub texCoords: [Vec2; 4],
}
impl Default for AtlasRegion {
	fn default() -> Self {
		Self { vertexOffsets: default(), texCoords: array::from_fn(|_| default()) }
	}
}
impl AtlasRegion {
	pub fn new(invImageDimensions: Vec2, srcPos: Vec2, srcDimensions: IVec2, posOffset: IVec2) -> Self {
		AtlasRegion {
			vertexOffsets: [[0, 0], [srcDimensions.x, 0], [0, srcDimensions.y], srcDimensions.to_array()]
				.map(|elem| posOffset - IVec2::from_array(elem)),
			texCoords: {
				let normSrcPos = srcPos * invImageDimensions;
				let normSrcDimensions = srcDimensions.as_vec2() * invImageDimensions;
				[
					Vec2::new(normSrcPos.x, normSrcPos.y),
					Vec2::new(normSrcPos.x + normSrcDimensions.x, normSrcPos.y),
					Vec2::new(normSrcPos.x, normSrcPos.y + normSrcDimensions.y),
					Vec2::new(normSrcPos.x + normSrcDimensions.x, normSrcPos.y + normSrcDimensions.y),
				]
			},
		}
	}
}

#[derive(Deserialize)]
pub struct AtlasDefTOML(pub HashMap<String, Vec<(usize, i32, i32, u32, u32, i32, i32)>>);

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
