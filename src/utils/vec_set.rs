use std::fmt;
use std::vec::Drain;

#[derive(Clone)]
pub(crate) struct VecSet<T> {
    values: Vec<T>,
}

impl<T> VecSet<T> {
    pub fn insert(&mut self, value: T) -> bool
    where
        T: PartialEq,
    {
        if !self.values.iter().any(|prev_value| prev_value == &value) {
            self.values.push(value);
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn contains(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        self.values.iter().any(|prev_value| prev_value == value)
    }

    pub fn remove(&mut self, value: &T) -> bool
    where
        T: PartialEq,
    {
        for (i, prev_value) in self.values.iter().enumerate() {
            if prev_value == value {
                self.values.swap_remove(i);
                return true;
            }
        }

        false
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.values
    }

    #[inline]
    pub fn clear(&mut self) {
        self.values.clear();
    }

    #[inline]
    pub fn drain(&mut self) -> Drain<T> {
        self.values.drain(..)
    }
}

impl<T> AsRef<[T]> for VecSet<T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        &self.values
    }
}

impl<T> Default for VecSet<T> {
    #[inline]
    fn default() -> Self {
        Self { values: Default::default() }
    }
}

impl<T> fmt::Debug for VecSet<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.values.iter()).finish()
    }
}
