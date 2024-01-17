/**
 * mod avatar
 *
 * Contains logic and rendering routines for the player avatar.
 *
 * @author Clint Bellanger, Des-Nerger
 * @license GPLv3
 */
use {
	crate::{
		input_state::{InputCommand::*, *},
		map_iso::*,
		utils::{
			default, AtlasDefTOML, AtlasRegion,
			Direction::{self, *},
			LenConst_Ext, Renderable, __,
		},
	},
	core::cell::RefCell,
	glam::{IVec2, Vec2},
	sdl2::{
		image::LoadTexture,
		render::{Texture, TextureCreator},
		video::WindowContext,
	},
	std::fs,
	strum::EnumCount,
};

enum AvatarState {
	STANCE,
	RUN,
}

const FRAME_COUNT: usize = 32;
type Sprites = [AtlasRegion; Direction::COUNT * FRAME_COUNT];

pub struct Avatar<'map, 'nonMap> {
	sprites: Sprites,
	image: Texture<'nonMap>,
	input: &'nonMap RefCell<InputState>,
	map: &'map RefCell<MapIso<'nonMap>>,

	curState: AvatarState,
	pos: IVec2,
	direction: Direction,
	curFrame: i32,
	displayedFrame: i32,
	animForward: bool,
}

impl<'map, 'nonMap> Avatar<'map, 'nonMap> {
	pub fn new(
		textureCreator: &'nonMap TextureCreator<WindowContext>,
		input: &'nonMap RefCell<InputState>,
		map: &'map RefCell<MapIso<'nonMap>>,
	) -> Self {
		let iter =
			toml_edit::de::from_str::<AtlasDefTOML>(&fs::read_to_string("atlas-defs/male-sprites.toml").unwrap())
				.unwrap()
				.0
				.into_iter();
		assert_eq!(iter.size_hint(), (1, Some(1)));
		for (imagePath, vec) in iter {
			let (image, mut sprites) = (textureCreator.load_texture(imagePath).unwrap(), [default(); Sprites::LEN]);
			let invImageDimensions = {
				let query = image.query();
				Vec2::new(query.width as _, query.height as _).recip()
			};
			for (i, srcX, srcY, srcWidth, srcHeight, offsetX, offsetY) in vec.into_iter() {
				sprites[i] = AtlasRegion::new(
					invImageDimensions,
					Vec2::new(srcX as _, srcY as _),
					IVec2::new(srcWidth as _, srcHeight as _),
					IVec2::new(offsetX, offsetY),
				);
			}
			let mаp;
			return Self {
				input,
				map,

				// other init
				curState: AvatarState::STANCE,
				pos: {
					mаp = map.borrow();
					mаp.spawn
				},
				direction: mаp.spawnDirection,

				curFrame: 1,
				displayedFrame: 0,
				animForward: true,

				sprites,
				image,
			};
		}
		unreachable!()
	}

	pub fn pressingMove(&self) -> bool {
		let pressing = &self.input.borrow().pressing;
		[UP, DOWN, LEFT, RIGHT].iter().any(|&inputCommand| pressing[inputCommand as __])
	}

	pub fn mоve(&mut self) -> bool {
		let isDiag = (self.direction as __) % 2 == 0;
		self.map.borrow_mut().collider.mоve(
			&mut self.pos,
			IVec2::from_array(match self.direction {
				CLOCK09_00 => [-1, 1],
				CLOCK10_30 => [-1, 0],
				CLOCK12_00 => [-1, -1],
				CLOCK01_30 => [0, -1],
				CLOCK03_00 => [1, -1],
				CLOCK04_30 => [1, 0],
				CLOCK06_00 => [1, 1],
				CLOCK07_30 => [0, 1],
			}),
			{
				const DIAG_SPEED: i32 = 4;
				const H_V_SPEED: i32 = 6;
				[H_V_SPEED, DIAG_SPEED][isDiag as __]
			},
			isDiag,
		)
	}

	pub fn setDirection(&mut self) {
		// handle direction changes
		let pressing = &self.input.borrow().pressing;
		for tuple in [
			(&[UP, LEFT][..], CLOCK10_30),
			(&[UP, RIGHT][..], CLOCK01_30),
			(&[DOWN, RIGHT][..], CLOCK04_30),
			(&[DOWN, LEFT][..], CLOCK07_30),
			(&[LEFT][..], CLOCK09_00),
			(&[UP][..], CLOCK12_00),
			(&[RIGHT][..], CLOCK03_00),
			(&[DOWN][..], CLOCK06_00),
		] {
			if tuple.0.iter().all(|&inputCommand| pressing[inputCommand as __]) {
				self.direction = tuple.1;
				break;
			}
		}
	}

	pub fn logic(&mut self) {
		let a /*vatar */ = self;
		use AvatarState::*;
		match a.curState {
			STANCE => {
				(|| {
					(a.curFrame, a.animForward) = if a.animForward {
						a.curFrame += 1;
						if a.curFrame <= 23 {
							return;
						}
						(23, false)
					} else {
						a.curFrame -= 1;
						if a.curFrame >= 0 {
							return;
						}
						(0, true)
					};
				})();
				a.displayedFrame = a.curFrame / 6;

				// handle transitions to RUN
				a.setDirection();
				if a.pressingMove() {
					if a.mоve() {
						(a.curFrame, a.curState) = (1, RUN);
					}
				}
			}
			RUN => {
				a.curFrame += 1;
				if a.curFrame >= 16 {
					a.curFrame = 0;
				}
				a.displayedFrame = (a.curFrame / 2) + 4;

				// handle direction changes
				a.setDirection();

				// handle transition to STANCE
				(|| {
					if !a.pressingMove() {
					} else if !a.mоve() {
					} else {
						return;
					};
					a.curState = STANCE;
				})();
			}
		}

		// calc new cam position from player position
		// cam is focused at player position
		a.map.borrow_mut().cam = a.pos;
	}

	pub fn getRender(&self) -> Renderable<'_> {
		Renderable {
			mapPos: self.pos,
			image: &self.image,
			atlasRegion: &self.sprites[self.direction as __ * FRAME_COUNT + self.displayedFrame as __],
		}
	}
}
