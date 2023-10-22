mod from;

/// Values for attributes
///
/// Essentially a string, but using enum to reduce unnecessary allocation
pub enum AttributeValue {
    /// This attribute exists, but no value is set
    Empty,
    Boolean(bool),
    Integer(i64),
    Decimal(f64),
    Constant(&'static str),
    String(String),
}

