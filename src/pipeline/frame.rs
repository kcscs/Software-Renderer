use crate::color::*;

pub struct Frame<T, const N: usize, const M: usize>
where
    T: ValueT,
{
    pub(in crate::pipeline) color: [[ColorT<T>; M]; N],
    pub(in crate::pipeline) depth: [[f32; M]; N],
}

impl<T, const N: usize, const M: usize> Frame<T, N, M>
where
    T: ValueT,
{
    pub fn new() -> Self {
        Self {
            color: [[ColorT::new(T::zero(), T::zero(), T::zero(), T::one()); M]; N],
            depth: [[1f32; M]; N],
        }
    }
}

impl<const N: usize, const M: usize> Frame<f32, N, M> {
    pub fn byte_sequence(&self) -> Vec<u8> {
        let mut v = Vec::new();

        for y in (0..N).rev() {
            for x in 0..M {
                v.push((255f32 * self.color[y][x].r).round() as u8);
                v.push((255f32 * self.color[y][x].g).round() as u8);
                v.push((255f32 * self.color[y][x].b).round() as u8);
                v.push((255f32 * self.color[y][x].a).round() as u8);
            }
        }

        v
    }
}
