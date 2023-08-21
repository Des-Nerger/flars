use {
	crate::utils::{default, AtlasDefTOML, AtlasRegion},
	glam::IVec2,
	sdl2::{
		image::LoadTexture,
		rect::Rect,
		render::{Texture, TextureCreator},
		video::WindowContext,
	},
	std::fs,
};

pub struct Tileset<'a> {
	pub tiles: Box<[AtlasRegion]>,
	pub image: Texture<'a>,
}

impl<'a> Tileset<'a> {
	pub fn new(textureCreator: &'a TextureCreator<WindowContext>, tilesetPath: String) -> Self {
		let iter = toml_edit::de::from_str::<AtlasDefTOML>(&fs::read_to_string(tilesetPath).unwrap())
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
				tiles[i] = AtlasRegion {
					src: Rect::new(srcX, srcY, srcWidth, srcHeight),
					offset: IVec2::new(offsetX, offsetY),
				};
			}
			return Self {
				tiles: tiles.into_boxed_slice(),
				image: textureCreator.load_texture(imagePath).unwrap(),
			};
		}
		unreachable!()
	}
}
