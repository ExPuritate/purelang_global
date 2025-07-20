use derive_more::Deref;
use faststr::FastStr;
use proc_macros::ThreadSafe;
use std::borrow::Borrow;
use std::ops::Add;
use std::sync::Arc;
use std::{
    fmt::{Debug, Display},
    str::{FromStr, pattern::Pattern},
};

#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Hash, Deref, Default, ThreadSafe)]
pub struct StringName {
    s: FastStr,
}

impl StringName {
    pub const fn from_static_str(s: &'static str) -> Self {
        Self {
            s: FastStr::from_static_str(s),
        }
    }
    pub fn as_str(&self) -> &str {
        self.s.as_str()
    }
    pub fn to_vec(&self) -> Vec<u8> {
        self.s.as_bytes().to_vec()
    }
    pub fn from_arc_str(s: Arc<str>) -> Self {
        Self {
            s: FastStr::from_arc_str(s),
        }
    }
    pub fn from_arc_string(s: Arc<String>) -> Self {
        Self {
            s: FastStr::from_arc_string(s),
        }
    }
    pub fn from_string(s: String) -> Self {
        Self {
            s: FastStr::from_string(s),
        }
    }
}

impl StringName {
    pub fn contains<P: Pattern>(&self, pat: P) -> bool {
        self.s.contains(pat)
    }
}

impl Debug for StringName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <FastStr as Debug>::fmt(&self.s, f)
    }
}

impl Display for StringName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <FastStr as Display>::fmt(&self.s, f)
    }
}

impl Borrow<str> for StringName {
    fn borrow(&self) -> &str {
        self.s.as_str()
    }
}

impl PartialEq<str> for StringName {
    fn eq(&self, other: &str) -> bool {
        self.eq(&StringName {
            s: FastStr::from_string(other.to_owned()),
        })
    }
}

impl<T: AsRef<str>> From<T> for StringName {
    fn from(value: T) -> Self {
        StringName {
            s: FastStr::from_str(value.as_ref()).unwrap(),
        }
    }
}

impl<T: AsRef<str>> Add<T> for StringName {
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.as_ref();
        Self {
            s: FastStr::from_string(self.s.to_string() + rhs),
        }
    }
}
