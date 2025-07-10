use ordered_float::{Float, FloatCore, NotNan};

pub struct NPos2<T: FloatCore = f32> {
    x: NotNan<T>,
    y: NotNan<T>,
}
