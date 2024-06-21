use super::*;

impl Default for HtmlElement {
    fn default() -> Self {
        HtmlElement { tag: Default::default(), classes: IndexSet::default(), attributes: Default::default(), children: vec![] }
    }
}

impl Element for HtmlElement {
    fn build(self) -> HtmlElement {
        self
    }

    fn get_tag(&self) -> &str {
        &self.tag
    }
    fn set_tag<S>(&mut self, tag: S) -> bool
    where
        S: Into<Cow<'static, str>>,
    {
        self.tag = tag.into();
        true
    }
    fn get_attribute(&self, name: &str) -> Option<&AttributeValue> {
        self.attributes.get(name)
    }
    fn mut_attribute(&mut self, name: &str) -> Option<&mut AttributeValue> {
        self.attributes.get_mut(name)
    }
    fn set_attribute<S>(&mut self, name: S, value: AttributeValue) -> bool
    where
        S: Into<Cow<'static, str>>,
    {
        self.attributes.insert(name.into(), value);
        true
    }
    fn get_classes(&self) -> impl Iterator<Item = &str> {
        self.classes.iter().map(|s| s.as_ref())
    }
    fn set_classes<I>(&mut self, classes: I) -> bool
    where
        I: Iterator<Item = Cow<'static, str>>,
    {
        self.classes = classes.collect();
        true
    }
    fn add_class<S>(&mut self, class: S) -> bool
    where
        S: Into<Cow<'static, str>>,
    {
        self.classes.insert(class.into())
    }
    fn remove_class<S>(&mut self, class: &str) -> bool {
        self.classes.remove(class)
    }
    fn add_child<T>(&mut self, child: T) -> bool
    where
        T: Into<HtmlNode>,
    {
        self.children.push(child.into());
        true
    }
    fn get_children(&self) -> impl Iterator<Item = &HtmlNode> {
        self.children.iter()
    }
    fn mut_children(&mut self) -> impl Iterator<Item = &mut HtmlNode> {
        self.children.iter_mut()
    }
    fn set_children<I>(&mut self, children: I) -> bool
    where
        I: Iterator<Item = HtmlNode>,
    {
        self.children = children.collect();
        true
    }
}
