use std::borrow::Cow;
use std::fmt::Debug;

use indexmap::IndexMap;

use crate::values::AttributeValue;

#[allow(unused_variables)]
pub trait Element: Default + Debug {
    fn build(self) -> HtmlElement;

    fn get_tag(&self) -> &str;

    fn set_tag<S>(&mut self, tag: S) -> bool where S: Into<Cow<'static, str>> {
        false
    }
    fn get_attribute(&self, name: &str) -> Option<&AttributeValue> {
        None
    }
    fn set_attribute<S>(&mut self, name: S, value: AttributeValue) -> bool where S: Into<Cow<'static, str>> {
        false
    }

    fn get_class<S>(&self, name: S, value: AttributeValue) -> bool where S: Into<Cow<'static, str>> {
        false
    }

    fn set_class<S>(&self, name: S, value: AttributeValue) -> bool where S: Into<Cow<'static, str>> {
        false
    }
}

#[derive(Debug)]
pub struct HtmlElement {
    tag: Cow<'static, str>,
    classes: Vec<Cow<'static, str>>,
    attributes: IndexMap<Cow<'static, str>, AttributeValue>,
}

impl Default for HtmlElement {
    fn default() -> Self {
        HtmlElement {
            tag: Default::default(),
            classes: vec![],
            attributes: Default::default(),
        }
    }
}

impl Element for HtmlElement {
    fn build(self) -> HtmlElement {
        self
    }

    fn get_tag(&self) -> &str {
        &self.tag
    }
    fn set_tag<S>(&mut self, tag: S) -> bool where S: Into<Cow<'static, str>> {
        todo!()
    }
    fn get_attribute(&self, name: &str) -> Option<&AttributeValue> {
        self.attributes.get(name)
    }
    fn set_attribute<S>(&mut self, name: S, value: AttributeValue) -> bool where S: Into<Cow<'static, str>> {
        self.attributes.insert(name.into(), value);
        true
    }
    fn get_class<S>(&self, name: S, value: AttributeValue) -> bool where S: Into<Cow<'static, str>> {
        self.classes.contains(&name.into())
    }
    fn set_class<S>(&self, name: S, value: AttributeValue) -> bool where S: Into<Cow<'static, str>> {
        todo!()
    }
}