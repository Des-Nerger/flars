use {
	crate::utils::default,
	glam::IVec2,
	sdl2::{
		image::LoadTexture,
		rect::Rect,
		render::{Texture, TextureCreator},
		video::WindowContext,
	},
	serde::Deserialize,
	std::{collections::HashMap, fs},
};

pub struct Tileset<'a> {
	pub tiles: Vec<TileDef>,
	pub sprites: Texture<'a>,
}

pub struct TileDef {
	pub src: Rect,
	pub offset: IVec2,
}

impl Default for TileDef {
	fn default() -> Self {
		Self { src: Rect::new(default(), default(), default(), default()), offset: default() }
	}
}

impl<'a> Tileset<'a> {
	pub fn new(textureCreator: &'a TextureCreator<WindowContext>, tilesetPath: String) -> Self {
		#[derive(Deserialize)]
		struct TilesetTOML(HashMap<String, Vec<(usize, i32, i32, u32, u32, i32, i32)>>);
		let iter = toml_edit::de::from_str::<TilesetTOML>(&fs::read_to_string(tilesetPath).unwrap())
			.unwrap()
			.0
			.into_iter();
		assert_eq!(iter.size_hint(), (1, Some(1)));
		for (imagePath, vec) in iter {
			let mut tiles = Vec::new();

			/* Greater indices are likely to come latter, hence the .rev optimization. */
			for (i, srcX, srcY, srcWidth, srcHeight, offsetX, offsetY) in vec.into_iter().rev() {
				if i >= tiles.len() {
					tiles.resize_with(i + 1, default);
				}
				tiles[i] =
					TileDef { src: Rect::new(srcX, srcY, srcWidth, srcHeight), offset: IVec2::new(offsetX, offsetY) }
			}
			return Self { tiles, sprites: textureCreator.load_texture(imagePath).unwrap() };
		}
		unreachable!()
	}
}
