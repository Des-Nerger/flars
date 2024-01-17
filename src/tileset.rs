use {
	crate::utils::{default, AtlasDefTOML, AtlasRegion},
	glam::{IVec2, Vec2},
	sdl2::{
		image::LoadTexture,
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
			let (image, mut tiles) = (textureCreator.load_texture(imagePath).unwrap(), Vec::new());
			let invImageDimensions = {
				let query = image.query();
				Vec2::new(query.width as _, query.height as _).recip()
			};

			/* Greater indices are likely to come latter, hence the .rev optimization. */
			for (i, srcX, srcY, srcWidth, srcHeight, offsetX, offsetY) in vec.into_iter().rev() {
				if i >= tiles.len() {
					tiles.resize_with(i + 1, default);
				}
				tiles[i] = AtlasRegion::new(
					invImageDimensions,
					Vec2::new(srcX as _, srcY as _),
					IVec2::new(srcWidth as _, srcHeight as _),
					IVec2::new(offsetX, offsetY),
				);
			}
			return Self { tiles: tiles.into_boxed_slice(), image };
		}
		unreachable!()
	}
}
