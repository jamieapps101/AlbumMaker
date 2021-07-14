use super::util::*;

use std::fmt::{Debug,Formatter};

#[derive(Debug)]
struct HtmlImg<E:HtmlElement<E>> {
    class:        Vec<CssClass>,
    sub_elements: Vec<E>,
    id:           Option<String>,
    src:          Option<String>,
    alt:          Option<String>,
}

impl <E:HtmlElement<E>> HtmlImg <E> {
    fn set_src(mut self, src: String) -> Self {
        self.src = Some(src);
        return self;
    }

    fn set_alt(mut self, alt: String) -> Self {
        self.alt = Some(alt);
        return self;
    }
}

fn to_property_string(opt_prop: Option<String>) -> String {
    if let Some(prop) = opt_prop {
        return format!("\"{}\"",prop);
    } else {
        return String::from("");
    }
}

impl<E:HtmlElement<E>> HtmlElement<E> for HtmlImg<E> {
    fn new() -> Self{
        return HtmlImg {
            class: Vec::new(),
            sub_elements: Vec::new(),
            id: None,
            src: None,
            alt: None,
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
        write!(f, "{}<img class={} src={} alt={}>\n", 
            space_pad(indent), 
            to_class_string(self.class),
            to_property_string(self.src),
            to_property_string(self.alt)).unwrap();
        
    }
}