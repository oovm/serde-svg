///
#[derive(Debug, Copy, Clone)]
pub enum HtmlError {
    ///
    UnknownError,
}
///
pub type Result<T> = core::result::Result<T, HtmlError>;
