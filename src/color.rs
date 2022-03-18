use num_traits::Num;

pub trait ValueT : Num + Copy {}

#[derive(Clone, Copy)]
pub struct ColorT<T>
where
    T: ValueT,
{
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T> ColorT<T>
where
    T: ValueT,
{
    pub fn new(r: T, g: T, b: T, a: T) -> Self {
        Self { r, g, b, a }
    }
}

impl ValueT for f32 {}

pub type Color = ColorT<f32>;
