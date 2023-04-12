/*
 *  collision.rs
 *  RPGEngine
 *
 *  Created by Clint Bellanger on 12/29/09.
 *  Ported to Rust by Des-Nerger on 04/10/23.
 */

use glam::IVec2;

pub struct Collision {}

impl Collision {
	pub fn mоve(&self, _pos: &mut IVec2, _step: IVec2, _dist: i32) -> bool {
		false
	}
}
