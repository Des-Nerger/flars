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
	sdl2::pixels::RColor,
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
			&mut sdl2
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
			&RefCell::new(InputState::new(sdl2.event_pump().unwrap())),
		)
	};
	screen.set_logical_size(SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
	screen.set_draw_color(RColor::RGB(0xC, 0xC, 0xC));
	let textureCreator = &screen.texture_creator();
	sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");
	let screenTexture = &mut textureCreator
		.create_texture_target(textureCreator.default_pixel_format(), SCREEN_WIDTH, SCREEN_HEIGHT)
		.unwrap();
	sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "0");
	lеt!(engine = &mut GameEngine::new(textureCreator, input));
	unlet!(textureCreator);
	let mut nextFrame_instant = Instant::now() + delay;
	loopIterationBeginning(&mut input.borrow_mut());
	fn loopIterationBeginning(input: &mut RefMut<'_, InputState>) {
		input.handle();
	}
	loop {
		engine.logic();
		screen
			.with_texture_canvas(screenTexture, |screen| {
				screen.clear();
				engine.render(screen);
			})
			.unwrap();
		screen.clear();
		screen.copy(screenTexture, None, None).unwrap();

		thread::sleep(nextFrame_instant - Instant::now());
		nextFrame_instant += delay;

		screen.present();
		let input = &mut input.borrow_mut();

		// Engine done means the user escapes the main game menu.
		// Input done means the user closes the window.
		if engine.done
		// | input.done
		{
			break;
		}

		loopIterationBeginning(input);
	}
}
