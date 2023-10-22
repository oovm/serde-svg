use std::borrow::Cow;

use crate::values::AttributeValue;

pub trait Element {
    fn build(&self) -> HtmlElement;

    fn get_attribute(&self, name: &str) -> Option<&AttributeValue>;
    fn set_attribute<S: Into<Cow<'static, str>>>(&mut self, name: &str, value: AttributeValue);
}

pub struct HtmlElement {}