#![warn(clippy::pedantic, elided_lifetimes_in_paths, explicit_outlives_requirements)]
#![allow(
	non_camel_case_types,
	non_snake_case,
	confusable_idents,
	mixed_script_confusables,
	uncommon_codepoints
)]

mod avatar;
mod game_engine;
mod input_state;
mod map_iso;
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
	use {game_engine::*, input_state::*};

	const FPS: u32 = 24;
	let (delay, input, canvas) = {
		let sdl2Context = sdl2::init().unwrap();
		(
			Duration::from_secs(1) / FPS,
			&RefCell::new(InputState::new(sdl2Context.event_pump().unwrap())),
			&RefCell::new(
				sdl2Context
					.video()
					.unwrap()
					.window("", 640, 480)
					.position_centered()
					.build()
					.unwrap()
					.into_canvas()
					.build()
					.unwrap(),
			),
		)
	};
	let (mut nextFrame_instant, engine) = (Instant::now() + delay, &mut GameEngine::new(canvas, input));
	{
		let (canvas, input) = &mut (canvas.borrow_mut(), input.borrow_mut());
		canvas.set_draw_color(Color::RGB(0x00, 0x00, 0x00));
		loopIterationBeginning(canvas, input);
	}
	fn loopIterationBeginning(canvas: &mut RefMut<'_, Canvas<Window>>, input: &mut RefMut<'_, InputState>) {
		// black out
		canvas.clear();

		input.handle();
	}
	loop {
		engine.logic();
		engine.render();

		thread::sleep(nextFrame_instant - Instant::now());
		nextFrame_instant += delay;

		let (canvas, input) = &mut (canvas.borrow_mut(), input.borrow_mut());
		canvas.present();

		// Engine done means the user escapes the main game menu.
		// Input done means the user closes the window.
		if engine.done
		// | input.done
		{
			break;
		}

		loopIterationBeginning(canvas, input);
	}
}
