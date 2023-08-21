use {
	crate::{
		collider::*,
		settings::{SCREEN_CENTER, TILE_HEIGHT_HALF, TILE_WIDTH_HALF, UNITS_PER_TILE},
		tileset::*,
		unlet,
		utils::{default, uЗ2, Direction, RectExt, Renderable, __},
	},
	core::{array, iter, str::FromStr},
	glam::IVec2,
	sdl2::{
		rect::Rect,
		render::{Canvas, TextureCreator},
		video::{Window, WindowContext},
	},
	tiled_json_rs::{self as tiled, LayerType::TileLayer, TiledValue},
};

pub struct MapIso<'a> {
	pub widthLog2: u32,
	pub cam: IVec2,
	pub spawn: IVec2,
	pub spawnDirection: Direction,
	pub tileset: Tileset<'a>,

	pub background: Box<[u32]>,
	pub object: Box<[u32]>,
	pub collider: Collider,
}

impl<'a> MapIso<'a> {
	pub fn new(textureCreator: &'a TextureCreator<WindowContext>) -> Self {
		let (
			mut spawn,
			mut spawnDirection,
			mut tilesetPath,
			tiled::Map { width, height, properties, layers, .. },
		) = (default(), default(), default(), tiled::Map::load_from_file("map.tiled.json".as_ref()).unwrap());
		let widthLog2Ceil = uЗ2::log2Ceil(width);
		for (key, value) in properties.into_iter().filter_map(|(key, value)| {
			if let TiledValue::String(value) = value {
				Some((key, value))
			} else {
				None
			}
		}) {
			match key.as_str() {
				"spawnpoint" => {
					let mut iter = value.split(',').map(|s| i32::from_str(s).unwrap());
					spawn = IVec2::from_array(array::from_fn(|_| iter.next().unwrap()));
					spawnDirection = Direction::from_repr(iter.next().unwrap()).unwrap();
					assert_eq!(iter.next(), None);
				}
				"tileset" => {
					tilesetPath = value;
				}
				_ => {}
			}
		}
		let [mut background, mut object, mut colmap] =
			array::from_fn(|_| vec![default(); (height << widthLog2Ceil) as _].into_boxed_slice());
		{
			let [width, pow2Width] = [width as __, 1 << widthLog2Ceil];
			unlet!(widthLog2Ceil);
			for (layerName, srcData) in layers.into_iter().filter_map(|layer| {
				if let TileLayer(tileLayer) = layer.layer_type {
					Some((layer.name, tileLayer.data.into_boxed_slice()))
				} else {
					None
				}
			}) {
				let destData = match layerName.as_str() {
					"background" => &mut background,
					"object" => &mut object,
					"collision" => &mut colmap,
					_ => unreachable!(),
				};
				for (idxDest, idxSrc) in
					iter::zip((0..destData.len()).step_by(pow2Width), (0..srcData.len()).step_by(width))
				{
					(&mut destData[idxDest..][..width]).copy_from_slice(&srcData[idxSrc..][..width]);
				}
			}
		}
		Self {
			widthLog2: widthLog2Ceil,
			// cam(x,y) is where on the map the camera is pointing
			cam: default(),
			spawn,
			spawnDirection,
			tileset: Tileset::new(textureCreator, tilesetPath),
			background,
			object,
			collider: Collider::new(colmap, widthLog2Ceil as _),
		}
	}

	pub fn render(&mut self, screen: &mut Canvas<Window>, r: Renderable<'_>) {
		// r will become a list of renderables.  Everything not on the map already:
		// - hero
		// - npcs and monsters
		// - loot
		// maybe, special effects
		// we want to sort these by map draw order.  Then, we use a cursor to move through the
		// renderables while we're also moving through the map tiles.  After we draw each map tile we
		// check to see if it's time to draw the next renderable yet.

		let m /*apIso */ = self;
		let (width, height, [x0, y0]) = (
			1 << m.widthLog2,
			(m.background.len() >> m.widthLog2) as i32,
			[SCREEN_CENTER.x - (m.cam.x - m.cam.y), SCREEN_CENTER.y - ((m.cam.x + m.cam.y) / 2)],
		);
		const NO_TILE: u32 = 0;

		// todo: trim by screen rect
		// background
		{
			let (mut ij, mut pos) = (0, IVec2::new(x0, y0));
			for _j in 0..height {
				{
					let mut pos = pos;
					for _i in 0..width {
						let currentTile = m.background[ij];
						if currentTile != NO_TILE {
							let tileDef = &m.tileset.tiles[currentTile as __];
							screen
								.copy(
									&m.tileset.image,
									tileDef.src,
									Rect::fromIVec2s(pos - tileDef.offset, tileDef.src.dimensions()),
								)
								.unwrap();
						}
						ij += 1;
						pos += IVec2::new(TILE_WIDTH_HALF, TILE_HEIGHT_HALF);
					}
				}
				pos += IVec2::new(-TILE_WIDTH_HALF, TILE_HEIGHT_HALF);
			}
		}

		// todo: trim by screen rect
		// object layer
		{
			let (mut ij, mut pos) = (0, IVec2::new(x0, y0));
			for j in 0..height {
				{
					let mut pos = pos;
					for i in 0..width {
						let currentTile = m.object[ij];
						if currentTile != NO_TILE {
							let tileDef = &m.tileset.tiles[currentTile as __];
							screen
								.copy(
									&m.tileset.image,
									tileDef.src,
									Rect::fromIVec2s(pos - tileDef.offset, tileDef.src.dimensions()),
								)
								.unwrap();
						}

						// entities go in this layer
						if r.mapPos / UNITS_PER_TILE == IVec2::new(i, j) {
							// draw renderable
							screen
								.copy(r.image, r.src, Rect::fromIVec2s(SCREEN_CENTER - r.offset, r.src.dimensions()))
								.unwrap();
						}

						ij += 1;
						pos += IVec2::new(TILE_WIDTH_HALF, TILE_HEIGHT_HALF);
					}
				}
				pos += IVec2::new(-TILE_WIDTH_HALF, TILE_HEIGHT_HALF);
			}
		}
	}
}
