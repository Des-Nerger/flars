/*
 *  collider.rs
 *  RPGEngine
 *
 *  Created by Clint Bellanger on 12/29/09.
 *  Rewritten in Rust by Des-Nerger on 04/20/23.
 */

use {
	crate::{settings::UNITS_PER_TILE, utils::__},
	glam::IVec2,
};

#[derive(Debug)]
pub struct Collider {
	colmap: Box<[u32]>,
	widthLog2: i32,
}

impl Collider {
	pub fn new(colmap: Box<[u32]>, widthLog2: i32) -> Self {
		Self { colmap, widthLog2 }
	}

	/**
	 * This may be the worst way to implement collision detection but it's 5:35am and
	 * I have bronchitis.  -Clint
	 */
	pub fn mоve(&self, pos: &mut IVec2, step: IVec2, dist: i32, isDiag: bool) -> bool {
		for _ in 0..dist {
			let [x, y] = (*pos + step).to_array();
			if self.isEmpty(x, y) {
				[pos.x, pos.y] = [x, y];
				continue;
			}
			if isDiag {
				if self.isEmpty(x, pos.y) {
					pos.x = x; // slide along wall
					continue;
				}
				if self.isEmpty(pos.x, y) {
					pos.y = y; // slide along wall
					continue;
				}
			}
			return false; // absolute stop
		}
		true
	}

	pub fn isEmpty(&self, x: i32, y: i32) -> bool {
		self.colmap[((x / UNITS_PER_TILE) + ((y / UNITS_PER_TILE) << self.widthLog2)) as __] == 0
	}
}
