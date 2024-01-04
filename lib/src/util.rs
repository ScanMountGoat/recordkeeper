use recordkeeper_macros::SaveBin;
use thiserror::Error;

use crate::{error::SaveError, io::SaveBin};

/// The vector has reached its maximum or minimum length.
#[derive(Debug, Error, Clone, Copy)]
#[error("min/max length reached")]
pub struct CapacityError;

/// Nul-terminated string with fixed storage and maximum length.
///
/// Extra bytes are not guaranteed to be nulls.
#[derive(SaveBin, Debug)]
pub struct FixStr<const MAX: usize> {
    buf: [u8; MAX],
}

/// Dynamic array with fixed capacity.
#[derive(SaveBin, Debug, Clone)]
pub struct FixVec<T, const MAX: usize>
where
    T: SaveBin + std::fmt::Debug,
    SaveError: From<<T as SaveBin>::ReadError>,
    SaveError: From<<T as SaveBin>::WriteError>,
{
    buf: Box<[T; MAX]>,
    len: u64,
}

impl<T, const MAX: usize> FixVec<T, MAX>
where
    T: SaveBin + std::fmt::Debug,
    SaveError: From<<T as SaveBin>::ReadError>,
    SaveError: From<<T as SaveBin>::WriteError>,
{
    pub fn get(&self, i: usize) -> Option<&T> {
        if i >= self.len() {
            return None;
        }
        self.buf.get(i)
    }

    pub fn set(&mut self, i: usize, new: T) {
        assert!(i < self.len(), "index out of bounds");
        self.buf[i] = new;
    }

    pub fn try_push(&mut self, to_add: T) -> Result<(), CapacityError> {
        let len = self.len();
        if len >= MAX {
            return Err(CapacityError);
        }
        self.buf[len] = to_add;
        self.len += 1;
        Ok(())
    }

    pub fn len(&self) -> usize {
        // len <= MAX
        self.len as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.buf.iter().take(self.len as usize)
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub const fn capacity(&self) -> usize {
        MAX
    }
}

impl<T, const MAX: usize> FixVec<T, MAX>
where
    T: Default + SaveBin + std::fmt::Debug,
    SaveError: From<<T as SaveBin>::ReadError>,
    SaveError: From<<T as SaveBin>::WriteError>,
{
    pub fn try_pop(&mut self) -> Result<T, CapacityError> {
        let len = self.len();
        if len == 0 {
            return Err(CapacityError);
        }
        let res = std::mem::take(&mut self.buf[len - 1]);
        self.len -= 1;
        Ok(res)
    }
}
