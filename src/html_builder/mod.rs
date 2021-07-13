use std::{
    fmt::{Debug,Formatter,Error},
    iter::FromIterator,
};

trait HtmlElement<E:HtmlElement<E>> : Debug {
    fn new() -> Self;
    fn add_class(self, c: CssClass) -> Self;
    fn set_id(self, id: &str) -> Self;
    fn add_element(self, e: E) -> Self;
    fn render(self,indent: usize, f: &mut Formatter<'_>);
}

#[derive(Debug)]
struct CssClass {
    name: String,
}

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
        write!(f, "{}<ul>\n", space_pad(indent)).unwrap();
        for e in self.sub_elements {
            e.render(indent+1,f);
        }
        write!(f, "{}</ul>\n", space_pad(indent)).unwrap();
    }
}

fn space_pad(spaces: usize) -> String {
        String::from_iter((0..spaces).map(|_|" "))
}

// impl <E:HtmlElement<E>> Display for HtmlUl<E> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
//         write!(f, "{}<ul>\n", space_pad())
//     }
// }