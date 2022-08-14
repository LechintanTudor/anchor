use smallvec::{Drain, SmallVec};

#[derive(Debug)]
pub struct SmallVecSet<T, const N: usize> {
    values: SmallVec<[T; N]>,
}

impl<T, const N: usize> Default for SmallVecSet<T, N> {
    #[inline]
    fn default() -> Self {
        Self { values: Default::default() }
    }
}

impl<T, const N: usize> SmallVecSet<T, N>
where
    T: PartialEq,
{
    pub fn insert(&mut self, value: T) -> bool {
        if !self.values.iter().any(|prev_value| prev_value == &value) {
            self.values.push(value);
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn contains(&self, value: &T) -> bool {
        self.values.iter().any(|prev_value| prev_value == value)
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.values
    }

    pub fn remove(&mut self, value: &T) -> bool {
        for (i, prev_value) in self.values.iter().enumerate() {
            if prev_value == value {
                self.values.swap_remove(i);
                return true;
            }
        }

        false
    }

    #[inline]
    pub fn clear(&mut self) {
        self.values.clear();
    }

    #[inline]
    pub fn drain(&mut self) -> Drain<[T; N]> {
        self.values.drain(..)
    }
}
