use anyhow::{anyhow, Result};
use std::ops::{Add, AddAssign, Deref, Mul};

pub struct Vector<T> {
    data: Vec<T>,
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T> + AddAssign,
{
    if a.data.len() != b.data.len() {
        return Err(anyhow!(
            "Cannot dot product vectors of different lengths: {} and {}",
            a.data.len(),
            b.data.len()
        ));
    }

    let mut result = T::default();
    for i in 0..a.data.len() {
        result += a.data[i] * b.data[i];
    }
    Ok(result)
}

impl<T> Deref for Vector<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Vector { data: data.into() }
    }
}
