mod from;

pub enum HtmlValue {
    Null,
    Boolean(bool),
    Integer(i64),
    Decimal(f64),
    Constant(&'static str),
    String(String),
}

