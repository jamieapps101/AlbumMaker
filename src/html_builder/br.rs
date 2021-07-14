use super::util::*;

use std::{
    fmt::{Debug,Formatter},
};



#[derive(Debug)]
struct HtmlBr<E:HtmlElement<E>> {
    class:        Vec<CssClass>,
    sub_elements: Vec<E>,
    id:           Option<String>,
}

impl<E:HtmlElement<E>> HtmlElement<E> for HtmlBr<E> {
    fn new() -> Self{
        return HtmlBr {
            class: Vec::new(),
            sub_elements: Vec::new(),
            id: None,
        }
    }
    fn add_class(mut self, c: CssClass) -> Self {
        self.class.push(c);
        return self;
    }
    fn set_id(mut self, id: &str) -> Self {
        self.id = Some(String::from(id));
        return self;
    }
    fn add_element(mut self, e: E) -> Self {
        self.sub_elements.push(e);
        return self;
    }
    fn render(self,indent: usize, f: &mut Formatter<'_>) {
        write!(f, "{}<br>\n", space_pad(indent)).unwrap();
    }
}

