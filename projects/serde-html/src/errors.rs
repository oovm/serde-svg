///
#[derive(Debug, Copy, Clone)]
pub enum HtmlError {
    ///
    UnknownError
}
///
pub type Result<T> = std::result::Result<T, HtmlError>;
