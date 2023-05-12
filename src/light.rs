mod ambiant;
mod directional;
mod dot;

pub use dot::Dot;
pub use directional::Directional;
pub use ambiant::Ambiant;

use crate::{material::Color, math::Vector3D};

pub trait Light {
    fn color(&self) -> Color;
    fn intensity(&self) -> f32;
    fn direction_from(&self, point: Vector3D) -> Vector3D;
}
