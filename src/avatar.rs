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
			Renderable, __,
		},
	},
	core::cell::RefCell,
	glam::IVec2,
	sdl2::{
		image::LoadTexture,
		render::{Texture, TextureCreator},
		video::WindowContext,
	},
};

enum AvatarState {
	Stance,
	Run,
	Swing,
	Block,
	Hit,
	Die,
	Cast,
	Shoot,
}

pub struct Avatar<'a> {
	sprites: Texture<'a>,
	input: &'a RefCell<InputState>,
	map: &'a RefCell<MapIso>,

	curState: AvatarState,
	pos: IVec2,
	direction: Direction,
	lockSwing: bool,
	lockCast: bool,
	lockShoot: bool,
	curFrame: i32,
	displayedFrame: i32,
	animForward: bool,
	hVSpeed: i32,
	diagSpeed: i32,
}

impl<'a> Avatar<'a> {
	pub fn new(
		textureCreator: &'a TextureCreator<WindowContext>,
		input: &'a RefCell<InputState>,
		map: &'a RefCell<MapIso>,
	) -> Self {
		let mаp;
		Self {
			input,
			map,

			// other init
			curState: AvatarState::Stance,
			pos: {
				mаp = map.borrow();
				mаp.spawn
			},
			direction: mаp.spawnDirection,

			curFrame: 1,
			displayedFrame: 0,
			animForward: true,
			lockSwing: false,
			lockCast: false,
			lockShoot: false,

			hVSpeed: 6,
			diagSpeed: 4,

			sprites: textureCreator.load_texture("images/male_sprites.png").unwrap(),
		}
	}

	pub fn pressingMove(&self) -> bool {
		let pressing = &self.input.borrow().pressing;
		[UP, DOWN, LEFT, RIGHT].iter().any(|&inputCommand| pressing[inputCommand as __])
	}

	pub fn mоve(&mut self) -> bool {
		self.map.borrow_mut().collider.mоve(
			&mut self.pos,
			IVec2::from_array(match self.direction {
				Clock09_00 => [-1, 1],
				Clock10_30 => [-1, 0],
				Clock12_00 => [-1, -1],
				Clock01_30 => [0, -1],
				Clock03_00 => [1, -1],
				Clock04_30 => [1, 0],
				Clock06_00 => [1, 1],
				Clock07_30 => [0, 1],
			}),
			[self.diagSpeed, self.hVSpeed][(self.direction as __) % 2],
		)
	}

	pub fn setDirection(&mut self) {
		// handle direction changes
		let pressing = &self.input.borrow().pressing;
		for tuple in [
			(&[UP, LEFT][..], Clock10_30),
			(&[UP, RIGHT][..], Clock01_30),
			(&[DOWN, RIGHT][..], Clock04_30),
			(&[DOWN, LEFT][..], Clock07_30),
			(&[LEFT][..], Clock09_00),
			(&[UP][..], Clock12_00),
			(&[RIGHT][..], Clock03_00),
			(&[DOWN][..], Clock06_00),
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
			Stance => {
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
						(a.curFrame, a.curState) = (1, Run);
					}
				}
			}
			Run => {
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
					a.curState = Stance;
					break;
				}
			}
			_ => {}
		}
	}

	pub fn getRender(&self) -> Renderable {
		Renderable {}
	}
}
