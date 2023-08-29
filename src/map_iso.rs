use {
	crate::{
		collider::*,
		settings::{SCREEN_CENTER, TILE_HEIGHT_HALF, TILE_WIDTH_HALF, UNITS_PER_TILE},
		tileset::*,
		unlet,
		utils::{default, uЗ2, AtlasRegion, Direction, Intо, RectExt, Renderable, __},
	},
	core::{array, iter, str::FromStr},
	glam::IVec2,
	sdl2::{
		pixels::RColor,
		render::{Canvas, TextureCreator, Vertex},
		video::{Window, WindowContext},
	},
	tiled_json_rs::{self as tiled, LayerType::TileLayer, TiledValue},
};
const WHITE: RColor = RColor::WHITE;

pub struct MapIso<'a> {
	pub widthLog2: u32,
	pub cam: IVec2,
	pub spawn: IVec2,
	pub spawnDirection: Direction,
	pub tileset: Tileset<'a>,

	pub background: Box<[u32]>,
	pub object: Box<[u32]>,
	pub collider: Collider,
	verticesBuf: Vec<Vertex>,
	vertIndicesBuf: Vec<u32>,
}

trait VerticesVecExt {
	fn pushRect(&mut self, atlasRegion: &AtlasRegion, pos: IVec2);
}

impl VerticesVecExt for Vec<Vertex> {
	fn pushRect(&mut self, atlasRegion: &AtlasRegion, pos: IVec2) {
		let tileDimensions = atlasRegion.src.dimensions();
		let offsetPos = pos - atlasRegion.offset;
		self.extend_from_slice(&[
			Vertex::new(offsetPos.intо(), WHITE, atlasRegion.texCoords[0]),
			Vertex::new((offsetPos + IVec2::new(tileDimensions.x, 0)).intо(), WHITE, atlasRegion.texCoords[1]),
			Vertex::new((offsetPos + IVec2::new(0, tileDimensions.y)).intо(), WHITE, atlasRegion.texCoords[2]),
			Vertex::new((offsetPos + tileDimensions).intо(), WHITE, atlasRegion.texCoords[3]),
		]);
	}
}

trait VertIndicesVecExt {
	fn ensureCount(&mut self, count: usize);
}

impl VertIndicesVecExt for Vec<u32> {
	fn ensureCount(&mut self, count: usize) {
		if self.len() < count {
			let mut v /*ertIndex */ = (self.len() / 6 * 4) as u32;
			loop {
				self.extend_from_slice(&[v + 0, v + 1, v + 2, v + 2, v + 1, v + 3]);
				v += 4;
				if self.len() == count {
					break;
				}
			}
		}
	}
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
			verticesBuf: Vec::new(),
			vertIndicesBuf: Vec::new(),
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
		let (width, height, [x0, y0], mut vertIndicesCount) = (
			1 << m.widthLog2,
			(m.background.len() >> m.widthLog2) as i32,
			[SCREEN_CENTER.x - (m.cam.x - m.cam.y), SCREEN_CENTER.y - ((m.cam.x + m.cam.y) / 2)],
			0,
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
							m.verticesBuf.pushRect(&m.tileset.tiles[currentTile as __], pos);
							vertIndicesCount += 6;
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
							m.verticesBuf.pushRect(&m.tileset.tiles[currentTile as __], pos);
							vertIndicesCount += 6;
						}

						// entities go in this layer
						if r.mapPos / UNITS_PER_TILE == IVec2::new(i, j) {
							m.vertIndicesBuf.ensureCount(vertIndicesCount);
							screen
								.geometry(&m.tileset.image, &m.verticesBuf, Some(&m.vertIndicesBuf[..vertIndicesCount]))
								.unwrap();
							m.verticesBuf.clear();
							vertIndicesCount = 0;

							// draw renderable
							m.verticesBuf.pushRect(r.atlasRegion, SCREEN_CENTER);
							screen.geometry(r.image, &m.verticesBuf, Some(&m.vertIndicesBuf[..6])).unwrap();
							m.verticesBuf.clear();
						}

						ij += 1;
						pos += IVec2::new(TILE_WIDTH_HALF, TILE_HEIGHT_HALF);
					}
				}
				pos += IVec2::new(-TILE_WIDTH_HALF, TILE_HEIGHT_HALF);
			}
		}

		m.vertIndicesBuf.ensureCount(vertIndicesCount);
		screen
			.geometry(&m.tileset.image, &m.verticesBuf, Some(&m.vertIndicesBuf[..vertIndicesCount]))
			.unwrap();
		m.verticesBuf.clear();
	}
}
