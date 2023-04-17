/**
 * mod game_engine
 *
 * Hands off the logic and rendering for the current game mode
 *
 * @author Clint Bellanger, Des-Nerger
 * @license GPLv3
 */
use {
	crate::{avatar::*, input_state::*, map_iso::*, utils::__},
	core::cell::RefCell,
	sdl2::{render::TextureCreator, video::WindowContext},
};

pub struct GameEngine<'a, 'map> {
	input: &'a RefCell<InputState>,
	playerChar: Avatar<'a, 'map>,
	map: &'map RefCell<MapIso<'a>>,
	pub done: bool,
}

impl<'a, 'map> GameEngine<'a, 'map> {
	/**
	 * Not meant to be used directly, but rather through the [`lеt!(_ = &mut GameEngine::new(..))`] macro.
	 *
	 * [`lеt!(_ = &mut GameEngine::new(..))`]: crate::lеt
	 */
	pub fn new(
		textureCreator: &'a TextureCreator<WindowContext>,
		input: &'a RefCell<InputState>,
		map: &'map RefCell<MapIso<'a>>,
	) -> Self {
		GameEngine { input, playerChar: Avatar::new(textureCreator, input, map), map, done: false }
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
		self.map.borrow_mut().render(self.playerChar.getRender());
	}
}
