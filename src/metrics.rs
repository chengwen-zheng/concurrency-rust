// metrics data structure
// 基本功能： inc/dec/snapshot

use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
pub struct Metrics<T> {
    data: Arc<Mutex<HashMap<String, T>>>,
}

impl<T> Metrics<T>
where
    T: std::ops::AddAssign
        + std::ops::SubAssign
        + std::cmp::PartialEq
        + std::default::Default
        + Copy
        + From<i32>,
{
    pub fn new() -> Self {
        Metrics {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self
            .data
            .lock()
            .map_err(|e| anyhow!(format!("snapshot lock failed: {}", e.to_string())))?;
        let counter = data.entry(key.into()).or_insert(T::from(0));
        *counter += T::from(1);
        Ok(())
    }

    pub fn dec(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self
            .data
            .lock()
            .map_err(|e| anyhow!(format!("desc lock failed: {}", e.to_string())))?;
        let counter = data.entry(key.into()).or_insert(T::from(0));
        *counter -= T::from(1);
        Ok(())
    }

    pub fn snapshot(&self) -> Result<HashMap<String, T>>
    where
        T: Copy,
    {
        let data = self
            .data
            .lock()
            .map_err(|e| anyhow!(format!("snapshot lock failed: {}", e.to_string())))?;
        Ok(data.clone())
    }
}

impl<T> Default for Metrics<T>
where
    T: std::ops::AddAssign
        + std::ops::SubAssign
        + std::cmp::PartialEq
        + std::default::Default
        + Copy
        + From<i32>,
{
    fn default() -> Self {
        Self::new()
    }
}
