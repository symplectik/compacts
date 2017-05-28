use std::iter::FromIterator;
use super::{Seq16, Seq64, Rle16};

impl Seq16 {
    pub const THRESHOLD: usize = 1 << 12; // 16 * (1 << 12) == 65536

    pub fn new() -> Self {
        let weight = 0;
        let vector = Vec::new();
        Seq16 { weight, vector }
    }

    pub fn len(&self) -> usize {
        self.vector.len()
    }

    pub fn with_capacity(cap: usize) -> Self {
        let bounded = if cap <= Self::THRESHOLD {
            cap
        } else {
            Self::THRESHOLD
        };
        let weight = 0;
        let vector = Vec::with_capacity(bounded);
        Seq16 { weight, vector }
    }

    pub fn shrink_to_fit(&mut self) {
        self.vector.shrink_to_fit()
    }

    #[inline]
    fn search(&self, bit: &u16) -> Result<usize, usize> {
        self.vector.binary_search(bit)
    }

    #[inline]
    pub fn contains(&self, bit: u16) -> bool {
        self.search(&bit).is_ok()
    }

    #[inline]
    pub fn insert(&mut self, bit: u16) -> bool {
        let ok = self.search(&bit)
            .map_err(|i| self.vector.insert(i, bit))
            .is_err();
        if ok {
            self.weight += 1;
        }
        ok
    }

    #[inline]
    pub fn remove(&mut self, bit: u16) -> bool {
        let ok = self.search(&bit).map(|i| self.vector.remove(i)).is_ok();
        if ok {
            self.weight -= 1;
        }
        ok
    }
}

impl From<Seq64> for Seq16 {
    fn from(that: Seq64) -> Self {
        Seq16::from(&that)
    }
}
impl<'r> From<&'r Seq64> for Seq16 {
    fn from(that: &Seq64) -> Self {
        use std::u16;
        let mut vec16 = Seq16::with_capacity(that.weight as usize);
        let iter = that.vector.iter();
        for (i, w) in iter.cloned().enumerate().filter(|&(_, v)| v != 0) {
            for p in 0..<u64 as ::UnsignedInt>::WIDTH {
                if w & (1 << p) != 0 {
                    let bit = i * <u64 as ::UnsignedInt>::WIDTH + p;
                    debug_assert!(bit <= u16::MAX as usize);
                    vec16.insert(bit as u16);
                }
            }
        }
        vec16
    }
}

impl From<Rle16> for Seq16 {
    fn from(that: Rle16) -> Self {
        Seq16::from(&that)
    }
}
impl<'r> From<&'r Rle16> for Seq16 {
    fn from(that: &'r Rle16) -> Self {
        let mut seq16 = Seq16::with_capacity(that.weight as usize);
        seq16.weight = that.weight;
        for range in &that.ranges {
            seq16.vector.extend(range.clone());
        }
        seq16
    }
}

impl From<Vec<u16>> for Seq16 {
    fn from(vector: Vec<u16>) -> Self {
        debug_assert!(vector.len() <= super::CAPACITY);
        let weight = vector.len() as u32;
        Seq16 { weight, vector }
    }
}

impl FromIterator<u16> for Seq16 {
    fn from_iter<I>(i: I) -> Self
        where I: IntoIterator<Item = u16>
    {
        let iter = i.into_iter();
        Seq16::from(iter.collect::<Vec<u16>>())
    }
}
impl<'a> FromIterator<&'a u16> for Seq16 {
    fn from_iter<I>(i: I) -> Self
        where I: IntoIterator<Item = &'a u16>
    {
        let iter = i.into_iter();
        Seq16::from_iter(iter.cloned())
    }
}
