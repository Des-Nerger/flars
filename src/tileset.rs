use {
	crate::{
		renderer::Renderer,
		utils::{default, AtlasDefTOML, AtlasRegion},
	},
	glam::{IVec2, Vec2},
	glium::Texture2d,
	std::fs,
};

pub struct Tileset {
	pub tiles: Box<[AtlasRegion]>,
	pub image: Texture2d,
}

impl Tileset {
	pub fn new(renderer: &Renderer, tilesetPath: String) -> Self {
		let iter = toml_edit::de::from_str::<AtlasDefTOML>(&fs::read_to_string(tilesetPath).unwrap())
			.unwrap()
			.0
			.into_iter();
		assert_eq!(iter.size_hint(), (1, Some(1)));
		for (imagePath, vec) in iter {
			let (image, mut tiles) = (renderer.loadTexture2d(imagePath), Vec::new());
			let invImageDimensions = {
				let (width, height) = image.dimensions();
				Vec2::new(width as _, height as _).recip()
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
