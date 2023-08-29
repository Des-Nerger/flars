use {
	crate::utils::{default, AtlasDefTOML, AtlasRegion},
	glam::IVec2,
	sdl2::{
		image::LoadTexture,
		rect::{FPoint, Rect},
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
			let [invImageWidth, invImageHeight] = {
				let query = image.query();
				[1. / (query.width as f32), 1. / (query.height as f32)]
			};

			/* Greater indices are likely to come latter, hence the .rev optimization. */
			for (i, srcX, srcY, srcWidth, srcHeight, offsetX, offsetY) in vec.into_iter().rev() {
				if i >= tiles.len() {
					tiles.resize_with(i + 1, default);
				}
				let src = Rect::new(srcX, srcY, srcWidth, srcHeight);
				let normSrc = (
					(srcX as f32) * invImageWidth,
					(srcY as f32) * invImageHeight,
					(srcWidth as f32) * invImageWidth - f32::EPSILON,
					(srcHeight as f32) * invImageHeight - f32::EPSILON,
				);
				tiles[i] = AtlasRegion {
					src,
					offset: IVec2::new(offsetX, offsetY),
					texCoords: [
						FPoint::new(normSrc.0, normSrc.1),
						FPoint::new(normSrc.0 + normSrc.2, normSrc.1),
						FPoint::new(normSrc.0, normSrc.1 + normSrc.3),
						FPoint::new(normSrc.0 + normSrc.2, normSrc.1 + normSrc.3),
					],
				};
			}
			return Self { tiles: tiles.into_boxed_slice(), image };
		}
		unreachable!()
	}
}
