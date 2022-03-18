use crate::{
    color::Color,
    matrix::Vector3,
    pipeline::{
        barycentric::BarycentricCoord,
        shaders::{FragmentShader, VertexShader, ShaderProgram},
    },
};

pub struct Program {}

impl VertexShader for Program {
    fn vert(&self, in_pos: &Vector3) -> Vector3 {
        in_pos.clone()
    }
}

impl FragmentShader for Program {
    fn frag(&self, coord: &BarycentricCoord) -> Color {
        let red = Color::new(1f32, 0f32, 0f32, 1f32);
        let green = Color::new(0f32, 1f32, 0f32, 1f32);
        let blue = Color::new(0f32, 0f32, 1f32, 1f32);

        Self::barymix_color(&red, &green, &blue, coord)
    }
}

impl ShaderProgram for Program{}