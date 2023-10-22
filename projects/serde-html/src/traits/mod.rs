use std::borrow::Cow;
use std::fmt::Debug;

use indexmap::IndexMap;

use crate::values::AttributeValue;

#[allow(unused_variables)]
pub trait Element: Default + Debug {
    /// Construct html elements
    fn build(self) -> HtmlElement;

    /// Get the tag of the element, all html elements must have tag
    fn get_tag(&self) -> &str;

    /// Change the tag of the element and return whether the change is successful
    fn set_tag<S>(&mut self, tag: S) -> bool where S: Into<Cow<'static, str>> {
        false
    }
    /// Get the html attribute based on the given name
    fn get_attribute(&self, name: &str) -> Option<&AttributeValue> {
        None
    }
    /// Get the mutable html attribute based on the given name
    fn mut_attribute(&mut self, name: &str) -> Option<&mut AttributeValue> {
        None
    }
    /// According to the given name, set the html attribute and return whether it is successful.
    fn set_attribute<S>(&mut self, name: S, value: AttributeValue) -> bool where S: Into<Cow<'static, str>> {
        false
    }
    /// Get the legal html id.
    ///
    /// Note that Some will be returned only when the id exists and the value is a string.
    fn get_id(&self) -> Option<&str> {
        match self.get_attribute("id")? {
            AttributeValue::Constant(s) => { Some(s) }
            AttributeValue::String(s) => { Some(s) }
            _ => None
        }
    }
    /// Set html id and return whether successful
    fn set_id<S>(&mut self, id: S) -> bool where S: Into<Cow<'static, str>> {
        false
    }
    /// Get the legal html class.
    fn get_classes(&self) -> impl Iterator<Item=&str> {
        [].into_iter()
    }
    /// Set html class and return whether successful
    fn set_classes<S>(&self, classes: S, value: AttributeValue) -> bool where S: Into<Cow<'static, str>> {
        false
    }
    /// Add html class and return whether successful
    fn add_class<S>(&self, class: S) -> bool where S: Into<Cow<'static, str>> {
        false
    }
    /// Remove html class and return whether successful
    fn remove_class<S>(&self, class: S) -> bool where S: Into<Cow<'static, str>> {
        false
    }
}

#[derive(Debug)]
pub struct HtmlElement {
    tag: Cow<'static, str>,
    classes: Vec<Cow<'static, str>>,
    attributes: IndexMap<Cow<'static, str>, AttributeValue>,
    children: Vec<HtmlElement>,
}

impl Default for HtmlElement {
    fn default() -> Self {
        HtmlElement {
            tag: Default::default(),
            classes: vec![],
            attributes: Default::default(),
            children: vec![],
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
        self.tag = tag.into();
        true
    }
    fn get_attribute(&self, name: &str) -> Option<&AttributeValue> {
        self.attributes.get(name)
    }
    fn mut_attribute(&mut self, name: &str) -> Option<&mut AttributeValue> {
        todo!()
    }
    fn set_attribute<S>(&mut self, name: S, value: AttributeValue) -> bool where S: Into<Cow<'static, str>> {
        self.attributes.insert(name.into(), value);
        true
    }
    fn get_id(&self) -> Option<&str> {
        todo!()
    }
    fn set_id<S>(&mut self, id: S) -> bool where S: Into<Cow<'static, str>> {
        todo!()
    }
    fn get_classes(&self) -> impl Iterator<Item=Cow<str>> {
        self.classes.iter().map(move |s| match s {
            Cow::Borrowed(c) => { Cow::Borrowed(*c) }
            Cow::Owned(c) => { Cow::Borrowed(&**c) }
        })
    }
    fn set_classes<S>(&self, classes: S, value: AttributeValue) -> bool where S: Into<Cow<'static, str>> {
        todo!()
    }
    fn add_class<S>(&self, class: S) -> bool where S: Into<Cow<'static, str>> {
        todo!()
    }
    fn remove_class<S>(&self, class: S) -> bool where S: Into<Cow<'static, str>> {
        todo!()
    }
}