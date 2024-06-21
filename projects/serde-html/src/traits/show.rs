use super::*;

impl Display for HtmlElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("<")
    }
}
