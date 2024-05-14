// CmapMetric data structure-atomic map
// 基本功能： inc/dec/snapshot

use anyhow::Result;
use dashmap::DashMap;
use std::{fmt, sync::Arc};

#[derive(Debug, Clone)]
pub struct CmapMetric<T> {
    data: Arc<DashMap<String, T>>,
}

impl<T> CmapMetric<T>
where
    T: std::ops::AddAssign
        + std::ops::SubAssign
        + std::cmp::PartialEq
        + std::default::Default
        + Copy
        + From<i32>,
{
    pub fn new() -> Self {
        CmapMetric {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(T::from(0));
        *counter += T::from(1);
        Ok(())
    }

    pub fn dec(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(T::from(0));
        *counter -= T::from(1);
        Ok(())
    }
}

impl<T> Default for CmapMetric<T>
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

impl<T: std::fmt::Display> fmt::Display for CmapMetric<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}
