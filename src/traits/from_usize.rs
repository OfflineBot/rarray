pub trait FromUsize {
    fn from_usize(value: usize) -> Self;
}

impl FromUsize for f32 {
    fn from_usize(value: usize) -> f32 {
        value as f32
    }
}

impl FromUsize for f64 {
    fn from_usize(value: usize) -> f64 {
        value as f64
    }
}