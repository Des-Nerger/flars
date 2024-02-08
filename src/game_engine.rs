/**
 * mod game_engine
 *
 * Hands off the logic and rendering for the current game mode
 *
 * @author Clint Bellanger, Des-Nerger
 * @license GPLv3
 */
use {
	crate::{avatar::*, input_state::*, map_iso::*, renderer::Renderer, utils::__},
	core::cell::RefCell,
	glium::Surface,
};

pub struct GameEngine<'map, 'nonMap> {
	input: &'nonMap RefCell<InputState>,
	playerChar: Avatar<'map, 'nonMap>,
	map: &'map RefCell<MapIso<'nonMap>>,
	pub done: bool,
}

impl<'map, 'nonMap> GameEngine<'map, 'nonMap> {
	/**
	 * Not meant to be used directly, but rather through the [`lеt!(_ = &mut GameEngine::new(..))`] macro.
	 *
	 * [`lеt!(_ = &mut GameEngine::new(..))`]: crate::lеt
	 */
	pub fn new(
		renderer: &'nonMap Renderer,
		input: &'nonMap RefCell<InputState>,
		map: &'map RefCell<MapIso<'nonMap>>,
	) -> Self {
		GameEngine { input, playerChar: Avatar::new(renderer, input, map), map, done: false }
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
	pub fn render(&self, screen: &mut impl Surface) {
		// The strategy here is to make a list of Renderables from all objects not already on the map.
		// Pass this list/array to the map, which will draw them inline with the map tiles/objects.
		self.map.borrow_mut().render(screen, self.playerChar.getRender());
	}
}
