
pub struct BarycentricCoord {
    pub a: f32, pub b: f32, pub c: f32
}

impl BarycentricCoord {
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        BarycentricCoord{a,b,c}
    }
}