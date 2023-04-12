use {
	crate::{
		collision::*,
		utils::{default, Direction, Renderable},
	},
	core::cell::RefCell,
	glam::IVec2,
	sdl2::{
		render::{Canvas, TextureCreator},
		video::{Window, WindowContext},
	},
};

pub struct MapIso {
	pub spawn: IVec2,
	pub spawnDirection: Direction,
	pub collider: Collision,
}

impl MapIso {
	pub fn new(_screen: &RefCell<Canvas<Window>>, _textureCreator: &TextureCreator<WindowContext>) -> Self {
		Self { collider: Collision {}, spawn: default(), spawnDirection: default() }
	}

	pub fn render(&self, _r: Renderable) {}
}
