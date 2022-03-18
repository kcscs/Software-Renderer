use crate::color::*;
use crate::matrix::*;

use super::barycentric::BarycentricCoord;

pub trait VertexShader {
    fn vert(&self, in_pos: &Vector3) -> Vector3;
}

pub trait FragmentShader {
    fn frag(&self, coord: &BarycentricCoord) -> Color;

    fn barymix_color(a: &Color, b: &Color, c: &Color, coord: &BarycentricCoord) -> Color
    where Self: Sized {
        Color::new(
            a.r * coord.a + b.r * coord.b + c.r * coord.c,
            a.g * coord.a + b.g * coord.b + c.g * coord.c,
            a.b * coord.a + b.b * coord.b + c.b * coord.c,
            a.a * coord.a + b.a * coord.b + c.a * coord.c)
    }
}

pub trait ShaderProgram: VertexShader + FragmentShader {}
