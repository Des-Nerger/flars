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
			Direction::{self, *},
			RectExt, Renderable, __,
		},
	},
	core::cell::RefCell,
	glam::{IVec2, IVec4},
	sdl2::{
		image::LoadTexture,
		rect::Rect,
		render::{Texture, TextureCreator},
		video::WindowContext,
	},
};

enum AvatarState {
	STANCE,
	RUN,
}

pub struct Avatar<'a, 'map> {
	sprites: Texture<'a>,
	input: &'a RefCell<InputState>,
	map: &'map RefCell<MapIso<'a>>,

	curState: AvatarState,
	pos: IVec2,
	direction: Direction,
	curFrame: i32,
	displayedFrame: i32,
	animForward: bool,
}

impl<'a, 'map> Avatar<'a, 'map> {
	pub fn new(
		textureCreator: &'a TextureCreator<WindowContext>,
		input: &'a RefCell<InputState>,
		map: &'map RefCell<MapIso<'a>>,
	) -> Self {
		let mаp;
		Self {
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

			sprites: textureCreator.load_texture("images/male_sprites.png").unwrap(),
		}
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
				loop {
					(a.curFrame, a.animForward) = if a.animForward {
						a.curFrame += 1;
						if a.curFrame >= 24 {
							(23, false)
						} else {
							break;
						}
					} else {
						a.curFrame -= 1;
						if a.curFrame <= -1 {
							(0, true)
						} else {
							break;
						}
					};
				}
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
				loop {
					if !a.pressingMove() {
					} else if !a.mоve() {
					} else {
						break;
					};
					a.curState = STANCE;
					break;
				}
			}
		}

		// calc new cam position from player position
		// cam is focused at player position
		a.map.borrow_mut().cam = a.pos;
	}

	pub fn getRender(&self) -> Renderable<'_> {
		Renderable {
			mapPos: self.pos,
			sprite: &self.sprites,
			src: Rect::fromArray((128 * IVec4::new(self.displayedFrame, self.direction as _, 1, 1)).to_array()),
			offset: IVec2::new(64, 112),
		}
	}
}
