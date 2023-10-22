use std::num::TryFromIntError;

use super::*;

impl From<()> for AttributeValue {
    fn from(_: ()) -> Self {
        Self::Empty
    }
}

impl From<bool> for AttributeValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<u8> for AttributeValue {
    fn from(value: u8) -> Self {
        Self::Integer(value as i64)
    }
}

impl From<u16> for AttributeValue {
    fn from(value: u16) -> Self {
        Self::Integer(value as i64)
    }
}

impl From<u32> for AttributeValue {
    fn from(value: u32) -> Self {
        Self::Integer(value as i64)
    }
}

impl TryFrom<u64> for AttributeValue {
    type Error = TryFromIntError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(Self::Integer(i64::try_from(value)?))
    }
}

impl From<i8> for AttributeValue {
    fn from(value: i8) -> Self {
        Self::Integer(value as i64)
    }
}

impl From<i16> for AttributeValue {
    fn from(value: i16) -> Self {
        Self::Integer(value as i64)
    }
}


impl From<i32> for AttributeValue {
    fn from(value: i32) -> Self {
        Self::Integer(value as i64)
    }
}


impl From<i64> for AttributeValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<f32> for AttributeValue {
    fn from(value: f32) -> Self {
        Self::Decimal(value as f64)
    }
}

impl From<f64> for AttributeValue {
    fn from(value: f64) -> Self {
        Self::Decimal(value)
    }
}