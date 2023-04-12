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
	sdl2::{render::Canvas, video::Window},
};

pub struct GameEngine<'a> {
	screen: &'a RefCell<Canvas<Window>>,
	input: &'a RefCell<InputState>,
	playerChar: Avatar<'a>,
	map: &'a RefCell<MapIso>,
	pub done: bool,
}

impl<'a> GameEngine<'a> {
	/**
	 * Passthrough constructor; just to avoid publicizing all the fields.
	 * For the actual one, see [`lеt!(... = &mut GameEngine::new(...))`].
	 *
	 * [`lеt!(... = &mut GameEngine::new(...))`]: crate::lеt
	 */
	#[inline(always)]
	pub fn new(
		screen: &'a RefCell<Canvas<Window>>,
		input: &'a RefCell<InputState>,
		playerChar: Avatar<'a>,
		map: &'a RefCell<MapIso>,
		done: bool,
	) -> Self {
		GameEngine { screen, input, playerChar, map, done }
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
