use std::borrow::Cow;
use std::fmt::Debug;

use indexmap::{IndexMap, IndexSet};

use crate::values::AttributeValue;

/// The trait of all html elements
#[allow(unused_variables)]
pub trait Element: Default + Debug {
    /// The child element type
    type Child;
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
        match id.into() {
            Cow::Borrowed(s) => {
                self.set_attribute("id", AttributeValue::Constant(s))
            }
            Cow::Owned(s) => {
                self.set_attribute("id", AttributeValue::String(s))
            }
        }
    }
    /// Get the legal html class.
    fn get_classes(&self) -> impl Iterator<Item=&str> {
        [].into_iter()
    }
    /// Set html class and return whether successful
    fn set_classes<I>(&mut self, classes: I) -> bool where I: Iterator<Item=Cow<'static, str>> {
        false
    }
    /// Add html class and return whether successful
    fn add_class<S>(&mut self, class: S) -> bool where S: Into<Cow<'static, str>> {
        false
    }
    /// Remove html class and return whether successful
    fn remove_class<S>(&mut self, class: &str) -> bool {
        false
    }
    /// Get the child element based on the given index
    fn get_child(&self, index: usize) -> Option<&Self::Child> {
        self.get_children().nth(index)
    }
    /// Get the mutable child element based on the given index
    fn mut_child(&mut self, index: usize) -> Option<&mut Self::Child> {
        self.mut_children().nth(index)
    }
    /// Set the child element based on the given index and return whether successful
    fn set_child<T>(&mut self, index: usize, child: T) -> bool where T: Into<Self::Child> {
        match self.mut_children().nth(index) {
            Some(s) => {
                *s = child.into();
                true
            }
            None => {
                false
            }
        }
    }
    /// Add a child element and return whether successful
    fn add_child<T>(&mut self, child: T) -> bool where T: Into<Self::Child> {
        false
    }
    /// Remove the child element based on the given index and return whether successful
    fn get_children(&self) -> impl Iterator<Item=&Self::Child> {
        [].into_iter()
    }
    /// Get the mutable child element based on the given index
    fn mut_children(&mut self) -> impl Iterator<Item=&mut Self::Child> {
        [].into_iter()
    }
    /// Set the child element based on the given index and return whether successful
    fn set_children<I>(&mut self, children: I) -> bool where I: Iterator<Item=Self::Child> {
        false
    }
}

///
#[derive(Clone, Debug)]
pub struct HtmlElement {
    tag: Cow<'static, str>,
    classes: IndexSet<Cow<'static, str>>,
    attributes: IndexMap<Cow<'static, str>, AttributeValue>,
    children: Vec<HtmlElement>,
}

impl Default for HtmlElement {
    fn default() -> Self {
        HtmlElement {
            tag: Default::default(),
            classes: IndexSet::default(),
            attributes: Default::default(),
            children: vec![],
        }
    }
}

impl Element for HtmlElement {
    type Child = HtmlElement;

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
        self.attributes.get_mut(name)
    }
    fn set_attribute<S>(&mut self, name: S, value: AttributeValue) -> bool where S: Into<Cow<'static, str>> {
        self.attributes.insert(name.into(), value);
        true
    }
    fn get_classes(&self) -> impl Iterator<Item=&str> {
        self.classes.iter().map(|s| s.as_ref())
    }
    fn set_classes<I>(&mut self, classes: I) -> bool where I: Iterator<Item=Cow<'static, str>> {
        self.classes = classes.collect();
        true
    }
    fn add_class<S>(&mut self, class: S) -> bool where S: Into<Cow<'static, str>> {
        self.classes.insert(class.into())
    }
    fn remove_class<S>(&mut self, class: &str) -> bool {
        self.classes.remove(class)
    }
    fn add_child<T>(&mut self, child: T) -> bool where T: Into<Self::Child> {
        self.children.push(child.into());
        true
    }
    fn get_children(&self) -> impl Iterator<Item=&Self::Child> {
        self.children.iter()
    }
    fn mut_children(&mut self) -> impl Iterator<Item=&mut Self::Child> {
        self.children.iter_mut()
    }
    fn set_children<I>(&mut self, children: I) -> bool where I: Iterator<Item=Self::Child> {
        self.children = children.collect();
        true
    }
}