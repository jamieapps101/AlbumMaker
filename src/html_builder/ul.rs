use super::util::*;

use std::fmt::{Debug,Formatter};



#[derive(Debug)]
struct HtmlUl<E:HtmlElement<E>> {
    class:        Vec<CssClass>,
    sub_elements: Vec<E>,
    id:           Option<String>,
}

impl<E:HtmlElement<E>> HtmlElement<E> for HtmlUl<E> {
    fn new() -> Self{
        return HtmlUl {
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
        write!(f, "{}<ul class={}>\n", space_pad(indent),to_class_string(self.class)).unwrap();
        for e in self.sub_elements {
            e.render(indent+1,f);
        }
        write!(f, "{}</ul>\n", space_pad(indent)).unwrap();
    }
}