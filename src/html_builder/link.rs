use super::util::*;

use std::fmt::{Debug,Formatter};

#[derive(Debug)]
struct HtmlLink<E:HtmlElement<E>> {
    class:        Vec<CssClass>,
    sub_elements: Vec<E>,
    id:           Option<String>,
    href:         Option<String>,
}

impl <E:HtmlElement<E>> HtmlLink<E> {
    fn set_href(mut self, href: String) -> Self {
        self.href = Some(href);
        return self;
    }
}

fn to_href_string(opt_href: Option<String>) -> String {
    if let Some(href) = opt_href {
        return format!("\"{}\"",href);
    } else {
        return String::from("");
    }
}

impl<E:HtmlElement<E>> HtmlElement<E> for HtmlLink<E> {
    fn new() -> Self{
        return HtmlLink {
            class: Vec::new(),
            sub_elements: Vec::new(),
            id: None,
            href: None,
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
        write!(f, "{}<a class={} href={}>\n", space_pad(indent),to_class_string(self.class),to_href_string(self.href)).unwrap();
        for e in self.sub_elements {
            e.render(indent+1,f);
        }
        write!(f, "{}</a>\n", space_pad(indent)).unwrap();
    }
}