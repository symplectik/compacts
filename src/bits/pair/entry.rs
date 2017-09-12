use std::borrow::Cow;
use std::cmp::Ordering;

use bits;
use super::Assign;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry<'a> {
    pub(crate) key: u16,
    pub(crate) cow: Cow<'a, bits::Block>,
}
pub struct Entries<'a> {
    pub(crate) entries: Box<Iterator<Item = Entry<'a>> + 'a>,
}

impl<'a> PartialOrd for Entry<'a> {
    fn partial_cmp(&self, that: &Self) -> Option<Ordering> {
        Some(self.key.cmp(&that.key))
    }
}
impl<'a> Ord for Entry<'a> {
    fn cmp(&self, that: &Self) -> Ordering {
        self.key.cmp(&that.key)
    }
}

impl<'a> Entry<'a> {
    // fn iter<'r>(&'r self) -> impl Iterator<Item = u32> + 'r {
    //     let key = self.key;
    //     self.cow
    //         .iter()
    //         .map(move |low| <u32 as bits::Merge>::merge((key, low)))
    // }

    pub fn bits(self) -> impl Iterator<Item = u32> + 'a {
        let key = self.key;
        self.cow
            .into_owned()
            .into_iter()
            .map(move |low| <u32 as bits::Merge>::merge((key, low)))
    }
}

impl<'a> Iterator for Entries<'a> {
    type Item = Entry<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.entries.next()
    }
}

pub fn option_and<'a>(t: (Option<Entry<'a>>, Option<Entry<'a>>)) -> Option<Entry<'a>> {
    match t {
        (Some(mut lhs), Some(rhs)) => {
            lhs.cow.to_mut().and_assign(rhs.cow.as_ref());
            Some(lhs)
        }
        _ => None,
    }
}

pub fn option_or<'a>(t: (Option<Entry<'a>>, Option<Entry<'a>>)) -> Option<Entry<'a>> {
    match t {
        (Some(mut lhs), Some(rhs)) => {
            lhs.cow.to_mut().or_assign(rhs.cow.as_ref());
            Some(lhs)
        }
        (Some(lhs), None) => Some(lhs),
        (None, Some(rhs)) => Some(rhs),
        (None, None) => None,
    }
}

pub fn option_and_not<'a>(t: (Option<Entry<'a>>, Option<Entry<'a>>)) -> Option<Entry<'a>> {
    match t {
        (Some(mut lhs), Some(rhs)) => {
            lhs.cow.to_mut().and_not_assign(rhs.cow.as_ref());
            Some(lhs)
        }
        (Some(lhs), None) => Some(lhs),
        _ => None,
    }
}

pub fn option_xor<'a>(t: (Option<Entry<'a>>, Option<Entry<'a>>)) -> Option<Entry<'a>> {
    match t {
        (Some(mut lhs), Some(rhs)) => {
            lhs.cow.to_mut().xor_assign(rhs.cow.as_ref());
            Some(lhs)
        }
        (Some(lhs), None) => Some(lhs),
        (None, Some(rhs)) => Some(rhs),
        _ => None,
    }
}
