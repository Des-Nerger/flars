/**
 * settings
 *
 * @author Clint Bellanger, Des-Nerger
 * @license GPLv3
 */
use glam::IVec2;

pub const FPS: u32 = 24;
pub const SCREEN_WIDTH: u32 = 853;
pub const SCREEN_HEIGHT: u32 = 480;
pub const SCREEN_CENTER: IVec2 = IVec2::new((SCREEN_WIDTH / 2) as _, (SCREEN_HEIGHT / 2) as _);
pub const UNITS_PER_TILE: i32 = 32;
pub const TILE_WIDTH_HALF: i32 = 32;
pub const TILE_HEIGHT_HALF: i32 = 16;
