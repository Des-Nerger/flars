use {
	crate::{
		collider::*,
		tileset::*,
		utils::{default, Direction, Renderable, __},
	},
	core::{array, cell::RefCell, str::FromStr},
	glam::IVec2,
	sdl2::{
		rect::Rect,
		render::{Canvas, TextureCreator},
		video::{Window, WindowContext},
	},
	tiled_json_rs::{self as tiled, LayerType::TileLayer, TiledValue},
};

pub struct MapIso<'a> {
	screen: &'a RefCell<Canvas<Window>>,

	pub width: u32,
	pub height: u32,
	pub cam: IVec2,
	pub spawn: IVec2,
	pub spawnDirection: Direction,
	pub tileset: Tileset<'a>,

	pub background: Vec<u32>,
	pub object: Vec<u32>,
	pub collider: Collider,
}

impl<'a> MapIso<'a> {
	pub fn new(
		screen: &'a RefCell<Canvas<Window>>,
		textureCreator: &'a TextureCreator<WindowContext>,
	) -> Self {
		let (
			mut spawn,
			mut spawnDirection,
			mut tilesetPath,
			tiled::Map { width, height, properties, layers, .. },
		) = (default(), default(), default(), tiled::Map::load_from_file("map.tmj".as_ref()).unwrap());
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
					spawnDirection = Direction::try_from(iter.next().unwrap()).unwrap();
					assert_eq!(iter.next(), None);
				}
				"tileset" => {
					tilesetPath = value;
				}
				_ => {}
			}
		}
		let [mut background, mut object, mut collision] =
			array::from_fn(|_| vec![default(); (width * height) as _]);
		for (layerName, layerData) in layers.iter().filter_map(|layer| {
			if let TileLayer(tileLayer) = &layer.layer_type {
				Some((layer.name.as_str(), &tileLayer.data))
			} else {
				None
			}
		}) {
			match layerName {
				"background" => &mut background,
				"object" => &mut object,
				"collision" => &mut collision,
				_ => unreachable!(),
			}
			.copy_from_slice(layerData);
		}
		Self {
			screen,
			width,
			height,
			// cam(x,y) is where on the map the camera is pointing
			// units = 32
			cam: default(),
			spawn,
			spawnDirection,
			tileset: Tileset::new(textureCreator, tilesetPath),
			background,
			object,
			collider: Collider::new(collision),
		}
	}

	pub fn render(&mut self, r: Renderable<'_>) {
		// r will become a list of renderables.  Everything not on the map already:
		// - hero
		// - npcs and monsters
		// - loot
		// maybe, special effects
		// we want to sort these by map draw order.  Then, we use a cursor to move through the
		// renderables while we're also moving through the map tiles.  After we draw each map tile we
		// check to see if it's time to draw the next renderable yet.

		let m /*apIso */ = self;
		let screen = &mut m.screen.borrow_mut();

		// todo: trim by screen rect
		// background
		{
			let (mut ij, background) = (0, m.background.as_slice());
			for j in 0..m.height as i32 {
				for i in 0..m.width as i32 {
					let currentTile = background[ij];
					if currentTile != 0 {
						let tileDef = &m.tileset.tiles[currentTile as __];
						screen
							.copy(
								&m.tileset.sprites,
								tileDef.src,
								Rect::new(
									320 + (i * 32 - m.cam.x) - (j * 32 - m.cam.y) - tileDef.offset.x,
									240 + (i * 16 - (m.cam.x / 2)) + (j * 16 - (m.cam.y / 2)) - tileDef.offset.y,
									tileDef.src.width(),
									tileDef.src.height(),
								),
							)
							.unwrap();
					}
					ij += 1;
				}
			}
		}

		// todo: trim by screen rect
		// object layer
		{
			let (mut ij, object) = (0, m.object.as_slice());
			for j in 0..m.height as i32 {
				for i in 0..m.width as i32 {
					let currentTile = object[ij];
					if currentTile != 0 {
						let tileDef = &m.tileset.tiles[currentTile as __];
						screen
							.copy(
								&m.tileset.sprites,
								tileDef.src,
								Rect::new(
									320 + (i * 32 - m.cam.x) - (j * 32 - m.cam.y) - tileDef.offset.x,
									240 + (i * 16 - (m.cam.x / 2)) + (j * 16 - (m.cam.y / 2)) - tileDef.offset.y,
									tileDef.src.width(),
									tileDef.src.height(),
								),
							)
							.unwrap();
					}

					// entities go in this layer
					if r.mapPos.x / 32 == i && r.mapPos.y / 32 == j {
						// draw renderable
						screen
							.copy(
								r.sprite,
								r.src,
								Rect::new(320 - r.offset.x, 240 - r.offset.y, r.src.width(), r.src.height()),
							)
							.unwrap();
					}

					ij += 1;
				}
			}
		}
	}
}
