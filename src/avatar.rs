use {
	crate::{input_state::*, map_iso::*, utils::Renderable},
	core::cell::RefCell,
	sdl2::{render::Canvas, video::Window},
	std::rc::Rc,
};

// AVATAR State enum
enum _AVATAR {
	Stance,
	Run,
	Swing,
	Block,
	Hit,
	Die,
	Cast,
	Shoot,
}

pub struct Avatar {
	_map: Rc<RefCell<MapIso>>,
}

impl Avatar {
	pub fn new(
		_canvas: &RefCell<Canvas<Window>>,
		_input: &RefCell<InputState>,
		map: Rc<RefCell<MapIso>>,
	) -> Self {
		Self { _map: map }
	}

	pub fn logic(&self) {}

	pub fn getRender(&self) -> Renderable {
		Renderable {}
	}
}
