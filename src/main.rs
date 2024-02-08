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
mod renderer;
mod settings;
mod tileset;
mod utils;

use {
	core::cell::RefCell,
	glium::{Surface, Texture2d},
	glium_sdl2::DisplayBuild,
	sdl2::video::GLProfile::GLES,
	std::{
		thread,
		time::{Duration, Instant},
	},
};

fn main() {
	use {
		game_engine::*,
		input_state::*,
		renderer::{Renderer, ALMOST_BLACK},
		settings::{FPS, SCREEN_HEIGHT, SCREEN_WIDTH},
	};
	let (delay, renderer, input) = {
		let sdl2 = sdl2::init().unwrap();
		(
			Duration::from_secs(1) / FPS,
			{
				let video = &mut sdl2.video().unwrap();
				{
					let glAttr = video.gl_attr();
					glAttr.set_context_profile(GLES);
					glAttr.set_context_version(2, 0);
				}
				&mut Renderer::new(
					video
						.window(
							&format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
							SCREEN_WIDTH,
							SCREEN_HEIGHT,
						)
						.position_centered()
						// .resizable()
						.build_glium()
						.unwrap(),
				)
			},
			&RefCell::new(InputState::new(sdl2.event_pump().unwrap())),
		)
	};
	let screenTexture = &mut Texture2d::empty(&renderer.display, SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
	lеt!(engine = &mut GameEngine::new(renderer, input));
	let mut nextFrame_instant = Instant::now() + delay;
	loop {
		input.borrow_mut().handle();
		engine.logic();
		{
			let screen = &mut screenTexture.as_surface();
			screen.clear_color(ALMOST_BLACK[0], ALMOST_BLACK[1], ALMOST_BLACK[2], ALMOST_BLACK[3]);
			engine.render(screen);
		}
		{
			let mut screen = renderer.display.draw();
			screen.clear_color(ALMOST_BLACK[0], ALMOST_BLACK[1], ALMOST_BLACK[2], ALMOST_BLACK[3]);
			renderer.copy_wholeScreen(&mut screen, screenTexture);
			thread::sleep(nextFrame_instant - Instant::now());
			nextFrame_instant += delay;
			screen.finish().unwrap();
		}

		// Engine done means the user escapes the main game menu.
		// Input done means the user closes the window.
		if engine.done
		// | input.borrow().done
		{
			break;
		}
	}
}
