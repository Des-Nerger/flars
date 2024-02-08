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
		renderer::Renderer,
		utils::{
			default, AtlasDefTOML, AtlasRegion,
			Direction::{self, *},
			LenConst_Ext, Renderable, __,
		},
	},
	core::cell::RefCell,
	glam::{IVec2, Vec2},
	glium::Texture2d,
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
	image: Texture2d,
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
		renderer: &'nonMap Renderer,
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
			let (image, mut sprites) = (renderer.loadTexture2d(imagePath), [default(); Sprites::LEN]);
			let invImageDimensions = {
				let (width, height) = image.dimensions();
				Vec2::new(width as _, height as _).recip()
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
		let o = self;
		use AvatarState::*;
		match o.curState {
			STANCE => {
				(|| {
					(o.curFrame, o.animForward) = if o.animForward {
						o.curFrame += 1;
						if o.curFrame <= 23 {
							return;
						}
						(23, false)
					} else {
						o.curFrame -= 1;
						if o.curFrame >= 0 {
							return;
						}
						(0, true)
					};
				})();
				o.displayedFrame = o.curFrame / 6;

				// handle transitions to RUN
				o.setDirection();
				if o.pressingMove() {
					if o.mоve() {
						(o.curFrame, o.curState) = (1, RUN);
					}
				}
			}
			RUN => {
				o.curFrame += 1;
				if o.curFrame >= 16 {
					o.curFrame = 0;
				}
				o.displayedFrame = (o.curFrame / 2) + 4;

				// handle direction changes
				o.setDirection();

				// handle transition to STANCE
				(|| {
					if !o.pressingMove() {
					} else if !o.mоve() {
					} else {
						return;
					};
					o.curState = STANCE;
				})();
			}
		}

		// calc new cam position from player position
		// cam is focused at player position
		o.map.borrow_mut().cam = o.pos;
	}

	pub fn getRender(&self) -> Renderable<'_> {
		Renderable {
			mapPos: self.pos,
			image: &self.image,
			atlasRegion: &self.sprites[self.direction as __ * FRAME_COUNT + self.displayedFrame as __],
		}
	}
}
