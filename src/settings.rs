/**
 * settings
 *
 * @author Clint Bellanger, Des-Nerger
 * @license GPLv3
 */
use glam::IVec2;

pub const FPS: u32 = 24;
pub const SCREEN_WIDTH: u32 = 640;
pub const SCREEN_HEIGHT: u32 = 480;
pub const SCREEN_CENTER: IVec2 = IVec2::new((SCREEN_WIDTH / 2) as _, (SCREEN_HEIGHT / 2) as _);
pub const TILE_SHIFT: i32 = 5; // for fast bitshift divides
