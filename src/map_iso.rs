use {
	crate::utils::Renderable,
	core::cell::RefCell,
	sdl2::{render::Canvas, video::Window},
};

pub struct MapIso {}

impl MapIso {
	pub fn new(_canvas: &RefCell<Canvas<Window>>) -> Self {
		Self {}
	}

	pub fn render(&self, _r: Renderable) {}
}
