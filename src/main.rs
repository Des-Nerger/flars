#![windows_subsystem = "windows"]
#![warn(clippy::pedantic, elided_lifetimes_in_paths, explicit_outlives_requirements)]
#![allow(
	confusable_idents,
	mixed_script_confusables,
	non_camel_case_types,
	non_snake_case,
	uncommon_codepoints
)]

mod avatar;
mod collider;
mod game_engine;
mod input_state;
mod map_iso;
mod settings;
mod tileset;
mod utils;

use {
	core::cell::{RefCell, RefMut},
	sdl2::{pixels::Color, render::Canvas, video::Window},
	std::{
		thread,
		time::{Duration, Instant},
	},
};

fn main() {
	use {
		game_engine::*,
		input_state::*,
		settings::{FPS, SCREEN_HEIGHT, SCREEN_WIDTH},
	};
	let (delay, screen, input) = {
		let sdl2 = sdl2::init().unwrap();
		(
			Duration::from_secs(1) / FPS,
			&RefCell::new(
				sdl2
					.video()
					.unwrap()
					.window(
						&format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
						SCREEN_WIDTH,
						SCREEN_HEIGHT,
					)
					.position_centered()
					.resizable()
					.build()
					.unwrap()
					.into_canvas()
					.build()
					.unwrap(),
			),
			&RefCell::new(InputState::new(sdl2.event_pump().unwrap())),
		)
	};
	lеt!(engine = &mut GameEngine::new(screen, input));
	let mut nextFrame_instant = Instant::now() + delay;
	{
		let (screen, input) = &mut (screen.borrow_mut(), input.borrow_mut());
		screen.set_integer_scale(true).unwrap();
		screen.set_logical_size(SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
		screen.set_draw_color(Color::RGB(0xC, 0xC, 0xC));
		loopIterationBeginning(screen, input);
	}
	fn loopIterationBeginning(screen: &mut RefMut<'_, Canvas<Window>>, input: &mut RefMut<'_, InputState>) {
		// black out
		screen.clear();

		input.handle();
	}
	loop {
		engine.logic();
		engine.render();

		thread::sleep(nextFrame_instant - Instant::now());
		nextFrame_instant += delay;

		let (screen, input) = &mut (screen.borrow_mut(), input.borrow_mut());
		screen.present();

		// Engine done means the user escapes the main game menu.
		// Input done means the user closes the window.
		if engine.done
		// | input.done
		{
			break;
		}

		loopIterationBeginning(screen, input);
	}
}
