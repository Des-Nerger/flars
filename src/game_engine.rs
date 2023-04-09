use {
	crate::{avatar::*, input_state::*, map_iso::*, utils::*},
	core::cell::RefCell,
	sdl2::{render::Canvas, video::Window},
	std::rc::Rc,
};

pub struct GameEngine<'a> {
	_canvas: &'a RefCell<Canvas<Window>>,
	input: &'a RefCell<InputState>,
	playerChar: Avatar,
	map: Rc<RefCell<MapIso>>,
	pub done: bool,
}

impl<'a> GameEngine<'a> {
	pub fn new(canvas: &'a RefCell<Canvas<Window>>, input: &'a RefCell<InputState>) -> Self {
		let map = Rc::new(RefCell::new(MapIso::new(canvas)));
		Self {
			_canvas: canvas,
			input,
			playerChar: Avatar::new(canvas, input, Rc::clone(&map)),
			map,
			done: false,
		}
	}

	/**
	 * Process all actions for a single frame
	 */
	pub fn logic(&mut self) {
		self.playerChar.logic();

		let input = &self.input.borrow();
		if input.pressing[CANCEL as __] | input.done {
			self.done = true
		}
	}

	/**
	 * Render all graphics for a single frame
	 */
	pub fn render(&self) {
		// The strategy here is to make a list of Renderables from all objects not already on the map.
		// Pass this list/array to the map, which will draw them inline with the map tiles/objects.
		self.map.borrow().render(self.playerChar.getRender());
	}
}
