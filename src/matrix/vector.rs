use crate::matrix::*;

pub type VectorNT<T, const N: usize> = MatrixNMT<T, N, 1usize>;

pub type Vector3 = VectorNT<f32, 3usize>;

impl Vector3 {
    pub fn new3(x: f32, y: f32, z: f32) -> Self {
        Vector3 { m: [[x], [y], [z]] }
    }

    pub fn x(&self) -> &f32 {
        &self.m[0][0]
    }

    pub fn y(&self) -> &f32 {
        &self.m[1][0]
    }

    pub fn z(&self) -> &f32 {
        &self.m[2][0]
    }

    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.m[0][0]
    }

    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.m[1][0]
    }

    pub fn z_mut(&mut self) -> &mut f32 {
        &mut self.m[2][0]
    }
}

pub type Vector2 = VectorNT<f32, 2usize>;

impl Vector2 {
    pub fn new2(x: f32, y: f32) -> Self {
        Vector2{m: [[x], [y]]}
    }

    pub fn from_vector3(v: &Vector3) -> Self {
        Vector2::new2(*v.x(), *v.y())
    }

    pub fn x(&self) -> &f32 {
        &self.m[0][0]
    }

    pub fn y(&self) -> &f32 {
        &self.m[1][0]
    }

    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.m[0][0]
    }

    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.m[1][0]
    }
}