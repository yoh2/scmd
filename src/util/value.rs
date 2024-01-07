use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum SingleOrVec<T> {
    Single(T),
    Vec(Vec<T>),
}

impl<T> SingleOrVec<T> {
    pub fn len(&self) -> usize {
        match self {
            Self::Single(_) => 1,
            Self::Vec(v) => v.len(),
        }
    }

    pub fn as_slice(&self) -> &[T] {
        match self {
            Self::Single(s) => std::slice::from_ref(s),
            Self::Vec(v) => v.as_slice(),
        }
    }
}

/// Referable holds either of a reference to T or a entity of T, which is like [std::borrow::Cow],
/// but never request the ownership.
pub enum Referable<'a, T> {
    Borrowed(&'a T),
    Owned(T),
}

impl<'a, T> Deref for Referable<'a, T>
where
    Self: 'a,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Borrowed(x) => x,
            Self::Owned(x) => x,
        }
    }
}
