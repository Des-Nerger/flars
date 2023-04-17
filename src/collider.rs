/*
 *  collider.rs
 *  RPGEngine
 *
 *  Created by Clint Bellanger on 12/29/09.
 *  Ported to Rust by Des-Nerger on 04/_/23.
 */

use glam::IVec2;

#[derive(Debug)]
pub struct Collider {
	_collision: Vec<u32>,
}

impl Collider {
	pub fn new(_collision: Vec<u32>) -> Self {
		Self { _collision }
	}

	pub fn mоve(&self, pos: &mut IVec2, step: IVec2, dist: i32) -> bool {
		*pos += dist * step;
		true
	}
}
