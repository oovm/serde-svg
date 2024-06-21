use alloc::{borrow::Cow, string::String};
use core::{
    hash::{Hash, Hasher},
    num::TryFromIntError,
};

mod from;

/// Values for attributes
///
/// Essentially a string, but using enum to reduce unnecessary allocation
#[derive(Clone, Debug)]
pub struct AttributePair {
    key: Cow<'static, str>,
    value: AttributeValue,
}

/// Values for attributes
///
/// Essentially a string, but using enum to reduce unnecessary allocation
#[derive(Clone, Debug)]
pub enum AttributeValue {
    /// This attribute exists, but no value is set
    Empty,
    Boolean(bool),
    Integer(i64),
    Decimal(f64),
    Constant(&'static str),
    String(String),
}

impl Eq for AttributeValue {}

impl PartialEq for AttributeValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // nothing
            (Self::Empty, Self::Empty) => true,
            // number like
            (Self::Boolean(a), Self::Boolean(b)) => a == b,
            (Self::Integer(a), Self::Integer(b)) => a == b,
            (Self::Decimal(a), Self::Decimal(b)) => a == b,
            // string like
            (Self::Constant(a), Self::Constant(b)) => a == b,
            (Self::Constant(a), Self::String(b)) => a == b,
            (Self::String(a), Self::Constant(b)) => a == b,
            (Self::String(a), Self::String(b)) => a == b,
            _ => false,
        }
    }
}

impl Hash for AttributeValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Empty => {}
            Self::Boolean(b) => b.hash(state),
            Self::Integer(i) => i.hash(state),
            Self::Decimal(f) => f.to_bits().hash(state),
            Self::Constant(s) => s.hash(state),
            Self::String(s) => s.hash(state),
        }
    }
}
