use {
	crate::utils::__,
	ary::ary,
	core::array,
	sdl2::{event::Event, keyboard::Scancode, EventPump},
};

#[derive(Clone, Copy)]
#[repr(usize)]
pub enum InputCommand {
	CANCEL,
	ACCEPT,
	UP,
	DOWN,
	LEFT,
	RIGHT,
	KEY_COUNT,
}
pub use InputCommand::*;

pub struct InputState {
	eventPump: EventPump,
	pub pressing: [bool; KEY_COUNT as _],
	pub done: bool,
}

impl InputState {
	const BINDING: [Scancode; KEY_COUNT as _] = {
		ary![=>
			(CANCEL as __): Scancode::Escape,
			(ACCEPT as __): Scancode::Return,
			(UP as __): Scancode::Up,
			(DOWN as __): Scancode::Down,
			(LEFT as __): Scancode::Left,
			(RIGHT as __): Scancode::Right,
		]
	};

	pub fn new(eventPump: EventPump) -> Self {
		Self { eventPump, pressing: array::from_fn(|_| false), done: false }
	}

	pub fn handle(&mut self) {
		// Check for events
		for event in self.eventPump.poll_iter() {
			use Event::*;
			match event {
				KeyDown { scancode: Some(scancode), .. } => {
					for key in 0..KEY_COUNT as _ {
						if scancode == Self::BINDING[key] {
							self.pressing[key] = true;
							break;
						}
					}
				}
				KeyUp { scancode: Some(scancode), .. } => {
					for key in 0..KEY_COUNT as _ {
						if scancode == Self::BINDING[key] {
							self.pressing[key] = false;
							break;
						}
					}
				}
				Quit { .. } => self.done = true,
				_ => {}
			}
		}
	}
}
